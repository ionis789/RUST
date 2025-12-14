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


// --- Helper Functions ---

fn parse_size(s: &str) -> Result<u64> {
    let s = s.to_lowercase();
    // Verificam ce suffix are
    if let Some(stripped) = s.strip_suffix("gb") {
        Ok(stripped.trim().parse::<u64>()? * 1024 * 1024 * 1024)
    } else if let Some(stripped) = s.strip_suffix("mb") {
        Ok(stripped.trim().parse::<u64>()? * 1024 * 1024)
    } else if let Some(stripped) = s.strip_suffix("kb") {
        Ok(stripped.trim().parse::<u64>()? * 1024)
    } else if let Some(stripped) = s.strip_suffix("b") {
        Ok(stripped.trim().parse::<u64>()?)
    } else {
        // Default bytes daca nu a fost introdus tipul 
        Ok(s.trim().parse::<u64>()?)
    }
}

// Functie care calculeaza hash-ul SHA256 (amprenta digitala)
fn calculate_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    // Returnam hash-ul ca string hexazecimal
    hex::encode(hasher.finalize())
}
// --- Functia de SPLIT (Taierea) ---
// Aceasta functie citeste fisierul mare si il imparte in bucati mici
fn split_file(path_str: &str, size_str: &str) -> Result<()> {
    // Calculam marimea chunk-ului in bytes 
    let chunk_size = parse_size(size_str).context("Formatul marimii este invalid")?;
    
    println!("Incepem splituirea fisierului '{}' in bucati de {} bytes...", path_str, chunk_size);
    
    let path = Path::new(path_str);
    let mut file = File::open(path).context("Nu am putut deschide fisierul de intrare")?;
    let total_len = file.metadata()?.len();
    
    // Seteam buffer-ul pentru citire
    let mut buffer = vec![0u8; chunk_size as usize];
    let mut parts_info = Vec::new(); // Lista unde tinem minte ce am creat
    let mut part_num = 1;

    loop {
        // Citim o bucata din fisier
        let n = file.read(&mut buffer)?;
        if n == 0 { break; } 

        let chunk_data = &buffer[..n]; // Datele efective (poate ultima bucata e mai mica)
        
        let hash = calculate_hash(chunk_data);
        
        let part_filename = format!("{}.part{:04}.split", path_str, part_num);
        
        let mut part_file = File::create(&part_filename)?;
        part_file.write_all(chunk_data)?;
        
        println!("Creat: {} ({} bytes)", part_filename, n);

        parts_info.push(PartInfo {
            filename: part_filename,
            hash,
        });

        part_num += 1;
    }

    let manifest = FileManifest {
        original_filename: path_str.to_string(),
        total_size: total_len,
        parts: parts_info,
    };

    let manifest_filename = format!("{}.manifest.json", path_str);
    let manifest_file = File::create(&manifest_filename)?;
    
    serde_json::to_writer_pretty(manifest_file, &manifest)?;

    println!("Gata! Manifestul a fost salvat in '{}'.", manifest_filename);
    Ok(())
}



fn main() -> Result<()> {
    // Parse arguments
    let cli = Cli::parse();

    match cli.command {
        Commands::Split { file, size } => {
            println!("Apelam functia de split pentru: {} cu size: {}", file, size);
             split_file(&file, &size)?; 
        }
        Commands::Unsplit { file } => {
            println!("Apelam functia de unsplit pentru: {}", file);
            // unsplit_file(&file)?; 
        }
    }

    Ok(())
}


