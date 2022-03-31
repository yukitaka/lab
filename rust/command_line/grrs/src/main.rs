use std::thread::sleep;
use std::time::Duration;
use clap::Parser;
use anyhow::{Context, Result};
use log::{info, warn};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;

    let pb = indicatif::ProgressBar::new(100);
    for _i in 0..100 {
        sleep(Duration::from_millis(10));
        pb.inc(2);
    }
    pb.finish_with_message("done");

    println!("file content: {}", content);
    Ok(())
}
