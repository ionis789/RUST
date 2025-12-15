use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use anyhow::{Context, Result};

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

fn calculate_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    // Returnam hash-ul ca string hexazecimal
    hex::encode(hasher.finalize())
}


fn split_file(filename: &str, size_str: &str) -> Result<()> {
    
    // 1. Calea catre fisierul de intrare: tests/nume.txt
    let input_path = Path::new("tests").join(filename);
    
    if !input_path.exists() {
        return Err(anyhow::anyhow!("Fisierul '{:?}' nu exista!", input_path));
    }

    let chunk_size = parse_size(size_str).context("Format invalid la marime")?;
    println!("Split '{:?}' in bucati de {} bytes...", input_path, chunk_size);
    
    let mut file = File::open(&input_path).context("Nu pot deschide fisierul")?;
    let total_len = file.metadata()?.len();

    // 2. Folderul de iesire: tests/nume.txt_parts
    let output_dir_name = format!("{}_parts", filename);
    let output_dir = Path::new("tests").join(&output_dir_name);
    
    if !output_dir.exists() {
        fs::create_dir(&output_dir)?;
    }

    let mut buffer = vec![0u8; chunk_size as usize];
    let mut parts_info = Vec::new();
    let mut part_num = 1;

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 { break; }

        let chunk_data = &buffer[..n];
        let hash = calculate_hash(chunk_data);

        let part_name = format!("part{:04}.split", part_num);
        let part_path = output_dir.join(&part_name);
        
        let mut part_file = File::create(&part_path)?;
        part_file.write_all(chunk_data)?;
        
        println!("Creat: {:?} ({} bytes)", part_path, n);

        parts_info.push(PartInfo {
            filename: part_name,
            hash,
        });

        part_num += 1;
    }

    let manifest = FileManifest {
        original_filename: filename.to_string(),
        total_size: total_len,
        parts: parts_info,
    };

    let manifest_path = output_dir.join("manifest.json");
    let manifest_file = File::create(&manifest_path)?;
    serde_json::to_writer_pretty(manifest_file, &manifest)?;

    println!("Gata! Rezultatul e in folderul: {:?}", output_dir);
    Ok(())
}



fn unsplit_file(filename: &str) -> Result<()> {
    // Caut folderul in tests/nume.txt_parts
    let parts_dir_name = format!("{}_parts", filename);
    let parts_dir = Path::new("tests").join(&parts_dir_name);

    if !parts_dir.exists() {
        return Err(anyhow::anyhow!("Folderul de split nu exista: {:?}", parts_dir));
    }

    // Citesc manifestul
    let manifest_path = parts_dir.join("manifest.json");
    println!("Citesc manifestul din {:?}...", manifest_path);
    
    let manifest_file = File::open(&manifest_path).context("Manifest lipsa")?;
    let manifest: FileManifest = serde_json::from_reader(manifest_file)?;

    // Fisierul rezultat va fi tot in tests, cu prefixul 'restored_'
    let output_filename = format!("restored_{}", manifest.original_filename);
    let output_path = Path::new("tests").join(&output_filename);

    let mut output_file = File::create(&output_path)?;

    println!("Reasamblez in '{:?}'...", output_path);

    for part in manifest.parts {
        let part_path = parts_dir.join(&part.filename);

        if !part_path.exists() {
            return Err(anyhow::anyhow!("Lipseste: {:?}", part_path));
        }

        let mut part_file = File::open(&part_path)?;
        let mut buffer = Vec::new();
        part_file.read_to_end(&mut buffer)?;

        let current_hash = calculate_hash(&buffer);
        if current_hash != part.hash {
            return Err(anyhow::anyhow!("Coruptie la {:?}", part_path));
        }

        output_file.write_all(&buffer)?;
    }

    println!("Succes! Fisierul a fost refacut: {:?}", output_path);
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
                unsplit_file(&file)?; 
        }
    }

    Ok(())
}


