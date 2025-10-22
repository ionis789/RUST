use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write, BufWriter};
use std::time::Instant;

// --- Problem 1: Longest Lines ---

/// Finds the longest line by bytes and by chars from a string.
fn p1_longest_lines(content: &str) {
    let mut max_bytes = 0;
    let mut max_byte_line = "";
    let mut max_chars = 0;
    let mut max_char_line = "";

    for line in content.lines() {
        let current_bytes = line.len();
        if current_bytes > max_bytes {
            max_bytes = current_bytes;
            max_byte_line = line;
        }

        let current_chars = line.chars().count();
        if current_chars > max_chars {
            max_chars = current_chars;
            max_char_line = line;
        }
    }

    println!("Longest line by bytes ({} bytes): {}", max_bytes, max_byte_line);
    println!("Longest line by chars ({} chars): {}", max_chars, max_char_line);
}

// --- Problem 2: ROT13 Cipher ---

/// Encrypts a string using ROT13.
/// Fails if a non-ASCII character is found.
fn p2_rot13(content: &str) -> Result<String, &'static str> {
    let mut result = String::new();

    for c in content.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let c_u8 = c as u8;
            let rotated = (c_u8 - base + 13) % 26 + base;
            result.push(rotated as char);
        } else if c.is_ascii() {
            // Keep other ASCII chars (numbers, spaces, punctuation)
            result.push(c);
        } else {
            // Fail on non-ASCII
            return Err("Found non-ASCII character.");
        }
    }
    Ok(result)
}

// --- Problem 3: Abbreviations ---

/// Replaces hardcoded abbreviations in a string.
fn p3_abbreviations(content: &str) -> String {
    let mut abbrevs = HashMap::new();
    abbrevs.insert("pt", "pentru");
    abbrevs.insert("ptr", "pentru");
    abbrevs.insert("dl", "domnul");
    abbrevs.insert("dna", "doamna");

    let mut output_words = Vec::new();
    // Split by space, as per the problem description.
    for word in content.split(' ') {
        match abbrevs.get(word) {
            Some(full_word) => output_words.push(*full_word),
            None => output_words.push(word),
        }
    }

    output_words.join(" ")
}

// --- Problem 4: Hosts File Parser ---

/// Parses a hosts file content and prints entries.
fn p4_parse_hosts(content: &str) {
    println!("Parsing hosts file entries...");
    for line in content.lines() {
        let trimmed_line = line.trim();

        // Ignore empty lines or comments
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        // Split by whitespace and take the first two parts
        let parts: Vec<&str> = trimmed_line.split_whitespace().collect();
        
        if parts.len() >= 2 {
            let ip = parts[0];
            let hostname = parts[1];
            println!("{} => {}", hostname, ip);
        }
    }
}

// --- Bonus: File Generator and Optimized ROT13 ---

/// Generates a large file by repeating a pattern.
fn bonus_generate_file(path: &str, size_gb: u64) -> io::Result<()> {
    let file = fs::File::create(path)?;
    // Use a BufWriter for much better performance.
    let mut writer = BufWriter::new(file);
    
    let target_bytes = size_gb * 1024 * 1024 * 1024;
    let pattern = "This is a repeating string for the large file. 1234567890. ABCDEFG.\n";
    let pattern_bytes = pattern.as_bytes();
    let pattern_len = pattern_bytes.len() as u64;
    let mut bytes_written = 0u64;

    while bytes_written < target_bytes {
        writer.write_all(pattern_bytes)?;
        bytes_written += pattern_len;
    }

    writer.flush()?;
    Ok(())
}

/// Optimized ROT13 that processes the file in chunks (buffers)
/// instead of loading the
/// entire file into memory.
fn bonus_rot13_optimized(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = fs::File::open(input_path)?;
    let mut reader = BufReader::new(input_file);
    
    let output_file = fs::File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    // Use a 64KB buffer for reading
    let mut buffer = [0u8; 64 * 1024];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }

        // Get the slice of data we just read
        let data_slice = &mut buffer[..bytes_read];

        // Process the buffer in-place
        for byte in data_slice.iter_mut() {
            let b = *byte;
            if (b'a'..=b'z').contains(&b) {
                *byte = (b - b'a' + 13) % 26 + b'a';
            } else if (b'A'..=b'Z').contains(&b) {
                *byte = (b - b'A' + 13) % 26 + b'A';
            } else if !b.is_ascii() {
                // Found non-ASCII, stop and return error
                return Err("Found non-ASCII character.".into());
            }
            // Other ASCII bytes (spaces, newlines, numbers) are left as-is
        }

        // Write the processed buffer
        writer.write_all(data_slice)?;
    }
    
    writer.flush()?;
    Ok(())
}


// --- Main Function to Run Problems ---

fn main() {
    // --- Run P1 ---
    println!("--- P1: Longest Lines ---");
    let p1_content = "strings are fun\n🎁🎶🎉👀🎈🎃🍕☕🍉\nrust\nsupercalifragilisticexpialidocious";
    fs::write("p1_input.txt", p1_content).expect("Failed to write p1 file");
    let p1_read = fs::read_to_string("p1_input.txt").expect("Failed to read p1 file");
    p1_longest_lines(&p1_read);
    println!();

    // --- Run P2 ---
    println!("--- P2: ROT13 Cipher ---");
    // Success case
    let p2_success_content = "Hello World! This is a test 123.";
    fs::write("p2_input.txt", p2_success_content).expect("Failed to write p2 file");
    let p2_read = fs::read_to_string("p2_input.txt").expect("Failed to read p2 file");
    
    match p2_rot13(&p2_read) {
        Ok(encrypted) => {
            fs::write("p2_output.txt", &encrypted).expect("Failed to write p2 output");
            println!("P2 Success: '{}'", encrypted);
        }
        Err(e) => println!("P2 Success Case Failed: {}", e),
    }

    // Failure case
    let p2_fail_content = "Hello non-ASCII: ășț";
    match p2_rot13(p2_fail_content) {
        Ok(encrypted) => println!("P2 Fail Case Succeeded (unexpected): {}", encrypted),
        Err(e) => println!("P2 Fail Case (Expected): Error: {}", e),
    }
    println!();

    // --- Run P3 ---
    println!("--- P3: Abbreviations ---");
    let p3_content = "Am fost la dl Matei pt că m-a invitat cu o zi înainte.";
    fs::write("p3_input.txt", p3_content).expect("Failed to write p3 file");
    let p3_read = fs::read_to_string("p3_input.txt").expect("Failed to read p3 file");
    
    let p3_result = p3_abbreviations(&p3_read);
    println!("P3 Input: {}", p3_read);
    println!("P3 Output: {}", p3_result);
    println!();
    
    // --- Run P4 ---
    println!("--- P4: Hosts File Parser ---");
    // Select the correct path for the OS
    let hosts_path = if cfg!(windows) {
        "C:\\Windows\\System32\\drivers\\etc\\hosts"
    } else {
        "/etc/hosts"
    };

    match fs::read_to_string(hosts_path) {
        Ok(content) => p4_parse_hosts(&content),
        Err(e) => {
            // If file is not readable (e.g. permissions), use a dummy string
            println!("Could not read '{}'. Error: {}. Using dummy content.", hosts_path, e);
            let dummy_content = "# This is a dummy hosts file\n\
                                 127.0.0.1   localhost loopback\n\
                                 ::1         localhost\n\
                                 # 10.0.0.1    my-server";
            p4_parse_hosts(dummy_content);
        }
    }
    println!();

    // --- Run Bonus ---
    println!("--- BONUS: Optimized ROT13 ---");
    println!("Bonus code is commented out in main.rs to prevent long run times.");
    println!("Uncomment the section below to run it.");

    /*
    // --- UNCOMMENT TO RUN BONUS ---
    {
        // Use a smaller size for quick testing (e.g., 1GB)
        // Note: 4GB will take a while!
        let size_gb = 1; 
        let big_file_path = "large_test_file.txt";
        let optimized_output_path = "large_test_file_optimized.txt";

        println!("Generating {}GB file (this may take a while)...", size_gb);
        let start_gen = Instant::now();
        if let Err(e) = bonus_generate_file(big_file_path, size_gb) {
            eprintln!("Failed to generate file: {}", e);
            return;
        }
        println!("File generation took: {:?}", start_gen.elapsed());


        println!("Running optimized ROT13...");
        let start_opt = Instant::now();
        if let Err(e) = bonus_rot13_optimized(big_file_path, optimized_output_path) {
            eprintln!("Optimized ROT13 failed: {}", e);
        } else {
            println!("Optimized ROT13 took: {:?}", start_opt.elapsed());
        }
        
        // Clean up the large files
        fs::remove_file(big_file_path).ok();
        fs::remove_file(optimized_output_path).ok();
    }
    // --- END OF BONUS SECTION ---
    */
}

