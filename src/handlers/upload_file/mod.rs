use actix_web::{post, HttpResponse, Responder, web};
use actix_multipart::Multipart;
use futures::StreamExt;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;
use log::{info, error, warn};
use std::collections::HashMap;
use crate::utils::init::{UPLOAD_DIR, init_upload_dir};
use crate::utils::size_checker::get_folder_size;

const MAX_USER_STORAGE: u64 = 500 * 1024 * 1024; // 500MB in bytes


#[post("s4u/users/file/upload")]
pub async fn upload(
    mut payload: Multipart,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    // ✅ Get email from query parameters
    let email = match query.get("email") {
        Some(e) => e.trim(),
        None => return HttpResponse::BadRequest().body("❌ Missing email query parameter"),
    };

    // ✅ Initialize user-specific upload directory
    init_upload_dir(Some(email));
    let user_dir = format!("{}/{}", UPLOAD_DIR, email);

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
                let filepath = format!("{}/{}", user_dir, filename);

                // ✅ Check folder size before uploading
                let current_size = get_folder_size(&user_dir);
                if current_size >= MAX_USER_STORAGE {
                    return HttpResponse::Forbidden().body("❌ Storage limit (500MB) reached.");
                }

                let mut file = match File::create(Path::new(&filepath)) {
                    Ok(f) => f,
                    Err(e) => {
                        error!("❌ Failed to create file '{}': {}", filepath, e);
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
                                error!("❌ Failed to write to file '{}': {}", filepath, e);
                                return HttpResponse::InternalServerError().body("File upload failed.");
                            }
                        }
                        Err(e) => {
                            error!("❌ Error reading file chunk: {}", e);
                            return HttpResponse::InternalServerError().body("Error reading file.");
                        }
                    }
                }

                info!("✅ File uploaded: {}", filepath);
                return HttpResponse::Ok().body(format!("✅ File uploaded to: {}", filepath));
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
