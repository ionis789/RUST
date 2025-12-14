use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

// CLI structure - parseaza argumentele din command line
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Comanda de split: ./app split <file> -s 10mb
    Split {
        file: String,
        #[arg(short, long)]
        size: String,
    },
    // Comanda de unsplit: ./app unsplit <file>
    Unsplit {
        file: String,
    },
}

// Structura pentru manifest file (.json)
// Asta tine minte ordinea bucatilor si hash-urile lor
#[derive(Serialize, Deserialize, Debug)]
struct FileManifest {
    original_filename: String,
    total_size: u64,
    parts: Vec<PartInfo>,
}

// Info despre fiecare chunk
#[derive(Serialize, Deserialize, Debug)]
struct PartInfo {
    filename: String,
    hash: String, // SHA256 hash for integrity check
}

fn main() -> Result<()> {
    // Parse arguments
    let cli = Cli::parse();

    match cli.command {
        Commands::Split { file, size } => {
            println!("Apelam functia de split pentru: {} cu size: {}", file, size);
            // split_file(&file, &size)?; 
        }
        Commands::Unsplit { file } => {
            println!("Apelam functia de unsplit pentru: {}", file);
            // unsplit_file(&file)?; 
        }
    }

    Ok(())
}