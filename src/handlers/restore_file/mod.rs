use actix_web::{post, web, HttpResponse, Responder};
use std::{collections::HashMap, fs, path::Path};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::utils::init::UPLOAD_DIR;

#[derive(Serialize, Deserialize)]
struct TrashMeta(HashMap<String, String>);

#[post("/s4u/users/file/restore")]
pub async fn restore_file(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let email = query.get("email").map(|s| s.trim()).unwrap_or("");
    let trashed_name = query.get("name").map(|s| s.trim()).unwrap_or("");

    if email.is_empty() || trashed_name.is_empty() {
        return HttpResponse::BadRequest().body("❌ Required: email and name (trashed name)");
    }

    let trash_folder = format!("{}/{}/trash", UPLOAD_DIR, email);
    let trashed_path = format!("{}/{}", trash_folder, trashed_name);
    let trash_meta_path = format!("{}/.trash_meta.json", trash_folder);

    if !Path::new(&trashed_path).exists() {
        return HttpResponse::BadRequest().body("❌ Trashed file does not exist");
    }

    // Load trash metadata
    let mut meta_map = if Path::new(&trash_meta_path).exists() {
        let content = fs::read_to_string(&trash_meta_path).unwrap_or_else(|_| "{}".into());
        serde_json::from_str::<TrashMeta>(&content).unwrap_or(TrashMeta(HashMap::new())).0
    } else {
        return HttpResponse::InternalServerError().body("❌ Trash metadata not found");
    };

    // Find original filename
    let original_name = match meta_map.get(trashed_name) {
        Some(name) => name.clone(),
        None => return HttpResponse::BadRequest().body("❌ Original filename not found in metadata"),
    };

    let restore_path = format!("{}/{}/{}", UPLOAD_DIR, email, original_name);

    // Move back to original location
    if let Err(e) = fs::rename(&trashed_path, &restore_path) {
        return HttpResponse::InternalServerError().body(format!("❌ Failed to restore file: {}", e));
    }

    // Remove metadata entry
    meta_map.remove(trashed_name);
    if let Ok(json) = serde_json::to_string_pretty(&TrashMeta(meta_map)) {
        if let Err(e) = fs::write(&trash_meta_path, json) {
            return HttpResponse::InternalServerError().body(format!("❌ Failed to update trash metadata: {}", e));
        }
    }

    HttpResponse::Ok().body(format!("✅ File '{}' restored successfully", original_name))
}
