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

                    let (flag, entries) = hash::same(&f1);
                    
                    if flag{
                        println!("No duplicates in the file");                       
                    }else{
                        println!("there are duplicate files in the directory given.\n the paths of those are given above:\n");
                        hash::print_path(&entries);
                    }
                },
                Some(name) => {
                    let (flag, entries) = hash::same(&name);

                    if flag{
                        println!("No duplicates in the file");
                    }else{
                        println!("there are duplicate files in the directory given.\n the paths of those are given below:\n");
                        hash::print_path(&entries);
                    }
                },
            }
            
        },

    }
}
