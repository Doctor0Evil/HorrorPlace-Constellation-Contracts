// crates/hpc-chat-director/src/validate/manifests.rs

//! Repository manifest validation: routing, tier rules, and policy checks.
//!
//! Validates that AI-generated artifacts comply with repo-level policies:
//! correct targetRepo for objectKind, tier-appropriate content, one-file-per-request,
//! mandatory deadledgerRef, minimum RWF, and cross-repo reference rules.

use serde::{Deserialize, Serialize};

use crate::model::manifest_types::{AuthoringHints, RepoManifest, Tier};
use crate::model::request_types::AiAuthoringRequest;
use crate::model::response_types::AiAuthoringResponse;

/// High-level manifest validation result.
#[derive(Debug)]
pub struct ManifestValidationResult {
    pub diagnostics: Vec<ManifestDiagnostic>,
    pub is_fatal: bool,
}

/// Structured manifest-related diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestDiagnostic {
    pub code: ManifestDiagnosticCode,
    pub message: String,
    pub severity: ManifestDiagnosticSeverity,
    pub charter_rationale: Option<String>,
    pub suggested_alternative_repo: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestDiagnosticSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestDiagnosticCode {
    InvalidTargetRepo,
    ObjectKindNotAllowedInRepo,
    TierPolicyViolation,
    OneFilePerRequestViolation,
    MissingDeadledgerRef,
    CrossRepoRefNotAllowed,
    RwfBelowTierMinimum,
}

/// Context for manifest-based validation queries.
pub struct ManifestContext<'a> {
    /// All known manifests keyed by repo name.
    pub manifests_by_repo: &'a [RepoManifest],
}

impl<'a> ManifestContext<'a> {
    /// Find a manifest by repo name.
    pub fn find_manifest(&self, repo_name: &str) -> Option<&'a RepoManifest> {
        self.manifests_by_repo.iter().find(|m| m.repo == repo_name)
    }

    /// Resolve which repo owns a given ID (for cross-repo reference validation).
    ///
    /// The real implementation should consult registry state; this is a placeholder.
    pub fn lookup_repo_for_id(&self, id: &str) -> Option<String> {
        if id.starts_with("event.") {
            Some("HorrorPlace-Atrocity-Seeds".to_string())
        } else if id.starts_with("region.") {
            Some("HorrorPlace-Atrocity-Seeds".to_string())
        } else if id.starts_with("bundle.") {
            Some("HorrorPlace-Black-Archivum".to_string())
        } else {
            None
        }
    }
}

/// Entry point: validate routing and tier/policy rules for a response.
pub fn validate_manifests(
    ctx: &ManifestContext,
    req: &AiAuthoringRequest,
    resp: &AiAuthoringResponse,
) -> ManifestValidationResult {
    let mut diagnostics = Vec::new();
    let mut is_fatal = false;

    // Determine target repo from the response payload.
    let target_repo = resp.target_repo();
    let Some(manifest) = ctx.find_manifest(target_repo) else {
        diagnostics.push(ManifestDiagnostic {
            code: ManifestDiagnosticCode::InvalidTargetRepo,
            message: format!(
                "Target repo '{}' is not known in the current constellation.",
                target_repo
            ),
            severity: ManifestDiagnosticSeverity::Error,
            charter_rationale: None,
            suggested_alternative_repo: None,
        });
        return ManifestValidationResult {
            diagnostics,
            is_fatal: true,
        };
    };

    // Check objectKind is allowed in this repo.
    if !manifest.allows_object_kind(&req.object_kind) {
        let (charter, suggestion) = manifest.tier_violation_hints_for(&req.object_kind);
        diagnostics.push(ManifestDiagnostic {
            code: ManifestDiagnosticCode::ObjectKindNotAllowedInRepo,
            message: format!(
                "Object kind '{}' is not allowed in repo '{}'.",
                req.object_kind, manifest.repo
            ),
            severity: ManifestDiagnosticSeverity::Error,
            charter_rationale: charter,
            suggested_alternative_repo: suggestion,
        });
        is_fatal = true;
    }

    // Tier policy checks: e.g., Tier 1 = contracts only, no raw narrative.
    if let Some(d) = check_tier_policy(manifest, req) {
        if matches!(d.severity, ManifestDiagnosticSeverity::Error) {
            is_fatal = true;
        }
        diagnostics.push(d);
    }

    // One-file-per-request enforcement (if enabled in manifest rules).
    if manifest.rules.one_file_per_request {
        if resp.additional_artifacts_count() > 0 {
            diagnostics.push(ManifestDiagnostic {
                code: ManifestDiagnosticCode::OneFilePerRequestViolation,
                message: "Response contains more than one artifact in a one-file-per-request repo."
                    .to_string(),
                severity: ManifestDiagnosticSeverity::Error,
                charter_rationale: manifest
                    .authoring_hints
                    .one_file_per_request_rationale
                    .clone(),
                suggested_alternative_repo: None,
            });
            is_fatal = true;
        }
    }

    // Dead-Ledger requirements.
    if manifest.rules.require_deadledger_ref {
        if !resp.has_deadledger_ref() {
            diagnostics.push(ManifestDiagnostic {
                code: ManifestDiagnosticCode::MissingDeadledgerRef,
                message: "Dead-Ledger reference is required by this repo's policy but missing."
                    .to_string(),
                severity: ManifestDiagnosticSeverity::Error,
                charter_rationale: manifest.authoring_hints.deadledger_rationale.clone(),
                suggested_alternative_repo: None,
            });
            is_fatal = true;
        }
    }

    // Optional: RWF-gated routing for higher tiers.
    if let Some(d) = check_rwf_threshold(manifest, resp) {
        if matches!(d.severity, ManifestDiagnosticSeverity::Error) {
            is_fatal = true;
        }
        diagnostics.push(d);
    }

    // Optional: cross-repository reference checks.
    if let Some(diags) = validate_cross_refs(manifest, ctx, req) {
        if !diags.is_empty() {
            is_fatal = true;
            diagnostics.extend(diags.into_iter());
        }
    }

    ManifestValidationResult {
        diagnostics,
        is_fatal,
    }
}

/// Tier-specific policy checks, including charter rationale.
fn check_tier_policy(
    manifest: &RepoManifest,
    req: &AiAuthoringRequest,
) -> Option<ManifestDiagnostic> {
    match manifest.tier {
        Tier::T1 => {
            if manifest.is_raw_content_kind(&req.object_kind) {
                let (charter, suggestion) = manifest.tier_violation_hints_for(&req.object_kind);
                Some(ManifestDiagnostic {
                    code: ManifestDiagnosticCode::TierPolicyViolation,
                    message: format!(
                        "Raw narrative content for '{}' is not allowed in Tier 1 repo '{}'.",
                        req.object_kind, manifest.repo
                    ),
                    severity: ManifestDiagnosticSeverity::Error,
                    charter_rationale: charter,
                    suggested_alternative_repo: suggestion,
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

/// RWF gates: enforce minimum reliability weighting for higher tiers.
fn check_rwf_threshold(
    manifest: &RepoManifest,
    resp: &AiAuthoringResponse,
) -> Option<ManifestDiagnostic> {
    let Some(min_rwf) = manifest.rules.min_rwf_for_tier else {
        return None;
    };
    let Some(rwf) = resp.rwf_score() else {
        return None;
    };

    if rwf < min_rwf {
        Some(ManifestDiagnostic {
            code: ManifestDiagnosticCode::RwfBelowTierMinimum,
            message: format!(
                "RWF score {:.3} is below the minimum {:.3} required for repo '{}'.",
                rwf, min_rwf, manifest.repo
            ),
            severity: ManifestDiagnosticSeverity::Error,
            charter_rationale: manifest.authoring_hints.tier_rationale.clone(),
            suggested_alternative_repo: manifest.authoring_hints.default_staging_repo.clone(),
        })
    } else {
        None
    }
}

/// Cross-repo dependency validation hook.
fn validate_cross_refs(
    source_manifest: &RepoManifest,
    ctx: &ManifestContext,
    req: &AiAuthoringRequest,
) -> Option<Vec<ManifestDiagnostic>> {
    if req.referenced_ids.is_empty() {
        return None;
    }

    let mut diags = Vec::new();

    for id in &req.referenced_ids {
        if let Some(target_repo) = ctx.lookup_repo_for_id(id) {
            if !source_manifest.allows_cross_ref_to(&target_repo) {
                diags.push(ManifestDiagnostic {
                    code: ManifestDiagnosticCode::CrossRepoRefNotAllowed,
                    message: format!(
                        "Cross-repo reference '{}' to repo '{}' is not allowed from repo '{}'.",
                        id, target_repo, source_manifest.repo
                    ),
                    severity: ManifestDiagnosticSeverity::Error,
                    charter_rationale: source_manifest
                        .authoring_hints
                        .cross_repo_rationale
                        .clone(),
                    suggested_alternative_repo: None,
                });
            }
        }
    }

    if diags.is_empty() {
        None
    } else {
        Some(diags)
    }
}

// Helper methods to be implemented on your manifest and response types:
//
// impl RepoManifest {
//     pub fn allows_object_kind(&self, kind: &str) -> bool { /* ... */ }
//     pub fn is_raw_content_kind(&self, kind: &str) -> bool { /* ... */ }
//     pub fn tier_violation_hints_for(&self, kind: &str) -> (Option<String>, Option<String>) {
//         // Look up AuthoringHints charter text and suggestedAlternativeRepo.
//     }
//     pub fn allows_cross_ref_to(&self, target_repo: &str) -> bool { /* ... */ }
// }
//
// impl AiAuthoringResponse {
//     pub fn target_repo(&self) -> &str { /* ... */ }
//     pub fn additional_artifacts_count(&self) -> usize { /* ... */ }
//     pub fn has_deadledger_ref(&self) -> bool { /* ... */ }
//     pub fn rwf_score(&self) -> Option<f64> { /* ... */ }
// }
