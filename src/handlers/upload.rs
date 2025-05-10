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

#[post("/file/upload")]
pub async fn upload(mut payload: Multipart) -> impl Responder {
    init_upload_dir();

    while let Some(field) = payload.next().await {
        match field {
            Ok(mut field) => {
                let filename = Uuid::new_v4().to_string() + ".png";
                let filepath = format!("{}/{}", UPLOAD_DIR, filename);

                let mut file = match File::create(Path::new(&filepath)) {
                    Ok(f) => f,
                    Err(e) => {
                        error!("❌ Failed to create file: {}", e);
                        return HttpResponse::InternalServerError().body("File creation failed.");
                    }
                };

                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(data) => {
                            if let Err(e) = file.write_all(&data) {
                                error!("❌ Failed to write data to file: {}", e);
                                return HttpResponse::InternalServerError().body("File upload failed.");
                            }
                        }
                        Err(e) => {
                            error!("❌ Error reading file chunk: {}", e);
                            return HttpResponse::InternalServerError().body("Error reading file.");
                        }
                    }
                }

                info!("✅ File uploaded successfully: {}", filename);
                return HttpResponse::Ok().body(format!("File uploaded successfully: {}", filename));
            }
            Err(e) => {
                error!("❌ Error processing the file: {}", e);
                return HttpResponse::BadRequest().body("Error processing the file.");
            }
        }
    }

    warn!("⚠️ No file uploaded.");
    HttpResponse::BadRequest().body("No file uploaded.")
}
