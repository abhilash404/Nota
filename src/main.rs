mod args;
mod hash;

use std::env;
use args::{Cli,Args};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Args::Same { input } => {

            match input {
                None => {
                    let current_dir = env::current_dir().unwrap();
                    let f1 = current_dir.to_string_lossy().to_string();
                    //let f2 = r"E:\test\f2.txt".to_string();
                    if hash::same(&f1){
                        println!("No duplicates in the file");                       
                    }else{
                        println!("there are duplicate files in the directory given.\n the hash of those are:\n");
                    }
                },
                Some(name) => {
                    if hash::same(&name){
                        println!("No duplicates in the file");
                    }else{
                        println!("there are duplicate files in the directory given.\n the hash of those are given above:\n");
                    }
                },
            }
            
        },

    }
}
