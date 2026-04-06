//! CHAT_DIRECTOR — Schema-driven authoring compiler for Horror$Place
//!
//! This crate provides the canonical interface for AI-chat agents to generate,
//! validate, and apply constellation artifacts. All operations are driven by
//! the schema spine and repo manifests; no governance logic is hardcoded.
//!
//! # Core Workflow
//!
//! ```no_run
//! use hpc_chat_director::ChatDirector;
//! use std::path::Path;
//!
//! // Load the constellation environment
//! let director = ChatDirector::load_environment(Path::new("/path/to/constellation"))?;
//!
//! // Plan: normalize intent into a structured request
//! let request = director.plan_from_prompt("Create a marsh region seed")?;
//!
//! // Generate: AI produces a response matching ai-authoring-response-v1.json
//! // let response: AiAuthoringResponse = ai_generate(&request)?;
//!
//! // Validate: check schema, invariants, manifests, envelopes
//! let validated = director.validate_response(&request, &response)?;
//!
//! // Apply: write to disk at manifest-approved path
//! director.apply(&validated)?;
//! # Ok::<_, hpc_chat_director::Error>(())
//! ```
//!
//! # Machine-Readable Diagnostics
//!
//! Every error includes structured remediation hints:
//!
//! ```rust
//! use hpc_chat_director::{Error, ValidationError};
//!
//! match director.validate_response(&req, &resp) {
//!     Ok(file) => { /* proceed */ }
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

// Optional modules gated by features
#[cfg(feature = "http-service")]
pub mod service;

pub mod generate;
pub mod cli;

// Re-export public API surface
pub use config::{Config, ContextMode, EnvironmentSummary};
pub use errors::{Error, PhaseError, ValidationError, Remediation};
pub use manifests::{ManifestContext, ManifestDiagnostic, ManifestValidationResult};
pub use model::{
    request_types::{AiAuthoringRequest, RequestDefaults},
    response_types::{AiAuthoringResponse, PrismEnvelope, ValidatedFile},
    spine_types::{ContractFamily, InvariantSpec, MetricSpec, SchemaSpine},
    manifest_types::{RepoManifest, Tier, TargetRule},
};
pub use phases::Phase;
pub use spine::{DerivedMetrics, ObjectKindProfile, DefaultBands};
pub use validate::{ValidationResult, RankedDiagnostic};

/// Core façade for constellation authoring operations.
///
/// All methods are pure with respect to governance: they consult the spine
/// and manifests, never hardcode policy. State is limited to cached schema
/// compilation for performance.
pub struct ChatDirector {
    config: Config,
    spine: SchemaSpine,
    manifests: Vec<RepoManifest>,
    /// Cached compiled schemas for performance (not part of public API)
    #[doc(hidden)]
    schema_cache: once_cell::sync::OnceCell<std::collections::HashMap<String, jsonschema::Validator>>,
}

impl ChatDirector {
    /// Load the constellation environment from a root path.
    ///
    /// Discovers spine, manifests, and registries according to `config.rs`
    /// resolution rules. Returns an error if required files are missing
    /// or malformed.
    pub fn load_environment(root: &std::path::Path) -> Result<Self, Error> {
        let config = Config::detect(root)?;
        let spine = spine::load(&config)?;
        let manifests = manifests::load_all(&config)?;
        
        Ok(Self {
            config,
            spine,
            manifests,
            schema_cache: once_cell::sync::OnceCell::new(),
        })
    }

    /// Normalize a natural-language prompt into a structured authoring request.
    ///
    /// Uses spine metadata and manifest routing to resolve:
    /// - `objectKind` from intent (with `candidateKinds` if ambiguous)
    /// - `targetRepo` and `targetPath` from manifest rules
    /// - Default invariant/metric bands from `safe_defaults`
    ///
    /// Does not generate content — only plans the request shape.
    pub fn plan_from_prompt(
        &self,
        prompt: &str,
        defaults: Option<RequestDefaults>,
    ) -> Result<AiAuthoringRequest, Error> {
        // Implementation delegated to internal planner
        cli::plan::normalize_prompt(prompt, &self.spine, &self.manifests, defaults)
    }

    /// Validate an AI-generated response against the full constraint stack.
    ///
    /// Layers (in order):
    /// 1. JSON Schema conformance (`validate/schema.rs`)
    /// 2. Invariant/metric ranges and interactions (`validate/invariants.rs`)
    /// 3. Manifest routing and tier policies (`validate/manifests.rs`)
    /// 4. Envelope structure and provenance (`validate/envelopes.rs`)
    ///
    /// Returns `ValidatedFile` on success, or `ValidationError` with
    /// machine-readable remediation hints on failure.
    pub fn validate_response(
        &self,
        req: &AiAuthoringRequest,
        resp: &AiAuthoringResponse,
    ) -> Result<ValidatedFile, ValidationError> {
        validate::validate_response(self, req, resp)
    }

    /// Apply a validated file to disk at its manifest-approved path.
    ///
    /// Respects `--dry-run` semantics via config; optionally runs
    /// post-apply hooks declared in the target repo's manifest.
    pub fn apply(&self, file: &ValidatedFile) -> Result<(), Error> {
        cli::apply::write_validated(file, &self.config, &self.manifests)
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

    /// Return environment summary for path planning.
    ///
    /// Includes discovered repos, active spine version, and available
    /// objectKinds. AI agents query this before proposing file paths.
    pub fn environment_summary(&self) -> EnvironmentSummary {
        self.config.summary()
    }

    /// Suggest canonical target paths for a given intent and objectKind.
    ///
    /// Returns `(targetRepo, targetPath, tier)` tuples derived from
    /// manifest `defaultTargetPaths` and `allowedSchemas`. Eliminates
    /// directory-guessing by AI agents.
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseRuleSet {
    pub allowed_object_kinds: Vec<String>,
    pub allowed_repos: Vec<String>,
    pub invariant_strictness: InvariantStrictness,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvariantStrictness {
    /// Phase 0: schema-only validation
    SchemaOnly,
    /// Phase 1-2: ranges enforced, interactions soft
    Relaxed,
    /// Phase 3-4: full enforcement including cross-metric rules
    Strict,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvariantCatalogEntry {
    pub name: String,
    pub canonical_range: serde_json::Value, // { "min": 0.0, "max": 1.0 } or similar
    pub tier_overrides: std::collections::HashMap<Tier, serde_json::Value>,
    pub description: String,
    pub compatible_with: Vec<String>, // metric names this invariant interacts with
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricCatalogEntry {
    pub name: String,
    pub target_band: serde_json::Value,
    pub description: String,
    pub telemetry_hook: Option<String>,
}

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
    pub confidence: f64, // 0.0-1.0, based on manifest specificity
}
