//! Repo manifest loading and routing logic.
//!
//! Manifests define per-repo policies: allowed schemas, default paths,
//! tier classifications, and AI-specific authoring rules. This module
//! loads manifests and exposes routing queries for CHAT_DIRECTOR.

use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::errors::Error;
use crate::model::manifest_types::{RepoManifest, Tier, TargetRule, Policy, AuthoringHints};

/// Load all repo manifests from the configured directory.
pub fn load_all(config: &Config) -> Result<Vec<RepoManifest>, Error> {
    let mut manifests = Vec::new();
    
    for entry in std::fs::read_dir(&config.manifests_dir)
        .map_err(|e| Error::ManifestLoad {
            message: format!("Failed to read manifests dir: {}", e),
            path: config.manifests_dir.clone(),
        })? 
    {
        let entry = entry.map_err(|e| Error::ManifestLoad {
            message: format!("Failed to read manifest entry: {}", e),
            path: config.manifests_dir.clone(),
        })?;
        
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") 
            && path.file_name().and_then(|n| n.to_str()).map_or(false, |n| n.starts_with("repo-manifest.hpc."))
        {
            let manifest = load_single(&path)?;
            manifests.push(manifest);
        }
    }
    
    Ok(manifests)
}

/// Load a single manifest file.
fn load_single(path: &Path) -> Result<RepoManifest, Error> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| Error::ManifestLoad {
            message: format!("Failed to read manifest: {}", e),
            path: path.to_path_buf(),
        })?;
    
    let manifest: RepoManifest = serde_json::from_str(&content)
        .map_err(|e| Error::ManifestParse {
            message: format!("Failed to parse manifest: {}", e),
            path: path.to_path_buf(),
        })?;
    
    Ok(manifest)
}

/// Context for manifest-based validation queries.
pub struct ManifestContext<'a> {
    pub manifests_by_repo: &'a [RepoManifest],
}

impl<'a> ManifestContext<'a> {
    /// Find a manifest by repo name.
    pub fn find_manifest(&self, repo_name: &str) -> Option<&'a RepoManifest> {
        self.manifests_by_repo.iter().find(|m| m.repo == repo_name)
    }
    
    /// Resolve which repo owns a given ID (for cross-repo reference validation).
    pub fn lookup_repo_for_id(&self, id: &str) -> Option<String> {
        // Simplified: real version would consult registry schemas
        // to map ID prefixes to owning repos
        if id.starts_with("event.") {
            Some("HorrorPlace-Atrocity-Seeds".to_string())
        } else if id.starts_with("region.") {
            Some("HorrorPlace-Atrocity-Seeds".to_string())
        } else if id.starts_with("bundle.") {
            Some("HorrorPlace-Black-Archivum".to_string())
        } else {
            None
        }
    }
}

/// Suggest canonical target paths for an intent and objectKind.
///
/// Uses manifest `defaultTargetPaths` and `allowedSchemas` to propose
/// `(targetRepo, targetPath, tier)` tuples.
pub fn suggest_paths(
    intent: &str,
    object_kind: &str,
    manifests: &[RepoManifest],
    _spine: &crate::spine::SchemaSpine,
) -> Vec<crate::CanonicalTarget> {
    manifests
        .iter()
        .filter(|m| m.allows_object_kind(object_kind))
        .filter_map(|manifest| {
            manifest.default_path_for(object_kind).map(|template| {
                let target_path = expand_template(&template, intent);
                crate::CanonicalTarget {
                    target_repo: manifest.repo.clone(),
                    target_path,
                    tier: manifest.tier,
                    confidence: 1.0,
                }
            })
        })
        .collect()
}

/// Expand a path template with intent-derived values.
fn expand_template(template: &str, intent: &str) -> String {
    // Simple expansion; real version would use intent parsing
    template
        .replace("{id}", &slugify(intent))
        .replace("{tile_class}", "TODO_TILE")
        .replace("{tier}", "TODO_TIER")
}

/// Convert intent string to URL-safe slug.
fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

/// High-level manifest validation result.
#[derive(Debug)]
pub struct ManifestValidationResult {
    pub diagnostics: Vec<ManifestDiagnostic>,
    pub is_fatal: bool,
}

/// Structured manifest-related diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestDiagnostic {
    pub code: ManifestDiagnosticCode,
    pub message: String,
    pub severity: ManifestDiagnosticSeverity,
    pub charter_rationale: Option<String>,
    pub suggested_alternative_repo: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestDiagnosticSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestDiagnosticCode {
    InvalidTargetRepo,
    ObjectKindNotAllowedInRepo,
    TierPolicyViolation,
    OneFilePerRequestViolation,
    MissingDeadledgerRef,
    CrossRepoRefNotAllowed,
    RwfBelowTierMinimum,
}
