use actix_web::{post, web, HttpResponse, Responder};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::utils::init::UPLOAD_DIR;

#[post("/s4u/users/file/share")]
pub async fn share_file(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let owner = query.get("owner_email").map(|s| s.trim()).unwrap_or("");
    let receiver = query.get("shared_with").map(|s| s.trim()).unwrap_or("");
    let file = query.get("name").map(|s| s.trim()).unwrap_or("");

    if owner.is_empty() || receiver.is_empty() || file.is_empty() {
        println!("❌ Missing required query parameters");
        return HttpResponse::BadRequest().body("❌ Required: owner_email, shared_with, name");
    }

    let src = format!("{}/{}/{}", UPLOAD_DIR, owner, file);
    let dst_folder = format!("{}/{}/shared_from_{}", UPLOAD_DIR, receiver, owner);
    let dst = format!("{}/{}", dst_folder, file);

    if !Path::new(&src).exists() {
        println!("❌ File not found: {}", src);
        return HttpResponse::BadRequest().body("❌ File does not exist");
    }

    if let Err(e) = fs::create_dir_all(&dst_folder) {
        println!("❌ Failed to create destination folder: {}", e);
        return HttpResponse::InternalServerError().body("❌ Folder creation failed");
    }

    match fs::copy(&src, &dst) {
        Ok(_) => {
            println!("✅ File '{}' shared from '{}' to '{}'", file, owner, receiver);
            HttpResponse::Ok().body(format!("✅ File '{}' shared", file))
        }
        Err(e) => {
            println!("❌ File sharing failed: {}", e);
            HttpResponse::InternalServerError().body("❌ Failed to share file")
        }
    }
}
