use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::io::read::load_text;

mod repl;

#[derive(Parser)]
#[command(name = "savebreaker")]
#[command(version)]
#[command(about = "Universal save game analyzer")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Inspect a save file (one-off)
    Inspect { file: String },

    /// Diff two save files (one-off)
    Diff { file_a: String, file_b: String },

    /// Interactive REPL (load once, run many commands)
    Repl,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Inspect { file } => {
            let save = load_text(&file)?;
            println!("Inspecting: {}", save.path.display());
            println!("chars: {}", save.text.len());
            println!("preview: {}", save.preview(80));
        }

        Commands::Diff { file_a, file_b } => {
            let a = load_text(&file_a)?;
            let b = load_text(&file_b)?;
            println!("Diffing:");
            println!("A: {} ({} chars)", a.path.display(), a.text.len());
            println!("B: {} ({} chars)", b.path.display(), b.text.len());
            println!("(diff not implemented yet)");
        }

        Commands::Repl => {
            repl::run_repl()?;
        }
    }

    Ok(())
}

