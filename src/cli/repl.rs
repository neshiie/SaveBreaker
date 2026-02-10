use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::{Result, anyhow};
use colored::*;

use crate::analyze::entropy::shannon_entropy_str;
use crate::core::types::SaveInput;
use crate::io::read::load_text;

pub fn run_repl() -> Result<()> {
    println!("{}", "SaveBreaker REPL".red().bold());
    println!("{}", "Type 'help' for commands.\n".cyan());

    let mut saves: HashMap<String, SaveInput> = HashMap::new();

    loop {
        // prompt
        print!("{}", "!SaveBreaker#> ".red().bold());
        io::stdout().flush()?; // ensure prompt displays

        // read line
        let mut line = String::new();
        let n = io::stdin().read_line(&mut line)?;
        if n == 0 {
            // EOF (Ctrl-D)
            println!();
            break;
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // split into tokens (simple whitespace split)
        let parts: Vec<&str> = line.split_whitespace().collect();
        let cmd = parts[0].to_lowercase();

        match cmd.as_str() {
            "help" => print_help(),

            "exit" | "quit" => break,

            "list" => {
                if saves.is_empty() {
                    println!("(no saves loaded)");
                } else {
                    for (name, s) in &saves {
                        println!("{name}: {} ({} chars)", s.path.display(), s.text.len());
                    }
                }
            }

            "load" => {
                // load <name> <path>
                if parts.len() < 3 {
                    println!("usage: load <name> <path>");
                    continue;
                }
                let name = parts[1].to_string();
                let path = PathBuf::from(parts[2]);

                let save = load_text(&path)?;
                println!(
                    "loaded '{name}' from {} ({} chars)",
                    save.path.display(),
                    save.text.len()
                );

                saves.insert(name, save);
            }

            "peek" => {
                // peek <name>
                if parts.len() < 2 {
                    println!("usage: peek <name>");
                    continue;
                }
                let name = parts[1];

                let save = saves
                    .get(name)
                    .ok_or_else(|| anyhow!("no such save loaded: '{name}'. Try: list"))?;

                println!("{} {name}", "name:".cyan());
                println!("{} {}", "path:".cyan(), save.path.display());
                println!("{} {}", "chars:".cyan(), save.text.len());
                println!("{} {}", "preview:".cyan(), save.preview(120));
            }

            "diff" => {
                // diff <a> <b>
                if parts.len() < 3 {
                    println!("usage: diff <name_a> <name_b>");
                    continue;
                }
                let a = parts[1];
                let b = parts[2];

                let sa = saves.get(a).ok_or_else(|| anyhow!("no such save: '{a}'"))?;
                let sb = saves.get(b).ok_or_else(|| anyhow!("no such save: '{b}'"))?;

                println!("diff (placeholder):");
                println!("  {a}: {} chars", sa.text.len());
                println!("  {b}: {} chars", sb.text.len());
                println!("(we'll implement byte diff next)");
            }

            "entropy" => {
                // entropy <name>
                if parts.len() < 2 {
                    println!("usage: entropy <name>");
                    continue;
                }
                let name = parts[1];

                let save = saves
                    .get(name)
                    .ok_or_else(|| anyhow!("no such save loaded: '{name}'. Try: list"))?;

                let h = shannon_entropy_str(&save.text);
                println!("entropy({name}) = {:.6} bits/byte (0..8)", h);
            }

            _ => {
                println!("unknown command: '{cmd}'. Try 'help'.");
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("commands:");
    println!("  help                 show this help");
    println!("  load <name> <path>   load a save into memory");
    println!("  list                 list loaded saves");
    println!("  inspect <name>       inspect a loaded save");
    println!("  diff <a> <b>         compare two loaded saves (placeholder)");
    println!("  exit | quit          leave the REPL");
    println!("  entropy <name>       compute Shannon entropy (bits/byte)");
}
