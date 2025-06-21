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
  }
} 