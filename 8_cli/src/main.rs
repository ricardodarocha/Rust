#![allow(unused)]

use clap::Parser;
use colored::*;

/// Search for a word in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The word to look for
    word: String,
    /// The filename to the file to read
    #[clap(parse(from_os_str))]
    filename: std::path::PathBuf,
}

fn main() {

    let args = Cli::parse();
    println!("searching for {}", &args.word.red());
    let content = std::fs::read_to_string(&args.filename)
        .expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.word) {
            println!("{}", line.replace(&args.word,&args.word.red()));
        }
    }

}
