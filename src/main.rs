use lib::*;

use exitfailure::ExitFailure;
use failure::ResultExt;
use log::*;
use simplelog::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;

/// Display any lines in a file matching a pattern
#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    /// Target pattern
    pattern: String,

    /// Path to file
    path: PathBuf,

    #[structopt(short, long = "log-level", default_value = "off")]
    level: LevelFilter,
}

fn main() -> Result<(), ExitFailure> {
    let args = Opt::from_args();

    CombinedLogger::init(vec![TermLogger::new(
        args.level,
        Config::default(),
        TerminalMode::Mixed,
    )])?;

    info!("Running: {:?}", std::env::current_exe()?);
    info!("Args: {:?}", args);

    trace!("Opening file {:?}", &args.path);
    let file = File::open(&args.path).with_context(|e| {
        error!("{}", e);
        format!("Could not read file {:?}", args.path)
    })?;

    if &args.pattern == "" {
        trace!("Pattern empty, return no lines");
        return Ok(());
    }

    trace!("Setting up input buffer");
    let reader = BufReader::new(file);

    trace!("Setting up output buffer");
    let mut writer = BufWriter::new(io::stdout());

    for line in reader.lines() {
        let content = line.with_context(|e| {
            error!("{}", e);
            format!("Could not read line in file {:?}", args.path)
        })?;

        trace!("Checking `{}` for pattern `{}`", content, &args.pattern);
        if matches(&args.pattern, &content) {
            trace!("Writing `{}` to output buffer", content);
            writeln!(&mut writer, "{}", content)?;
        }
    }

    trace!("Flushing output buffer");
    &mut writer.flush();

    info!("Done");

    Ok(())
}
