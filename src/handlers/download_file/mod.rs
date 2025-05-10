use actix_web::{get, web, HttpResponse, Responder};
use std::path::Path;
use std::fs;
use actix_web::http::header;
use log::{info, error};
use crate::utils::init::UPLOAD_DIR;
use std::collections::HashMap;

#[get("s4u/users/files/downloads/{filename}")]
pub async fn download_file(
    path: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let filename = path.into_inner();

    // ✅ Get email from query parameters
    let email = match query.get("email") {
        Some(e) => e.trim(),
        None => return HttpResponse::BadRequest().body("❌ Missing email query parameter"),
    };

    let filepath = format!("{}/{}/{}", UPLOAD_DIR, email, filename);

    if Path::new(&filepath).exists() {
        match fs::read(&filepath) {
            Ok(file_data) => {
                info!("✅ File downloaded: {}", filepath);
                HttpResponse::Ok()
                    .content_type("application/octet-stream")
                    .append_header((header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", filename)))
                    .body(file_data)
            }
            Err(e) => {
                error!("❌ Failed to read file: {}", e);
                HttpResponse::InternalServerError().body("Failed to read file")
            }
        }
    } else {
        error!("❌ File not found: {}", filepath);
        HttpResponse::NotFound().body("File not found")
    }
}
