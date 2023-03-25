mod words;
use clap::Parser;
//use std::env;
//use std::fs;
use std::path::PathBuf;
use crate::words::words::get;
/// A fast spell checker for everyone.
#[derive(Parser)]
struct Cli {
    /// path
    path: Option<String>,
}

fn main() {
    const DEBUG: bool = false;
    let args = Cli::parse();

    let path = match args.path {
        Some(path) => path,
        None => "./".to_owned(),
    };
    let path = PathBuf::from(path);
    let words = get();

    if path.is_dir() {
        println!("{:?}",words)
    } else {
        panic!("Path is not a directory.")
    }
    //println!("Hello, {}!",args.path.display());
}
