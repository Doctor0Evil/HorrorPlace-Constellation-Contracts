//! CLI subcommand: `hpc-chat-director validate-response`.
//!
//! Reads an AiAuthoringResponse from stdin or file, runs the full
//! validation pipeline, and outputs either a ValidatedFile or
//! structured diagnostics in CliErrorSchema format.

use std::path::Path;
use std::io::Read;
use serde::{Deserialize, Serialize};
use crate::model::request_types::AiAuthoringRequest;
use crate::model::response_types::AiAuthoringResponse;
use crate::errors::{Error, ValidationError, Remediation};

/// CLI error schema for machine-readable diagnostics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CliErrorSchema {
    /// Schema version for this error format.
    pub schema_version: String,
    /// Exit code mapping (0=success, 1-10=various failures).
    pub exit_code: u8,
    /// Human-readable summary.
    pub summary: String,
    /// Metadata about the validation context.
    #[serde(default)]
    pub metadata: ValidationMetadata,
    /// Ranked diagnostics list.
    #[serde(default)]
    pub diagnostics: Vec<RankedDiagnostic>,
}

/// Metadata about the validation run.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationMetadata {
    /// Object kind being validated.
    #[serde(default)]
    pub object_kind: Option<String>,
    /// Contract family.
    #[serde(default)]
    pub contract_family: Option<String>,
    /// Phase from the request.
    #[serde(default)]
    pub phase: Option<u8>,
    /// Tier from the request.
    #[serde(default)]
    pub tier: Option<String>,
    /// Target repo.
    #[serde(default)]
    pub target_repo: Option<String>,
    /// Order of validation layers executed.
    #[serde(default)]
    pub validation_pipeline_order: Vec<String>,
    /// Whether validation short-circuited early.
    #[serde(default)]
    pub short_circuited: bool,
    /// Timestamp of validation.
    #[serde(default)]
    pub timestamp: Option<String>,
}

/// Ranked diagnostic for iterative correction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankedDiagnostic {
    /// Machine-readable error code.
    pub code: String,
    /// Validation layer that produced this diagnostic.
    pub layer: String,
    /// Severity: error blocks apply, warning is advisory.
    pub severity: String,
    /// Human-readable message.
    pub message: String,
    /// JSON Pointer to the offending field.
    pub json_pointer: String,
    /// Optional submitted value that caused the failure.
    #[serde(default)]
    pub submitted_value: Option<serde_json::Value>,
    /// Optional expected value or range.
    #[serde(default)]
    pub expected: Option<serde_json::Value>,
    /// Machine-readable remediation hint.
    #[serde(default)]
    pub remediation: Option<Remediation>,
    /// Fix ordering: lower numbers should be addressed first.
    pub fix_order: u32,
    /// Optional interaction effects for cross-metric failures.
    #[serde(default)]
    pub interaction_effects: Vec<crate::validate::InteractionEffect>,
}

/// Run the validate-response subcommand.
pub fn run(
    request_file: &Path,
    response_file: &Path,
    director: &crate::ChatDirector,
) -> Result<(), Error> {
    // 1. Load request and response
    let req: AiAuthoringRequest = load_json_file(request_file)?;
    let resp: AiAuthoringResponse = load_json_file(response_file)?;

    // 2. Run validation pipeline
    match director.validate_response(&req, &resp) {
        Ok(validated) => {
            // Success: output ValidatedFile as JSON
            let output = serde_json::to_string_pretty(&validated)?;
            println!("{}", output);
            Ok(())
        }
        Err(validation_err) => {
            // Failure: output CliErrorSchema
            let error_schema = build_cli_error_schema(
                &req,
                &resp,
                &validation_err,
                director,
            );
            let output = serde_json::to_string_pretty(&error_schema)?;
            eprintln!("{}", output);
            
            // Return appropriate exit code via error
            Err(Error::Cli {
                exit_code: error_schema.exit_code,
                message: error_schema.summary,
            })
        }
    }
}

/// Load JSON from a file.
fn load_json_file<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, Error> {
    let content = std::fs::read_to_string(path).map_err(|e| Error::Io {
        message: format!("Failed to read file: {}", e),
        path: path.to_path_buf(),
    })?;
    
    serde_json::from_str(&content).map_err(|e| Error::Parse {
        message: format!("Failed to parse JSON: {}", e),
        path: path.to_path_buf(),
    })
}

/// Build a CliErrorSchema from a ValidationError.
fn build_cli_error_schema(
    req: &AiAuthoringRequest,
    resp: &AiAuthoringResponse,
    err: &ValidationError,
    director: &crate::ChatDirector,
) -> CliErrorSchema {
    // Determine exit code based on error type
    let exit_code = match err.code.as_str() {
        "SCHEMA_VALIDATION_FAILED" => 1,
        "INVARIANT_OUT_OF_RANGE" => 1,
        "MANIFEST_POLICY_VIOLATION" => 4,
        "ENVELOPE_VALIDATION_FAILED" => 5,
        "PHASE_VIOLATION" => 3,
        _ => 1, // Default to validation failure
    };

    // Build metadata
    let metadata = ValidationMetadata {
        object_kind: Some(req.object_kind.clone()),
        contract_family: Some(req.object_kind.clone()), // Simplified
        phase: Some(req.phase as u8),
        tier: Some(format!("{:?}", req.tier)),
        target_repo: Some(resp.target_repo.clone()),
        validation_pipeline_order: vec![
            "schema".into(),
            "invariants".into(),
            "manifest".into(),
            "envelope".into(),
        ],
        short_circuited: false, // Could track actual short-circuit
        timestamp: Some(chrono::Utc::now().to_rfc3339()),
    };

    // Build diagnostics from error
    let diagnostics = vec![RankedDiagnostic {
        code: err.code.clone(),
        layer: "validation".into(),
        severity: "error".into(),
        message: err.message.clone(),
        json_pointer: err.json_pointer.clone(),
        submitted_value: None, // Could extract from error context
        expected: err.remediation.as_ref().map(|r| serde_json::json!(&r.expected)),
        remediation: err.remediation.clone(),
        fix_order: 1, // Could compute based on error type
        interaction_effects: Vec::new(), // Could populate from invariant errors
    }];

    CliErrorSchema {
        schema_version: "v1".into(),
        exit_code,
        summary: format!("Validation failed: {}", err.code),
        metadata,
        diagnostics,
    }
}
