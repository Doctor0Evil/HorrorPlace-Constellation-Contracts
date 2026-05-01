//! Repository manifest validation: routing, tier rules, and policy checks.

use serde::{Deserialize, Serialize};

use crate::model::manifest_types::{AiRole, RepoManifest, Tier};
use crate::model::request_types::AiAuthoringRequest;
use crate::model::response_types::AiAuthoringResponse;
use crate::model::{
    PolicyChecklist, RouteExplanation, ValidationDiagnostic, ValidationLayer, ValidationResult,
    ValidationSeverity,
};

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
    pub manifests_by_repo: &'a [RepoManifest],
}

impl<'a> ManifestContext<'a> {
    pub fn find_manifest(&self, repo_name: &str) -> Option<&'a RepoManifest> {
        self.manifests_by_repo
            .iter()
            .find(|m| m.repo_name == repo_name)
    }

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

/// Validate routing and tier/policy compliance for a given object kind and repo.
pub fn validate_routing(
    object_kind: &str,
    role: AiRole,
    target_repo: &RepoManifest,
    rwf_score: Option<f64>,
) -> ValidationResult {
    let mut diagnostics = Vec::new();
    let mut ok = true;

    if !target_repo.allows_object_kind(object_kind) {
        ok = false;
        let (charter, suggestion) = target_repo.tier_violation_hints_for(object_kind);
        let mut message = format!(
            "Object kind '{}' is not allowed in repo '{}'.",
            object_kind, target_repo.repo_name
        );
        if let Some(reason) = &charter {
            message.push(' ');
            message.push_str(reason);
        }
        let remediation = suggestion.map(|s| {
            format!(
                "Route this artifact to staging repo '{}' or another repo that accepts this object kind.",
                s
            )
        });

        diagnostics.push(ValidationDiagnostic {
            layer: ValidationLayer::Manifest,
            severity: ValidationSeverity::Error,
            code: "ROUTING_OBJECT_KIND_FORBIDDEN".to_string(),
            message,
            json_pointer: None,
            remediation,
        });
    }

    if let Some(rule) = target_repo.find_rule_for(object_kind) {
        if !rule.allowed_roles.is_empty() && !rule.allowed_roles.contains(&role) {
            ok = false;
            diagnostics.push(ValidationDiagnostic {
                layer: ValidationLayer::Manifest,
                severity: ValidationSeverity::Error,
                code: "ROUTING_ROLE_FORBIDDEN".to_string(),
                message: format!(
                    "Role '{:?}' is not allowed to author object kind '{}' in repo '{}'.",
                    role, object_kind, target_repo.repo_name
                ),
                json_pointer: None,
                remediation: Some(
                    "Use an authorized AI role for this operation, or target a repo that permits this role."
                        .to_string(),
                ),
            });
        }

        if !rule.notes.is_empty() {
            let note = rule.notes.join(" ");
            diagnostics.push(ValidationDiagnostic {
                layer: ValidationLayer::Manifest,
                severity: ValidationSeverity::Info,
                code: "ROUTING_NOTES".to_string(),
                message: format!(
                    "Routing rule notes for object kind '{}' in repo '{}': {}",
                    object_kind, target_repo.repo_name, note
                ),
                json_pointer: None,
                remediation: None,
            });
        }
    }

    if let Some(min_rwf) = target_repo.min_rwf_for_tier {
        if let Some(rwf) = rwf_score {
            if rwf < min_rwf {
                ok = false;
                diagnostics.push(ValidationDiagnostic {
                    layer: ValidationLayer::Manifest,
                    severity: ValidationSeverity::Error,
                    code: "RWF_BELOW_MINIMUM".to_string(),
                    message: format!(
                        "RWF score {:.3} is below the minimum {:.3} required for repo '{}'.",
                        rwf, min_rwf, target_repo.repo_name
                    ),
                    json_pointer: None,
                    remediation: Some(
                        "Route this artifact to a staging repo or improve confidence before targeting this repo."
                            .to_string(),
                    ),
                });
            }
        }
    }

    ValidationResult { ok, diagnostics }
}

/// Build a routing explanation for AI tools.
pub fn explain_route(
    object_kind: &str,
    tier: Option<Tier>,
    manifest: &RepoManifest,
) -> RouteExplanation {
    let path_hint = manifest
        .default_path_for(object_kind)
        .unwrap_or("contracts/{objectKind}/{id}.json")
        .to_string();

    let mut notes = Vec::new();
    if let (charter, _) = manifest.tier_violation_hints_for(object_kind) {
        if let Some(c) = charter {
            notes.push(c);
        }
    }

    RouteExplanation {
        object_kind: object_kind.to_string(),
        tier: tier.map(|t| t.as_str().to_string()),
        target_repo: manifest.repo_name.clone(),
        target_path_hint: path_hint,
        policy_notes: notes,
    }
}

/// Convert a manifest into a policy checklist diagnostic block.
pub fn checklist_for_manifest(manifest: &RepoManifest) -> PolicyChecklist {
    manifest.to_policy_checklist()
}

/// Entry point: validate routing and tier/policy rules for a response.
pub fn validate_manifests(
    ctx: &ManifestContext,
    req: &AiAuthoringRequest,
    resp: &AiAuthoringResponse,
) -> ManifestValidationResult {
    let mut diagnostics = Vec::new();
    let mut is_fatal = false;

    let target_repo_name = resp.target_repo();
    let Some(manifest) = ctx.find_manifest(target_repo_name) else {
        diagnostics.push(ManifestDiagnostic {
            code: ManifestDiagnosticCode::InvalidTargetRepo,
            message: format!(
                "Target repo '{}' is not known in the current constellation.",
                target_repo_name
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

    if !manifest.allows_object_kind(&req.object_kind) {
        let (charter, suggestion) = manifest.tier_violation_hints_for(&req.object_kind);
        diagnostics.push(ManifestDiagnostic {
            code: ManifestDiagnosticCode::ObjectKindNotAllowedInRepo,
            message: format!(
                "Object kind '{}' is not allowed in repo '{}'.",
                req.object_kind, manifest.repo_name
            ),
            severity: ManifestDiagnosticSeverity::Error,
            charter_rationale: charter,
            suggested_alternative_repo: suggestion,
        });
        is_fatal = true;
    }

    if let Some(d) = check_tier_policy(manifest, req) {
        if matches!(d.severity, ManifestDiagnosticSeverity::Error) {
            is_fatal = true;
        }
        diagnostics.push(d);
    }

    if manifest.rules.one_file_per_request && resp.additional_artifacts_count() > 0 {
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

    if manifest.rules.require_deadledger_ref && !resp.has_deadledger_ref() {
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

    if let Some(d) = check_rwf_threshold(manifest, resp) {
        if matches!(d.severity, ManifestDiagnosticSeverity::Error) {
            is_fatal = true;
        }
        diagnostics.push(d);
    }

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

fn check_tier_policy(
    manifest: &RepoManifest,
    req: &AiAuthoringRequest,
) -> Option<ManifestDiagnostic> {
    match manifest.tier {
        Tier::T1Core => {
            if manifest.is_raw_content_kind(&req.object_kind) {
                let (charter, suggestion) = manifest.tier_violation_hints_for(&req.object_kind);
                Some(ManifestDiagnostic {
                    code: ManifestDiagnosticCode::TierPolicyViolation,
                    message: format!(
                        "Raw narrative content for '{}' is not allowed in T1-core repo '{}'.",
                        req.object_kind, manifest.repo_name
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
                rwf, min_rwf, manifest.repo_name
            ),
            severity: ManifestDiagnosticSeverity::Error,
            charter_rationale: manifest.authoring_hints.tier_rationale.clone(),
            suggested_alternative_repo: manifest
                .authoring_hints
                .default_staging_repo
                .clone(),
        })
    } else {
        None
    }
}

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
                        id, target_repo, source_manifest.repo_name
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
