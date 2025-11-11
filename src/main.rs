use clap::CommandFactory;
use clap::Parser;
use std::fs;

use cow_parser::{get_commands, parse_cow_source};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(name = "FILE")]
    file_path: Option<String>,
    #[arg(long)]
    credits: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.credits {
        println!("Cow Parser created by Lapko Dmytro.");
        println!("Built with Rust and the `excellent` `pest` parser toolkit.");
        return;
    }

    if let Some(path) = cli.file_path {
        let source = fs::read_to_string(&path).expect("ERROR: Could not read file");

        let commands = get_commands(&source).expect("ERROR: parsin of .cow is unsuccesfull");

        println!("Succesfully prased .cow file");
        println!("{:?}", commands);
    } else {
        let _ = Cli::command().print_help();
    }
}
