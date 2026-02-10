use anyhow::Result;
mod cli;
mod io; 

fn main() -> Result<()> {
    cli::run()
}
