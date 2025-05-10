use actix_web::{get, HttpResponse, Responder};
use std::fs;
use log::{info, error};
use crate::models::file_meta::FileMeta;
use crate::utils::init::UPLOAD_DIR;
use crate::init_upload_dir;

#[get("/files/all")]
pub async fn list_files() -> impl Responder {
    init_upload_dir();

    let paths = match fs::read_dir(UPLOAD_DIR) {
        Ok(entries) => entries,
        Err(e) => {
            error!("❌ Error reading upload directory: {}", e);
            return HttpResponse::InternalServerError().body("Failed to read upload directory.");
        }
    };

    let file_meta: Vec<FileMeta> = paths
        .filter_map(|entry| entry.ok())
        .map(|entry| {
            let path = entry.path();
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            let size = path.metadata().unwrap().len();
            FileMeta {
                filename,
                size,
                path: path.to_string_lossy().to_string(),
            }
        })
        .collect();

    info!("✅ List of files returned successfully.");
    HttpResponse::Ok().json(file_meta)
}
