mod args;
mod hash;
mod filter;

use std::env;
use args::{Cli,Args};
use clap::Parser;
use hash::File;
use std::io;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Args::Same { input, filter, quarantine } => {
            let path = match input {
                Some(name) => name.clone(),
                None => env::current_dir().unwrap().to_string_lossy().to_string(),
            };
            match (*filter, *quarantine){
                (true,true) => {
                    println!("not allowed");
                },
                (true, false) =>{
                    let entries= process_path(&path);
                    filter::filter(&entries, &path);
                },
                (false, true) => {
                    let entires = process_path(&path);
                    filter::qurantine(&entires, &path);
                    println!("the above file have been quarentined.");
                    decide(&entires, &path);
                }
                (false,false) => {
                    let entries= process_path(&path);
                },
            };  
        },

    }
}

fn process_path(path: &String) -> Vec<File>{
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
    entries
}

fn decide(entries: &Vec<File>, path: &String){
    println!("do you want to filter them out? [Y/N]");
    let mut decision = String::new();
    io::stdin().read_line(&mut decision);
    let decision = decision.trim();

    match decision{
        "Y"|"y" => filter::filter(&entries, &path),
        "N"|"n" => println!("Thank you"),
        _ => {
            println!("invalid input.");
            decide(&entries, &path);
        }
    }
}


// fn process_input(input: Option<String>) -> Vec<file>{
//     match input {
//         None => {
//             let f1 = env::current_dir().unwrap().to_string_lossy().to_string();
//             let entries = process_path(&f1);
//         },
//         Some(name) => {
//             let entries = process_path(&name),
//         }
//     }
//     entries
// }