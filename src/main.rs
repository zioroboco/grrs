use exitfailure::ExitFailure;
use failure::ResultExt;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter, Write};
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
    let mut writer = BufWriter::new(io::stdout());

    for line in reader.lines() {
        let content =
            line.with_context(|_| format!("Could not read line in file {:?}", args.path))?;

        if content.contains(&args.pattern) {
            writeln!(&mut writer, "{}", content)?;
        }
    }

    &mut writer.flush();

    Ok(())
}
