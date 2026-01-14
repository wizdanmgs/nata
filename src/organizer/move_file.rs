use crate::error::Result;
use crate::undo::MoveRecord;
use std::fs;
use std::path::PathBuf;

pub fn move_file(
    from: PathBuf,
    to: PathBuf,
    dry_run: bool,
    log: &mut Vec<MoveRecord>,
) -> Result<()> {
    if dry_run {
        println!("[dry-run] {:?} -> {:?}", from, to);
    } else {
        fs::create_dir_all(to.parent().unwrap())?;
        fs::rename(&from, &to)?;
        log.push(MoveRecord { from, to })
    }
    Ok(())
}
