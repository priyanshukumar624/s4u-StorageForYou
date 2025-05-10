use actix_web::{delete, web, HttpResponse, Responder};
use std::fs;
use std::path::Path;
use crate::utils::init::UPLOAD_DIR;
use log::{info, error};
use std::collections::HashMap;

#[delete("s4u/files/delete/{filename}")]
pub async fn delete_file(
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
        if let Err(e) = fs::remove_file(&filepath) {
            error!("❌ Failed to delete file: {}", e);
            return HttpResponse::InternalServerError().body("Failed to delete file.");
        }
        info!("✅ File deleted successfully: {}", filepath);
        HttpResponse::Ok().body("✅ File deleted successfully.")
    } else {
        error!("❌ File not found: {}", filepath);
        HttpResponse::NotFound().body("❌ File not found.")
    }
}
