use actix_web::{post, web, HttpResponse, Responder};
use std::fs;
use std::collections::HashMap;
use crate::utils::init::UPLOAD_DIR;

#[post("/s4u/users/folder/create")]
pub async fn create_folder(query: web::Query<HashMap<String, String>>) -> impl Responder {
    // Extract parameters with logging
    let email = match query.get("email") {
        Some(val) => {
            println!("📧 Received folder creation request for email: {}", val);
            val
        },
        None => {
            println!("❌ Missing email parameter in request");
            return HttpResponse::BadRequest().body("Missing email");
        },
    };

    let folder = match query.get("folder") {
        Some(val) => {
            println!("📁 Requested folder name: {}", val);
            val
        },
        None => {
            println!("❌ Missing folder parameter in request");
            return HttpResponse::BadRequest().body("Missing folder name");
        },
    };

    let path = format!("{}/{}/{}", UPLOAD_DIR, email, folder);
    println!("🛠️ Attempting to create folder at path: {}", path);

    match fs::create_dir_all(&path) {
        Ok(_) => {
            println!("✅ Successfully created folder '{}' for user '{}'", folder, email);
            HttpResponse::Ok().body(format!("✅ Folder '{}' created", folder))
        },
        Err(e) => {
            println!("🔥 Failed to create folder '{}': {}", folder, e);
            HttpResponse::InternalServerError().body(format!("❌ Failed to create folder: {}", e))
        }
    }
}