//! High-level validation orchestration for CHAT_DIRECTOR.
//!
//! This module coordinates the multi-layered validation pipeline:
//! 1. Schema-level checks (JSON Schema conformance)
//! 2. Phase enforcement (lifecycle and role gates)
//! 3. Invariant/metric enforcement (numeric ranges, cross-metric rules)
//! 4. Manifest/tier policy checks (routing, one-file-per-request)
//! 5. Envelope structural validation (prism fields, provenance)
//!
//! All validation errors are structured and machine-readable, with
//! remediation hints for AI auto-correction loops.

pub mod schema;
pub mod invariants;
pub mod manifests;
pub mod envelopes;

use serde::{Deserialize, Serialize};

use crate::errors::{Severity, ValidationError, ValidationLayer};
use crate::model::request_types::AiAuthoringRequest;
use crate::model::response_types::{AiAuthoringResponse, ValidatedFile, SoftDiagnostic, ValidatedProvenance, ValidationStep};
use crate::spine::SpineIndex;

/// Result of the full validation pipeline, as seen by library callers.
#[derive(Debug)]
pub struct ValidationResult {
    pub passed: bool,
    pub diagnostics: Vec<ValidationError>,
    pub layer_reached: ValidationLayer,
}

impl ValidationResult {
    /// Return diagnostics sorted by severity, layer, and fix cost (fix_order).
    pub fn ranked_diagnostics(&self) -> Vec<ValidationError> {
        let mut out = self.diagnostics.clone();
        out.sort_by(|a, b| rank_diag(a, b));
        out
    }
}

fn rank_diag(a: &ValidationError, b: &ValidationError) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;

    let sev_rank = |s: Severity| match s {
        Severity::Error => 0,
        Severity::Warning => 1,
        Severity::Info => 2,
    };
    let layer_rank = |l: ValidationLayer| match l {
        ValidationLayer::Schema => 0,
        ValidationLayer::Phase => 1,
        ValidationLayer::Invariant => 2,
        ValidationLayer::Manifest => 3,
        ValidationLayer::Envelope => 4,
    };

    match sev_rank(a.severity).cmp(&sev_rank(b.severity)) {
        Less => Less,
        Greater => Greater,
        Equal => match layer_rank(a.layer).cmp(&layer_rank(b.layer)) {
            Less => Less,
            Greater => Greater,
            Equal => a.fix_order.cmp(&b.fix_order),
        },
    }
}

/// Orchestrate the five-layer validation pipeline.
///
/// Layer order:
/// 1. Schema (short-circuits on failure)
/// 2. Phase
/// 3. Invariant/metric
/// 4. Manifest
/// 5. Envelope
pub fn validate_response(
    spine: &SpineIndex,
    req: &AiAuthoringRequest,
    resp: &AiAuthoringResponse,
) -> ValidationResult {
    let mut diagnostics: Vec<ValidationError> = Vec::new();

    // 1. Schema layer (hard short-circuit)
    let schema_errors = schema::validate_against_schema(spine, req, resp);
    if !schema_errors.is_empty() {
        diagnostics.extend(schema_errors);
        return ValidationResult {
            passed: false,
            diagnostics,
            layer_reached: ValidationLayer::Schema,
        };
    }

    // 2. Phase layer
    let phase_errors = crate::phases::validate_phase(req, resp);
    diagnostics.extend(phase_errors);
    let has_phase_error = diagnostics
        .iter()
        .any(|d| d.layer == ValidationLayer::Phase && d.severity == Severity::Error);

    // 3. Invariant/metric layer
    let invariant_result = invariants::validate_invariants(spine, req, resp);
    diagnostics.extend(invariants::as_validation_errors(
        invariant_result.diagnostics,
    ));

    // 4. Manifest/tier layer
    let manifest_errors = manifests::validate_manifests(req, resp);
    diagnostics.extend(manifest_errors);

    // 5. Envelope layer
    let envelope_errors = envelopes::validate_envelope(req, resp);
    diagnostics.extend(envelope_errors);

    let passed = !diagnostics.iter().any(|d| d.severity == Severity::Error);

    let layer_reached = if !passed && has_phase_error {
        ValidationLayer::Phase
    } else if !passed {
        earliest_failing_layer(&diagnostics)
    } else {
        ValidationLayer::Envelope
    };

    ValidationResult {
        passed,
        diagnostics,
        layer_reached,
    }
}

fn earliest_failing_layer(diags: &[ValidationError]) -> ValidationLayer {
    let mut layer = ValidationLayer::Envelope;
    for d in diags.iter().filter(|d| d.severity == Severity::Error) {
        if layer_index(d.layer) < layer_index(layer) {
            layer = d.layer;
        }
    }
    layer
}

fn layer_index(layer: ValidationLayer) -> u8 {
    match layer {
        ValidationLayer::Schema => 0,
        ValidationLayer::Phase => 1,
        ValidationLayer::Invariant => 2,
        ValidationLayer::Manifest => 3,
        ValidationLayer::Envelope => 4,
    }
}

/// Early warning for request pre-validation (schema/phase/spine only).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarlyWarning {
    pub code: String,
    pub message: String,
    pub remediation: String,
    pub fix_order: u32,
}

/// Pre-validate an AiAuthoringRequest before generation.
///
/// Catches phase violations, unknown objectKinds, and obviously
/// unresolvable references early, saving AI generation cycles.
pub fn validate_request_early(
    spine: &SpineIndex,
    req: &AiAuthoringRequest,
) -> Vec<EarlyWarning> {
    let mut warnings = Vec::new();

    // Phase permission
    if let Err(phase_err) = crate::phases::check_phase_permission(req) {
        warnings.push(EarlyWarning {
            code: phase_err.code,
            message: phase_err.message,
            remediation: "Adjust phase or objectKind to match the phase permission matrix".into(),
            fix_order: 1,
        });
    }

    // ObjectKind exists in spine
    if spine.describe_object_kind(req.object_kind).is_none() {
        warnings.push(EarlyWarning {
            code: "UNKNOWN_OBJECT_KIND".into(),
            message: format!("Object kind {:?} not found in schema spine", req.object_kind),
            remediation: "Use an objectKind defined in the schema spine".into(),
            fix_order: 1,
        });
    }

    // Referenced IDs – placeholder resolvability check
    for id in &req.referenced_ids {
        if id.starts_with("REF-unknown") {
            warnings.push(EarlyWarning {
                code: "UNRESOLVABLE_REFERENCE".into(),
                message: format!("Referenced ID '{}' cannot be resolved (placeholder check)", id),
                remediation: "Ensure referenced IDs exist in the registry before generation".into(),
                fix_order: 2,
            });
        }
    }

    warnings
}

/// Lift a successful ValidationResult into a ValidatedFile with provenance.
///
/// Call this only when `result.passed == true`.
pub fn into_validated_file(
    result: ValidationResult,
    req: &AiAuthoringRequest,
    resp: &AiAuthoringResponse,
) -> ValidatedFile {
    let soft_diagnostics: Vec<SoftDiagnostic> = result
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .map(|d| SoftDiagnostic {
            code: d.code.clone(),
            message: d.remediation.clone().unwrap_or_else(|| d.expected.clone()),
            json_pointer: d.json_pointer.clone(),
            suggestion: d
                .remediation
                .as_ref()
                .map(|r| r.clone()),
        })
        .collect();

    let validation_trace: Vec<ValidationStep> = vec![
        ValidationStep {
            layer: "Schema".to_string(),
            passed: true,
            error: None,
        },
        ValidationStep {
            layer: "Phase".to_string(),
            passed: true,
            error: None,
        },
        ValidationStep {
            layer: "Invariant".to_string(),
            passed: true,
            error: None,
        },
        ValidationStep {
            layer: "Manifest".to_string(),
            passed: true,
            error: None,
        },
        ValidationStep {
            layer: "Envelope".to_string(),
            passed: true,
            error: None,
        },
    ];

    ValidatedFile {
        target_repo: resp.target_repo.clone(),
        target_path: resp.target_path.clone(),
        content: resp.artifact.clone(),
        content_hash: compute_content_hash(&resp.artifact),
        soft_diagnostics,
        provenance: ValidatedProvenance {
            validated_at: chrono::Utc::now().to_rfc3339(),
            validator_version: env!("CARGO_PKG_VERSION").to_string(),
            request_id: Some(req.request_id.clone()),
            validation_trace,
        },
    }
}

/// Compute SHA-256 hash of canonical JSON content.
fn compute_content_hash(content: &serde_json::Value) -> String {
    use sha2::{Digest, Sha256};

    let canonical = serde_json::to_string(content).unwrap_or_else(|_| "null".to_string());
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let digest = hasher.finalize();
    hex::encode(digest)
}
