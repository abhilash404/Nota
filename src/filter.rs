use tempfile::TempDir;
use std::path::Path;
use std::io;
use std::fs;
use crate::hash::File;



pub fn filter(q_path: &String){

  fs::remove_dir_all(q_path).unwrap();
  println!("your files have been filtered out✅");
  
}

pub fn qurantine(hashed: &Vec<File>, path: &String) -> String{

  let q_path = Path::new(path).join("quarantine");
  fs::create_dir_all(&q_path).unwrap();

  for file in hashed{

    let og_path = Path::new(&file.path);
    let new_path = q_path.join(og_path.file_name().unwrap());
    fs::rename(&og_path,&new_path).unwrap();

  }

  q_path.to_string_lossy().to_string()
}