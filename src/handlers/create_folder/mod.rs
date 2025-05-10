use actix_web::{post, web, HttpResponse, Responder};
use std::fs;
use std::collections::HashMap;
use crate::utils::init::UPLOAD_DIR;

#[post("/s4u/users/folder/create")]
pub async fn create_folder(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let email = match query.get("email") {
        Some(val) => val,
        None => return HttpResponse::BadRequest().body("Missing email"),
    };
    let folder = match query.get("folder") {
        Some(val) => val,
        None => return HttpResponse::BadRequest().body("Missing folder name"),
    };

    let path = format!("{}/{}/{}", UPLOAD_DIR, email, folder);

    if fs::create_dir_all(&path).is_ok() {
        HttpResponse::Ok().body(format!("✅ Folder '{}' created", folder))
    } else {
        HttpResponse::InternalServerError().body("❌ Failed to create folder")
    }
}
