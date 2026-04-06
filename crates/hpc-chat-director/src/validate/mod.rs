//! High-level validation orchestration for CHAT_DIRECTOR.
//!
//! This module coordinates the multi-layered validation pipeline:
//! 1. Schema-level checks (JSON Schema conformance)
//! 2. Invariant/metric enforcement (numeric ranges, cross-metric rules)
//! 3. Manifest/tier policy checks (routing, one-file-per-request)
//! 4. Envelope structural validation (prism fields, provenance)
//!
//! All validation errors are structured and machine-readable, with
//! remediation hints for AI auto-correction loops.

pub mod schema;
pub mod invariants;
pub mod manifests;
pub mod envelopes;

use crate::model::request_types::AiAuthoringRequest;
use crate::model::response_types::{AiAuthoringResponse, ValidatedFile};
use crate::model::spine_types::SchemaSpine;
use crate::model::manifest_types::RepoManifest;
use crate::errors::{Error, ValidationError, Remediation};
use serde::{Deserialize, Serialize};

/// Result of the full validation pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    /// Whether validation passed (all layers).
    pub passed: bool,
    /// All diagnostics collected, ranked by severity and fix order.
    pub diagnostics: Vec<RankedDiagnostic>,
    /// Optional validated file if all checks passed.
    pub validated_file: Option<ValidatedFile>,
    /// Metadata about which layers were executed.
    pub layers_executed: Vec<ValidationLayer>,
}

/// Diagnostic ranked for iterative correction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankedDiagnostic {
    /// Machine-readable error code.
    pub code: String,
    /// Validation layer that produced this diagnostic.
    pub layer: ValidationLayer,
    /// Severity: error blocks apply, warning is advisory.
    pub severity: DiagnosticSeverity,
    /// Human-readable message.
    pub message: String,
    /// JSON Pointer to the offending field.
    pub json_pointer: String,
    /// Optional submitted value that caused the failure.
    pub submitted_value: Option<serde_json::Value>,
    /// Optional expected value or range.
    pub expected: Option<serde_json::Value>,
    /// Machine-readable remediation hint for AI auto-correction.
    pub remediation: Option<Remediation>,
    /// Fix ordering: lower numbers should be addressed first.
    pub fix_order: u32,
    /// Optional interaction effects for cross-metric failures.
    pub interaction_effects: Vec<InteractionEffect>,
}

/// Validation layer identifier.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationLayer {
    /// JSON Schema structural validation.
    Schema,
    /// Invariant/metric numeric enforcement.
    Invariants,
    /// Manifest routing and tier policy checks.
    Manifest,
    /// Envelope structure and provenance validation.
    Envelope,
}

/// Diagnostic severity level.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    /// Blocks artifact acceptance; must be fixed.
    Error,
    /// Advisory; artifact accepted but quality may be improved.
    Warning,
    /// Informational; no action required.
    Info,
}

/// Cross-metric interaction effect record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionEffect {
    /// Rule ID that triggered the interaction (e.g., "XMIT_001").
    pub rule_id: String,
    /// Source metric that caused the effect.
    pub source_metric: String,
    /// Target metric that was adjusted.
    pub target_metric: String,
    /// Effect type: amplify, suppress, or gate.
    pub effect_type: String,
    /// Adjusted band after interaction applied.
    pub adjusted_band: serde_json::Value,
    /// Human-readable explanation of the interaction.
    pub reason: String,
}

/// Entry point for validating an AI-generated response.
///
/// Executes the full validation pipeline in order:
/// schema → invariants → manifest → envelope.
/// Returns a ValidationResult with all diagnostics.
pub fn validate_response(
    director: &crate::ChatDirector,
    req: &AiAuthoringRequest,
    resp: &AiAuthoringResponse,
) -> Result<ValidatedFile, ValidationError> {
    let mut diagnostics = Vec::new();
    let mut layers_executed = Vec::new();

    // Layer 1: Schema validation (short-circuit on failure)
    layers_executed.push(ValidationLayer::Schema);
    match schema::validate_schema(resp, &director.spine) {
        Ok(_) => {}
        Err(schema_errors) => {
            diagnostics.extend(schema_errors.into_iter().map(|e| RankedDiagnostic {
                code: e.code,
                layer: ValidationLayer::Schema,
                severity: DiagnosticSeverity::Error,
                message: e.message,
                json_pointer: e.json_pointer,
                submitted_value: e.submitted,
                expected: e.expected,
                remediation: e.remediation.map(|r| Remediation {
                    json_pointer: r.json_pointer,
                    expected: r.expected,
                    suggestion: r.suggestion,
                }),
                fix_order: 1,
                interaction_effects: Vec::new(),
            }));
            // Short-circuit: schema failures block later layers
            return Err(ValidationError {
                code: "SCHEMA_VALIDATION_FAILED".into(),
                json_pointer: "/".into(),
                message: "Schema validation failed; fix structural issues first".into(),
                remediation: Some(Remediation {
                    json_pointer: diagnostics.first().map(|d| d.json_pointer.clone()).unwrap_or_default(),
                    expected: "Valid JSON matching the referenced schema".into(),
                    suggestion: "Check schemaref and ensure all required fields are present".into(),
                }),
            });
        }
    }

    // Layer 2: Invariant/metric validation
    layers_executed.push(ValidationLayer::Invariants);
    match invariants::validate_invariants(resp, &director.spine, req.tier) {
        Ok(invariant_results) => {
            for result in invariant_results {
                if !result.passed {
                    let explanation = invariants::explain_invariant_failure(&result);
                    diagnostics.push(RankedDiagnostic {
                        code: explanation.code,
                        layer: ValidationLayer::Invariants,
                        severity: DiagnosticSeverity::Error,
                        message: explanation.message,
                        json_pointer: explanation.json_pointer,
                        submitted_value: Some(serde_json::json!(result.submitted_value)),
                        expected: Some(serde_json::json!(explanation.expected)),
                        remediation: Some(Remediation {
                            json_pointer: explanation.json_pointer.clone(),
                            expected: explanation.expected.clone(),
                            suggestion: explanation.remediation.clone(),
                        }),
                        fix_order: 2,
                        interaction_effects: result.interaction_effects,
                    });
                }
            }
        }
        Err(err) => {
            diagnostics.push(RankedDiagnostic {
                code: err.code,
                layer: ValidationLayer::Invariants,
                severity: DiagnosticSeverity::Error,
                message: err.message,
                json_pointer: err.json_pointer,
                submitted_value: err.submitted,
                expected: err.expected,
                remediation: err.remediation.map(|r| Remediation {
                    json_pointer: r.json_pointer,
                    expected: r.expected,
                    suggestion: r.suggestion,
                }),
                fix_order: 2,
                interaction_effects: Vec::new(),
            });
        }
    }

    // Layer 3: Manifest/tier policy validation
    layers_executed.push(ValidationLayer::Manifest);
    let manifest_ctx = manifests::ManifestContext {
        manifests_by_repo: &director.manifests,
    };
    let manifest_result = manifests::validate_manifests(&manifest_ctx, req, resp);
    for diag in manifest_result.diagnostics {
        diagnostics.push(RankedDiagnostic {
            code: format!("{:?}", diag.code),
            layer: ValidationLayer::Manifest,
            severity: match diag.severity {
                manifests::ManifestDiagnosticSeverity::Error => DiagnosticSeverity::Error,
                manifests::ManifestDiagnosticSeverity::Warning => DiagnosticSeverity::Warning,
                manifests::ManifestDiagnosticSeverity::Info => DiagnosticSeverity::Info,
            },
            message: diag.message,
            json_pointer: "/targetRepo".into(), // Manifest errors typically target repo/path
            submitted_value: Some(serde_json::json!(&resp.target_repo)),
            expected: Some(serde_json::json!("A repo that accepts this objectKind at this tier")),
            remediation: Some(Remediation {
                json_pointer: "/targetRepo".into(),
                expected: diag.suggested_alternative_repo.clone().unwrap_or_default(),
                suggestion: diag.charter_rationale.unwrap_or_default(),
            }),
            fix_order: 3,
            interaction_effects: Vec::new(),
        });
    }
    if manifest_result.is_fatal {
        return Err(ValidationError {
            code: "MANIFEST_POLICY_VIOLATION".into(),
            json_pointer: "/targetRepo".into(),
            message: "Manifest policy check failed".into(),
            remediation: Some(Remediation {
                json_pointer: "/targetRepo".into(),
                expected: "A repo with compatible tier and policy".into(),
                suggestion: "Review authoringHints in the target repo's manifest".into(),
            }),
        });
    }

    // Layer 4: Envelope structural validation
    layers_executed.push(ValidationLayer::Envelope);
    match envelopes::validate_envelope(&resp.envelope, &director.config) {
        Ok(_) => {}
        Err(envelope_errors) => {
            for err in envelope_errors {
                diagnostics.push(RankedDiagnostic {
                    code: err.code,
                    layer: ValidationLayer::Envelope,
                    severity: DiagnosticSeverity::Error,
                    message: err.message,
                    json_pointer: err.json_pointer,
                    submitted_value: err.submitted_value,
                    expected: err.expected,
                    remediation: Some(Remediation {
                        json_pointer: err.json_pointer.clone(),
                        expected: err.expected.clone().unwrap_or_default(),
                        suggestion: err.remediation,
                    }),
                    fix_order: 4,
                    interaction_effects: Vec::new(),
                });
            }
            return Err(ValidationError {
                code: "ENVELOPE_VALIDATION_FAILED".into(),
                json_pointer: "/envelope".into(),
                message: "Envelope structural validation failed".into(),
                remediation: Some(Remediation {
                    json_pointer: "/envelope".into(),
                    expected: "A well-formed prism-envelope-v1 structure".into(),
                    suggestion: "Use wrap_in_prism_envelope helper to ensure correct structure".into(),
                }),
            });
        }
    }

    // If we have errors but no fatal short-circuit, return them
    let has_errors = diagnostics.iter().any(|d| matches!(d.severity, DiagnosticSeverity::Error));
    if has_errors {
        // Sort diagnostics by fix_order, then severity, then layer
        diagnostics.sort_by(|a, b| {
            a.fix_order.cmp(&b.fix_order)
                .then_with(|| a.severity.cmp(&b.severity))
                .then_with(|| a.layer.cmp(&b.layer))
        });
        return Err(ValidationError {
            code: "VALIDATION_ERRORS".into(),
            json_pointer: diagnostics.first().map(|d| d.json_pointer.clone()).unwrap_or_default(),
            message: format!("{} validation error(s) found", diagnostics.iter().filter(|d| matches!(d.severity, DiagnosticSeverity::Error)).count()),
            remediation: Some(Remediation {
                json_pointer: diagnostics.first().map(|d| d.json_pointer.clone()).unwrap_or_default(),
                expected: "All diagnostics addressed".into(),
                suggestion: "Fix errors in fix_order sequence; warnings are advisory".into(),
            }),
        });
    }

    // All checks passed; construct ValidatedFile
    let content_hash = compute_content_hash(&resp.artifact)?;
    Ok(ValidatedFile {
        target_repo: resp.target_repo.clone(),
        target_path: resp.target_path.clone(),
        content: resp.artifact.clone(),
        content_hash,
        soft_diagnostics: diagnostics
            .into_iter()
            .filter(|d| matches!(d.severity, DiagnosticSeverity::Warning))
            .map(|d| crate::model::response_types::SoftDiagnostic {
                code: d.code,
                message: d.message,
                json_pointer: d.json_pointer,
                suggestion: d.remediation.map(|r| r.suggestion),
            })
            .collect(),
        provenance: crate::model::response_types::ValidatedProvenance {
            validated_at: chrono::Utc::now().to_rfc3339(),
            validator_version: env!("CARGO_PKG_VERSION").to_string(),
            request_id: None, // Could be populated from request if tracked
            validation_trace: layers_executed
                .into_iter()
                .map(|layer| crate::model::response_types::ValidationStep {
                    layer: format!("{:?}", layer),
                    passed: true,
                    error: None,
                })
                .collect(),
        },
    })
}

/// Compute SHA-256 hash of canonical JSON content.
fn compute_content_hash(content: &serde_json::Value) -> Result<String, Error> {
    use sha2::{Sha256, Digest};
    
    // Canonical JSON: sorted keys, compact separators, ASCII
    let canonical = serde_json::to_string(content)
        .map_err(|e| Error::Internal {
            message: format!("Failed to canonicalize JSON for hashing: {}", e),
        })?;
    
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let digest = hasher.finalize();
    
    Ok(hex::encode(digest))
}

/// Pre-validate an AiAuthoringRequest before generation.
///
/// Catches phase violations, unknown objectKinds, and unresolvable
/// referencedIds early, saving AI generation cycles.
pub fn validate_request_early(
    director: &crate::ChatDirector,
    req: &AiAuthoringRequest,
) -> Vec<EarlyWarning> {
    let mut warnings = Vec::new();
    
    // Check phase permission
    if let Err(phase_err) = crate::phases::check_phase_permission(
        req,
        crate::model::spine_types::ContractFamily::from_object_kind(&req.object_kind),
    ) {
        warnings.push(EarlyWarning {
            code: "PHASE_VIOLATION".into(),
            message: format!("Phase {:?} does not permit {}", req.phase, req.object_kind),
            remediation: "Adjust phase or objectKind to match phase permission matrix".into(),
            fix_order: 1,
        });
    }
    
    // Check objectKind exists in spine
    if director.spine.describe_object_kind(&req.object_kind).is_none() {
        warnings.push(EarlyWarning {
            code: "UNKNOWN_OBJECT_KIND".into(),
            message: format!("Object kind '{}' not found in schema spine", req.object_kind),
            remediation: "Use an objectKind defined in the schema spine".into(),
            fix_order: 1,
        });
    }
    
    // Check referencedIds resolvability (simplified)
    for id in &req.referenced_ids {
        if id.starts_with("REF-unknown") {
            warnings.push(EarlyWarning {
                code: "UNRESOLVABLE_REFERENCE".into(),
                message: format!("Referenced ID '{}' cannot be resolved", id),
                remediation: "Ensure referenced IDs exist in the registry".into(),
                fix_order: 2,
            });
        }
    }
    
    warnings
}

/// Early warning for request pre-validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarlyWarning {
    pub code: String,
    pub message: String,
    pub remediation: String,
    pub fix_order: u32,
}
