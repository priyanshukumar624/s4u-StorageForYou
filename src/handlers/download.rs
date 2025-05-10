use actix_web::{get, web, HttpResponse, Responder};
use std::path::Path;
use std::fs;
use actix_web::http::header;
use log::{info, error};
use crate::utils::init::UPLOAD_DIR;

#[get("/files/downloads/{filename}")]
pub async fn download_file(path: web::Path<String>) -> impl Responder {
    let filename = path.into_inner();
    let filepath = format!("{}/{}", UPLOAD_DIR, filename);

    if Path::new(&filepath).exists() {
        info!("✅ File downloaded: {}", filename);
        HttpResponse::Ok()
            .content_type("application/octet-stream")
            .append_header((header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", filename)))
            .body(fs::read(filepath).unwrap())
    } else {
        error!("❌ File not found: {}", filename);
        HttpResponse::NotFound().body("File not found")
    }
}
