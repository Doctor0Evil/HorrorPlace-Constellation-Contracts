//! Environment detection and path resolution for CHAT_DIRECTOR.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::errors::ChatDirectorError;
use crate::model::manifest_types::{RepoManifest, Tier};
use crate::model::{CapabilityCatalog, InvariantSummary, MetricSummary, PhaseSummary, RepoSummary};
use crate::spine::SpineIndex;

/// Execution context that adjusts validation strictness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextMode {
    Local,
    CI,
    EditorPlugin,
}

/// Runtime configuration for CHAT_DIRECTOR.
#[derive(Debug, Clone)]
pub struct Config {
    root: PathBuf,
    manifests: Vec<RepoManifest>,
    context_mode: ContextMode,
    spine_version: String,
    envelope_versions: Vec<String>,
    output_dir: Option<PathBuf>,
}

impl Config {
    /// Detect the constellation root and load manifests.
    pub fn detect(root_hint: &Path) -> Result<Self, ChatDirectorError> {
        let root = detect_root(root_hint)?;
        let manifests = load_manifests(&root)?;

        let spine_version = env::var("HPC_SPINE_VERSION").unwrap_or_else(|_| "v1".to_string());

        let envelope_versions = env::var("HPC_ENVELOPE_VERSIONS")
            .map(|v| v.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_else(|_| vec!["v1".to_string()]);

        let context_mode = match env::var("HPC_CONTEXT_MODE").as_deref() {
            Ok("ci") | Ok("CI") => ContextMode::CI,
            Ok("editor") | Ok("EDITOR") => ContextMode::EditorPlugin,
            _ => ContextMode::Local,
        };

        Ok(Config {
            root,
            manifests,
            context_mode,
            spine_version,
            envelope_versions,
            output_dir: None,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn manifests(&self) -> &[RepoManifest] {
        &self.manifests
    }

    pub fn context_mode(&self) -> ContextMode {
        self.context_mode
    }

    pub fn spine_version(&self) -> &str {
        &self.spine_version
    }

    pub fn envelope_versions(&self) -> &[String] {
        &self.envelope_versions
    }

    pub fn output_dir(&self) -> Option<&Path> {
        self.output_dir.as_deref()
    }

    pub fn with_output_dir(mut self, dir: PathBuf) -> Self {
        self.output_dir = Some(dir);
        self
    }

    /// Build a capability catalog from this config and a loaded spine.
    pub fn catalog_with_spine(&self, spine: &SpineIndex) -> CapabilityCatalog {
        let invariants = spine
            .inner()
            .invariants
            .iter()
            .map(|inv| InvariantSummary {
                name: inv.name.clone(),
                description: inv.description.clone(),
                min: inv.min,
                max: inv.max,
            })
            .collect();

        let metrics = spine
            .inner()
            .metrics
            .iter()
            .map(|m| MetricSummary {
                name: m.name.clone(),
                description: m.description.clone(),
                target_min: m.target_min,
                target_max: m.target_max,
            })
            .collect();

        let mut object_kinds = Vec::new();
        for family in &spine.inner().contract_families {
            if !object_kinds.contains(&family.object_kind) {
                object_kinds.push(family.object_kind.clone());
            }
        }

        let repos = self
            .manifests
            .iter()
            .map(|m| RepoSummary {
                name: m.repo_name.clone(),
                tier: m.tier.as_str().to_string(),
                path: repo_path_for(&self.root, &m.repo_name)
                    .unwrap_or_else(|| String::from(".")),
            })
            .collect();

        let phases = vec![
            PhaseSummary {
                id: crate::spine::Phase::Schema0.id(),
                name: crate::spine::Phase::Schema0.name().to_string(),
                description: crate::spine::Phase::Schema0.description().to_string(),
            },
            PhaseSummary {
                id: crate::spine::Phase::Registry1.id(),
                name: crate::spine::Phase::Registry1.name().to_string(),
                description: crate::spine::Phase::Registry1.description().to_string(),
            },
            PhaseSummary {
                id: crate::spine::Phase::Bundles2.id(),
                name: crate::spine::Phase::Bundles2.name().to_string(),
                description: crate::spine::Phase::Bundles2.description().to_string(),
            },
            PhaseSummary {
                id: crate::spine::Phase::LuaPolicy3.id(),
                name: crate::spine::Phase::LuaPolicy3.name().to_string(),
                description: crate::spine::Phase::LuaPolicy3.description().to_string(),
            },
            PhaseSummary {
                id: crate::spine::Phase::Adapters4.id(),
                name: crate::spine::Phase::Adapters4.name().to_string(),
                description: crate::spine::Phase::Adapters4.description().to_string(),
            },
        ];

        CapabilityCatalog {
            spine_version: Some(spine.version().to_string()),
            available_object_kinds: object_kinds,
            available_repos: repos,
            invariants,
            metrics,
            phases,
        }
    }
}

fn detect_root(root_hint: &Path) -> Result<PathBuf, ChatDirectorError> {
    if root_hint.join("schemas").exists() {
        return Ok(root_hint.to_path_buf());
    }

    if let Ok(env_root) = env::var("HPC_CONSTELLATION_ROOT") {
        let path = PathBuf::from(env_root);
        if path.join("schemas").exists() {
            return Ok(path);
        }
    }

    let mut current = env::current_dir().map_err(|e| {
        ChatDirectorError::Config(format!("failed to read current directory: {}", e))
    })?;
    loop {
        if current.join("schemas").exists() && current.join("Cargo.toml").exists() {
            return Ok(current);
        }
        if !current.pop() {
            break;
        }
    }

    Err(ChatDirectorError::Config(
        "failed to detect constellation root".to_string(),
    ))
}

fn load_manifests(root: &Path) -> Result<Vec<RepoManifest>, ChatDirectorError> {
    let dir = root.join("manifests");
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut manifests = Vec::new();
    for entry in fs::read_dir(&dir).map_err(|e| {
        ChatDirectorError::Config(format!("failed to read manifests directory: {}", e))
    })? {
        let entry = entry.map_err(|e| {
            ChatDirectorError::Config(format!("failed to read manifests directory: {}", e))
        })?;
        let path = entry.path();
        if path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with("repo-manifest.hpc.") && n.ends_with(".json"))
            .unwrap_or(false)
        {
            let data = fs::read_to_string(&path)
                .map_err(|e| ChatDirectorError::Io(path.clone(), e))?;
            let manifest: RepoManifest =
                serde_json::from_str(&data).map_err(ChatDirectorError::InvalidSpine)?;
            manifests.push(manifest);
        }
    }

    Ok(manifests)
}

fn repo_path_for(root: &Path, repo_name: &str) -> Option<String> {
    let candidate = root.join(repo_name);
    if candidate.exists() {
        return candidate.to_str().map(|s| s.to_string());
    }
    None
}

/// Summary of the detected constellation environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentSummary {
    pub root: PathBuf,
    pub spine_version: String,
    pub context_mode: ContextMode,
    pub repos: Vec<RepoSummary>,
    pub object_kinds: Vec<String>,
}

impl EnvironmentSummary {
    pub fn from_config_and_spine(config: &Config, spine: &SpineIndex) -> Self {
        let mut object_kinds = Vec::new();
        for family in &spine.inner().contract_families {
            if !object_kinds.contains(&family.object_kind) {
                object_kinds.push(family.object_kind.clone());
            }
        }

        let repos = config
            .manifests()
            .iter()
            .map(|m| RepoSummary {
                name: m.repo_name.clone(),
                tier: m.tier.as_str().to_string(),
                path: repo_path_for(config.root(), &m.repo_name)
                    .unwrap_or_else(|| String::from(".")),
            })
            .collect();

        EnvironmentSummary {
            root: config.root().to_path_buf(),
            spine_version: config.spine_version().to_string(),
            context_mode: config.context_mode(),
            repos,
            object_kinds,
        }
    }
}
