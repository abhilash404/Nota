

use tempfile::TempDir;
use std::path::Path;
use std::io;
use std::fs;
use crate::hash::File;



pub fn filter(hashed: &Vec<File>, path: &String){

  
  // for ele in hashed{
  //   println!("{}",ele.hash)
  // }
  println!("filtered")
}

pub fn qurantine(hashed: &Vec<File>, path: &String){

  let q_path = Path::new(path).join("quarantine");
  fs::create_dir_all(&q_path).unwrap();

  for file in hashed{

    let og_path = Path::new(&file.path);
    let new_path = q_path.join(og_path.file_name().unwrap());
    fs::rename(&og_path,&new_path).unwrap();

  }
}