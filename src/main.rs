extern crate clap;
extern crate walkdir;
extern crate regex;

mod cli;
// use cli::get_options;

use walkdir::{DirEntry, WalkDir};

fn main() {
    
    let options = cli::get_options();
    println!("options: {:?}", options);
}
