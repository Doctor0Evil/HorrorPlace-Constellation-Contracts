//! Typed representations of AI authoring responses and envelopes.
//!
//! Mirrors `ai-authoring-response-v1.json` and `prism-envelope-v1.json`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::model::spine_types::Tier;

/// Wrapper for AI-generated artifact responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiAuthoringResponse {
    /// Canonical schema URI for this response type.
    #[serde(rename = "$schema")]
    pub schema_ref: String,
    /// The primary generated artifact content.
    pub artifact: serde_json::Value,
    /// Envelope metadata for provenance.
    pub envelope: PrismEnvelope,
    /// Target repository for this artifact.
    pub target_repo: String,
    /// Target path within the repository.
    pub target_path: String,
    /// Optional registry diffs if this artifact updates discovery.
    #[serde(default)]
    pub registry_diffs: Vec<RegistryDiff>,
    /// Optional additional artifacts (max 2 for tightly coupled).
    #[serde(default)]
    pub additional_artifacts: Vec<serde_json::Value>,
    /// Generation metadata.
    pub generated_by: GeneratedBy,
}

impl AiAuthoringResponse {
    /// Count of additional artifacts beyond the primary.
    pub fn additional_artifacts_count(&self) -> usize {
        self.additional_artifacts.len()
    }

    /// Check if Dead-Ledger reference is present.
    pub fn has_deadledger_ref(&self) -> bool {
        self.envelope.deadledger_ref.is_some()
    }

    /// Get RWF score if present.
    pub fn rwf_score(&self) -> Option<f64> {
        self.envelope.prisma_meta.as_ref()?.rwf
    }
}

/// Cryptographic envelope for artifact provenance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrismEnvelope {
    /// Envelope schema version.
    pub envelope_version: String,
    /// Timestamp of generation.
    pub timestamp: String,
    /// Schema reference for the payload.
    pub schema_ref: String,
    /// Optional Dead-Ledger reference.
    #[serde(default)]
    pub deadledger_ref: Option<DeadLedgerRef>,
    /// Optional ZKP commitment placeholder.
    #[serde(default)]
    pub zkp_commitment: Option<String>,
    /// Prism metadata block.
    #[serde(default)]
    pub prisma_meta: Option<PrismMeta>,
    /// Optional cryptographic signature placeholder.
    #[serde(default)]
    pub signature: Option<String>,
}

/// Dead-Ledger reference for attestation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeadLedgerRef {
    /// Proof envelope ID.
    pub proof_envelope_id: String,
    /// Verifier reference.
    pub verifier_ref: String,
    /// Circuit type for the proof.
    pub circuit_type: String,
    /// Required proofs for this artifact.
    pub required_proofs: Vec<String>,
}

/// Prism metadata for telemetry and governance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrismMeta {
    /// Reliability Weighting Factor.
    #[serde(default)]
    pub rwf: Option<f64>,
    /// Review status.
    #[serde(default)]
    pub review_status: Option<ReviewStatus>,
    /// Generation phase.
    #[serde(default)]
    pub generation_phase: Option<u8>,
    /// Optional telemetry hooks.
    #[serde(default)]
    pub telemetry_hooks: Vec<String>,
}

/// Review status for governance tracking.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    Draft,
    UnderReview,
    Approved,
    Rejected,
}

/// Agent identification for provenance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedBy {
    /// Agent profile ID.
    pub agent_id: String,
    /// Model/version identifier.
    #[serde(default)]
    pub model_version: Option<String>,
    /// Session ID for traceability.
    #[serde(default)]
    pub session_id: Option<String>,
}

/// Registry update for discovery surfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryDiff {
    /// Registry file path.
    pub registry_path: String,
    /// Action: add, update, or remove.
    pub action: RegistryAction,
    /// Entry ID being modified.
    pub entry_id: String,
    /// New entry content if adding/updating.
    #[serde(default)]
    pub entry: Option<serde_json::Value>,
}

/// Registry modification action.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryAction {
    Add,
    Update,
    Remove,
}

/// Validated artifact ready for apply.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatedFile {
    /// Target repository.
    pub target_repo: String,
    /// Target path within repo.
    pub target_path: String,
    /// Validated content.
    pub content: serde_json::Value,
    /// SHA-256 hash of canonical content.
    pub content_hash: String,
    /// Soft diagnostics (warnings) that passed validation.
    #[serde(default)]
    pub soft_diagnostics: Vec<SoftDiagnostic>,
    /// Provenance metadata.
    pub provenance: ValidatedProvenance,
}

/// Warning that passed validation but flags quality issues.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftDiagnostic {
    /// Diagnostic code.
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// JSON Pointer to affected field.
    pub json_pointer: String,
    /// Optional suggestion for improvement.
    pub suggestion: Option<String>,
}

/// Provenance for validated artifacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatedProvenance {
    /// Timestamp of validation.
    pub validated_at: String,
    /// Validator version.
    pub validator_version: String,
    /// Request ID that triggered validation.
    pub request_id: Option<String>,
    /// Validation trace for learning.
    #[serde(default)]
    pub validation_trace: Vec<ValidationStep>,
}

/// Step in the validation pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationStep {
    /// Layer name (schema, invariant, manifest, envelope).
    pub layer: String,
    /// Whether this step passed.
    pub passed: bool,
    /// Optional error message if failed.
    #[serde(default)]
    pub error: Option<String>,
}
