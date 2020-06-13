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

    #[structopt(long, short, default_value = "error")]
    log_level: LevelFilter,
}

fn main() -> Result<(), ExitFailure> {
    let args = Opt::from_args();

    CombinedLogger::init(vec![TermLogger::new(
        args.log_level,
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

    trace!("Setting up input buffer");
    let reader = BufReader::new(file);

    trace!("Setting up output buffer");
    let mut writer = BufWriter::new(io::stdout());

    for line in reader.lines() {
        let content = line.with_context(|e| {
            error!("{}", e);
            format!("Could not read line in file {:?}", args.path)
        })?;
        if grrs::matches(&content, &args.pattern) {
            trace!("Writing `{}` to output buffer", content);
            writeln!(&mut writer, "{}", &content)?;
        }
    }

    trace!("Flushing output buffer");
    &mut writer.flush();

    info!("Done");

    Ok(())
}
