use exitfailure::ExitFailure;
use failure::ResultExt;
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

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();

    let file =
        File::open(&args.path).with_context(|_| format!("Could not read file {:?}", args.path))?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let content = line.unwrap();
        if content.contains(&args.pattern) {
            println!("{}", content);
        }
    }

    Ok(())
}
