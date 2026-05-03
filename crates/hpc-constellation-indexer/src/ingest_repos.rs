// hpc-constellation-indexer/src/ingest_repos.rs

use rusqlite::Connection;
use crate::ConstellationConfig;

pub fn ingest_repos(conn: &Connection, config: &ConstellationConfig) -> anyhow::Result<()> {
    // Insert into repos and repo_manifests/routing_rules.
    unimplemented!()
}
