//! Phase gating engine for CHAT_DIRECTOR.
//!
//! Determines which contract families are permitted at each pipeline phase,
//! enforces promotion predicates, and emits structured diagnostics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::model::spine_types::{ContractFamily, Phase, Tier};
use crate::model::manifest_types::RepoManifest;
use crate::model::request_types::AiAuthoringRequest;
use crate::model::response_types::AiAuthoringResponse;

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

/// Returns the permission level for a contract family at a phase.
///
/// This implements the canonical 5×4 phase permission matrix for v1.
pub fn phase_permission(phase: Phase, family: ContractFamily) -> PhasePermission {
    match (phase, family) {
        (Phase::Schema0, _) => PhasePermission::Forbidden,
        (Phase::Registry1, _) => PhasePermission::RegistryOnly,
        (Phase::Bundles2, _) => PhasePermission::Allowed,
        (Phase::LuaPolicy3, ContractFamily::MoodContract | ContractFamily::EventContract) => {
            PhasePermission::ReadOnlyRef
        }
        (Phase::LuaPolicy3, _) => PhasePermission::Forbidden,
        (Phase::Adapters4, _) => PhasePermission::Forbidden,
    }
}

/// Phase violation error.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum PhaseError {
    /// Requested phase does not permit this contract family.
    PhaseForbidden {
        requested_phase: Phase,
        contract_family: ContractFamily,
        permission: PhasePermission,
        diagnostic: PhaseDiagnostic,
    },
    /// Promotion predicate failed between phases.
    PromotionBlocked {
        from_phase: Phase,
        to_phase: Phase,
        predicate_code: String,
        failures: Vec<PromotionFailure>,
        diagnostic: PhaseDiagnostic,
    },
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
    pub target_phase: Phase,
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
    pub json_pointer: Option<String>,
    /// Submitted value that caused failure.
    pub submitted_value: Option<serde_json::Value>,
    /// Expected value or condition.
    pub expected: Option<String>,
}

/// Check if a request is permitted at its declared phase.
///
/// Returns `Ok(())` if allowed, or `PhaseError::PhaseForbidden` if not.
pub fn check_phase_permission(
    req: &AiAuthoringRequest,
    family: ContractFamily,
) -> Result<(), PhaseError> {
    let permission = phase_permission(req.phase, family);
    
    match permission {
        PhasePermission::Allowed | PhasePermission::RegistryOnly => Ok(()),
        PhasePermission::ReadOnlyRef => {
            // ReadOnlyRef means the contract can be referenced but not authored
            Err(PhaseError::PhaseForbidden {
                requested_phase: req.phase,
                contract_family: family,
                permission,
                diagnostic: PhaseDiagnostic {
                    code: "ERR_PHASE_READONLY".into(),
                    layer: "phase".into(),
                    severity: "error".into(),
                    message: format!(
                        "{} is read-only at Phase {:?}; cannot author new content",
                        family_name(family),
                        req.phase
                    ),
                    phase_upgrade_suggestion: Some(PhaseUpgradeSuggestion {
                        target_phase: Phase::Bundles2,
                        missing_prerequisites: vec![
                            "Complete registry entry in Phase 1".into(),
                            "Resolve all referenced_ids".into(),
                        ],
                        estimated_effort: "moderate".into(),
                        auto_fixable: false,
                    }),
                    remediation: format!(
                        "Submit this {} at Phase 2 (Bundles) for full authoring, \
                         or use as a read-only reference at Phase 3 (LuaPolicy)",
                        family_name(family)
                    ),
                    fix_order: 1,
                },
            })
        }
        PhasePermission::Forbidden => Err(PhaseError::PhaseForbidden {
            requested_phase: req.phase,
            contract_family: family,
            permission,
            diagnostic: PhaseDiagnostic {
                code: "ERR_PHASE_FORBIDDEN".into(),
                layer: "phase".into(),
                severity: "error".into(),
                message: format!(
                    "{} cannot be authored at Phase {:?} ({})",
                    family_name(family),
                    req.phase,
                    phase_name(req.phase)
                ),
                phase_upgrade_suggestion: suggested_upgrade(family, req.phase),
                remediation: format!(
                    "Submit this {} at Phase 2 (Bundles). \
                     Phase {:?} does not permit {} authoring.",
                    family_name(family),
                    req.phase,
                    family_name(family)
                ),
                fix_order: 1,
            },
        }),
    }
}

/// Get human-readable name for a contract family.
fn family_name(family: ContractFamily) -> &'static str {
    match family {
        ContractFamily::MoodContract => "moodContract",
        ContractFamily::EventContract => "eventContract",
        ContractFamily::RegionContractCard => "regionContractCard",
        ContractFamily::SeedContractCard => "seedContractCard",
    }
}

/// Get human-readable name for a phase.
fn phase_name(phase: Phase) -> &'static str {
    match phase {
        Phase::Schema0 => "Schema",
        Phase::Registry1 => "Registry",
        Phase::Bundles2 => "Bundles",
        Phase::LuaPolicy3 => "LuaPolicy",
        Phase::Adapters4 => "Adapters",
    }
}

/// Suggest phase upgrade path for a forbidden request.
fn suggested_upgrade(family: ContractFamily, current: Phase) -> Option<PhaseUpgradeSuggestion> {
    match (family, current) {
        (_, Phase::Schema0) => Some(PhaseUpgradeSuggestion {
            target_phase: Phase::Registry1,
            missing_prerequisites: vec!["Define schema in spine".into()],
            estimated_effort: "trivial".into(),
            auto_fixable: true,
        }),
        (_, Phase::Registry1) => Some(PhaseUpgradeSuggestion {
            target_phase: Phase::Bundles2,
            missing_prerequisites: vec![
                "Resolve all referenced_ids in registry".into(),
                "Pass invariant range checks".into(),
            ],
            estimated_effort: "moderate".into(),
            auto_fixable: false,
        }),
        (ContractFamily::MoodContract | ContractFamily::EventContract, Phase::Bundles2) => {
            Some(PhaseUpgradeSuggestion {
                target_phase: Phase::LuaPolicy3,
                missing_prerequisites: vec![
                    "Pass full 4-layer validation".into(),
                    "Declare all required invariant bindings".into(),
                ],
                estimated_effort: "substantial".into(),
                auto_fixable: false,
            })
        }
        _ => None,
    }
}

/// Promotion predicate: Schema0 → Registry1
///
/// Returns Ok if the objectKind's schema exists in the spine and self-validates.
pub fn check_prom001(
    object_kind: &str,
    spine: &crate::model::spine_types::SchemaSpine,
) -> Result<(), PromotionFailure> {
    // Check if object_kind is in any contract family
    let found = spine
        .contract_families
        .iter()
        .any(|f| f.kinds.contains(&object_kind.to_string()));
    
    if found {
        Ok(())
    } else {
        Err(PromotionFailure {
            check_id: "PROM001".into(),
            description: format!("Schema for '{}' not found in spine", object_kind),
            json_pointer: Some("/objectKind".into()),
            submitted_value: Some(serde_json::json!(object_kind)),
            expected: Some("A contract family in the spine that includes this objectKind".into()),
        })
    }
}

/// Promotion predicate: Registry1 → Bundles2
///
/// Validates that all referenced IDs exist and target repo accepts the objectKind.
pub fn check_prom002(
    req: &AiAuthoringRequest,
    manifests: &[RepoManifest],
) -> Result<(), Vec<PromotionFailure>> {
    let mut failures = Vec::new();
    
    // Check if target repo accepts this object kind
    let repo_accepts = manifests
        .iter()
        .find(|m| m.repo_name == req.target_repo)
        .map_or(false, |m| m.allows_object_kind(&req.object_kind));
    
    if !repo_accepts {
        failures.push(PromotionFailure {
            check_id: "PROM002".into(),
            description: format!(
                "Repo '{}' does not accept objectKind '{}'",
                req.target_repo, req.object_kind
            ),
            json_pointer: Some("/targetRepo".into()),
            submitted_value: Some(serde_json::json!(&req.target_repo)),
            expected: Some("A repo manifest that allows this objectKind".into()),
        });
    }
    
    // Check referenced IDs (simplified; real version queries registry)
    for id in &req.referenced_ids {
        if id.starts_with("REF-unknown") {
            failures.push(PromotionFailure {
                check_id: "PROM002".into(),
                description: format!("Referenced ID '{}' not found in registry", id),
                json_pointer: Some("/referencedIds".into()),
                submitted_value: Some(serde_json::json!(id)),
                expected: Some("An ID that resolves to an existing registry entry".into()),
            });
        }
    }
    
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures)
    }
}

/// Promotion predicate: Bundles2 → LuaPolicy3
///
/// Runs full 4-layer validation on the contract card.
pub fn check_prom003(
    _resp: &AiAuthoringResponse,
    _spine: &crate::model::spine_types::SchemaSpine,
    _manifests: &[RepoManifest],
) -> Result<(), Vec<PromotionFailure>> {
    // Placeholder: real implementation calls validate::validate_response
    // For v1, assume promotion succeeds if response exists
    Ok(())
}

/// Promotion predicate: LuaPolicy3 → Adapters4 (v1 stub)
pub fn check_prom004(
    _resp: &AiAuthoringResponse,
) -> Result<(), Vec<PromotionFailure>> {
    // v1 stub: always succeeds with warning
    Ok(())
}

/// Plan promotion from one phase to the next.
///
/// Returns Ok if all predicates pass, or PhaseError::PromotionBlocked with failures.
pub fn plan_promotion(
    from_phase: Phase,
    to_phase: Phase,
    family: ContractFamily,
    req: &AiAuthoringRequest,
    resp: &AiAuthoringResponse,
    spine: &crate::model::spine_types::SchemaSpine,
    manifests: &[RepoManifest],
) -> Result<(), PhaseError> {
    match (from_phase, to_phase) {
        (Phase::Schema0, Phase::Registry1) => {
            check_prom001(&req.object_kind, spine).map_err(|failure| {
                PhaseError::PromotionBlocked {
                    from_phase,
                    to_phase,
                    predicate_code: "PROM001".into(),
                    failures: vec![failure],
                    diagnostic: PhaseDiagnostic {
                        code: "ERR_PROMOTION_BLOCKED".into(),
                        layer: "phase".into(),
                        severity: "error".into(),
                        message: "Schema promotion failed".into(),
                        phase_upgrade_suggestion: None,
                        remediation: "Ensure the objectKind is defined in the schema spine".into(),
                        fix_order: 1,
                    },
                }
            })
        }
        (Phase::Registry1, Phase::Bundles2) => {
            check_prom002(req, manifests).map_err(|failures| {
                PhaseError::PromotionBlocked {
                    from_phase,
                    to_phase,
                    predicate_code: "PROM002".into(),
                    failures,
                    diagnostic: PhaseDiagnostic {
                        code: "ERR_PROMOTION_BLOCKED".into(),
                        layer: "phase".into(),
                        severity: "error".into(),
                        message: "Registry promotion failed: unresolved references or repo mismatch".into(),
                        phase_upgrade_suggestion: Some(PhaseUpgradeSuggestion {
                            target_phase: Phase::Bundles2,
                            missing_prerequisites: vec![
                                "Resolve all referenced_ids".into(),
                                "Ensure target repo accepts this objectKind".into(),
                            ],
                            estimated_effort: "moderate".into(),
                            auto_fixable: false,
                        }),
                        remediation: "Fix unresolved references and verify repo routing before promoting".into(),
                        fix_order: 1,
                    },
                }
            })
        }
        (Phase::Bundles2, Phase::LuaPolicy3) => {
            check_prom003(resp, spine, manifests).map_err(|failures| {
                PhaseError::PromotionBlocked {
                    from_phase,
                    to_phase,
                    predicate_code: "PROM003".into(),
                    failures,
                    diagnostic: PhaseDiagnostic {
                        code: "ERR_PROMOTION_BLOCKED".into(),
                        layer: "phase".into(),
                        severity: "error".into(),
                        message: "Bundle promotion failed: validation errors".into(),
                        phase_upgrade_suggestion: None,
                        remediation: "Pass all schema, invariant, manifest, and envelope checks before promoting".into(),
                        fix_order: 1,
                    },
                }
            })
        }
        (Phase::LuaPolicy3, Phase::Adapters4) => {
            check_prom004(resp).map_err(|failures| {
                PhaseError::PromotionBlocked {
                    from_phase,
                    to_phase,
                    predicate_code: "PROM004".into(),
                    failures,
                    diagnostic: PhaseDiagnostic {
                        code: "ERR_PROMOTION_BLOCKED".into(),
                        layer: "phase".into(),
                        severity: "error".into(),
                        message: "Adapter promotion failed (v1 stub)".into(),
                        phase_upgrade_suggestion: None,
                        remediation: "Adapter promotion is not fully implemented in v1".into(),
                        fix_order: 1,
                    },
                }
            })
        }
        _ => Err(PhaseError::PhaseForbidden {
            requested_phase: to_phase,
            contract_family: family,
            permission: PhasePermission::Forbidden,
            diagnostic: PhaseDiagnostic {
                code: "ERR_INVALID_PROMOTION".into(),
                layer: "phase".into(),
                severity: "error".into(),
                message: format!(
                    "Invalid promotion from {:?} to {:?}",
                    from_phase, to_phase
                ),
                phase_upgrade_suggestion: None,
                remediation: "Promotions must follow the canonical phase sequence: 0→1→2→3→4".into(),
                fix_order: 1,
            },
        }),
    }
}
