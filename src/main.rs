use clap::{Parser, Subcommand};

use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    id: u64,
    name: String,
}

pub fn read_records() -> Result<Vec<Record>, std::io::Error> {
    let data = fs::read_to_string("data.json")?;
    let records: Vec<Record> = serde_json::from_str(&data)?;

    Ok(records)
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            println!("List records")
        }
    }
}
