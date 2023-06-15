use clap::Parser;

use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let matched_lines = match open_and_filter(&args) {
        Ok(val) => val,
        Err(_) => {
            panic!("no good")
        }
    };
    matched_lines.iter().for_each(|item| println!("{}", item));
}

fn open_and_filter(args: &Cli) -> Result<Vec<String>, Error> {
    let f = File::open(&args.path)?;
    let reader = BufReader::new(&f);

    let mut matched_lines = Vec::new();

    for lines in reader.lines() {
        let line = lines?;
        if line.contains(&args.pattern) {
            matched_lines.push(line);
        }
    }
    Ok(matched_lines)
}
