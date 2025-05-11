use std::fs;
use std::path::Path;

/// Recursively get the total size of a folder in bytes.
pub fn get_folder_size(path: &str) -> u64 {
    fn dir_size(path: &Path) -> u64 {
        if path.is_file() {
            fs::metadata(path).map(|m| m.len()).unwrap_or(0)
        } else if path.is_dir() {
            fs::read_dir(path)
                .map(|entries| {
                    entries
                        .filter_map(Result::ok)
                        .map(|entry| dir_size(&entry.path()))
                        .sum()
                })
                .unwrap_or(0)
        } else {
            0
        }
    }

    dir_size(Path::new(path))
}
