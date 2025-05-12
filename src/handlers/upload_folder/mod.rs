use actix_web::{post, web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::StreamExt;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use log::{info, error, warn};
use std::collections::HashMap;
use crate::utils::init::{UPLOAD_DIR, init_upload_dir};
use crate::utils::size_checker::get_folder_size;

const MAX_USER_STORAGE: u64 = 500 * 1024 * 1024; // 500MB
// const MAX_USER_STORAGE: u64 = 1 * 1024 * 1024; // 1MB in bytes


#[post("s4u/users/folder/upload")]
pub async fn upload_folder(
    mut payload: Multipart,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let email = match query.get("email") {
        Some(e) => e.trim(),
        None => return HttpResponse::BadRequest().body("❌ Missing email query parameter"),
    };

    let subfolder = query.get("subfolder").map(|s| s.trim()).unwrap_or("");

    // Initialize user-specific upload directory
    init_upload_dir(Some(email));
    let mut user_base_dir = PathBuf::from(UPLOAD_DIR);
    user_base_dir.push(email);

    // If subfolder is specified, add it to the path
    if !subfolder.is_empty() {
        user_base_dir.push(subfolder);
    }

    // Create full path if not exists
    if let Err(e) = create_dir_all(&user_base_dir) {
        error!("❌ Failed to create directory: {:?}", e);
        return HttpResponse::InternalServerError().body("Failed to create user directory");
    }

    while let Some(field) = payload.next().await {
        match field {
            Ok(mut field) => {
                let content_disposition = field.content_disposition();
                let original_filename = match content_disposition.get_filename() {
                    Some(name) => name,
                    None => return HttpResponse::BadRequest().body("❌ Missing filename."),
                };

                let extension = Path::new(original_filename)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("bin");

                let filename = format!("{}.{}", Uuid::new_v4(), extension);
                let filepath = user_base_dir.join(&filename);

                let current_size = get_folder_size(&user_base_dir.to_string_lossy());
                if current_size >= MAX_USER_STORAGE {
                    return HttpResponse::Forbidden().body("❌ Storage limit (500MB) reached.");
                }

                let mut file = match File::create(&filepath) {
                    Ok(f) => f,
                    Err(e) => {
                        error!("❌ Failed to create file '{}': {}", filepath.display(), e);
                        return HttpResponse::InternalServerError().body("File creation failed.");
                    }
                };

                let mut written_size = 0;

                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(data) => {
                            written_size += data.len() as u64;
                            if current_size + written_size > MAX_USER_STORAGE {
                                let _ = std::fs::remove_file(&filepath);
                                return HttpResponse::Forbidden().body("❌ Upload exceeds 500MB limit.");
                            }

                            if let Err(e) = file.write_all(&data) {
                                error!("❌ Failed to write to file '{}': {}", filepath.display(), e);
                                return HttpResponse::InternalServerError().body("File upload failed.");
                            }
                        }
                        Err(e) => {
                            error!("❌ Error reading file chunk: {}", e);
                            return HttpResponse::InternalServerError().body("Error reading file.");
                        }
                    }
                }

                info!("✅ File uploaded: {}", filepath.display());
                return HttpResponse::Ok().body(format!("✅ File uploaded to: {}", filepath.display()));
            }
            Err(e) => {
                error!("❌ Error processing field: {}", e);
                return HttpResponse::BadRequest().body("Error processing the file.");
            }
        }
    }

    warn!("⚠️ No file uploaded.");
    HttpResponse::BadRequest().body("⚠️ No file uploaded.")
}
