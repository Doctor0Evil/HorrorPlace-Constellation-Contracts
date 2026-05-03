// hpc-constellation-indexer/src/ingest_files_chunks.rs

use rusqlite::Connection;

pub fn ingest_files_and_chunks(conn: &Connection, repo_id: i64, repo_path: &str) -> anyhow::Result<()> {
    // Walk the repo tree, populate files, and ingest analysis/chunk-manifest.ndjson
    // into chunks where present.
    unimplemented!()
}
