use clap::Parser;
use hpc_routing_validator::{validate_routing_system, RepoManifest, RoutingSpine};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Lint routing manifests and spine for the Horror.Place VM-constellation.
#[derive(Parser, Debug)]
#[command(name = "hpc-routing-lint")]
#[command(version)]
#[command(about = "Validate routing spine and repo manifests for uniqueness and sovereignty.", long_about = None)]
struct Cli {
    /// Root directory to scan for repo manifests.
    #[arg(long, default_value = ".")]
    root: String,

    /// Path to the routing spine instance JSON.
    #[arg(long, default_value = "spine/hpc-routing-spine-v1.json")]
    spine: String,

    /// If set, print detailed JSON report to stdout.
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Serialize)]
struct LintReport {
    manifest_files: Vec<String>,
    spine_file: String,
    error_count: usize,
    errors: Vec<hpc_routing_validator::RoutingValidationError>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let root = PathBuf::from(&cli.root);
    let spine_path = PathBuf::from(&cli.spine);

    // 1. Find all repo-manifest files.
    let manifest_paths = find_manifest_files(&root);

    if manifest_paths.is_empty() {
        eprintln!(
            "hpc-routing-lint: no repo-manifest.hpc.*.json files found under '{}'.",
            root.display()
        );
    }

    // 2. Load manifests.
    let mut manifests = Vec::new();
    let mut manifest_files_for_report = Vec::new();

    for path in manifest_paths {
        match fs::read_to_string(&path) {
            Ok(text) => match serde_json::from_str::<RepoManifest>(&text) {
                Ok(m) => {
                    manifest_files_for_report.push(path.to_string_lossy().to_string());
                    manifests.push(m);
                }
                Err(err) => {
                    eprintln!(
                        "hpc-routing-lint: failed to parse manifest {}: {}",
                        path.display(),
                        err
                    );
                }
            },
            Err(err) => {
                eprintln!(
                    "hpc-routing-lint: failed to read manifest {}: {}",
                    path.display(),
                    err
                );
            }
        }
    }

    if manifests.is_empty() {
        eprintln!("hpc-routing-lint: no valid manifests loaded; exiting with error.");
        std::process::exit(1);
    }

    // 3. Load routing spine.
    let spine_text = fs::read_to_string(&spine_path).map_err(|e| {
        anyhow::anyhow!(
            "Failed to read routing spine at {}: {}",
            spine_path.display(),
            e
        )
    })?;
    let spine: RoutingSpine = serde_json::from_str(&spine_text).map_err(|e| {
        anyhow::anyhow!(
            "Failed to parse routing spine at {}: {}",
            spine_path.display(),
            e
        )
    })?;

    // 4. Run validation.
    let errors = validate_routing_system(&manifests, &spine);
    let error_count = errors.len();

    let report = LintReport {
        manifest_files: manifest_files_for_report,
        spine_file: spine_path.to_string_lossy().to_string(),
        error_count,
        errors: errors.clone(),
    };

    // 5. Print report.
    if cli.json {
        // JSON report to stdout, human summary to stderr.
        let json = serde_json::to_string_pretty(&report)?;
        println!("{}", json);
        if error_count == 0 {
            eprintln!("hpc-routing-lint: OK ({} manifests, no errors).", report.manifest_files.len());
        } else {
            eprintln!(
                "hpc-routing-lint: {} error(s) detected. See JSON report above.",
                error_count
            );
        }
    } else {
        // Human summary to stdout; machine-readable JSON to stderr if there are errors.
        println!(
            "hpc-routing-lint: scanned {} manifest(s) and routing spine '{}'.",
            report.manifest_files.len(),
            report.spine_file
        );

        if error_count == 0 {
            println!("hpc-routing-lint: OK (no errors).");
        } else {
            println!("hpc-routing-lint: {} error(s) detected:", error_count);
            for err in &report.errors {
                println!(
                    "- {:?}: {} (objectKind={:?}, tier={:?}, repo={:?}, routeId={:?})",
                    err.code, err.message, err.objectKind, err.tier, err.repo, err.routeId
                );
            }

            let json = serde_json::to_string_pretty(&report)?;
            eprintln!("hpc-routing-lint JSON report:\n{}", json);
        }
    }

    // 6. Exit code: non-zero if errors exist.
    if error_count > 0 {
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
