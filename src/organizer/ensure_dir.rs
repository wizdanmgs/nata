use crate::error::Result;
use std::fs;
use std::path::PathBuf;

pub fn ensure_dir(dir: &PathBuf, dry_run: bool, created_dirs: &mut Vec<PathBuf>) -> Result<()> {
    if dry_run {
        println!("[dry-run] creating directory: {}", dir.display());
        return Ok(());
    }

    if !dir.exists() {
        fs::create_dir_all(dir)?;
        created_dirs.push(dir.clone());
    }

    Ok(())
}
