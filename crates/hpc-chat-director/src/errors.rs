//! Error types for CHAT_DIRECTOR.
//!
//! All errors are machine-readable and include remediation hints
//! for AI auto-correction loops.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Top-level error type for CHAT_DIRECTOR operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Configuration or environment detection error.
    #[error("Config error: {message}")]
    Config {
        message: String,
        path: PathBuf,
    },

    /// I/O error.
    #[error("I/O error: {message}")]
    Io {
        message: String,
        path: PathBuf,
    },

    /// JSON/UTF-8 parse error for input files.
    #[error("Parse error: {message}")]
    Parse {
        message: String,
        path: PathBuf,
    },

    /// Schema spine loading or parsing error.
    #[error("Spine error: {message}")]
    SpineLoad {
        message: String,
        path: PathBuf,
    },

    /// Schema spine parsing error (structural/schema mismatch).
    #[error("Spine parse error: {message}")]
    SpineParse {
        message: String,
        path: PathBuf,
    },

    /// Manifest loading or parsing error.
    #[error("Manifest error: {message}")]
    ManifestLoad {
        message: String,
        path: PathBuf,
    },

    /// Manifest parsing error (structural/schema mismatch).
    #[error("Manifest parse error: {message}")]
    ManifestParse {
        message: String,
        path: PathBuf,
    },

    /// Phase violation error.
    #[error("Phase error: {0}")]
    Phase(#[from] PhaseError),

    /// Validation error (schema, invariants, manifests, or envelope).
    #[error("Validation error: {code}")]
    Validation {
        code: String,
        inner: ValidationError,
    },

    /// CLI-specific error with exit code.
    #[error("CLI error (exit {exit_code}): {message}")]
    Cli {
        exit_code: u8,
        message: String,
    },

    /// Internal error (should not happen in well-formed usage).
    #[error("Internal error: {message}")]
    Internal {
        message: String,
    },

    /// JSON serialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

impl Error {
    /// Convert Error to CLI exit code, matching CHAT_DIRECTOR v1 registry.
    ///
    /// 0  SUCCESS
    /// 1  VALIDATION_FAILURE
    /// 2  CONFIG_ERROR (spine, manifest, IO, parse)
    /// 3  PHASE_VIOLATION
    /// 4  ROUTING_ERROR (manifest-level routing) – surfaced via Validation
    /// 5  ENVELOPE_ERROR – surfaced via Validation
    /// 10 INTERNAL_ERROR
    pub fn exit_code(&self) -> u8 {
        match self {
            Error::Config { .. } => 2,
            Error::Io { .. } => 2,
            Error::Parse { .. } => 2,
            Error::SpineLoad { .. } | Error::SpineParse { .. } => 2,
            Error::ManifestLoad { .. } | Error::ManifestParse { .. } => 2,
            Error::Phase(_) => 3,
            Error::Validation { .. } => 1,
            Error::Cli { exit_code, .. } => *exit_code,
            Error::Internal { .. } => 10,
            Error::Json(_) => 2,
        }
    }
}

/// Validation layer, shared with validate/mod.rs.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ValidationLayer {
    Schema,
    Phase,
    Invariant,
    Manifest,
    Envelope,
}

/// Severity of a diagnostic.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Cross-metric interaction explanation (e.g., XMIT_001).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionEffect {
    pub interaction_id: String,
    pub description: String,
}

/// High-level validation error used inside the library.
///
/// This is the unified diagnostic shape for schema, invariants, manifest,
/// envelope, and phase validations.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationError {
    /// Machine-readable error code (e.g., "ERR_CIC_RANGE").
    pub code: String,
    /// Validation layer that produced this error.
    pub layer: ValidationLayer,
    /// Severity of the diagnostic.
    pub severity: Severity,
    /// JSON Pointer to the offending field.
    pub json_pointer: String,
    /// Optional submitted value that caused the failure.
    #[serde(default)]
    pub submitted_value: Option<serde_json::Value>,
    /// Expected value or range description.
    pub expected: String,
    /// Machine-readable remediation hint.
    pub remediation: String,
    /// Suggested fix ordering for iterative correction.
    pub fix_order: u32,
    /// Cross-metric interaction effects (e.g., XMIT_001).
    #[serde(default)]
    pub interaction_effects: Vec<InteractionEffect>,
}

/// CLI-facing error envelope (printed to stderr in --json mode).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CliErrorSchema {
    pub schema_version: String,
    pub exit_code: i32,
    pub error_count: usize,
    pub warning_count: usize,
    pub diagnostics: Vec<CliDiagnostic>,
    pub metadata: CliErrorMetadata,
}

/// Per-diagnostic payload for CLI consumers.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CliDiagnostic {
    pub code: String,
    pub layer: ValidationLayer,
    pub severity: Severity,
    pub json_pointer: String,
    pub submitted_value: Option<serde_json::Value>,
    pub expected: String,
    pub remediation: String,
    pub fix_order: u32,
    pub interaction_effects: Vec<InteractionEffect>,
}

/// Additional metadata to aid AI/humans.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CliErrorMetadata {
    pub spine_version: Option<String>,
    pub object_kind: Option<String>,
    pub tier: Option<String>,
    pub phase: Option<i32>,
    pub timestamp_utc: String,
}

/// Deterministic mapping from ValidationError list to exit code.
pub fn exit_code_for_diagnostics(diags: &[ValidationError]) -> i32 {
    use Severity::*;
    use ValidationLayer::*;

    let has_error = diags.iter().any(|d| d.severity == Error);
    if !has_error {
        return 0;
    }

    let mut worst_layer: Option<ValidationLayer> = None;
    for d in diags.iter().filter(|d| d.severity == Error) {
        worst_layer = Some(match (worst_layer, d.layer) {
            (None, l) => l,
            (Some(current), next) => rank_layer(current, next),
        });
    }

    match worst_layer {
        Some(Schema) | Some(Invariant) => 1,
        Some(Phase) => 3,
        Some(Manifest) => 4,
        Some(Envelope) => 5,
        None => 0,
    }
}

/// Layer ranking helper: lower index is "more fundamental".
fn rank_layer(a: ValidationLayer, b: ValidationLayer) -> ValidationLayer {
    fn idx(layer: ValidationLayer) -> u8 {
        match layer {
            ValidationLayer::Schema => 0,
            ValidationLayer::Phase => 1,
            ValidationLayer::Invariant => 2,
            ValidationLayer::Manifest => 3,
            ValidationLayer::Envelope => 4,
        }
    }
    if idx(a) <= idx(b) {
        a
    } else {
        b
    }
}

/// Convert internal ValidationError list into a CliErrorSchema.
pub fn to_cli_error_schema(
    diags: Vec<ValidationError>,
    spine_version: Option<String>,
    object_kind: Option<String>,
    tier: Option<String>,
    phase: Option<i32>,
    timestamp_utc: String,
) -> CliErrorSchema {
    let exit_code = exit_code_for_diagnostics(&diags);
    let error_count = diags
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .count();
    let warning_count = diags
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .count();

    let diagnostics = diags
        .into_iter()
        .map(|d| CliDiagnostic {
            code: d.code,
            layer: d.layer,
            severity: d.severity,
            json_pointer: d.json_pointer,
            submitted_value: d.submitted_value,
            expected: d.expected,
            remediation: d.remediation,
            fix_order: d.fix_order,
            interaction_effects: d.interaction_effects,
        })
        .collect();

    CliErrorSchema {
        schema_version: "cli-error-v1".to_string(),
        exit_code,
        error_count,
        warning_count,
        diagnostics,
        metadata: CliErrorMetadata {
            spine_version,
            object_kind,
            tier,
            phase,
            timestamp_utc,
        },
    }
}

/// Phase-related error.
#[derive(Debug, Error, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PhaseError {
    /// Requested phase does not permit this contract family.
    #[error("Phase {requested_phase:?} does not permit {contract_family:?}")]
    PhaseForbidden {
        requested_phase: crate::model::spine_types::Phase,
        contract_family: crate::model::spine_types::ContractFamily,
        permission: PhasePermission,
        diagnostic: PhaseDiagnostic,
    },

    /// Promotion predicate failed between phases.
    #[error("Promotion from {from_phase:?} to {to_phase:?} blocked")]
    PromotionBlocked {
        from_phase: crate::model::spine_types::Phase,
        to_phase: crate::model::spine_types::Phase,
        predicate_code: String,
        failures: Vec<PromotionFailure>,
        diagnostic: PhaseDiagnostic,
    },
}

/// Permission level for a contract family at a given phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PhasePermission {
    /// Full contract authoring allowed.
    Allowed,
    /// Only registry entry allowed (no full card).
    RegistryOnly,
    /// Read-only reference allowed (no writes).
    ReadOnlyRef,
    /// Operation forbidden at this phase.
    Forbidden,
}

/// Structured diagnostic for phase errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseDiagnostic {
    /// Machine-readable error code.
    pub code: String,
    /// Always "phase" for layer identification.
    pub layer: String,
    /// Always "error" for severity (for now).
    pub severity: String,
    /// Human-readable message.
    pub message: String,
    /// Optional suggestion for phase upgrade.
    #[serde(default)]
    pub phase_upgrade_suggestion: Option<PhaseUpgradeSuggestion>,
    /// Remediation hint for AI agents.
    pub remediation: String,
    /// Fix ordering for iterative correction.
    pub fix_order: u32,
}

/// Suggestion for advancing to a higher phase.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseUpgradeSuggestion {
    /// Target phase for promotion.
    pub target_phase: crate::model::spine_types::Phase,
    /// Missing prerequisites blocking promotion.
    pub missing_prerequisites: Vec<String>,
    /// Estimated effort: "trivial", "moderate", "substantial".
    pub estimated_effort: String,
    /// Whether AI can auto-fix these prerequisites.
    pub auto_fixable: bool,
}

/// Individual promotion check failure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromotionFailure {
    /// Check ID (e.g., "PROM002").
    pub check_id: String,
    /// Description of what failed.
    pub description: String,
    /// JSON Pointer to offending field if applicable.
    #[serde(default)]
    pub json_pointer: Option<String>,
    /// Submitted value that caused failure.
    #[serde(default)]
    pub submitted_value: Option<serde_json::Value>,
    /// Expected value or condition.
    #[serde(default)]
    pub expected: Option<String>,
}

/// Convenience conversion: wrap a ValidationError as Error::Validation.
impl From<ValidationError> for Error {
    fn from(inner: ValidationError) -> Self {
        Error::Validation {
            code: inner.code.clone(),
            inner,
        }
    }
}
