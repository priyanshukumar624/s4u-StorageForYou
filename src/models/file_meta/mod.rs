use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FileMeta {
    pub filename: String,
    pub size: u64,
    pub path: String,
}
