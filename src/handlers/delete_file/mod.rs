use actix_web::{delete, web, HttpResponse, Responder};
use std::fs;
use std::path::Path;
use crate::utils::init::UPLOAD_DIR;
use log::{info, error};

#[delete("s4u/files/delete/{filename}")]
pub async fn delete_file(path: web::Path<String>) -> impl Responder {
    let filename = path.into_inner();
    let filepath = format!("{}/{}", UPLOAD_DIR, filename);

    if Path::new(&filepath).exists() {
        if let Err(e) = fs::remove_file(filepath) {
            error!("❌ Failed to delete file: {}", e);
            return HttpResponse::InternalServerError().body("Failed to delete file.");
        }
        info!("✅ File deleted successfully: {}", filename);
        HttpResponse::Ok().body("File deleted successfully")
    } else {
        error!("❌ File not found: {}", filename);
        HttpResponse::NotFound().body("File not found")
    }
}
