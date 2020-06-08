use structopt::StructOpt;

/// Display any lines in a file matching a pattern
#[derive(StructOpt)]
struct Cli {
    /// Target pattern
    pattern: String,
    /// Path to file
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    println!("Hello, world!");
}
