mod args;
mod hash;

use std::env;
use args::{Cli,Args};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Args::Same { input, filter, quarantine } => {
            match (*filter, *quarantine){
                (true, true) => {
                    println!("both filters cannot be used together");
                },
                (true, false) =>{
                    println!("filtered");
                },
                (false, true) => {
                    println!("quarantine");
                }
                (false,false) => {
                    match input {
                        None => {
                            let current_dir = env::current_dir().unwrap();
                            let f1 = current_dir.to_string_lossy().to_string();

                            let (flag, entries) = hash::same(&f1);

                            if flag{
                                println!("No duplicates in the file");                       
                            }else{
                                println!("there are {} duplicate files in the directory given.\nthe paths of those are:\n",entries.len());
                                hash::print_path(&entries);
                            }
                        },

                    
                        Some(name) => {
                            let (flag, entries) = hash::same(&name);

                            if flag{
                                println!("No duplicates in the file");
                            }else{
                                println!("there {} are duplicate files in the directory given.\n the paths of those are:\n", entries.len());
                                hash::print_path(&entries);
                            }
                        },
                    }
                },
            }   
        },

    }
}
