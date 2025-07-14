use std::path::Path;
use std::fs;
use crate::hash::File;
use crate::utils::get_files;
use crate::report::create_report;



pub fn filter(q_path: &String){
  fs::remove_dir_all(q_path).unwrap();
  println!("your files have been filtered out✅");

}

pub fn qurantine(hashed: &Vec<File>, path: &String, large: Option<&str>) -> String {
    let q_path = Path::new(path).join("quarantine");
    fs::create_dir_all(&q_path).unwrap();

    let files = get_files(path, large);

    for entry in files {
        let file_path = entry.path();
        let file_path_str = file_path.to_string_lossy().to_string();

        // Normalize paths for comparison
        let is_hashed = hashed.iter().any(|f| {
            Path::new(&f.path).canonicalize().ok() == file_path.canonicalize().ok()
        });

        if !is_hashed {
            let new_path = q_path.join(file_path.file_name().unwrap());
            fs::rename(&file_path, &new_path).unwrap();
        }
    }

    let q_path_str = q_path.to_string_lossy().to_string();
    create_report(&q_path_str);
    q_path_str
}