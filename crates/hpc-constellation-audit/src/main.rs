// crates/hpc-constellation-audit/src/main.rs
use clap::Parser;
use hpc_routing_validator::{RepoManifest, RoutingSpine};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod wiring;
use wiring::{WiringPlan, WiringRepo};

#[derive(Parser, Debug)]
#[command(name = "hpc-constellation-audit")]
#[command(version)]
#[command(about = "Audit wiring plan, repo manifests, and routing spine for consistency.", long_about = None)]
struct Cli {
    /// Root directory for repo-manifest files and spine.
    #[arg(long, default_value = ".")]
    root: String,

    /// Wiring plan JSON file.
    #[arg(long, default_value = "docs/constellation-repo-wiring-plan.v1.json")]
    wiring: String,

    /// Routing spine instance JSON file.
    #[arg(long, default_value = "spine/hpc-routing-spine-v1.json")]
    spine: String,

    /// Print JSON report to stdout.
    #[arg(long)]
    json: bool
}

#[derive(Debug, Clone, Serialize)]
pub enum AuditIssueKind {
    WiringMissingManifest,
    WiringTierMismatch,
    WiringObjectKindMissingInManifest,
    ManifestObjectKindMissingInWiring,
    WiringRepoMissingInSpine,
    WiringObjectKindMissingInSpine,
    SpineRouteRepoMissingInWiring,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditIssue {
    pub kind: AuditIssueKind,
    pub message: String,
    pub repoName: Option<String>,
    pub objectKind: Option<String>,
    pub tier: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuditReport {
    pub wiring_file: String,
    pub spine_file: String,
    pub manifest_files: Vec<String>,
    pub issue_count: usize,
    pub issues: Vec<AuditIssue>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let root = PathBuf::from(&cli.root);
    let wiring_path = PathBuf::from(&cli.wiring);
    let spine_path = PathBuf::from(&cli.spine);

    // 1. Load wiring plan.
    let wiring_text = fs::read_to_string(&wiring_path)?;
    let wiring_plan: WiringPlan = serde_json::from_str(&wiring_text)?;

    // 2. Load manifests.
    let manifest_paths = find_manifest_files(&root);
    let mut manifests: HashMap<String, RepoManifest> = HashMap::new();
    let mut manifest_files_for_report = Vec::new();

    for path in manifest_paths {
        let text = match fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to read manifest {}: {}", path.display(), e);
                continue;
            }
        };
        match serde_json::from_str::<RepoManifest>(&text) {
            Ok(m) => {
                manifest_files_for_report.push(path.to_string_lossy().to_string());
                manifests.insert(format!("{:?}", m.repoName), m);
            }
            Err(e) => {
                eprintln!("Failed to parse manifest {}: {}", path.display(), e);
            }
        }
    }

    // 3. Load routing spine.
    let spine_text = fs::read_to_string(&spine_path)?;
    let spine: RoutingSpine = serde_json::from_str(&spine_text)?;

    // 4. Run audit.
    let issues = audit(&wiring_plan, &manifests, &spine);
    let report = AuditReport {
        wiring_file: wiring_path.to_string_lossy().to_string(),
        spine_file: spine_path.to_string_lossy().to_string(),
        manifest_files: manifest_files_for_report,
        issue_count: issues.len(),
        issues,
    };

    if cli.json {
        let json = serde_json::to_string_pretty(&report)?;
        println!("{}", json);
    } else {
        println!(
            "hpc-constellation-audit: wiring={}, spine={}",
            report.wiring_file, report.spine_file
        );
        println!("Loaded {} manifest(s).", report.manifest_files.len());
        if report.issue_count == 0 {
            println!("No issues found.");
        } else {
            println!("Found {} issue(s):", report.issue_count);
            for issue in &report.issues {
                println!(
                    "- {:?}: {} (repo={:?}, objectKind={:?}, tier={:?})",
                    issue.kind, issue.message, issue.repoName, issue.objectKind, issue.tier
                );
            }
        }
    }

    if report.issue_count > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn find_manifest_files(root: &Path) -> Vec<PathBuf> {
    let mut results = Vec::new();
    let pattern = "repo-manifest.hpc.";

    for entry in WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.starts_with(pattern) && name.ends_with(".json") {
                results.push(path.to_path_buf());
            }
        }
    }

    results
}
