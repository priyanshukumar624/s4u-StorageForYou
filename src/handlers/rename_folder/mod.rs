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

    println!("🔍 Checking if old folder exists at: {}", old_path);

    if !Path::new(&old_path).exists() {
        println!("🚫 Old folder does not exist at: {}", old_path);
        return HttpResponse::BadRequest().body("❌ Old folder does not exist.");
    }

    println!("🔍 Checking if new folder exists at: {}", new_path);
    if Path::new(&new_path).exists() {
        println!("🚫 New folder already exists at: {}", new_path);
        return HttpResponse::BadRequest().body("❌ New folder name already exists.");
    }

    match fs::rename(&old_path, &new_path) {
        Ok(_) => {
            println!("✅ Folder successfully renamed from '{}' to '{}'", old, new);
            HttpResponse::Ok().body(format!("✅ Folder renamed to '{}'", new))
        },
        Err(e) => {
            println!("🔥 ❌ Rename failed from '{}' to '{}': {}", old, new, e);
            HttpResponse::InternalServerError().body(format!("❌ Rename failed: {}", e))
        },
    }
}
