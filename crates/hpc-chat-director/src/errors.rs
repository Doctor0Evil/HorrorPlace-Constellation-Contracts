//! Error types for CHAT_DIRECTOR.
//!
//! All errors are machine-readable and include remediation hints
//! for AI auto-correction loops.

use thiserror::Error;
use serde::{Deserialize, Serialize};

/// Top-level error type for CHAT_DIRECTOR operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Configuration or environment detection error.
    #[error("Config error: {message}")]
    Config {
        message: String,
        path: std::path::PathBuf,
    },
    
    /// I/O error.
    #[error("I/O error: {message}")]
    Io {
        message: String,
        path: std::path::PathBuf,
    },
    
    /// JSON parse error.
    #[error("Parse error: {message}")]
    Parse {
        message: String,
        path: std::path::PathBuf,
    },
    
    /// Schema spine loading or parsing error.
    #[error("Spine error: {message}")]
    SpineLoad {
        message: String,
        path: std::path::PathBuf,
    },
    
    /// Schema spine parsing error.
    #[error("Spine parse error: {message}")]
    SpineParse {
        message: String,
        path: std::path::PathBuf,
    },
    
    /// Manifest loading or parsing error.
    #[error("Manifest error: {message}")]
    ManifestLoad {
        message: String,
        path: std::path::PathBuf,
    },
    
    /// Manifest parsing error.
    #[error("Manifest parse error: {message}")]
    ManifestParse {
        message: String,
        path: std::path::PathBuf,
    },
    
    /// Phase violation error.
    #[error("Phase error: {0}")]
    Phase(#[from] PhaseError),
    
    /// Validation error (schema, invariants, manifests, or envelope).
    #[error("Validation error: {code}")]
    Validation(#[from] ValidationError),
    
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
    /// Always "error" for severity.
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
    /// Estimated effort: trivial, moderate, substantial.
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

/// Validation error for schema, invariants, manifests, or envelope checks.
#[derive(Debug, Error, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationError {
    /// Machine-readable error code.
    pub code: String,
    /// JSON Pointer to the offending field.
    pub json_pointer: String,
    /// Human-readable message.
    pub message: String,
    /// Optional remediation hint.
    #[serde(default)]
    pub remediation: Option<Remediation>,
    /// Optional expected value or range.
    #[serde(default)]
    pub expected: Option<serde_json::Value>,
    /// Optional submitted value.
    #[serde(default)]
    pub submitted: Option<serde_json::Value>,
}

/// Machine-readable remediation hint for auto-correction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remediation {
    /// JSON Pointer to the field that needs fixing.
    pub json_pointer: String,
    /// Expected value or range.
    pub expected: String,
    /// One-line corrective instruction.
    pub suggestion: String,
}

/// Convert Error to CLI exit code.
impl Error {
    pub fn exit_code(&self) -> u8 {
        match self {
            Error::Config { .. } => 2,
            Error::Io { .. } => 2,
            Error::Parse { .. } => 2,
            Error::SpineLoad { .. } | Error::SpineParse { .. } => 2,
            Error::ManifestLoad { .. } | Error::ManifestParse { .. } => 2,
            Error::Phase(_) => 3,
            Error::Validation(_) => 1,
            Error::Cli { exit_code, .. } => *exit_code,
            Error::Internal { .. } => 10,
            Error::Json(_) => 2,
        }
    }
}

/// Convert ValidationError to CLI error for output.
impl From<ValidationError> for Error {
    fn from(err: ValidationError) -> Self {
        Error::Validation(err)
    }
}
