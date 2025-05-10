use std::fs;
use std::path::Path;
use log::{error, info};

pub const UPLOAD_DIR: &str = "./uploads/";

/// Initialize the base upload directory or a user-specific subdirectory
pub fn init_upload_dir(sub_dir: Option<&str>) {
    let dir_path = match sub_dir {
        Some(sub) => format!("{}{}", UPLOAD_DIR, sub),
        None => UPLOAD_DIR.to_string(),
    };

    let path = Path::new(&dir_path);

    if !path.exists() {
        match fs::create_dir_all(path) {
            Ok(_) => info!("✅ Created directory: {}", dir_path),
            Err(e) => error!("❌ Failed to create directory {}: {}", dir_path, e),
        }
    } else {
        info!("📁 Directory already exists: {}", dir_path);
    }
}
