use actix_web::{get, web, HttpResponse, Responder};
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

use crate::utils::init::UPLOAD_DIR;

#[get("/s4u/users/search")]
pub async fn search_files_and_folders(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let email = query.get("email").map(|s| s.trim()).unwrap_or("");
    let search_term = query.get("query").map(|s| s.trim()).unwrap_or("");

    if email.is_empty() || search_term.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "❌ Required: email and query parameters"
        }));
    }

    let user_root = format!("{}/{}", UPLOAD_DIR, email);

    if !Path::new(&user_root).exists() {
        return HttpResponse::NotFound().json(serde_json::json!({
            "error": "❌ User folder not found"
        }));
    }

    let mut results = Vec::new();

    for entry in WalkDir::new(&user_root).into_iter().filter_map(Result::ok) {
        let file_name = entry.file_name().to_string_lossy().to_lowercase();
        if file_name.contains(&search_term.to_lowercase()) {
            if let Ok(path) = entry.path().strip_prefix(&user_root) {
                results.push(path.to_string_lossy().to_string());
            }
        }
    }

    if results.is_empty() {
        return HttpResponse::Ok().json(serde_json::json!({
            "message": "🔍 No matching files or folders found"
        }));
    }

    HttpResponse::Ok().json(serde_json::json!({
        "results": results
    }))
}
