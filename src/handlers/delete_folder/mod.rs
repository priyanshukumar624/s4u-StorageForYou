use actix_web::{delete, web, HttpResponse, Responder};
use std::collections::HashMap;
use crate::utils::init::UPLOAD_DIR;
use std::fs;

#[delete("/s4u/users/folder/delete")]
pub async fn delete_folder(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let email = match query.get("email") {
        Some(val) => val,
        None => return HttpResponse::BadRequest().body("Missing email"),
    };
    let folder = match query.get("folder") {
        Some(val) => val,
        None => return HttpResponse::BadRequest().body("Missing folder name"),
    };

    let path = format!("{}/{}/{}", UPLOAD_DIR, email, folder);

    match fs::remove_dir_all(&path) {
        Ok(_) => HttpResponse::Ok().body("✅ Folder deleted"),
        Err(e) => HttpResponse::InternalServerError().body(format!("❌ Delete failed: {}", e)),
    }
}
