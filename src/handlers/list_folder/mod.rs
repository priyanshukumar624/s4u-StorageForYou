use actix_web::{get, web, HttpResponse, Responder};
use std::collections::HashMap;
use std::fs;
use crate::utils::init::UPLOAD_DIR;

#[get("/s4u/users/folder/retrieve-all-list")]
pub async fn list_folders(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let email = match query.get("email") {
        Some(val) => val,
        None => return HttpResponse::BadRequest().body("Missing email"),
    };

    let user_path = format!("{}/{}", UPLOAD_DIR, email);

    match fs::read_dir(&user_path) {
        Ok(entries) => {
            let mut folders = Vec::new();
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(folder_name) = path.file_name().and_then(|n| n.to_str()) {
                        folders.push(folder_name.to_string());
                    }
                }
            }

            HttpResponse::Ok().json(folders) // returns JSON array
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("❌ Failed to list folders: {}", e)),
    }
}
