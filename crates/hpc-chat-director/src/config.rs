//! Environment detection and path resolution for CHAT_DIRECTOR.
//!
//! This module discovers the constellation layout across heterogeneous
//! checkouts (monorepo, multi-repo, nested worktrees) and resolves paths
//! to the schema spine, repo manifests, and registry directories.

use std::path::{Path, PathBuf};
use crate::errors::Error;

/// Runtime configuration for CHAT_DIRECTOR.
///
/// Resolved from environment variables, CLI flags, and filesystem discovery.
#[derive(Debug, Clone)]
pub struct Config {
    /// Root of the constellation checkout.
    pub root: PathBuf,
    /// Path to the schema spine index.
    pub spine_index: PathBuf,
    /// Directory containing repo manifests.
    pub manifests_dir: PathBuf,
    /// Directory containing registry examples and formats.
    pub registry_dir: PathBuf,
    /// Directory for generated output (apply target).
    pub output_dir: Option<PathBuf>,
    /// Execution context mode affecting strictness.
    pub context_mode: ContextMode,
    /// Active spine version (from env or spine file).
    pub spine_version: String,
    /// Accepted envelope versions for validation.
    pub envelope_versions: Vec<String>,
}

/// Execution context that adjusts validation strictness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextMode {
    /// Local development: warnings for non-critical issues.
    Local,
    /// CI/CD: hard-fail on any policy violation.
    CI,
    /// Editor plugin: minimal output, fast checks.
    EditorPlugin,
}

impl Config {
    /// Detect and resolve the constellation environment from a root path.
    ///
    /// Searches for required files and directories, applying fallbacks
    /// and environment variable overrides. Returns an error if critical
    /// components are missing.
    pub fn detect(root: &Path) -> Result<Self, Error> {
        let root = root.canonicalize().map_err(|e| Error::Config {
            message: format!("Cannot canonicalize root path: {}", e),
            path: root.to_path_buf(),
        })?;

        // Environment variable overrides
        let spine_version = std::env::var("HPC_SPINE_VERSION")
            .unwrap_or_else(|_| "v1".to_string());
        
        let envelope_versions = std::env::var("HPC_ENVELOPE_VERSIONS")
            .map(|v| v.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_else(|_| vec!["v1".to_string()]);

        // Resolve spine index path
        let spine_index = root.join("schemas/core/schema-spine-index-v1.json");
        if !spine_index.exists() {
            return Err(Error::Config {
                message: "Schema spine index not found".into(),
                path: spine_index.clone(),
            });
        }

        // Resolve manifests directory
        let manifests_dir = root.join("manifests");
        if !manifests_dir.exists() {
            return Err(Error::Config {
                message: "Manifests directory not found".into(),
                path: manifests_dir.clone(),
            });
        }

        // Resolve registry directory
        let registry_dir = root.join("registry");
        if !registry_dir.exists() {
            return Err(Error::Config {
                message: "Registry directory not found".into(),
                path: registry_dir.clone(),
            });
        }

        // Context mode from env or default
        let context_mode = match std::env::var("HPC_CONTEXT_MODE").as_deref() {
            Ok("ci") | Ok("CI") => ContextMode::CI,
            Ok("editor") | Ok("EDITOR") => ContextMode::EditorPlugin,
            _ => ContextMode::Local,
        };

        Ok(Self {
            root,
            spine_index,
            manifests_dir,
            registry_dir,
            output_dir: None,
            context_mode,
            spine_version,
            envelope_versions,
        })
    }

    /// Return a summary of the detected environment for AI discovery.
    pub fn summary(&self) -> EnvironmentSummary {
        EnvironmentSummary {
            root: self.root.clone(),
            spine_version: self.spine_version.clone(),
            context_mode: self.context_mode,
            // Repos and objectKinds populated by caller via manifest loading
            repos: Vec::new(),
            object_kinds: Vec::new(),
        }
    }

    /// Set the output directory for `apply` operations.
    pub fn with_output_dir(mut self, dir: PathBuf) -> Self {
        self.output_dir = Some(dir);
        self
    }
}

/// Summary of the detected constellation environment.
///
/// Returned by `Config::summary()` for AI pre-flight discovery.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentSummary {
    /// Root path of the constellation checkout.
    pub root: PathBuf,
    /// Active schema spine version.
    pub spine_version: String,
    /// Execution context mode affecting strictness.
    pub context_mode: ContextMode,
    /// Discovered repos with name, tier, and path.
    pub repos: Vec<RepoSummary>,
    /// Available objectKind values in this environment.
    pub object_kinds: Vec<String>,
}

/// Summary of a discovered repository.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoSummary {
    pub name: String,
    pub tier: crate::model::manifest_types::Tier,
    pub path: PathBuf,
}

/// Suggest canonical target paths for an intent and objectKind.
///
/// Uses manifest `defaultTargetPaths` and `allowedSchemas` to propose
/// `(targetRepo, targetPath, tier)` tuples. Returns empty Vec if no
/// valid routing can be determined.
pub fn suggest_paths(
    intent: &str,
    object_kind: &str,
    manifests: &[crate::model::manifest_types::RepoManifest],
    spine: &crate::spine::SchemaSpine,
) -> Vec<crate::CanonicalTarget> {
    use crate::model::manifest_types::RepoManifest;
    
    manifests
        .iter()
        .filter(|m| m.allows_object_kind(object_kind))
        .filter_map(|manifest| {
            manifest.default_path_for(object_kind).map(|template| {
                // Simple template expansion: {id} -> placeholder
                let target_path = template
                    .replace("{id}", "TODO_ID")
                    .replace("{tile_class}", "TODO_TILE");
                
                crate::CanonicalTarget {
                    target_repo: manifest.repo.clone(),
                    target_path,
                    tier: manifest.tier,
                    confidence: 1.0, // Could be refined based on manifest specificity
                }
            })
        })
        .collect()
}
