use clap::{Parser,Subcommand};

#[derive(Parser,Debug)]
#[command(name="nota")]
pub struct Cli{
  #[command(subcommand)]
  pub command: Args,
}

#[derive(Subcommand, Debug)]
pub enum Args{
  Same{
    #[arg()]
    input: Option<String>,
    #[arg(short = 'f', long = "f", conflicts_with = "quarantine")]
    filter: bool,
    #[arg(short = 'q', long = "q", conflicts_with = "filter")]
    quarantine: bool,  
  }
} 


