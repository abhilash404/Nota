use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{BufReader,Read};
use walkdir::WalkDir;
use std::collections::HashMap;

/*pub fn same(path1: &String, path2: &String) -> bool{

  let mut hash1: String= hashed(path1);
  let mut hash2: String= hashed(path2);

  if hash1 == hash2{
    true
  }else{
    false
  }
}*/

pub fn same(path: &String) -> bool{

  let mut hashed: HashMap<String,i32> = HashMap::new();
  let mut flag: bool = true;

  for entry in WalkDir::new(path){

    let entry = entry.unwrap();
    let path = entry.path();

    if path.is_file(){
      let mut hash: String = hashing(&path.to_string_lossy().to_string());
      if let Some(val) = hashed.get_mut(&hash){
        flag = false;
        *val += 1;
      }else{
        hashed.insert(hash.clone(),1);
      }
    }else{
      continue;
    }
  }

  for (key,value) in &hashed{
    if *value > 1{
      println!("{}",key);
    }
  }
  flag
}




fn hashing(path: &String) -> String{
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