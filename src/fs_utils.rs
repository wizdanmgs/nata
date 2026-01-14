use crate::error::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub fn list_files(dir: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        } else if recursive && path.is_dir() {
            files.extend(list_files(&path, true)?);
        }
    }

    Ok(files)
}
