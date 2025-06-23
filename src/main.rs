mod args;
mod hash;
mod filter;

use std::env;
use args::{Cli,Args};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Args::Same { input, filter, quarantine } => {
            match (*filter, *quarantine){
                (true,true) => {
                    println!("not allowed");
                }
                (true, false) =>{
                    filter::filter();
                },
                (false, true) => {
                    filter::qurantine();
                }
                (false,false) => {
                    process_input(input.clone());
                },
            }   
        },

    }
}

fn process_path(path: &String) {
    let (flag, entries) = hash::same(path);
    if flag {
        println!("No duplicates in the file");
    } else {
        println!(
            "There are {} duplicate files in the directory given.\nThe paths of those are:\n",
            entries.len()
        );
        hash::print_path(&entries);
    }
}

fn process_input(input: Option<String>){
    match input {
        None => {
            let f1 = env::current_dir().unwrap().to_string_lossy().to_string();
            process_path(&f1);
        },
        Some(name) => process_path(&name),
    }
}