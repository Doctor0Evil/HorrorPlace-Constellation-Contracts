//! CLI subcommand: `hpc-chat-director init`.
//!
//! Scans for the schema spine and repo manifests, verifies environment
//! integrity, and prints a structured summary for AI discovery.

use std::path::Path;
use serde::Serialize;
use crate::config::Config;
use crate::spine;
use crate::manifests;
use crate::errors::Error;

#[derive(Serialize)]
struct InitReport {
    status: String,
    spine_version: String,
    manifest_count: usize,
    object_kinds: Vec<String>,
    warnings: Vec<String>,
}

/// Run the init command.
pub fn run(root: &Path) -> Result<(), Error> {
    println!("Initializing HorrorPlace-Constellation-Contracts environment...");

    // 1. Detect configuration
    let config = Config::detect(root)?;
    println!("✅ Config detected: Root={}", config.root.display());

    // 2. Load spine
    let spine = spine::load(&config)?;
    println!("✅ Spine loaded: Version={}", spine.version);

    // 3. Load manifests
    let manifests = manifests::load_all(&config)?;
    println!("✅ Manifests loaded: Count={}", manifests.len());

    // 4. Collect object kinds
    let object_kinds = manifests
        .iter()
        .flat_map(|m| m.allowed_object_kinds.iter().cloned())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    // 5. Report results
    let report = InitReport {
        status: "success".to_string(),
        spine_version: spine.version,
        manifest_count: manifests.len(),
        object_kinds,
        warnings: Vec::new(), // Could populate with manifest inconsistencies
    };

    // Output as JSON for AI parsing
    let json_output = serde_json::to_string_pretty(&report)?;
    println!("{}", json_output);

    Ok(())
}
