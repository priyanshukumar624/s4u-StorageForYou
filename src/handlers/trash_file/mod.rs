use actix_web::{post, web, HttpResponse, Responder};
use std::{collections::HashMap, fs, path::Path, io::Write};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use crate::utils::init::UPLOAD_DIR;

#[derive(Serialize, Deserialize)]
struct TrashMeta(HashMap<String, String>);

#[post("/s4u/users/file/trash")]
pub async fn move_file_to_trash(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let email = query.get("email").map(|s| s.trim()).unwrap_or("");
    let file = query.get("name").map(|s| s.trim()).unwrap_or("");

    if email.is_empty() || file.is_empty() {
        return HttpResponse::BadRequest().body("❌ Required: email and file name");
    }

    let original_path = format!("{}/{}/{}", UPLOAD_DIR, email, file);
    if !Path::new(&original_path).exists() {
        return HttpResponse::BadRequest().body("❌ File does not exist");
    }

    let timestamp = Utc::now().timestamp();
    let trash_folder = format!("{}/{}/trash", UPLOAD_DIR, email);
    let trashed_file_name = format!("{}_{}", file, timestamp);
    let trashed_path = format!("{}/{}", trash_folder, trashed_file_name);
    let trash_meta_path = format!("{}/.trash_meta.json", trash_folder);

    // Create trash folder if not exists
    if let Err(e) = fs::create_dir_all(&trash_folder) {
        return HttpResponse::InternalServerError().body(format!("❌ Could not create trash folder: {}", e));
    }

    // Move file to trash
    if let Err(e) = fs::rename(&original_path, &trashed_path) {
        return HttpResponse::InternalServerError().body(format!("❌ Failed to move file to trash: {}", e));
    }

    // Load or create trash meta file
    let mut meta_map = if Path::new(&trash_meta_path).exists() {
        let content = fs::read_to_string(&trash_meta_path).unwrap_or_else(|_| "{}".into());
        serde_json::from_str::<TrashMeta>(&content).unwrap_or(TrashMeta(HashMap::new())).0
    } else {
        HashMap::new()
    };

    meta_map.insert(trashed_file_name.clone(), file.to_string());

    // Save updated metadata
    if let Ok(json) = serde_json::to_string_pretty(&TrashMeta(meta_map)) {
        if let Err(e) = fs::write(&trash_meta_path, json) {
            return HttpResponse::InternalServerError().body(format!("❌ Failed to write trash metadata: {}", e));
        }
    }

    HttpResponse::Ok().body(format!("✅ File '{}' moved to trash", file))
}
