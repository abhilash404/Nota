

use tempfile::TempDir;
use std::path::Path;
use std::io;
use crate::hash::File;



pub fn filter(hashed: Vec<File>){
  for ele in hashed{
    println!("{}",ele.hash)
  }
}

pub fn qurantine(){
  println!("covid covid");
}