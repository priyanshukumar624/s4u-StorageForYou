use std::fs;
use std::path::Path;
use log::{error, info};

pub const UPLOAD_DIR: &str = "./uploads/";

pub fn init_upload_dir() {
    if !Path::new(UPLOAD_DIR).exists() {
        if let Err(e) = fs::create_dir_all(UPLOAD_DIR) {
            error!("❌ Failed to create upload directory: {}", e);
        } else {
            info!("✅ Upload directory created.");
        }
    }
}
