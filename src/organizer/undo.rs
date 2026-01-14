use crate::error::Result;
use crate::undo::{clear_log, load_log};
use std::fs;
use std::path::Path;

pub fn undo(base: &Path) -> Result<()> {
    let records = load_log(base)?;

    for r in records.iter().rev() {
        fs::rename(&r.to, &r.from)?;
    }

    clear_log(base)?;
    Ok(())
}
