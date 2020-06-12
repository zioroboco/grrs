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
    let result = std::fs::read_to_string(&args.path);

    let content = match result {
        Ok(content) => content,
        Err(error) => {
            panic!("Uh oh: {}", error);
        }
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
