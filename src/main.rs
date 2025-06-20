mod args;
mod hash;

use args::{Cli,Args};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Args::Same => {
            let f1 = r"E:\test\f1.txt".to_string();
            let f2 = r"E:\test\f2.txt".to_string();
            if hash::same(&f1,&f2){
                println!("jai shri ram");
            }else{
                println!("alla hu akhbar");
            }
        },
    }
}

fn sqr (a:&i32, b: &i32) -> i32{
    a*b
}