use actix_web::{get, web, HttpResponse, Responder};
use std::fs;
use std::collections::HashMap;
use log::{info, error};
use crate::models::file_meta::FileMeta;
use crate::utils::init::{UPLOAD_DIR, init_upload_dir};

#[get("s4u/users/files/retrieve-all")]
pub async fn list_files(query: web::Query<HashMap<String, String>>) -> impl Responder {
    // ✅ Get email from query parameters
    let email = match query.get("email") {
        Some(e) => e.trim(),
        None => return HttpResponse::BadRequest().body("❌ Missing email query parameter"),
    };

    // ✅ Ensure user's upload directory is initialized
    init_upload_dir(Some(email));
    let user_dir = format!("{}/{}", UPLOAD_DIR, email);

    // ✅ Read directory entries for that user
    let paths = match fs::read_dir(&user_dir) {
        Ok(entries) => entries,
        Err(e) => {
            error!("❌ Error reading upload directory for {}: {}", email, e);
            return HttpResponse::InternalServerError().body("Failed to read user's upload directory.");
        }
    };

    // ✅ Collect metadata
    let file_meta: Vec<FileMeta> = paths
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            let filename = path.file_name()?.to_string_lossy().to_string();
            let size = path.metadata().ok()?.len();
            Some(FileMeta {
                filename,
                size,
                path: path.to_string_lossy().to_string(),
            })
        })
        .collect();

    info!("✅ File list returned for user '{}'", email);
    HttpResponse::Ok().json(file_meta)
}
