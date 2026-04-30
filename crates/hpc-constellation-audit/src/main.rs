// crates/hpc-constellation-audit/src/main.rs
use clap::Parser;
use hpc_routing_validator::{RepoManifest, RoutingSpine};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod wiring;
use wiring::WiringPlan;

#[derive(Parser, Debug)]
#[command(name = "hpc-constellation-audit")]
#[command(version)]
#[command(about = "Audit wiring plan, repo manifests, and routing spine for consistency.", long_about = None)]
struct Cli {
    #[arg(long, default_value = ".")]
    root: String,

    #[arg(long, default_value = "docs/constellation-repo-wiring-plan.v1.json")]
    wiring: String,

    #[arg(long, default_value = "spine/hpc-routing-spine-v1.json")]
    spine: String,

    #[arg(long)]
    json: bool,

    #[arg(long)]
    emit_fixes: bool
}

#[derive(Debug, Clone, Serialize)]
pub enum AuditIssueKind {
    WiringMissingManifest,
    WiringTierMismatch,
    WiringObjectKindMissingInManifest,
    ManifestObjectKindMissingInWiring,
    WiringRepoMissingInSpine,
    WiringObjectKindMissingInSpine,
    SpineRouteRepoMissingInWiring
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditIssue {
    pub kind: AuditIssueKind,
    pub message: String,
    pub repoName: Option<String>,
    pub objectKind: Option<String>,
    pub tier: Option<String>
}

#[derive(Debug, Clone, Serialize)]
pub enum FixKind {
    ManifestAddAllowedObjectKind,
    ManifestRemoveAllowedObjectKind,
    ManifestAddMissingRepoManifest,
    ManifestAdjustTier,
    WiringAddObjectKindForRepo,
    WiringRemoveObjectKindForRepo,
    WiringAdjustTier,
    WiringAddRepo,
    WiringRemoveRepo,
    SpineAddObjectKindEntry,
    SpineRemoveObjectKindEntry,
    SpineAddRepoReference,
    SpineRemoveRepoReference
}

#[derive(Debug, Clone, Serialize)]
pub struct FixSuggestion {
    pub kind: FixKind,
    pub targetFile: String,
    pub description: String,
    pub jsonPointer: String,
    pub patchValue: serde_json::Value
}

#[derive(Debug, Serialize)]
pub struct AuditReport {
    pub wiring_file: String,
    pub spine_file: String,
    pub manifest_files: Vec<String>,
    pub issue_count: usize,
    pub issues: Vec<AuditIssue>,
    pub fixes: Vec<FixSuggestion>
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let root = PathBuf::from(&cli.root);
    let wiring_path = PathBuf::from(&cli.wiring);
    let spine_path = PathBuf::from(&cli.spine);

    let wiring_text = fs::read_to_string(&wiring_path)?;
    let wiring_plan: WiringPlan = serde_json::from_str(&wiring_text)?;

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

    let spine_text = fs::read_to_string(&spine_path)?;
    let spine: RoutingSpine = serde_json::from_str(&spine_text)?;

    let issues = audit(&wiring_plan, &manifests, &spine);
    let fixes = if cli.emit_fixes {
        generate_fixes(&issues, &wiring_plan)
    } else {
        Vec::new()
    };

    let report = AuditReport {
        wiring_file: wiring_path.to_string_lossy().to_string(),
        spine_file: spine_path.to_string_lossy().to_string(),
        manifest_files: manifest_files_for_report,
        issue_count: issues.len(),
        issues,
        fixes
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
            if !report.fixes.is_empty() {
                println!("Fix suggestions available (enable --json --emit-fixes to see full patch data).");
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

fn audit(
    wiring: &WiringPlan,
    manifests: &HashMap<String, RepoManifest>,
    spine: &RoutingSpine
) -> Vec<AuditIssue> {
    let mut issues = Vec::new();

    let mut spine_repos: HashSet<String> = HashSet::new();
    let mut spine_object_kinds: HashSet<String> = HashSet::new();

    for ok in &spine.objectKinds {
        spine_object_kinds.insert(ok.name.clone());
        for route in &ok.routes {
            spine_repos.insert(format!("{:?}", route.repo));
        }
    }

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
            "HorrorPlace-Obscura-Nexus" => "HorrorPlaceObscuraNexus".to_string(),
            "HorrorPlace-Liminal-Continuum" => "HorrorPlaceLiminalContinuum".to_string(),
            "HorrorPlace-Process-Gods-Research" => "HorrorPlaceProcessGodsResearch".to_string(),
            "HorrorPlace-Redacted-Chronicles" => "HorrorPlaceRedactedChronicles".to_string(),
            "HorrorPlace-Neural-Resonance-Lab" => "HorrorPlaceNeuralResonanceLab".to_string(),
            other => other.to_string()
        }
    }

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
                    tier: Some(repo.tier.clone())
                });
                continue;
            }
        };

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
                tier: Some(wiring_tier.clone())
            });
        }

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
                    tier: Some(repo.tier.clone())
                });
            }
        }

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
                    tier: Some(repo.tier.clone())
                });
            }
        }
    }

    let wiring_repo_names: HashSet<String> = wiring
        .repos
        .iter()
        .map(|r| wiring_repo_to_manifest_key(&r.repoName))
        .collect();

    for repo in &wiring.repos {
        let key = wiring_repo_to_manifest_key(&repo.repoName);
        if !spine_repos.contains(&key) {
            issues.push(AuditIssue {
                kind: AuditIssueKind::WiringRepoMissingInSpine,
                message: format!(
                    "Repo {} (key={}) appears in wiring plan but never appears as a route target in the routing spine.",
                    repo.repoName, key
                ),
                repoName: Some(repo.repoName.clone()),
                objectKind: None,
                tier: Some(repo.tier.clone())
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
                    tier: Some(repo.tier.clone())
                });
            }
        }
    }

    for spine_repo_key in spine_repos {
        if !wiring_repo_names.contains(&spine_repo_key) {
            issues.push(AuditIssue {
                kind: AuditIssueKind::SpineRouteRepoMissingInWiring,
                message: format!(
                    "Routing spine references repo key {} that does not appear in the wiring plan.",
                    spine_repo_key
                ),
                repoName: None,
                objectKind: None,
                tier: None
            });
        }
    }

    issues
}

fn generate_fixes(issues: &[AuditIssue], wiring: &WiringPlan) -> Vec<FixSuggestion> {
    let mut fixes = Vec::new();

    let mut wiring_object_kinds_per_repo: HashMap<String, HashSet<String>> = HashMap::new();
    for repo in &wiring.repos {
        let mut set = HashSet::new();
        for ok in &repo.objectKinds {
            set.insert(ok.name.clone());
        }
        wiring_object_kinds_per_repo.insert(repo.repoName.clone(), set);
    }

    for issue in issues {
        match issue.kind {
            AuditIssueKind::WiringObjectKindMissingInManifest => {
                if let (Some(repo_name), Some(ref object_kind)) = (&issue.repoName, &issue.objectKind)
                {
                    let manifest_file = format!("manifests/repo-manifest.hpc.{}.json", repo_name);
                    fixes.push(FixSuggestion {
                        kind: FixKind::ManifestAddAllowedObjectKind,
                        targetFile: manifest_file,
                        description: format!(
                            "Add objectKind={} to allowedObjectKinds for repo {} so it matches the wiring plan.",
                            object_kind, repo_name
                        ),
                        jsonPointer: "/allowedObjectKinds".to_string(),
                        patchValue: serde_json::json!({
                            "op": "add",
                            "value": object_kind
                        })
                    });
                }
            }
            AuditIssueKind::ManifestObjectKindMissingInWiring => {
                if let (Some(repo_name), Some(ref object_kind)) = (&issue.repoName, &issue.objectKind)
                {
                    let manifest_file = format!("manifests/repo-manifest.hpc.{}.json", repo_name);
                    fixes.push(FixSuggestion {
                        kind: FixKind::ManifestRemoveAllowedObjectKind,
                        targetFile: manifest_file,
                        description: format!(
                            "Remove objectKind={} from allowedObjectKinds for repo {} or add it to the wiring plan.",
                            object_kind, repo_name
                        ),
                        jsonPointer: "/allowedObjectKinds".to_string(),
                        patchValue: serde_json::json!({
                            "op": "remove",
                            "value": object_kind
                        })
                    });
                }
            }
            AuditIssueKind::WiringMissingManifest => {
                if let Some(repo_name) = &issue.repoName {
                    let manifest_file = format!("manifests/repo-manifest.hpc.{}.json", repo_name);
                    let object_kinds = wiring_object_kinds_per_repo
                        .get(repo_name)
                        .cloned()
                        .unwrap_or_default();
                    fixes.push(FixSuggestion {
                        kind: FixKind::ManifestAddMissingRepoManifest,
                        targetFile: manifest_file,
                        description: format!(
                            "Create a new manifest for repo {} based on wiring plan (allowedObjectKinds from wiring).",
                            repo_name
                        ),
                        jsonPointer: "".to_string(),
                        patchValue: serde_json::json!({
                            "schemaVersion": "1.0.0",
                            "schemaRef": "schemas/core/repo-manifest-v1.json",
                            "repoName": repo_name,
                            "tier": issue.tier.clone().unwrap_or_else(|| "T1-core".to_string()),
                            "kind": "TODO_KIND",
                            "description": "TODO: fill description",
                            "visibility": "private",
                            "implicitDeny": true,
                            "allowedObjectKinds": object_kinds.into_iter().collect::<Vec<_>>()
                        })
                    });
                }
            }
            AuditIssueKind::WiringTierMismatch => {
                if let (Some(repo_name), Some(wiring_tier)) = (&issue.repoName, &issue.tier) {
                    let manifest_file = format!("manifests/repo-manifest.hpc.{}.json", repo_name);
                    fixes.push(FixSuggestion {
                        kind: FixKind::ManifestAdjustTier,
                        targetFile: manifest_file,
                        description: format!(
                            "Align manifest tier with wiring plan for repo {}. Consider changing manifest.tier to {} or updating wiring.",
                            repo_name, wiring_tier
                        ),
                        jsonPointer: "/tier".to_string(),
                        patchValue: serde_json::json!({
                            "op": "replace",
                            "value": wiring_tier
                        })
                    });
                }
            }
            _ => {}
        }
    }

    fixes
}
