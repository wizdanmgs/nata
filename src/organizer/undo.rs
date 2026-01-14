use crate::error::Result;
use crate::undo::{clear_log, load_log};
use std::fs;
use std::path::Path;

pub fn undo(base: &Path) -> Result<()> {
    let log = load_log(base)?;

    // Restore files (reversed order)
    for m in log.moves.iter().rev() {
        if m.to.exists() {
            fs::rename(&m.to, &m.from)?;
        }
    }

    // Remove directories we created (deepest first)
    let mut dirs = log.created_dirs.clone();
    dirs.sort_by_key(|b| std::cmp::Reverse(b.components().count()));

    for dir in dirs {
        if dir.exists() && dir.read_dir()?.next().is_none() {
            fs::remove_dir(dir)?;
        }
    }

    clear_log(base)?;
    Ok(())
}
