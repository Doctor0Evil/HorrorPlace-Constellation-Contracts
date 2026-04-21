// crates/hpc-chat-director/src/cli_describe.rs

use std::io::{self, Write};

use clap::Args;
use serde::Serialize;

use crate::ChatDirector;

/// Simple filter structure for narrowing the capability catalog.
#[derive(Debug, Default, Clone)]
pub struct CatalogFilter {
    pub object_kind: Option<String>,
    pub phase: Option<u8>,
    pub tier: Option<String>,
    pub repo: Option<String>,
}

/// CLI arguments for `hpc-chat-director describe`.
#[derive(Debug, Args)]
pub struct DescribeArgs {
    /// Filter by object kind name (e.g., "moodContract").
    #[arg(long = "object-kind")]
    pub object_kind: Option<String>,

    /// Filter by phase id (0-4).
    #[arg(long = "phase")]
    pub phase: Option<u8>,

    /// Filter by tier name (e.g., "Tier1Public").
    #[arg(long = "tier")]
    pub tier: Option<String>,

    /// Filter by repository name (e.g., "Horror.Place").
    #[arg(long = "repo")]
    pub repo: Option<String>,
}

/// Shape of the capability catalog as emitted by the CLI.
/// This should mirror the internal CapabilityCatalog type from lib.rs.
#[derive(Debug, Serialize)]
pub struct DescribeCatalog {
    pub schema_version: String,
    pub binary_version: String,
    pub spine_version: String,
    pub object_kinds: Vec<ObjectKindProfile>,
    pub invariants: Vec<InvariantProfile>,
    pub metrics: Vec<MetricProfile>,
    pub phases: Vec<PhaseProfile>,
    pub tiers: Vec<TierProfile>,
    pub repositories: Vec<RepoProfile>,
}

// The following structs mirror the stable JSON schema for the catalog.
// They can be thin wrappers around the internal types returned by ChatDirector.

#[derive(Debug, Serialize)]
pub struct ObjectKindProfile {
    pub name: String,
    pub schema_ref: String,
    pub priority: String,
    pub allowed_phases: Vec<u8>,
    pub allowed_tiers: Vec<String>,
    pub required_invariants: Vec<String>,
    pub required_metrics: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct InvariantProfile {
    pub name: String,
    pub abbreviation: String,
    pub category: String,
    pub applies_to: Vec<String>,
    pub value_type: String,
    pub range: Range,
    pub tier_overrides: std::collections::HashMap<String, Range>,
}

#[derive(Debug, Serialize)]
pub struct MetricProfile {
    pub name: String,
    pub abbreviation: String,
    pub applies_to: Vec<String>,
    pub band: Range,
    pub tier_overrides: std::collections::HashMap<String, Range>,
}

#[derive(Debug, Serialize)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Serialize)]
pub struct PhaseProfile {
    pub id: u8,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct TierProfile {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct RepoProfile {
    pub repo_name: String,
    pub tier: String,
    pub implicit_deny: bool,
    pub allowed_object_kinds: Vec<String>,
    pub schema_whitelist: Vec<String>,
    pub default_paths: std::collections::HashMap<String, String>,
    pub policies: Vec<String>,
    pub authoring_hints: Vec<AuthoringHintProfile>,
}

#[derive(Debug, Serialize)]
pub struct AuthoringHintProfile {
    pub rule_id: String,
    pub description: String,
}

/// Entry point for the `describe` subcommand.
/// This should be called from `cli::run` or equivalent dispatcher.
pub fn run_describe(
    director: &ChatDirector,
    args: DescribeArgs,
    binary_version: &str,
) -> anyhow::Result<()> {
    let filter = CatalogFilter {
        object_kind: args.object_kind,
        phase: args.phase,
        tier: args.tier,
        repo: args.repo,
    };

    let internal_catalog = director.catalog(filter);

    // Map internal catalog into the stable DescribeCatalog shape if needed.
    // If your internal CapabilityCatalog already matches this shape and derives
    // Serialize, you can skip the mapping and serialize it directly.
    let catalog = DescribeCatalog {
        schema_version: internal_catalog.schema_version,
        binary_version: binary_version.to_string(),
        spine_version: internal_catalog.spine_version,
        object_kinds: internal_catalog.object_kinds,
        invariants: internal_catalog.invariants,
        metrics: internal_catalog.metrics,
        phases: internal_catalog.phases,
        tiers: internal_catalog.tiers,
        repositories: internal_catalog.repositories,
    };

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    serde_json::to_writer_pretty(&mut handle, &catalog)?;
    writeln!(&mut handle)?;

    Ok(())
}
