use anyhow::Result;
use clap::{Parser, Subcommand};
use crate::io::read::read_text_trimmed;

#[derive(Parser)]
#[command(name = "savebreaker")]
pub struct Cli{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// print contents of file
    Inspect {
        file: String,
    },

    /// diff between two files
    Diff {
        file_a: String,
        file_b: String,
    },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Inspect { file } => {
            println!("Inspecting: {file}");
            
            let s = read_text_trimmed(&file)?;
            println!("Chars: {}", s.len());
            println!("First 80: {}", &s[..s.len().min(80)]);
        }

        Commands::Diff { file_a, file_b } => {
            println!("Printing diff between {file_a} and {file_b}");
        }
    }

    Ok(())
}

