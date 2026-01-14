mod cli;
mod error;
mod fs_utils;
mod organizer;
mod undo;

use clap::Parser;
use error::{OrganizerError, Result};
use undo::MoveRecord;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let mut log = Vec::<MoveRecord>::new();

    if cli.undo {
        organizer::undo::undo(&cli.path)?;
        return Ok(());
    }

    let mode = cli.by.ok_or(OrganizerError::InvalidUsage)?;

    match mode {
        cli::Mode::Extension => {
            organizer::by_extension::organize(&cli.path, cli.recursive, cli.dry_run, &mut log)?
        }
        cli::Mode::Date => {
            organizer::by_date::organize(&cli.path, cli.recursive, cli.dry_run, &mut log)?
        }
    }

    if !cli.dry_run {
        undo::save_log(&cli.path, &log)?;
    }

    Ok(())
}
