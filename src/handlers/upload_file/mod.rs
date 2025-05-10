use actix_web::{post, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::StreamExt;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;
use log::{info, error, warn};
use crate::utils::init::UPLOAD_DIR;
use crate::init_upload_dir;
use actix_web::web;
use std::collections::HashMap;
 use std::fs;


#[post("s4u/file/upload")]
pub async fn upload(
    mut payload: Multipart,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    use crate::utils::init::init_upload_dir;

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
                let filename = Uuid::new_v4().to_string() + ".png";
                let filepath = format!("{}/{}", user_dir, filename);

                let mut file = match File::create(Path::new(&filepath)) {
                    Ok(f) => f,
                    Err(e) => {
                        error!("❌ Failed to create file '{}': {}", filepath, e);
                        return HttpResponse::InternalServerError().body("File creation failed.");
                    }
                };

                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(data) => {
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

                info!("✅ File uploaded successfully: {}", filepath);
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
