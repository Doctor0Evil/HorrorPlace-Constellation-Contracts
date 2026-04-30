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
// crates/hpc-constellation-audit/src/main.rs (continued)

fn audit(
    wiring: &WiringPlan,
    manifests: &HashMap<String, RepoManifest>,
    spine: &RoutingSpine,
) -> Vec<AuditIssue> {
    let mut issues = Vec::new();

    // Build quick lookups.
    let mut spine_repos: HashSet<String> = HashSet::new();
    let mut spine_object_kinds: HashSet<String> = HashSet::new();

    for ok in &spine.objectKinds {
        spine_object_kinds.insert(ok.name.clone());
        for route in &ok.routes {
            spine_repos.insert(format!("{:?}", route.repo));
        }
    }

    // Map repoName strings in wiring to manifest repo key ("HorrorPlace-Atrocity-Seeds" -> "HorrorPlaceAtrocitySeeds" style).
    let manifest_repo_keys: HashSet<String> = manifests.keys().cloned().collect();

    // Helper: convert wiring repoName to manifest key via RepoName enum Debug format.
    fn wiring_repo_to_manifest_key(name: &str) -> String {
        match name {
            "Horror.Place" => "HorrorPlace".to_string(),
            "HorrorPlace-Constellation-Contracts" => "HorrorPlaceConstellationContracts".to_string(),
            "HorrorPlace-Codebase-of-Death" => "HorrorPlaceCodebaseOfDeath".to_string(),
            "HorrorPlace-Black-Archivum" => "HorrorPlaceBlackArchivum".to_string(),
            "HorrorPlace-Spectral-Foundry" => "HorrorPlaceSpectralFoundry".to_string(),
            "HorrorPlace-Atrocity-Seeds" => "HorrorPlaceAtrocitySeeds".to_string(),
            "Horror.Place-Orchestrator" => "HorrorPlaceOrchestrator".to_string(),
            "HorrorPlace-Dead-Ledger-Network" => "HorrorPlaceDeadLedgerNetwork".to_string(),
            "HorrorPlace-RotCave" => "HorrorPlaceRotCave".to_string(),
            other => other.to_string(),
        }
    }

    // Build a map of wiring->objectKinds per repo for later checks.
    let mut wiring_object_kinds_per_repo: HashMap<String, HashSet<String>> = HashMap::new();
    for repo in &wiring.repos {
        let key = wiring_repo_to_manifest_key(&repo.repoName);
        wiring_object_kinds_per_repo
            .entry(key)
            .or_default()
            .extend(repo.objectKinds.iter().map(|ok| ok.name.clone()));
    }

    // 1. Wiring ↔ Manifests
    for repo in &wiring.repos {
        let manifest_key = wiring_repo_to_manifest_key(&repo.repoName);
        let manifest = match manifests.get(&manifest_key) {
            Some(m) => m,
            None => {
                issues.push(AuditIssue {
                    kind: AuditIssueKind::WiringMissingManifest,
                    message: format!(
                        "Wiring lists repo {} but no corresponding manifest was found (key={}).",
                        repo.repoName, manifest_key
                    ),
                    repoName: Some(repo.repoName.clone()),
                    objectKind: None,
                    tier: Some(repo.tier.clone()),
                });
                continue;
            }
        };

        // Tier match.
        let wiring_tier = &repo.tier;
        let manifest_tier = format!("{:?}", manifest.tier);
        if !manifest_tier.contains(wiring_tier) {
            issues.push(AuditIssue {
                kind: AuditIssueKind::WiringTierMismatch,
                message: format!(
                    "Tier mismatch for repo {}: wiring tier={} vs manifest tier={}.",
                    repo.repoName, wiring_tier, manifest_tier
                ),
                repoName: Some(repo.repoName.clone()),
                objectKind: None,
                tier: Some(wiring_tier.clone()),
            });
        }

        // ObjectKinds: each wiring objectKind must appear in manifest.allowedObjectKinds.
        let allowed: HashSet<String> = manifest.allowedObjectKinds.iter().cloned().collect();
        for ok in &repo.objectKinds {
            if !allowed.contains(&ok.name) {
                issues.push(AuditIssue {
                    kind: AuditIssueKind::WiringObjectKindMissingInManifest,
                    message: format!(
                        "Repo {} wiring declares objectKind={} but manifest.allowedObjectKinds does not include it.",
                        repo.repoName, ok.name
                    ),
                    repoName: Some(repo.repoName.clone()),
                    objectKind: Some(ok.name.clone()),
                    tier: Some(repo.tier.clone()),
                });
            }
        }

        // Manifest objectKinds that are not in wiring.
        for mk in &manifest.allowedObjectKinds {
            if !repo.objectKinds.iter().any(|ok| &ok.name == mk) {
                issues.push(AuditIssue {
                    kind: AuditIssueKind::ManifestObjectKindMissingInWiring,
                    message: format!(
                        "Repo {} manifest.allowedObjectKinds includes {} but wiring plan for this repo does not.",
                        repo.repoName, mk
                    ),
                    repoName: Some(repo.repoName.clone()),
                    objectKind: Some(mk.clone()),
                    tier: Some(repo.tier.clone()),
                });
            }
        }
    }

    // 2. Wiring ↔ Routing spine
    // For now, we check: (a) each wiring repo appears as a route target if the wiring implies routing, and
    // (b) each wiring objectKind has a spine entry.
    let wiring_repo_names: HashSet<String> =
        wiring.repos.iter().map(|r| wiring_repo_to_manifest_key(&r.repoName)).collect();

    for repo in &wiring.repos {
        let key = wiring_repo_to_manifest_key(&repo.repoName);
        if !spine_repos.contains(&key) {
            // Not all repos must appear in spine, but we flag it to keep topology explicit.
            issues.push(AuditIssue {
                kind: AuditIssueKind::WiringRepoMissingInSpine,
                message: format!(
                    "Repo {} (key={}) appears in wiring plan but never appears as a route target in the routing spine.",
                    repo.repoName, key
                ),
                repoName: Some(repo.repoName.clone()),
                objectKind: None,
                tier: Some(repo.tier.clone()),
            });
        }
        for ok in &repo.objectKinds {
            if !spine_object_kinds.contains(&ok.name) {
                issues.push(AuditIssue {
                    kind: AuditIssueKind::WiringObjectKindMissingInSpine,
                    message: format!(
                        "Wiring declares objectKind={} for repo {} but the routing spine has no entry with this name.",
                        ok.name, repo.repoName
                    ),
                    repoName: Some(repo.repoName.clone()),
                    objectKind: Some(ok.name.clone()),
                    tier: Some(repo.tier.clone()),
                });
            }
        }
    }

    // 3. SpineRouteRepoMissingInWiring: ensure all repos in spine are in wiring.
    for spine_repo_key in spine_repos {
        if !wiring_repo_names.contains(&spine_repo_key) {
            issues.push(AuditIssue {
                kind: AuditIssueKind::SpineRouteRepoMissingInWiring,
                message: format!(
                    "Routing spine references repo key {:?} that does not appear in the wiring plan.",
                    spine_repo_key
                ),
                repoName: None,
                objectKind: None,
                tier: None,
            });
        }
    }

    issues
}
