mod args;
mod hash;
mod filter;
mod utils;
mod report;

use std::env;
use args::{Cli,Args};
use clap::Parser;
use hash::File;
use std::io;
use utils::restore;
use std::path::Path;


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Args::Same { input, large, filter, quarantine } => {
            let path = match input {
                Some(name) => name.clone(),
                None => env::current_dir().unwrap().to_string_lossy().to_string(),
            };
            let large = large.as_deref();

            match (*filter, *quarantine){
                (true,true) => {
                    println!("not allowed");
                },
                (true, false) =>{
                    let entries= process_path(&path,large);
                    let q_path = filter::qurantine(&entries, &path, large);
                    filter::filter(&q_path);
                },
                (false, true) => {
                    let entries = process_path(&path,large);
                    let q_path = filter::qurantine(&entries, &path, large);
                    println!("the above file have been quarentined.");
                    decide(&q_path);
                }
                (false,false) => {
                    let entries= process_path(&path,large);
                },
            };  
        },
        Args::Filter {quarantine, input, large} => {
            let large = large.as_deref();
            let path = match input {
                Some(name) => name.clone(),
                None => env::current_dir().unwrap().to_string_lossy().to_string(),
            };
            if *quarantine {
                let entries = process_path(&path,large);
                let q_path = filter::qurantine(&entries, &path, large);
                println!("the above file have been quarentined.");
                decide(&q_path);
            }else {
                let entries= process_path(&path,large);
            }
        },
        Args::Restore {input  } => {
            let path = match input {
                Some(name) => name.clone(),
                None => env::current_dir().unwrap().to_string_lossy().to_string(),
            };
            let file_path = Path::new(&path).join("quarantine");
            let file_path_str = file_path.to_string_lossy().to_string();
            restore(&file_path_str);
        }

    }
}

fn process_path(path: &String,large: Option<&str>) -> Vec<File>{
    let (flag, entries) = hash::same(path,large);
    if flag {
        println!("No duplicates in the file");
    } else {
        println!(
            "There are {} unique files in the directory given.\nThe paths of those are:\n",
            entries.len()
        );
        hash::print_path(&entries);
    }
    entries
}

fn decide(q_path: &String){
    println!("do you want to filter them out? [Y/N]");
    let mut decision = String::new();
    io::stdin().read_line(&mut decision).unwrap();
    let decision = decision.trim();

    match decision{
        "Y"|"y" => filter::filter(&q_path),
        "N"|"n" => println!("Thank you"),
        _ => {
            println!("invalid input.");
            decide(&q_path);
        }
    }
}