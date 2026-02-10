use anyhow::Result;
use clap::{Parser, Subcommand};

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
        }

        Commands::Diff { file_a, file_b } => {
            println!("Printing diff between {file_a} and {file_b}");
        }
    }

    Ok(())
}

