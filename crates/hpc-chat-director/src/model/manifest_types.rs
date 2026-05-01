//! Typed representations of repository manifests.
//! Manifests define per-repo policies: allowed schemas, default paths,
//! tier classifications, and AI-specific authoring rules.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::model::spine_types::Phase;

/// Tier classification shared across manifests.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Tier {
    #[serde(rename = "T1-core")]
    T1Core,
    #[serde(rename = "T2-vault")]
    T2Vault,
    #[serde(rename = "T3-research")]
    T3Research,
}

impl Tier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Tier::T1Core => "T1-core",
            Tier::T2Vault => "T2-vault",
            Tier::T3Research => "T3-research",
        }
    }

    pub fn describe(&self) -> TierDescription {
        let (name, description) = match self {
            Tier::T1Core => (
                "T1-core",
                "Public contract-only surfaces and orchestrator logic; no raw horror payloads.",
            ),
            Tier::T2Vault => (
                "T2-vault",
                "Vault-style repos for styles, seeds, personas, and experimental logic.",
            ),
            Tier::T3Research => (
                "T3-research",
                "Research tier for BCI, neural resonance, and redacted chronicles.",
            ),
        };

        TierDescription {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierDescription {
    pub name: String,
    pub description: String,
}

/// AI agent role for permission gating.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AiRole {
    Architect,
    Implementer,
    Auditor,
}

/// Per-repo manifest structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoManifest {
    /// Schema version for this manifest.
    #[serde(default)]
    pub schema_version: String,
    /// Logical repository name.
    #[serde(rename = "repoName")]
    pub repo_name: String,
    /// Tier classification for this repo.
    pub tier: Tier,
    /// Allowed object kinds for this repo.
    #[serde(default)]
    pub allowed_object_kinds: Vec<String>,
    /// Allowed schema URIs.
    #[serde(default)]
    pub allowed_schemas: Vec<String>,
    /// Default target path template for each object kind.
    #[serde(default)]
    pub default_target_paths: HashMap<String, String>,
    /// Policy rules for this repo.
    #[serde(default)]
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
    /// Detailed target rules keyed by object kind.
    #[serde(default)]
    pub target_rules: Vec<TargetRule>,
    /// If true, object kinds not listed in allowed_object_kinds are denied.
    #[serde(default)]
    pub implicit_deny: bool,
}

impl RepoManifest {
    /// Check if this manifest allows the given object kind.
    pub fn allows_object_kind(&self, kind: &str) -> bool {
        if self.allowed_object_kinds.contains(&kind.to_string()) {
            return true;
        }
        if self.implicit_deny {
            return false;
        }
        !self.allowed_object_kinds.is_empty()
    }

    /// Get the default path template for an object kind.
    pub fn default_path_for(&self, kind: &str) -> Option<&str> {
        self.default_target_paths.get(kind).map(|s| s.as_str())
    }

    /// Get charter rationale and suggested staging repo for tier violations.
    pub fn tier_violation_hints_for(&self) -> (Option<String>, Option<String>) {
        let charter = self.authoring_hints.tier_rationale.clone();
        let suggestion = self.authoring_hints.default_staging_repo.clone();
        (charter, suggestion)
    }

    /// Find a routing rule for a specific object kind.
    pub fn find_rule_for(&self, object_kind: &str) -> Option<&TargetRule> {
        self.target_rules
            .iter()
            .find(|r| r.object_kind == object_kind)
    }

    /// Build a policy checklist for this repo and tier.
    pub fn to_policy_checklist(&self) -> PolicyChecklist {
        let mut checklist = PolicyChecklist::new(&self.repo_name, &self.tier);

        if self.rules.one_file_per_request {
            checklist.add_item(
                "ONE_FILE_PER_REQUEST",
                "Enforce exactly one file per authoring request.",
            );
        }

        if self.rules.require_deadledger_ref {
            checklist.add_item(
                "DEAD_LEDGER_REF",
                "Require deadLedgerRef for historical or high-impact content.",
            );
        }

        if let Some(max_bytes) = self.rules.max_file_size_bytes {
            checklist.add_item(
                "MAX_FILE_SIZE",
                &format!("Maximum file size is {} bytes.", max_bytes),
            );
        }

        if self.min_rwf_for_tier.is_some() {
            checklist.add_item(
                "MIN_RWF_FOR_TIER",
                "Enforce minimum RWF score for production-tier acceptance.",
            );
        }

        checklist
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
    #[serde(default)]
    pub tier_rationale: Option<String>,
    /// Rationale for one-file-per-request.
    #[serde(default)]
    pub one_file_per_request_rationale: Option<String>,
    /// Rationale for Dead-Ledger requirements.
    #[serde(default)]
    pub deadledger_rationale: Option<String>,
    /// Rationale for cross-repo reference rules.
    #[serde(default)]
    pub cross_repo_rationale: Option<String>,
    /// Suggested staging repo for low-RWF artifacts.
    #[serde(default)]
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
    None,
    SameOrLower,
    SameOnly,
    Strict,
}

impl Default for TierIsolation {
    fn default() -> Self {
        TierIsolation::SameOrLower
    }
}

/// Routing rule mapping object kind to path and role patterns.
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
    /// Optional human/AI-readable notes for routing decisions.
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyChecklistItem {
    pub code: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyChecklist {
    pub repo_name: String,
    pub tier: String,
    pub items: Vec<PolicyChecklistItem>,
}

impl PolicyChecklist {
    pub fn new(repo_name: &str, tier: &Tier) -> Self {
        PolicyChecklist {
            repo_name: repo_name.to_string(),
            tier: tier.as_str().to_string(),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, code: &str, description: &str) {
        self.items.push(PolicyChecklistItem {
            code: code.to_string(),
            description: description.to_string(),
        });
    }
}
