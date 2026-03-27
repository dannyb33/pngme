mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::{Parser};

use crate::commands::{Commands, encode, decode, remove, print};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[command(name = "pngme")]
#[command(version = "1.0")]
#[command(about = "Png manipulation!", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encode(args) => encode(args),
        Commands::Decode(args) => decode(args),
        Commands::Remove(args) => remove(args),
        Commands::Print(args) => print(args)
    }
    
    Ok(())
}