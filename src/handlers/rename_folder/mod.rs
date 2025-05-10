use std::fs;
use std::path::Path;
use actix_web::{put, web, HttpResponse, Responder};
use std::collections::HashMap;
use crate::utils::init::UPLOAD_DIR;

#[put("/s4u/users/folder/rename")]
pub async fn rename_folder(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let email = query.get("email").map(|s| s.trim()).unwrap_or("");
    let old = query.get("old").map(|s| s.trim()).unwrap_or("");
    let new = query.get("new").map(|s| s.trim()).unwrap_or("");

    if email.is_empty() || old.is_empty() || new.is_empty() {
        return HttpResponse::BadRequest().body("❌ Missing required query parameters.");
    }

    let old_path = format!("{}/{}/{}", UPLOAD_DIR, email, old);
    let new_path = format!("{}/{}/{}", UPLOAD_DIR, email, new);

    if !Path::new(&old_path).exists() {
        return HttpResponse::BadRequest().body("❌ Old folder does not exist.");
    }

    if Path::new(&new_path).exists() {
        return HttpResponse::BadRequest().body("❌ New folder name already exists.");
    }

    match fs::rename(&old_path, &new_path) {
        Ok(_) => HttpResponse::Ok().body(format!("✅ Folder renamed to '{}'", new)),
        Err(e) => HttpResponse::InternalServerError().body(format!("❌ Rename failed: {}", e)),
    }
}
