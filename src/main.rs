use structopt::StructOpt;

/// Display any lines in a file matching a pattern
#[derive(StructOpt)]
struct Cli {
    /// Target pattern
    pattern: String,
    /// Path to file
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .map_err(|err| CustomError(format!("Error reading {:?}: {}", args.path, err)))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
