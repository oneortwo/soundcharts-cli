mod cli;
mod client;
mod config;
mod identifier;
mod output;
mod paginator;

use clap::Parser;
use cli::Cli;

fn main() {
    let _cli = Cli::parse();
    println!("Parsed CLI successfully");
}
