use std::path::Path;
use std::fs;
use crate::hash::File;
use walkdir::WalkDir;



pub fn filter(q_path: &String){

  fs::remove_dir_all(q_path).unwrap();
  println!("your files have been filtered out✅");

}

pub fn qurantine(hashed: &Vec<File>, path: &String) -> String{

  let q_path = Path::new(path).join("quarantine");
  fs::create_dir_all(&q_path).unwrap();

  for entry in WalkDir::new(path){
    let entry = entry.unwrap();
    let file_path = entry.path().to_string_lossy().to_string();
    if (entry.file_type().is_file() && !(hashed.iter().any(|f| f.path == file_path))){
      let og_path = entry.path();
      let new_path = q_path.join(og_path.file_name().unwrap());
      fs::rename(&og_path,&new_path).unwrap();
    }else{
      continue;
    }  
  }
  q_path.to_string_lossy().to_string()
}