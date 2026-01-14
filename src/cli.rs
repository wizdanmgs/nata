use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "organize")]
#[command(about = "Organize files by extension or date")]
pub struct Cli {
    pub path: PathBuf,

    #[arg(long, value_enum)]
    pub by: Option<Mode>,

    #[arg(long)]
    pub dry_run: bool,

    #[arg(long)]
    pub recursive: bool,

    #[arg(long)]
    pub undo: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Mode {
    Extension,
    Date,
}
