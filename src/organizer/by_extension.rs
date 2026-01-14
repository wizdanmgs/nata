use crate::error::Result;
use crate::fs_utils::list_files;
use crate::organizer::move_file::move_file;
use crate::undo::MoveRecord;
use std::fs;
use std::path::Path;

pub fn organize(
    dir: &Path,
    recursive: bool,
    dry_run: bool,
    log: &mut Vec<MoveRecord>,
) -> Result<()> {
    for file in list_files(dir, recursive)? {
        let ext = file
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("unknown");

        let target_dir = dir.join(ext);
        fs::create_dir_all(&target_dir)?;

        let target_path = target_dir.join(file.file_name().unwrap());
        move_file(file, target_path, dry_run, log)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn organizes_files_by_extension() {
        let dir = tempdir().unwrap();
        let mut log = Vec::<MoveRecord>::new();
        File::create(dir.path().join("a.txt")).unwrap();
        File::create(dir.path().join("b.jpg")).unwrap();

        organize(dir.path(), false, false, &mut log).unwrap();

        assert!(dir.path().join("txt/a.txt").exists());
        assert!(dir.path().join("jpg/b.jpg").exists());
    }
}
