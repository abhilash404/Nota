mod args;

use args::{Cli,Args};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Args::Shout { name } =>{
            println!("{}", name);
        },
        Args::Bhag => {
            println!("BHAGO!!!");
        },
        Args::Square { a,b } =>{
            let res= sqr(a,b);
            println!("{}", res);
        }
    }
}

fn sqr (a:&i32, b: &i32) -> i32{
    a*b
}