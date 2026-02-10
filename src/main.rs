use anyhow::Result;

mod cli;
mod io; 
mod core;

fn main() -> Result<()> {
    cli::run()
}
