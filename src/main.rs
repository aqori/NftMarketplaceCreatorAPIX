// src/main.rs
/*
 * Main executable for NftMarketplaceCreatorAPIX
 */

use clap::Parser;
use nftmarketplacecreatorapix::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMarketplaceCreatorAPIX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
