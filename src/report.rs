use serde::{Serialize, Deserialize};
use crate::utils::get_files;
use std::fs;
use std::io::Write;
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct Report {
    file_paths: Vec<String>,
    total_space_saved_bytes: u64,
    duplicate_file_count: u32,
}

pub fn create_report(path: &String){
  let random = None;
  let files = get_files(path,random);

  let mut dups: Vec<String> = Vec::new();
  let mut count:u32 = 0;
  let mut size : u64 = 0;

  for entry in files{
    let file_path= entry.path();
    let metadata = fs::metadata(&file_path).unwrap();
    let len = metadata.len();
    size += len;
    dups.push(file_path.to_string_lossy().to_string());
    count += 1;

  }

  let report = Report {
    file_paths: dups,
    total_space_saved_bytes: size,
    duplicate_file_count: count
  };

  let json = serde_json::to_string(&report).unwrap();
  let mut file = File::create("report.json").unwrap();
  file.write_all(json.as_bytes()).unwrap();

}