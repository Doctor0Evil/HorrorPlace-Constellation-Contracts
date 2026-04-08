//! CHATDIRECTOR — Schema-driven authoring compiler for Horror$Place
//!
//! This crate provides the canonical interface for AI-chat agents to generate,
//! validate, and apply constellation artifacts. All operations are driven by
//! the schema spine and repo manifests; no governance logic is hardcoded.
//!
//! # Core Workflow
//!
//! ```no_run
//! use hpc_chat_director::{ChatDirector, Config};
//!
//! // Load the constellation environment
//! let config = Config::detect(std::path::Path::new("/path/to/constellation"))?;
//! let director = ChatDirector::load_environment(config)?;
//!
//! // Plan: normalize intent into a structured request
//! let request = director.plan_from_prompt("Create a marsh region seed", None)?;
//!
//! // Generate: AI produces a response matching ai-authoring-response-v1.json
//! // let response: AiAuthoringResponse = ai_generate(&request)?;
//!
//! // Validate: check schema, invariants, manifests, envelopes
//! let result = director.validate_response(&request, &response)?;
//!
//! // Apply: write to disk at manifest-approved path
//! director.apply(&request, &response, false)?;
//! # Ok::<_, hpc_chat_director::Error>(())
//! ```
//!
//! # Machine-Readable Diagnostics
//!
//! Every error includes structured remediation hints:
//!
//! ```no_run
//! use hpc_chat_director::{Error, ValidationError};
//!
//! # let director = todo!();
//! # let req = todo!();
//! # let resp = todo!();
//! match director.validate_response(&req, &resp) {
//!     Ok(validation_result) => {
//!         if validation_result.passed {
//!             // proceed
//!         }
//!     }
//!     Err(Error::Validation(ValidationError { code, json_pointer, remediation, .. })) => {
//!         eprintln!("Error at {}: {} — {}", json_pointer, code, remediation);
//!         // AI can parse this directly into its next edit cycle
//!     }
//! }
//! ```

#![warn(missing_docs, rust_2018_idioms)]
#![deny(unsafe_code)]

pub mod config;
pub mod errors;
pub mod manifests;
pub mod model;
pub mod phases;
pub mod spine;
pub mod validate;

// Optional modules gated by features.
#[cfg(feature = "http-service")]
pub mod service;

pub mod generate;
pub mod cli;

// Re-export public API surface.
pub use config::{Config, ContextMode, EnvironmentSummary};
pub use errors::{Error, PhaseError, ValidationError, Remediation};
pub use manifests::{ManifestContext, ManifestDiagnostic, ManifestValidationResult, ManifestIndex};
pub use model::{
    manifest_types::{RepoManifest, TargetRule, Tier},
    request_types::{AiAuthoringRequest, RequestDefaults},
    response_types::{AiAuthoringResponse, PrismEnvelope},
    spine_types::{ContractFamily, InvariantSpec, MetricSpec, SchemaSpine},
};
pub use phases::Phase;
pub use spine::{DefaultBands, DerivedMetrics, ObjectKindProfile, SpineIndex};
pub use validate::{RankedDiagnostic, ValidationResult};

use crate::generate;
use crate::validate;
use once_cell::sync::OnceCell;
use std::collections::HashMap;

/// Core façade for constellation authoring operations.
///
/// All methods are pure with respect to governance: they consult the spine
/// and manifests, never hardcode policy. State is limited to cached schema
/// compilation for performance.
#[derive(Clone)]
pub struct ChatDirector {
    config: Config,
    spine: SpineIndex,
    manifests: ManifestIndex,
    /// Cached compiled schemas for performance (not part of public API).
    #[doc(hidden)]
    schema_cache: OnceCell<HashMap<String, jsonschema::Validator>>,
}

impl ChatDirector {
    /// Load the constellation environment from a resolved Config.
    ///
    /// `Config` is responsible for discovering the spine, manifests, and
    /// registry roots. This constructor fails if required files are missing
    /// or malformed.
    pub fn load_environment(config: Config) -> Result<Self, Error> {
        let spine = SpineIndex::load(&config)?;
        let manifests = ManifestIndex::load(&config)?;
        Ok(ChatDirector {
            config,
            spine,
            manifests,
            schema_cache: OnceCell::new(),
        })
    }

    /// Return a reference to the loaded schema spine index.
    pub fn spine(&self) -> &SpineIndex {
        &self.spine
    }

    /// Return a reference to the loaded repo manifest index.
    pub fn manifests(&self) -> &ManifestIndex {
        &self.manifests
    }

    /// Normalize a natural-language prompt into a structured authoring request.
    ///
    /// Uses spine metadata and manifest routing to resolve:
    /// - `objectKind` from intent (with `candidateKinds` if ambiguous)
    /// - `targetRepo` from manifest rules
    /// - Default invariant/metric bands from `safe_defaults`
    ///
    /// Does not generate content — only plans the request shape.
    pub fn plan_from_prompt(
        &self,
        prompt: &str,
        object_kind_hint: Option<String>,
    ) -> Result<AiAuthoringRequest, Error> {
        cli::plan::plan_from_prompt(self, prompt, object_kind_hint)
    }

    /// Generate a minimal, schema-compliant skeleton for the requested object.
    ///
    /// Skeletons are archetype-aware and include safe default invariant and
    /// metric bands derived from the spine.
    pub fn generate_skeleton(
        &self,
        request: &AiAuthoringRequest,
    ) -> Result<serde_json::Value, Error> {
        generate::dispatch_skeleton(&self.spine, &self.manifests, request)
            .map_err(Error::from)
    }

    /// Validate an AI-generated response against the full constraint stack.
    ///
    /// Layers (in order):
    /// 1. JSON Schema conformance (`validate::schema`)
    /// 2. Invariant/metric ranges and interactions (`validate::invariants`)
    /// 3. Manifest routing and tier policies (`validate::manifests`)
    /// 4. Envelope structure and provenance (`validate::envelopes`)
    pub fn validate_response(
        &self,
        req: &AiAuthoringRequest,
        resp: &AiAuthoringResponse,
    ) -> Result<ValidationResult, Error> {
        validate::run_full_pipeline(&self.config, &self.spine, &self.manifests, req, resp)
            .map_err(Error::from)
    }

    /// Apply a validated contract to disk at its manifest-approved path.
    ///
    /// When `dry_run` is true, no files are written; instead, the planned
    /// filesystem actions are returned in the `ApplyResult`.
    pub fn apply(
        &self,
        request: &AiAuthoringRequest,
        response: &AiAuthoringResponse,
        dry_run: bool,
    ) -> Result<model::apply::ApplyResult, Error> {
        cli::apply::apply_contract(self, request, response, dry_run).map_err(Error::from)
    }

    /// Return a capability catalog for pre-flight discovery.
    ///
    /// AI agents should call this once at session start to learn:
    /// - Known `objectKind` values and their required invariants
    /// - Allowed phases and tier restrictions per repo
    /// - Invariant/metric ranges and cross-metric interaction rules
    pub fn catalog(&self) -> CapabilityCatalog {
        CapabilityCatalog::from_spine_and_manifests(&self.spine, &self.manifests)
    }

    /// Return environment summary for path planning and diagnostics.
    pub fn environment_summary(&self) -> EnvironmentSummary {
        EnvironmentSummary::from_config_and_manifests(&self.config, &self.manifests)
    }

    /// Suggest canonical target paths for a given intent and objectKind.
    ///
    /// Returns `(targetRepo, targetPath, tier)` tuples derived from
    /// manifest path templates and schema allowlists.
    pub fn plan_paths(
        &self,
        intent: &str,
        object_kind: &str,
    ) -> Vec<CanonicalTarget> {
        manifests::suggest_paths(intent, object_kind, &self.manifests, &self.spine)
    }
}

/// Machine-readable catalog of constellation capabilities.
///
/// Returned by `ChatDirector::catalog()` for AI pre-flight discovery.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityCatalog {
    /// Known object kinds with their required invariants and metric targets.
    pub object_kinds: std::collections::HashMap<String, ObjectKindProfile>,
    /// Phase rules: which (phase, objectKind, repo) triples are allowed.
    pub phase_rules: std::collections::HashMap<Phase, PhaseRuleSet>,
    /// Invariant definitions with ranges, tier overrides, and descriptions.
    pub invariants: std::collections::HashMap<String, InvariantCatalogEntry>,
    /// Metric definitions with target bands and telemetry hooks.
    pub metrics: std::collections::HashMap<String, MetricCatalogEntry>,
    /// Repo routing hints: which repos accept which objectKinds at which tiers.
    pub repo_routing: std::collections::HashMap<String, RepoRoutingEntry>,
}

impl CapabilityCatalog {
    /// Build a catalog from spine and manifests.
    pub fn from_spine_and_manifests(
        spine: &SpineIndex,
        manifests: &ManifestIndex,
    ) -> Self {
        // Placeholder implementation; fill with real queries.
        let object_kinds = std::collections::HashMap::new();
        let phase_rules = std::collections::HashMap::new();
        let invariants = std::collections::HashMap::new();
        let metrics = std::collections::HashMap::new();
        let repo_routing = std::collections::HashMap::new();

        CapabilityCatalog {
            object_kinds,
            phase_rules,
            invariants,
            metrics,
            repo_routing,
        }
    }
}

/// Phase-specific rule set used in the capability catalog.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseRuleSet {
    /// Object kinds allowed in this phase.
    pub allowed_object_kinds: Vec<String>,
    /// Repos that may receive artifacts in this phase.
    pub allowed_repos: Vec<String>,
    /// Invariant strictness level for this phase.
    pub invariant_strictness: InvariantStrictness,
}

/// Invariant strictness levels for different phases.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvariantStrictness {
    /// Phase 0: schema-only validation.
    SchemaOnly,
    /// Phase 1–2: ranges enforced, interactions soft.
    Relaxed,
    /// Phase 3–4: full enforcement including cross-metric rules.
    Strict,
}

/// Catalog entry for an invariant.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvariantCatalogEntry {
    pub name: String,
    pub canonical_range: serde_json::Value,
    pub tier_overrides: std::collections::HashMap<Tier, serde_json::Value>,
    pub description: String,
    pub compatible_with: Vec<String>,
}

/// Catalog entry for a metric.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricCatalogEntry {
    pub name: String,
    pub target_band: serde_json::Value,
    pub description: String,
    pub telemetry_hook: Option<String>,
}

/// Routing hints for a single repo.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoRoutingEntry {
    pub tier: Tier,
    pub accepted_object_kinds: Vec<String>,
    pub default_path_template: String,
    pub policy_notes: Vec<String>,
}

/// Canonical target for file placement.
///
/// Returned by `ChatDirector::plan_paths()` to eliminate directory guessing.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanonicalTarget {
    pub target_repo: String,
    pub target_path: String,
    pub tier: Tier,
    /// Confidence (0.0–1.0) based on manifest specificity.
    pub confidence: f64,
}
