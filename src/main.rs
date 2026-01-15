mod cli;
mod error;
mod fs_utils;
mod organizer;
mod undo;

use clap::Parser;
use error::Result;
use undo::UndoLog;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let mut log = UndoLog {
        moves: Vec::new(),
        created_dirs: Vec::new(),
    };

    if cli.undo {
        organizer::undo::undo(&cli.path)?;
        return Ok(());
    }

    let mode = match cli.by {
        Some(m) => m,
        None => {
            eprintln!();
            eprintln!("ERROR: Missing required option --by");
            eprintln!();
            eprintln!("Hint:");
            eprintln!("  organize <path> --by <extension|date>");
            eprintln!("  organize <path> --by extension --dry-run");
            eprintln!("  organize <path> --undo");
            eprintln!();
            std::process::exit(2);
        }
    };

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
    } else {
        println!();
        println!("Dry-run complete.");
        println!("Run again without --dry-run to apply these changes.");
    }

    Ok(())
}
