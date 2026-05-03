// hpc-constellation-indexer/src/ingest_schemas.rs

use rusqlite::Connection;

pub fn ingest_schemas_for_repo(conn: &Connection, repo_id: i64, repo_path: &str) -> anyhow::Result<()> {
    // Scan schemas/ and populate schemas and schema_fields.
    unimplemented!()
}
