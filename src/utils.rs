use jwalk::{WalkDir,DirEntry};
use std::path::Path;
use std::env;
use std::fs;
use serde_json::Value;
use crate::report::Report;
use std::io::{self, Write};
pub fn get_files(path: &String, large: Option<&str>) -> Vec<DirEntry<((), ())>>{

  let files: Vec<_> = match large {
        Some(large) => {
            let min_size = 10 * 1024 * 1024;
            WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                if let Ok(metadata) = std::fs::metadata(e.path()) {
                    metadata.len() > min_size
                } else {
                    false
                }
            })
            .collect()
        },
        _ => {
            WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .collect()
        }
  };
  files
}

pub fn restore(quarantine_path_str: &str) {
    let cur_dir = env::current_dir().unwrap();
    let report_path = cur_dir.join("report.json");
    let quarantine_dir = Path::new(quarantine_path_str);

    if !report_path.exists() {
        println!("No files are quarantined in the directory.");
        return;
    }

    let contents = fs::read_to_string(&report_path).expect("Failed to read file");
    let data: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");

    if let Some(file_paths) = data.get("file_paths").and_then(|v| v.as_array()) {
        if file_paths.is_empty() {
            println!("No quarantined files found in report.json");
            return;
        }

        println!("Quarantined files:");
        for (i, path) in file_paths.iter().enumerate() {
            if let Some(path_str) = path.as_str() {
                println!("{}. {}", i + 1, path_str);
            }
        }

        println!("Enter the index of the file you want to restore:");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let index: usize = match input.trim().parse::<usize>() {
            Ok(num) if num > 0 && num <= file_paths.len() => num - 1,
            _ => {
                println!("Invalid index.");
                return;
            }
        };

        let quarantined_path_str = file_paths[index].as_str().unwrap();
        let quarantined_path = Path::new(quarantined_path_str);

        // Get the file name (e.g., "me-2.JPG")
        let file_name = quarantined_path.file_name().unwrap();

        // The parent directory of the quarantine folder (e.g., "E:/test")
        let quarantine_parent = quarantine_dir.parent().unwrap_or_else(|| Path::new(""));

        // The destination path in the root of the quarantine's parent directory
        let restore_dest = quarantine_parent.join(file_name);

        if quarantined_path.exists() {
            // Move the file from quarantine to the root directory
            match fs::rename(&quarantined_path, &restore_dest) {
                Ok(_) => println!("File restored to {}", restore_dest.display()),
                Err(e) => println!("Failed to restore file: {}", e),
            }
        } else {
            println!("Quarantined file not found: {}", quarantined_path.display());
        }
    } else {
        println!("No file_paths found in report.json");
    }
}