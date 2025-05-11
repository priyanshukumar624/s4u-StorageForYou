use actix_web::{get, web, HttpResponse, Responder};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::utils::init::UPLOAD_DIR;

const MAX_STORAGE_BYTES: u64 = 500 * 1024 * 1024;

#[get("/s4u/users/storage/remaining")]
pub async fn remaining_space(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let email = match query.get("email") {
        Some(e) => e.trim(),
        None => return HttpResponse::BadRequest().body("❌ Missing email"),
    };

    let user_dir = format!("{}/{}", UPLOAD_DIR, email);
    let path = Path::new(&user_dir);

    if !path.exists() {
        return HttpResponse::Ok().body("✅ You have 500MB available.");
    }

    let mut total_size = 0u64;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                total_size += metadata.len();
            }
        }
    }

    let remaining = MAX_STORAGE_BYTES.saturating_sub(total_size);
    let remaining_mb = remaining as f64 / (1024.0 * 1024.0);

    HttpResponse::Ok().body(format!("✅ You have {:.2} MB remaining.", remaining_mb))
}
