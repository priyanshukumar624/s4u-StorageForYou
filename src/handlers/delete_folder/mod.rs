use actix_web::{delete, web, HttpResponse, Responder};
use std::collections::HashMap;
use crate::utils::init::UPLOAD_DIR;
use std::fs;
use log::{error};

#[delete("/s4u/users/folder/delete")]
pub async fn delete_folder(query: web::Query<HashMap<String, String>>) -> impl Responder {
    // 🔍 Extract email
    let email = match query.get("email") {
        Some(val) => val,
        None => {
            error!("⚠️ Missing 'email' query parameter in request ❌");
            return HttpResponse::BadRequest().body("❌ Missing email");
        },
    };

    // 🔍 Extract folder
    let folder = match query.get("folder") {
        Some(val) => val,
        None => {
            error!("⚠️ Missing 'folder' query parameter in request ❌");
            return HttpResponse::BadRequest().body("❌ Missing folder name");
        },
    };

    let path = format!("{}/{}/{}", UPLOAD_DIR, email, folder);
    println!("📁 Attempting to delete folder at path: {}", path);

    match fs::remove_dir_all(&path) {
        Ok(_) => {
            println!("🗑️ ✅ Folder successfully deleted: {}", path);
            HttpResponse::Ok().body("✅ Folder deleted")
        },
        Err(e) => {
            error!("🔥 ❌ Failed to delete folder '{}': {}", path, e);
            HttpResponse::InternalServerError().body(format!("❌ Delete failed: {}", e))
        },
    }
}
