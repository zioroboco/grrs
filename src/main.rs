use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

/// Display any lines in a file matching a pattern
#[derive(StructOpt)]
struct Cli {
    /// Target pattern
    pattern: String,
    /// Path to file
    path: PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let args = Cli::from_args();

    let file = File::open(&args.path)
        .map_err(|err| CustomError(format!("Error reading `{:?}`: {}", args.path, err)))?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let content = line.unwrap();
        if content.contains(&args.pattern) {
            println!("{}", content);
        }
    }

    Ok(())
}
