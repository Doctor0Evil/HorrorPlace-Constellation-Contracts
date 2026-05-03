// hpc-constellation-indexer/src/main.rs

use std::path::PathBuf;
use clap::Parser;
use hpc_constellation_indexer::{ConstellationConfig, RepoConfig, build_constellation_index};

#[derive(Parser, Debug)]
#[command(name = "hpc-constellation-indexer")]
#[command(about = "Builds the constellation-wide SQLite index (constellation.db)")]
struct Args {
    /// Path to a JSON config describing repos and output DB location.
    #[arg(long)]
    config: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config_bytes = std::fs::read(&args.config)?;
    let config: ConstellationConfig = serde_json::from_slice(&config_bytes)?;
    build_constellation_index(&config)?;
    Ok(())
}
