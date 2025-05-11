use actix_web::{post, web, HttpResponse, Responder};
use std::{collections::HashMap, fs, path::Path};
use crate::utils::init::UPLOAD_DIR;
use crate::models::trash_meta::TrashMeta;

#[post("/s4u/users/folder/restore")]
pub async fn restore_folder(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let email = query.get("email").map(|s| s.trim()).unwrap_or("");
    let trashed_folder = query.get("name").map(|s| s.trim()).unwrap_or("");

    if email.is_empty() || trashed_folder.is_empty() {
        return HttpResponse::BadRequest().body("❌ Required: email and trashed folder name");
    }

    let trash_folder = format!("{}/{}/trash", UPLOAD_DIR, email);
    let trash_meta_path = format!("{}/.trash_meta.json", trash_folder);

    // Load trash metadata
    let meta_map = if Path::new(&trash_meta_path).exists() {
        let content = fs::read_to_string(&trash_meta_path).unwrap_or_else(|_| "{}".into());
        serde_json::from_str::<TrashMeta>(&content)
            .unwrap_or(TrashMeta(HashMap::new()))
            .0
    } else {
        HashMap::new()
    };

    if let Some(original_folder_name) = meta_map.get(trashed_folder) {
        let original_folder_name = original_folder_name.clone(); // ✅ Clone here to avoid borrow issues

        let trashed_path = format!("{}/{}", trash_folder, trashed_folder);
        let original_path = format!("{}/{}/{}", UPLOAD_DIR, email, original_folder_name);

        // Restore the folder
        if let Err(e) = fs::rename(&trashed_path, &original_path) {
            return HttpResponse::InternalServerError().body(format!("❌ Failed to restore folder: {}", e));
        }

        // Remove from metadata and update
        let mut meta_map = meta_map;
        meta_map.remove(trashed_folder);

        if let Ok(json) = serde_json::to_string_pretty(&TrashMeta(meta_map)) {
            if let Err(e) = fs::write(&trash_meta_path, json) {
                return HttpResponse::InternalServerError().body(format!("❌ Failed to update trash metadata: {}", e));
            }
        }

        HttpResponse::Ok().body(format!("✅ Folder '{}' restored", original_folder_name))
    } else {
        HttpResponse::BadRequest().body("❌ Folder not found in trash")
    }
}