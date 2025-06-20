use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{BufReader,Read};

pub fn same(path1: &String, path2: &String) -> bool{

  let mut hash1: String= hashed(path1);
  let mut hash2: String= hashed(path2);

  if hash1 == hash2{
    true
  }else{
    false
  }
}

fn hashed(path: &String) -> String{
  let file = File::open(path).unwrap();

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
  //Ok(format!("{:x}", result));
  format!("{:x}", result)
}