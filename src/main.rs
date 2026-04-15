mod cli;
mod client;
mod config;
mod paginator;

use clap::Parser;
use cli::Cli;

fn main() {
    let _cli = Cli::parse();
    println!("Parsed CLI successfully");
}
