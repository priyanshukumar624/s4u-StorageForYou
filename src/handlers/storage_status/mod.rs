use actix_web::{get, web, HttpResponse, Responder};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::models::storage_status::{FileDetail, StorageStatus};
use crate::utils::init::UPLOAD_DIR;

#[get("/s4u/users/storage/status")]
pub async fn storage_status(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let email = match query.get("email") {
        Some(e) => e.trim(),
        None => return HttpResponse::BadRequest().body("❌ Missing email query parameter"),
    };

    let user_dir = format!("{}/{}", UPLOAD_DIR, email);
    let path = Path::new(&user_dir);

    if !path.exists() {
        return HttpResponse::NotFound().body("❌ User directory not found");
    }

    let mut total_size = 0u64;
    let mut file_count = 0;
    let mut folder_count = 0;
    let mut items = Vec::new();

    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            let name = entry_path.file_name().unwrap().to_string_lossy().to_string();

            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    file_count += 1;
                    let size = metadata.len();
                    total_size += size;

                    items.push(FileDetail {
                        name,
                        size_kb: size / 1024,
                        path: entry_path.to_string_lossy().to_string(),
                        is_folder: false,
                    });
                } else if metadata.is_dir() {
                    folder_count += 1;
                    let size = get_folder_size(&entry_path);
                    total_size += size;

                    items.push(FileDetail {
                        name,
                        size_kb: size / 1024,
                        path: entry_path.to_string_lossy().to_string(),
                        is_folder: true,
                    });
                }
            }
        }
    }

    let used_mb = total_size as f64 / (1024.0 * 1024.0);
    let remaining_mb = (500.0 - used_mb).max(0.0);

    let status = StorageStatus {
        used_mb: (used_mb * 100.0).round() / 100.0,
        remaining_mb: (remaining_mb * 100.0).round() / 100.0,
        file_count,
        folder_count,
        items,
    };

    HttpResponse::Ok().json(status)
}

fn get_folder_size(path: &Path) -> u64 {
    let mut size = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    size += metadata.len();
                } else if metadata.is_dir() {
                    size += get_folder_size(&entry_path);
                }
            }
        }
    }

    size
}
