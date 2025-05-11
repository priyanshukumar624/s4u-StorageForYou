// src/handlers/share_folder.rs
use actix_web::{post, web, HttpResponse, Responder};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::utils::init::UPLOAD_DIR;

#[post("/s4u/users/folder/share")]
pub async fn share_folder(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let from = query.get("from").map(|s| s.trim()).unwrap_or("");
    let to = query.get("to").map(|s| s.trim()).unwrap_or("");
    let folder = query.get("folder").map(|s| s.trim()).unwrap_or("");

    if from.is_empty() || to.is_empty() || folder.is_empty() {
        return HttpResponse::BadRequest().body("❌ Missing required query parameters.");
    }

    let source = format!("{}/{}/{}", UPLOAD_DIR, from, folder);
    let target = format!("{}/{}/shared_from_{}_{}", UPLOAD_DIR, to, from, folder);

    if !Path::new(&source).exists() {
        return HttpResponse::BadRequest().body("❌ Source folder does not exist.");
    }

    match fs::create_dir_all(Path::new(&target).parent().unwrap_or_else(|| Path::new(UPLOAD_DIR))) {
        Ok(_) => (),
        Err(e) => return HttpResponse::InternalServerError().body(format!("❌ Failed to prepare target: {}", e)),
    }

    match fs_extra::dir::copy(
        &source,
        Path::new(&target).parent().unwrap(),
        &fs_extra::dir::CopyOptions::new().copy_inside(true)
    ) {
        Ok(_) => HttpResponse::Ok().body("✅ Folder shared successfully."),
        Err(e) => HttpResponse::InternalServerError().body(format!("❌ Failed to share folder: {}", e)),
    }
}
