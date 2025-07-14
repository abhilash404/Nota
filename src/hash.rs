use sha2::{Sha256, Digest};
use std::fs::File as Fs;
use std::io::{BufReader, Read};
use jwalk::WalkDir;
use std::collections::HashMap;
use std::path::Path;
use xxhash_rust::xxh3::Xxh3;
use rayon::prelude::*;


use crate::utils::get_files;

pub struct File {
    pub path: String,
    pub hash: String,
    pub value: i32,
}

pub fn print_path(entries: &Vec<File>) {
    for ele in entries {
        println!("{}", ele.path);
    }
}

pub fn same(path: &String, large: Option<&str>) -> (bool, Vec<File>) {

    let files = get_files(path, large);

    
    
    // let files: Vec<_> = WalkDir::new(path)
    //     .into_iter()
    //     .filter_map(|e| e.ok())
    //     .filter(|e| e.file_type().is_file())
    //     .collect();

    
    let hashed_files: Vec<File> = files.par_iter()
        .map(|entry| {
            let path = entry.path();
            let hash = hashing(&path);
            File {
                path: path.to_string_lossy().to_string(),
                hash: hash,
                value: 1,
            }
        })
        .collect();

    
    let mut map: HashMap<String, File> = HashMap::new();
    let mut flag = true;

    for file in hashed_files {
        let counter = map.entry(file.hash.clone()).or_insert(File {
            path: file.path.clone(),
            hash: file.hash.clone(),
            value: 0,
        });
        counter.value += 1;
        if counter.value > 1 {
            flag = false;
        }
    }

    let result: Vec<File> = map.into_values().collect();
    (flag, result)
}

fn hashing(path: &Path) -> String {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

    let hash = match ext.as_str() {
        "pdf" | "docx" | "txt" => {
            hashing_sha(&path.to_string_lossy())
        }
        "mp4" | "png" | "jpg" | "exe" | "bin" => {
            hashing_blake3(&path.to_string_lossy())
        }
        "log" | "tmp" | "bak" => {
            hashing_xxhash(&path.to_string_lossy())
        }
        _ => {
            hashing_blake3(&path.to_string_lossy()) 
        }
    };
    hash
}

fn hashing_sha(path: &str) -> String {
    let file = Fs::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = reader.read(&mut buffer).expect("Failed to read");
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    format!("{:x}", result)
}

fn hashing_blake3(path: &str) -> String {
    let file = Fs::open(path).expect("Could not open file");
    let mut reader = BufReader::new(file);
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0; 1024];

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }

    hasher.finalize().to_hex().to_string()
}

fn hashing_xxhash(path: &str) -> String {
    let file = Fs::open(path).expect("Could not open file");
    let mut reader = BufReader::new(file);
    let mut hasher = Xxh3::new();
    let mut buffer = [0; 1024];

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }

    format!("{:x}", hasher.digest())
}