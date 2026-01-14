use crate::error::Result;
use crate::fs_utils::list_files;
use crate::organizer::ensure_dir::ensure_dir;
use crate::organizer::move_file::move_file;
use crate::undo::UndoLog;
use chrono::{DateTime, Local};
use std::fs;
use std::path::Path;

pub fn organize(dir: &Path, recursive: bool, dry_run: bool, log: &mut UndoLog) -> Result<()> {
    for file in list_files(dir, recursive)? {
        let metadata = fs::metadata(&file)?;
        let modified = metadata.modified()?;

        let datetime: DateTime<Local> = modified.into();
        let folder_name = datetime.format("%Y-%m").to_string();

        let target_dir = dir.join(folder_name);
        ensure_dir(&target_dir, dry_run, &mut log.created_dirs)?;

        let target_path = target_dir.join(file.file_name().unwrap());
        move_file(file, target_path, dry_run, &mut log.moves)?;
    }

    Ok(())
}
