use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use cow_parser::{get_commands, optimize_cow_source, parse_cow_source};
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(name = "FILE")]
    file_path: Option<String>,

    #[arg(long, group = "action")]
    credits: bool,

    #[arg(long, group = "action", requires = "FILE")]
    check: bool,

    #[arg(long, group = "action", requires = "FILE")]
    display: bool,

    #[arg(long, group = "action", requires = "FILE")]
    optimize: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.credits {
        println!("Cow Parser created by Lapko Dmytro.");
        println!("Built with Rust and the `excellent` `pest` parser toolkit.");
        return Ok(());
    }

    if let Some(path) = cli.file_path {
        let source = fs::read_to_string(&path)
            .with_context(|| format!("ERROR: Could not read file `{}`", path))?;

        if cli.check {
            match parse_cow_source(&source) {
                Ok(_) => println!("OK: Parsing successful."),
                Err(e) => anyhow::bail!("ERROR: Parsing failed.\nDetails: {}", e),
            }
        } else if cli.display {
            let commands = get_commands(&source)
                .with_context(|| "ERROR: Failed to get commands from source file.")?;
            println!("Commands: {:?}", commands);
        } else if cli.optimize {
            let optimized_commands = optimize_cow_source(&source)
                .with_context(|| "ERROR: Failed to optimize source file.")?;
            println!("Optimized Commands: {:?}", optimized_commands);
        } else {
             let commands = get_commands(&source)
                .with_context(|| "ERROR: Failed to get commands from source file.")?;
            println!("Default action (display): {:?}", commands);
        }
    } else {
        Cli::command().print_help()?;
    }

    Ok(())
}