// tests/integration_test.rs

use std::fs;
use std::path::Path;
use std::env;
use std::io::Write;
use std::fs::File as Fs;

use Nota::hash::{same, File as HashFile};
use Nota::utils::{restore};
use Nota::filter::{filter,qurantine};
use Nota::args::{Cli, Args};

use clap::Parser;

// Helper: Create a file with given content
fn create_test_file(path: &Path, content: &[u8]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut file = Fs::create(path).unwrap();
    file.write_all(content).unwrap();
}

#[test]
fn test_hashing_and_duplicates() {
    let test_dir = Path::new("test_data/hash_test");
    fs::remove_dir_all(test_dir).ok();
    fs::create_dir_all(test_dir).unwrap();

    // Create two identical files and one different
    let file1 = test_dir.join("file1.txt");
    let file2 = test_dir.join("file2.txt");
    let file3 = test_dir.join("file3.txt");

    create_test_file(&file1, b"Hello World");
    create_test_file(&file2, b"Hello World");
    create_test_file(&file3, b"Different content");

    let (flag, files) = same(&test_dir.to_string_lossy().to_string(), None);

    // Should detect duplicates
    assert_eq!(flag, false);
    // Should be 2 unique hashes (file1/file2 and file3)
    assert_eq!(files.len(), 2);
    // One of the files should have value > 1 (duplicate count)
    let has_duplicate = files.iter().any(|f| f.value > 1);
    assert!(has_duplicate);
}

#[test]
fn test_quarantine_and_filter() {
    let test_dir = Path::new("test_data/quarantine_test");
    fs::remove_dir_all(test_dir).ok();
    fs::create_dir_all(test_dir).unwrap();

    // Create files
    let file1 = test_dir.join("file1.txt");
    let file2 = test_dir.join("file2.txt");
    create_test_file(&file1, b"Content1");
    create_test_file(&file2, b"Content2");

    // Dummy File struct for quarantine
    let files = vec![
        HashFile { path: file1.to_string_lossy().to_string(), hash: "hash1".to_string(), value: 1 },
    ];

    // Quarantine files
    let q_path = qurantine(&files, &test_dir.to_string_lossy().to_string(), None);

    // Quarantine directory should exist
    assert!(Path::new(&q_path).exists());
    // file1 should be moved to quarantine
    assert!(!file1.exists());
    // file2 should remain
    assert!(file2.exists());

    // Now filter (delete) quarantine
    filter(&q_path);
    // Quarantine directory should be removed
    assert!(!Path::new(&q_path).exists());
}

#[test]
fn test_restore() {
    let test_dir = Path::new("test_data/restore_test");
    fs::remove_dir_all(test_dir).ok();
    fs::create_dir_all(test_dir).unwrap();

    let quarantine_dir = test_dir.join("quarantine");
    fs::create_dir_all(&quarantine_dir).unwrap();

    // Create a file in quarantine
    let file_name = "file1.txt";
    let quarantined_file = quarantine_dir.join(file_name);
    create_test_file(&quarantined_file, b"Restore content");

    // Create a report.json in current dir with file_paths
    let report_path = env::current_dir().unwrap().join("report.json");
    let report_content = format!(r#"{{"file_paths": ["{}"]}}"#, quarantined_file.to_string_lossy());
    fs::write(&report_path, report_content).unwrap();

    // Call restore
    restore(&quarantine_dir.to_string_lossy());

    // The file should be moved to the parent of quarantine_dir
    let restored_file = quarantine_dir.parent().unwrap().join(file_name);
    assert!(restored_file.exists());

    // Cleanup
    fs::remove_file(&restored_file).unwrap();
    fs::remove_file(&report_path).unwrap();
}

#[test]
fn test_cli_parsing() {
    // Test parsing of CLI args
    let args = vec!["nota", "same", "E:/test", "large"];
    let cli = Cli::parse_from(args);

    match cli.command {
        Args::Same { input, large, .. } => {
            assert_eq!(input.unwrap(), "E:/test");
            assert_eq!(large.unwrap(), "large");
        },
        _ => panic!("Expected Same command"),
    }
}
