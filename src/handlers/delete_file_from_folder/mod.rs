use actix_web::{post, web, HttpResponse, Responder};
use std::{collections::HashMap, fs, path::Path};
use crate::utils::init::UPLOAD_DIR;

#[post("/s4u/users/file/delete")]
pub async fn del_file_from_folder(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let email = query.get("email").map(|s| s.trim()).unwrap_or("");
    let relative_path = query.get("path").map(|s| s.trim()).unwrap_or("");

    if email.is_empty() || relative_path.is_empty() {
        return HttpResponse::BadRequest().body("❌ Required: email and path");
    }

    // Final path: UPLOAD_DIR/email/suraj/a.txt
    let full_path = format!("{}/{}/{}", UPLOAD_DIR, email, relative_path);

    if !Path::new(&full_path).exists() {
        return HttpResponse::BadRequest().body("❌ File does not exist");
    }

    match fs::remove_file(&full_path) {
        Ok(_) => HttpResponse::Ok().body(format!("✅ File '{}' deleted", relative_path)),
        Err(e) => HttpResponse::InternalServerError().body(format!("❌ Failed to delete file: {}", e)),
    }
}
