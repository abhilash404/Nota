use sha2::{Sha256, Digest};
use std::fs::File as Fs;
use std::io::{BufReader,Read};
use walkdir::WalkDir;
use std::collections::HashMap;
use std::path::Path;
use xxhash_rust::xxh3::Xxh3;

pub struct File{
  pub path: String,
  pub hash: String,
  pub value: i32
}

pub fn print_path(entries: &Vec<File>){
  for ele in entries{
    if ele.value > 1{
      println!("{}",ele.path);
    }
  }
}
pub fn same(path: &String) -> (bool, Vec<File>){

  let mut hashed: Vec<File> = Vec::new();
  let mut flag: bool = true;

  for entry in WalkDir::new(path){

    let entry = entry.unwrap();
    let path = entry.path();

    if path.is_file(){
      let mut hash: String = hashing(path);

      let mut f: bool= false;
      for ele in &mut hashed{
        if ele.hash == hash{
          f = true;
          flag = false;
          ele.value += 1;
        }
      }
      if !f{
        hashed.push(File{
          path: path.to_string_lossy().to_string(),
          hash: hash,
          value: 1
        })
      }
    }else{
      continue;
    }
  }
  (flag,hashed)
}

fn hashing(path: &Path) -> String{
  let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
  let path_str = path.to_string_lossy();

  let hash = match ext.as_str() {
    "pdf" | "docx" | "txt" => {
      hashing_sha(&path_str)
    },
    "mp4" | "png" | "jpg" | "exe" | "bin" => {
      hashing_blake3(&path_str)
    },
    "log" | "tmp" | "bak" => {
      hashing_xxhash(&path_str)
    },
    _ => {
        unreachable!()
    },
  };
  hash
}


fn hashing_sha(path: &str) -> String{
  let file = Fs::open(path).unwrap();

  let mut reader = BufReader::new(file);
  let mut hasher = Sha256::new();

  let mut buffer = [0;1024];

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

fn hashing_blake3(path: &str) -> String{
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

fn hashing_xxhash(path: &str) -> String{
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