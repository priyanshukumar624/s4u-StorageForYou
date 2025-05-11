use serde::Serialize;

#[derive(Serialize)]
pub struct FileDetail {
    pub name: String,
    pub size_kb: u64,
    pub path: String,
    pub is_folder: bool,
}

#[derive(Serialize)]
pub struct StorageStatus {
    pub used_mb: f64,
    pub remaining_mb: f64,
    pub file_count: usize,
    pub folder_count: usize,
    pub items: Vec<FileDetail>,
}
