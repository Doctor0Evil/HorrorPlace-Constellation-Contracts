// hpc-constellation-indexer/src/ingest_registries.rs

use rusqlite::Connection;

pub fn ingest_registries_for_repo(conn: &Connection, repo_id: i64, repo_path: &str) -> anyhow::Result<()> {
    // Scan registry/*.ndjson and populate registries, registry_entries,
    // registry_entry_invariants, registry_entry_metrics.
    unimplemented!()
}
