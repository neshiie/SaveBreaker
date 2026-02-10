use anyhow::Result;

mod cli;
mod io; 
mod core;
mod analyze; 

fn main() -> Result<()> {
    cli::run()
}
