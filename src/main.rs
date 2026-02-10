use anyhow::Result;

mod analyze;
mod cli;
mod core;
mod io;

fn main() -> Result<()> {
    cli::run()
}
