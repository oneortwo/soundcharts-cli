mod cli;
mod client;
mod commands;
mod config;
mod identifier;
mod models;
mod output;
mod paginator;

use clap::Parser;
use cli::Cli;

fn main() {
    let _cli = Cli::parse();
    println!("Parsed CLI successfully");
}
