//! Typed representations of repository manifests.
//!
//! Manifests define per-repo policies: allowed schemas, default paths,
//! tier classifications, and AI-specific authoring rules.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::model::spine_types::{Phase, Tier};

/// Per-repo manifest structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoManifest {
    /// Logical repository name.
    pub repo: String,
    /// Tier classification for this repo.
    pub tier: Tier,
    /// Allowed object kinds for this repo.
    pub allowed_object_kinds: Vec<String>,
    /// Allowed schema URIs.
    pub allowed_schemas: Vec<String>,
    /// Default target path template for each object kind.
    pub default_target_paths: HashMap<String, String>,
    /// Policy rules for this repo.
    pub rules: RepoRules,
    /// AI-specific authoring hints.
    #[serde(default)]
    pub authoring_hints: AuthoringHints,
    /// Cross-repo reference policy.
    #[serde(default)]
    pub cross_ref_policy: CrossRefPolicy,
    /// Minimum RWF for production-tier acceptance.
    #[serde(default)]
    pub min_rwf_for_tier: Option<f64>,
}

impl RepoManifest {
    /// Check if this manifest allows the given object kind.
    pub fn allows_object_kind(&self, kind: &str) -> bool {
        self.allowed_object_kinds.contains(&kind.to_string())
    }

    /// Get the default path template for an object kind.
    pub fn default_path_for(&self, kind: &str) -> Option<&str> {
        self.default_target_paths.get(kind).map(|s| s.as_str())
    }

    /// Get charter rationale for tier violations.
    pub fn tier_violation_hints_for(&self, kind: &str) -> (Option<String>, Option<String>) {
        let charter = self.authoring_hints.tier_rationale.clone();
        let suggestion = self.authoring_hints.default_staging_repo.clone();
        (charter, suggestion)
    }
}

/// Policy rules for a repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoRules {
    /// Enforce one file per authoring request.
    #[serde(default)]
    pub one_file_per_request: bool,
    /// Require Dead-Ledger reference for historical content.
    #[serde(default)]
    pub require_deadledger_ref: bool,
    /// Maximum file size in bytes.
    #[serde(default)]
    pub max_file_size_bytes: Option<u64>,
    /// Forbidden fields per tier.
    #[serde(default)]
    pub forbidden_fields: HashMap<Tier, Vec<String>>,
    /// Mandatory prismMeta fields.
    #[serde(default)]
    pub mandatory_prism_fields: Vec<String>,
}

/// AI-specific authoring hints.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthoringHints {
    /// Rationale for tier restrictions.
    pub tier_rationale: Option<String>,
    /// Rationale for one-file-per-request.
    pub one_file_per_request_rationale: Option<String>,
    /// Rationale for Dead-Ledger requirements.
    pub deadledger_rationale: Option<String>,
    /// Rationale for cross-repo reference rules.
    pub cross_repo_rationale: Option<String>,
    /// Suggested staging repo for low-RWF artifacts.
    pub default_staging_repo: Option<String>,
}

/// Cross-repository reference policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossRefPolicy {
    /// Repos this repo may reference.
    #[serde(default)]
    pub allows_refs_to: Vec<String>,
    /// Repos that may reference this repo.
    #[serde(default)]
    pub allows_refs_from: Vec<String>,
    /// Maximum DET for cross-referenced content.
    #[serde(default)]
    pub max_det_for_refs: Option<f64>,
    /// Tier isolation level.
    #[serde(default)]
    pub tier_isolation: TierIsolation,
}

/// Tier isolation policy for cross-references.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TierIsolation {
    /// No isolation; any tier may reference.
    None,
    /// Only same or lower tiers may reference.
    SameOrLower,
    /// Only same tier may reference.
    SameOnly,
    /// Strict: no cross-tier references allowed.
    Strict,
}

/// Routing rule mapping object kind to path patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetRule {
    /// Object kind this rule applies to.
    pub object_kind: String,
    /// Path pattern with variables (e.g., "events/{id}.json").
    pub path_pattern: String,
    /// Allowed AI roles for this rule.
    #[serde(default)]
    pub allowed_roles: Vec<AiRole>,
    /// Phase restrictions for this rule.
    #[serde(default)]
    pub allowed_phases: Vec<Phase>,
}

/// AI agent role for permission gating.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRole {
    /// Architect: schemas and checklists only.
    Architect,
    /// Implementer: full contract generation.
    Implementer,
    /// Auditor: validation and review only.
    Auditor,
}
