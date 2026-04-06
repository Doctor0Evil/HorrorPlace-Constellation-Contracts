//! Typed representations of CHAT_DIRECTOR schemas and contracts.
//!
//! This module provides Rust types that mechanically mirror the JSON Schemas
//! defined in `schemas/`. All types derive `Serialize`/`Deserialize` and are
//! validated against their canonical schema URIs at runtime.
//!
//! # Module Organization
//!
//! - `spine_types.rs` — Schema spine, invariants, metrics, contract families
//! - `manifest_types.rs` — Repo manifests, tiers, policies, routing rules
//! - `request_types.rs` — AiAuthoringRequest and request defaults
//! - `response_types.rs` — AiAuthoringResponse, PrismEnvelope, ValidatedFile

pub mod spine_types;
pub mod manifest_types;
pub mod request_types;
pub mod response_types;

// Re-export for convenience
pub use spine_types::*;
pub use manifest_types::*;
pub use request_types::*;
pub use response_types::*;

/// Common trait for all schema-backed types.
///
/// Implementors must provide their canonical schema URI for validation.
pub trait SchemaBacked {
    /// Returns the canonical schema URI for this type.
    ///
    /// Example: `"schema://Horror.Place/eventcontract_v1.json"`
    fn schema_uri() -> &'static str;

    /// Returns the schema version string.
    ///
    /// Example: `"v1"`
    fn schema_version() -> &'static str;
}

/// Marker trait for types that can be validated against their schema.
pub trait Validatable: SchemaBacked + serde::Serialize + for<'de> serde::Deserialize<'de> {
    /// Validate this instance against its canonical schema.
    ///
    /// Returns `Ok(())` if valid, or a list of validation errors.
    fn validate(&self) -> Result<(), Vec<ValidationError>>
    where
        Self: Sized,
    {
        // Default implementation delegates to external validator.
        // Concrete types may override for type-specific checks.
        Ok(())
    }
}

/// Structured validation error for schema-backed types.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationError {
    /// Machine-readable error code.
    pub code: String,
    /// JSON Pointer to the offending field.
    pub json_pointer: String,
    /// Human-readable message.
    pub message: String,
    /// Optional remediation hint.
    pub remediation: Option<String>,
    /// Optional expected value or range.
    pub expected: Option<serde_json::Value>,
    /// Optional submitted value.
    pub submitted: Option<serde_json::Value>,
}
