//! Typed representations of CHAT_DIRECTOR schemas and contracts.
//!
//! This module provides Rust types that mechanically mirror the JSON Schemas
//! defined in `schemas/`. All types derive `Serialize`/`Deserialize` and are
//! intended to be validated against their canonical schema URIs at runtime.
//!
//! Module organization:
//! - `spine_types.rs`    — Schema spine, invariants, metrics, contract families
//! - `manifest_types.rs` — Repo manifests, tiers, policies, routing rules
//! - `request_types.rs`  — AiAuthoringRequest and request defaults
//! - `response_types.rs` — AiAuthoringResponse, PrismEnvelope, ValidatedFile

use serde::{Deserialize, Serialize;

pub mod spine_types;
pub mod manifest_types;
pub mod request_types;
pub mod response_types;

// Re-export for convenience so callers can use `model::InvariantSpec` etc.
pub use manifest_types::*;
pub use request_types::*;
pub use response_types::*;
pub use spine_types::*;

/// High-level description of what this ChatDirector instance can see in the current environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityCatalog {
    pub spine_version: Option<String>,
    pub available_object_kinds: Vec<String>,
    pub available_repos: Vec<RepoSummary>,
    pub invariants: Vec<InvariantSummary>,
    pub metrics: Vec<MetricSummary>,
    pub phases: Vec<PhaseSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoSummary {
    pub name: String,
    pub tier: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvariantSummary {
    pub name: String,
    pub description: String,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSummary {
    pub name: String,
    pub description: String,
    pub target_min: Option<f64>,
    pub target_max: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseSummary {
    pub id: u8,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteExplanation {
    pub object_kind: String,
    pub tier: Option<String>,
    pub target_repo: String,
    pub target_path_hint: String,
    pub policy_notes: Vec<String>,
}

/// Canonical description of a validation error with remediation hints for AI tools.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationDiagnostic {
    pub layer: ValidationLayer,
    pub severity: ValidationSeverity,
    pub code: String,
    pub message: String,
    pub json_pointer: Option<String>,
    pub remediation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationLayer {
    Schema,
    Invariants,
    Manifest,
    Envelope,
    Phase,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

/// Aggregated validation result returned by high-level validate functions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub ok: bool,
    pub diagnostics: Vec<ValidationDiagnostic>,
}

impl ValidationResult {
    pub fn is_ok(&self) -> bool {
        self.ok
    }

    pub fn first_error(&self) -> Option<&ValidationDiagnostic> {
        self.diagnostics
            .iter()
            .find(|d| d.severity == ValidationSeverity::Error)
    }

    pub fn ranked_diagnostics(&self) -> Vec<ValidationDiagnostic> {
        let mut items = self.diagnostics.clone();
        items.sort_by_key(|d| match d.severity {
            ValidationSeverity::Error => 0,
            ValidationSeverity::Warning => 1,
            ValidationSeverity::Info => 2,
        });
        items
    }
}

/// Common trait for all schema-backed types.
///
/// Implementors must provide their canonical schema URI and version.
pub trait SchemaBacked {
    fn schema_uri() -> &'static str;
    fn schema_version() -> &'static str;
}

/// Marker trait for types that can be validated against their schema.
pub trait Validatable:
    SchemaBacked + Serialize + for<'de> Deserialize<'de>
{
    /// Validate this instance against its canonical schema.
    ///
    /// Default implementation is a no-op; callers are expected to route
    /// through the central validator in `validate::schema`.
    fn validate(&self) -> Result<(), Vec<ValidationError>>
    where
        Self: Sized,
    {
        Ok(())
    }
}

/// Structured validation error for schema-backed types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationError {
    pub code: String,
    pub json_pointer: String,
    pub message: String,
    pub remediation: Option<String>,
    pub expected: Option<serde_json::Value>,
    pub submitted: Option<serde_json::Value>,
}
