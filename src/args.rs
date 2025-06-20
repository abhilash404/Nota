use clap::{Parser,Subcommand};

#[derive(Parser,Debug)]
#[command(name="nota")]
pub struct Cli{
  #[command(subcommand)]
  pub command: Args,
}

#[derive(Subcommand, Debug)]
pub enum Args{
  Shout{
    name: String,
  },

  Bhag,

  Square {
    a: i32,
    b: i32,
  },
}