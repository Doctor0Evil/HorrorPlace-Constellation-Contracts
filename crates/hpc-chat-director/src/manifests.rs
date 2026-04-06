//! Repo manifest loading and routing logic.
//!
//! Manifests define per-repo policies: allowed schemas, default paths,
//! tier classifications, and AI-specific authoring rules. This module
//! loads manifests and exposes routing queries for CHAT_DIRECTOR.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::errors::Error;
use crate::model::spine_types::{ObjectKind, Tier};

/// Per-repo manifest, mirroring the v1 spec at a minimal level.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoManifest {
    pub repo_name: String,
    pub tier: Tier,
    pub allowed_object_kinds: Vec<ObjectKind>,
    pub schema_whitelist: Vec<String>,
    pub default_paths: HashMap<ObjectKind, String>,
    pub policies: Vec<Policy>,
    pub ci_checks: Vec<CiCheck>,
    #[serde(default)]
    pub implicit_deny: bool,
    #[serde(default)]
    pub authoring_hints: Vec<AuthoringHint>,
}

impl RepoManifest {
    /// Returns true if this manifest accepts the given objectKind.
    pub fn allows_object_kind(&self, kind: ObjectKind) -> bool {
        self.allowed_object_kinds.contains(&kind)
    }

    /// Look up a default path template for an objectKind.
    pub fn default_path_for(&self, kind: ObjectKind) -> Option<&str> {
        self.default_paths.get(&kind).map(String::as_str)
    }
}

/// Policy enumeration for per-repo rules.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Policy {
    OneFilePerRequest,
    NoRawNarrativeInTier,
    RequireDeadLedgerRef,
    MaxFileSizeBytes(u64),
    MandatoryPrismMeta,
    ForbiddenFields(Vec<String>),
}

/// CI check enum stub; expand as schemas solidify.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum CiCheck {
    SchemaValidate,
    InvariantEnforce,
}

/// Secondary metadata for governance / hints.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthoringHint {
    pub rule_id: String,
    pub description: String,
}

/// A routing explanation object for AI and tooling.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteExplanation {
    pub object_kind: ObjectKind,
    pub requested_tier: Tier,
    pub repo_name: String,
    pub resolved_tier: Tier,
    pub schema_whitelist: Vec<String>,
    pub default_path: Option<String>,
    pub applicable_policies: Vec<Policy>,
    pub authoring_hints: Vec<AuthoringHint>,
}

/// Routing errors, aligned with v1 manifest spec.
#[derive(Debug, thiserror::Error)]
pub enum RoutingError {
    #[error("No manifest accepts objectKind {object_kind:?} at tier {tier:?}")]
    RouteNotFound { object_kind: ObjectKind, tier: Tier },
    #[error("Tier violation for objectKind {object_kind:?} at repo {repo_name}")]
    TierViolation {
        object_kind: ObjectKind,
        repo_name: String,
    },
    #[error("Implicit deny in repo {repo_name} for objectKind {object_kind:?}")]
    ImplicitDeny {
        object_kind: ObjectKind,
        repo_name: String,
    },
}

/// Aggregated view of all repo manifests.
#[derive(Debug, Default)]
pub struct ManifestIndex {
    pub manifests_by_repo: HashMap<String, RepoManifest>,
}

impl ManifestIndex {
    /// Load all `repo-manifest.hpc.*.json` files from a directory.
    pub fn load_from_dir(dir: &Path) -> anyhow::Result<Self> {
        let mut manifests_by_repo = HashMap::new();

        if !dir.exists() {
            anyhow::bail!("manifest directory does not exist: {}", dir.display());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if !is_repo_manifest_file(&path) {
                continue;
            }

            let data = fs::read_to_string(&path)?;
            let manifest: RepoManifest = serde_json::from_str(&data)?;
            manifests_by_repo.insert(manifest.repo_name.clone(), manifest);
        }

        Ok(ManifestIndex { manifests_by_repo })
    }

    /// Return a manifest by repo name.
    pub fn get(&self, repo_name: &str) -> Option<&RepoManifest> {
        self.manifests_by_repo.get(repo_name)
    }

    /// Iterate over all manifests.
    pub fn iter(&self) -> impl Iterator<Item = &RepoManifest> {
        self.manifests_by_repo.values()
    }

    /// Core routing function: choose a repo and path for objectKind + tier.
    pub fn explain_route(
        &self,
        object_kind: ObjectKind,
        tier: Tier,
    ) -> Result<RouteExplanation, RoutingError> {
        // Deterministic scan order: sort keys for stability.
        let mut manifests: Vec<&RepoManifest> = self.manifests_by_repo.values().collect();
        manifests.sort_by(|a, b| a.repo_name.cmp(&b.repo_name));

        for manifest in manifests {
            if !manifest.allowed_object_kinds.contains(&object_kind) {
                if manifest.implicit_deny {
                    return Err(RoutingError::ImplicitDeny {
                        object_kind,
                        repo_name: manifest.repo_name.clone(),
                    });
                }
                continue;
            }

            // v1 spec: tier must match exactly; promotion/demotion decisions live elsewhere.
            if manifest.tier != tier {
                return Err(RoutingError::TierViolation {
                    object_kind,
                    repo_name: manifest.repo_name.clone(),
                });
            }

            let default_path = manifest
                .default_paths
                .get(&object_kind)
                .cloned();

            return Ok(RouteExplanation {
                object_kind,
                requested_tier: tier,
                repo_name: manifest.repo_name.clone(),
                resolved_tier: manifest.tier,
                schema_whitelist: manifest.schema_whitelist.clone(),
                default_path,
                applicable_policies: manifest.policies.clone(),
                authoring_hints: manifest.authoring_hints.clone(),
            });
        }

        Err(RoutingError::RouteNotFound { object_kind, tier })
    }
}

fn is_repo_manifest_file(path: &PathBuf) -> bool {
    if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
        name.starts_with("repo-manifest.hpc.") && name.ends_with(".json")
    } else {
        false
    }
}

/// Load a single manifest file.
fn load_single(path: &Path) -> Result<RepoManifest, Error> {
    let content = std::fs::read_to_string(path).map_err(|e| Error::ManifestLoad {
        message: format!("Failed to read manifest: {e}"),
        path: path.to_path_buf(),
    })?;

    let manifest: RepoManifest = serde_json::from_str(&content).map_err(|e| Error::ManifestParse {
        message: format!("Failed to parse manifest: {e}"),
        path: path.to_path_buf(),
    })?;

    Ok(manifest)
}

/// Load all repo manifests from the configured directory using the Error type.
pub fn load_all(config: &Config) -> Result<Vec<RepoManifest>, Error> {
    let mut manifests = Vec::new();

    for entry in std::fs::read_dir(&config.manifests_dir).map_err(|e| Error::ManifestLoad {
        message: format!("Failed to read manifests dir: {e}"),
        path: config.manifests_dir.clone(),
    })? {
        let entry = entry.map_err(|e| Error::ManifestLoad {
            message: format!("Failed to read manifest entry: {e}"),
            path: config.manifests_dir.clone(),
        })?;

        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json")
            && path
                .file_name()
                .and_then(|n| n.to_str())
                .map_or(false, |n| n.starts_with("repo-manifest.hpc."))
        {
            let manifest = load_single(&path)?;
            manifests.push(manifest);
        }
    }

    Ok(manifests)
}

/// Context for manifest-based validation queries.
pub struct ManifestContext<'a> {
    pub manifests_by_repo: &'a [RepoManifest],
}

impl<'a> ManifestContext<'a> {
    /// Find a manifest by repo name.
    pub fn find_manifest(&self, repo_name: &str) -> Option<&'a RepoManifest> {
        self.manifests_by_repo
            .iter()
            .find(|m| m.repo_name == repo_name)
    }

    /// Resolve which repo owns a given ID (for cross-repo reference validation).
    pub fn lookup_repo_for_id(&self, id: &str) -> Option<String> {
        // Simplified: real version would consult registry schemas
        // to map ID prefixes to owning repos.
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
/// Uses manifest `default_paths` and `schema_whitelist` to propose
/// `(targetRepo, targetPath, tier)` tuples.
pub fn suggest_paths(
    intent: &str,
    object_kind: ObjectKind,
    manifests: &[RepoManifest],
    _spine: &crate::spine::SchemaSpine,
) -> Vec<crate::CanonicalTarget> {
    manifests
        .iter()
        .filter(|m| m.allows_object_kind(object_kind))
        .filter_map(|manifest| {
            manifest.default_path_for(object_kind).map(|template| {
                let target_path = expand_template(template, intent);
                crate::CanonicalTarget {
                    target_repo: manifest.repo_name.clone(),
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
    // Simple expansion; real version would use intent parsing.
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
