// src/main.rs
/*
 * Main executable for SuperOpti
 */

use clap::Parser;
use superopti::{Result, run};

#[derive(Parser)]
#[command(version, about = "SuperOpti - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
