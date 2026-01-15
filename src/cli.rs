use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "nata",
    version,
    about = "Organize files by extension or date",
    after_help = r#"
EXAMPLES:
  organize ~/Downloads --by extension
  organize ~/Downloads --by date --recursive
  organize ~/Downloads --by extension --dry-run
  organize ~/Downloads --by date --recursive --dry-run
  organize ~/Downloads --undo

TIPS:
  • Always try --dry-run first to preview changes
  • --undo reverts only the LAST successful run
  • --recursive includes subdirectories
"#
)]
pub struct Cli {
    /// Target directory to organize
    pub path: PathBuf,

    /// Organize files by the selected mode
    #[arg(long, value_enum)]
    pub by: Option<Mode>,

    /// Preview changes without touching the filesystem
    #[arg(long)]
    pub dry_run: bool,

    /// Include subdirectories recursively
    #[arg(long)]
    pub recursive: bool,

    /// Undo the last organize operation
    #[arg(long)]
    pub undo: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Mode {
    Extension,
    Date,
}
