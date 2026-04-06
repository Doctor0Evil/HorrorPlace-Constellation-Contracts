// crates/hpc-chat-director/src/validate/invariants.rs

//! Invariant and metric enforcement logic.
//!
//! Translates the invariant and entertainment-metric spines into
//! concrete numeric enforcement rules. Validates ranges, applies
//! cross-metric interactions (XMIT rules), and emits structured
//! diagnostics and explanation records.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::errors::{InteractionEffect, Severity, ValidationError, ValidationLayer};
use crate::model::request_types::AiAuthoringRequest;
use crate::model::response_types::AiAuthoringResponse;
use crate::spine::{InvariantSpec, MetricSpec, SpineIndex};

/// Invariant-level diagnostic with cross-metric effects and fix ordering.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvariantDiagnostic {
    pub code: String,
    pub json_pointer: String,
    pub submitted_value: Option<serde_json::Value>,
    pub expected: String,
    pub remediation: String,
    pub severity: Severity,
    pub fix_order: u32,
    pub interaction_effects: Vec<InteractionEffect>,
}

/// All invariant/metric diagnostics for a single artifact.
#[derive(Debug, Default)]
pub struct InvariantValidationResult {
    pub diagnostics: Vec<InvariantDiagnostic>,
}

/// Placeholder type for storing cross-metric state (e.g., adjusted bands).
#[derive(Debug, Default)]
pub struct XmitState {
    pub adjusted_metric_bands: HashMap<String, (f64, f64)>,
}

/// Entry point: enforce invariants and entertainment metrics for a response.
pub fn validate_invariants(
    spine: &SpineIndex,
    req: &AiAuthoringRequest,
    resp: &AiAuthoringResponse,
) -> InvariantValidationResult {
    let mut result = InvariantValidationResult::default();

    let object_kind = req.object_kind;
    let tier = req.tier;

    // Raw invariant and metric maps extracted from payload.
    let invariants = extract_invariants(resp);
    let metrics = extract_metrics(resp);

    // Pass 1: raw range checks against spine.
    apply_raw_range_checks(
        spine,
        object_kind,
        tier,
        &invariants,
        &metrics,
        &mut result.diagnostics,
    );

    // Pass 2: apply cross-metric XMIT rules, possibly generating additional diagnostics.
    let xmit_state = apply_all_xmit_rules(
        spine,
        object_kind,
        tier,
        &invariants,
        &metrics,
        &mut result.diagnostics,
    );

    // Pass 3: post-XMIT checks, using any adjusted bands from XMIT.
    apply_post_xmit_checks(
        spine,
        object_kind,
        tier,
        &invariants,
        &metrics,
        &xmit_state,
        &mut result.diagnostics,
    );

    result
}

/// Extract invariant values from the response payload.
fn extract_invariants(resp: &AiAuthoringResponse) -> HashMap<String, f64> {
    let mut out = HashMap::new();
    if let Some(bindings) = resp.payload.get("invariantBindings") {
        if let Some(obj) = bindings.as_object() {
            for (name, spec) in obj {
                if let Some(value) = spec.get("value").and_then(|v| v.as_f64()) {
                    out.insert(name.clone(), value);
                }
            }
        }
    }
    out
}

/// Extract metric targets from the response payload.
fn extract_metrics(resp: &AiAuthoringResponse) -> HashMap<String, f64> {
    let mut out = HashMap::new();
    if let Some(targets) = resp.payload.get("metricTargets") {
        if let Some(obj) = targets.as_object() {
            for (name, spec) in obj {
                if let Some(value) = spec.get("target").and_then(|v| v.as_f64()) {
                    out.insert(name.clone(), value);
                }
            }
        }
    }
    out
}

/// Pass 1: strict range checks from spine for each invariant/metric.
fn apply_raw_range_checks(
    spine: &SpineIndex,
    object_kind: crate::model::spine_types::ObjectKind,
    tier: crate::model::spine_types::Tier,
    invariants: &HashMap<String, f64>,
    metrics: &HashMap<String, f64>,
    diags: &mut Vec<InvariantDiagnostic>,
) {
    for (name, value) in invariants {
        if let Some(spec) = spine.describe_invariant(name) {
            if let Some((min, max)) = spine.range_for_invariant(&spec, object_kind, tier) {
                if value < &min || value > &max {
                    diags.push(out_of_range_invariant(&spec, *value, min, max));
                }
            }
        }
    }

    for (name, value) in metrics {
        if let Some(spec) = spine.describe_metric(name) {
            let (min, max) = spine.target_band_for_metric(&spec, object_kind, tier);
            if value < &min || value > &max {
                diags.push(out_of_range_metric(&spec, *value, min, max));
            }
        }
    }
}

/// Pass 2: apply all XMIT_XXX cross-metric rules; may record adjusted bands and diagnostics.
fn apply_all_xmit_rules(
    spine: &SpineIndex,
    object_kind: crate::model::spine_types::ObjectKind,
    tier: crate::model::spine_types::Tier,
    invariants: &HashMap<String, f64>,
    metrics: &HashMap<String, f64>,
    diags: &mut Vec<InvariantDiagnostic>,
) -> XmitState {
    let mut state = XmitState::default();

    // XMIT_001: DET amplifies CDL floor when DET >= 8.0
    if let (Some(det), Some(cdl)) = (invariants.get("DET"), metrics.get("CDL")) {
        if *det >= 8.0 {
            let (floor, _) = spine.cdl_floor_for_det(object_kind, tier, *det);
            if *cdl < floor {
                diags.push(InvariantDiagnostic {
                    code: "ERR_XMIT_001".to_string(),
                    json_pointer: "/metricTargets/CDL/target".to_string(),
                    submitted_value: Some(serde_json::json!(cdl)),
                    expected: format!("CDL >= {:.2} when DET >= 8.0", floor),
                    remediation: format!(
                        "Increase CDL to at least {:.2} or lower DET below 8.0.",
                        floor
                    ),
                    severity: Severity::Error,
                    fix_order: 2,
                    interaction_effects: vec![InteractionEffect {
                        interaction_id: "XMIT_001".to_string(),
                        description:
                            "High DET raises the minimum acceptable CDL floor for mood/event contracts."
                                .to_string(),
                    }],
                });
            }
        }
    }

    // XMIT_002: CIC widens SHCI bands when CIC >= 0.7
    if let (Some(cic), Some(shci)) = (invariants.get("CIC"), invariants.get("SHCI")) {
        if *cic >= 0.7 {
            let (base_min, base_max) = spine.shci_band_for_object_kind(object_kind, tier);
            let (adj_min, adj_max) =
                spine.shci_band_with_cic(object_kind, tier, *cic, base_min, base_max);
            state
                .adjusted_metric_bands
                .insert("SHCI".to_string(), (adj_min, adj_max));

            if *shci < adj_min || *shci > adj_max {
                diags.push(InvariantDiagnostic {
                    code: "ERR_XMIT_002".to_string(),
                    json_pointer: "/invariantBindings/SHCI/value".to_string(),
                    submitted_value: Some(serde_json::json!(shci)),
                    expected: format!(
                        "SHCI in [{:.2}, {:.2}] when CIC >= 0.7",
                        adj_min, adj_max
                    ),
                    remediation:
                        "Adjust SHCI into the widened band or reduce CIC below 0.7.".to_string(),
                    severity: Severity::Warning,
                    fix_order: 3,
                    interaction_effects: vec![InteractionEffect {
                        interaction_id: "XMIT_002".to_string(),
                        description:
                            "High CIC widens acceptable SHCI bands; out-of-range SHCI weakens spectral-history coupling."
                                .to_string(),
                    }],
                });
            }
        }
    }

    // XMIT_003: DET suppresses ARR floor for certain archetypes.
    if let (Some(det), Some(arr)) = (invariants.get("DET"), metrics.get("ARR")) {
        let (floor, suppressed_floor) = spine.arr_floor_for_det(object_kind, tier, *det);
        let effective_floor = suppressed_floor.unwrap_or(floor);
        if *arr < effective_floor {
            diags.push(InvariantDiagnostic {
                code: "ERR_XMIT_003".to_string(),
                json_pointer: "/metricTargets/ARR/target".to_string(),
                submitted_value: Some(serde_json::json!(arr)),
                expected: format!("ARR >= {:.2} for DET {:.2}", effective_floor, det),
                remediation: format!(
                    "Increase ARR to at least {:.2}, or lower DET to reduce suppression.",
                    effective_floor
                ),
                severity: Severity::Warning,
                fix_order: 4,
                interaction_effects: vec![InteractionEffect {
                    interaction_id: "XMIT_003".to_string(),
                    description:
                        "Very high DET may suppress ARR floor; extreme horror accepts lower audience retention."
                            .to_string(),
                }],
            });
        }
    }

    // XMIT_004: AOS amplifies EMD band.
    if let (Some(aos), Some(emd)) = (invariants.get("AOS"), metrics.get("EMD")) {
        if *aos >= 0.6 {
            let (min, max) = spine.emd_band_for_aos(object_kind, tier, *aos);
            state
                .adjusted_metric_bands
                .insert("EMD".to_string(), (min, max));
            if *emd < min || *emd > max {
                diags.push(InvariantDiagnostic {
                    code: "ERR_XMIT_004".to_string(),
                    json_pointer: "/metricTargets/EMD/target".to_string(),
                    submitted_value: Some(serde_json::json!(emd)),
                    expected: format!("EMD in [{:.2}, {:.2}] when AOS >= 0.6", min, max),
                    remediation:
                        "Adjust EMD into the amplified band or reduce AOS below 0.6.".to_string(),
                    severity: Severity::Error,
                    fix_order: 2,
                    interaction_effects: vec![InteractionEffect {
                        interaction_id: "XMIT_004".to_string(),
                        description:
                            "High AOS amplifies EMD targets; entertainment variance must track ambient oscillation."
                                .to_string(),
                    }],
                });
            }
        }
    }

    // XMIT_005: LSG gates HVF floor.
    if let (Some(lsg), Some(hvf)) = (invariants.get("LSG"), invariants.get("HVF")) {
        if *lsg <= 0.2 {
            let floor = spine.hvf_floor_for_lsg(object_kind, tier, *lsg);
            if *hvf < floor {
                diags.push(InvariantDiagnostic {
                    code: "ERR_XMIT_005".to_string(),
                    json_pointer: "/invariantBindings/HVF/value".to_string(),
                    submitted_value: Some(serde_json::json!(hvf)),
                    expected: format!("HVF >= {:.2} when LSG <= 0.2", floor),
                    remediation: format!(
                        "Increase HVF to at least {:.2} or raise LSG to a more stable value.",
                        floor
                    ),
                    severity: Severity::Error,
                    fix_order: 1,
                    interaction_effects: vec![InteractionEffect {
                        interaction_id: "XMIT_005".to_string(),
                        description:
                            "Unstable liminal spaces enforce a higher horror viscosity floor."
                                .to_string(),
                    }],
                });
            }
        }
    }

    state
}

/// Pass 3: post-XMIT re-checks; ensures diagnostics are aware of adjusted bands.
fn apply_post_xmit_checks(
    spine: &SpineIndex,
    object_kind: crate::model::spine_types::ObjectKind,
    tier: crate::model::spine_types::Tier,
    invariants: &HashMap<String, f64>,
    _metrics: &HashMap<String, f64>,
    xmit_state: &XmitState,
    diags: &mut Vec<InvariantDiagnostic>,
) {
    // Derived metrics like SPR, SHCI plausibility checks.
    if let Some(derived) = spine.compute_derived_metrics(invariants) {
        if let Some(spr) = derived.spr {
            let (min, max) = spine.spr_band(object_kind, tier);
            if spr < min || spr > max {
                diags.push(InvariantDiagnostic {
                    code: "ERR_SPR_DERIVED".to_string(),
                    json_pointer: "/invariantBindings/SPR/value".to_string(),
                    submitted_value: Some(serde_json::json!(spr)),
                    expected: format!("SPR in [{:.2}, {:.2}]", min, max),
                    remediation:
                        "Adjust CIC, MDI, AOS, LSG, or FCF so that derived SPR falls in the plausible band."
                            .to_string(),
                    severity: Severity::Error,
                    fix_order: 3,
                    interaction_effects: vec![],
                });
            }
        }

        if let Some(shci) = derived.shci {
            let (min, max) = xmit_state
                .adjusted_metric_bands
                .get("SHCI")
                .cloned()
                .unwrap_or_else(|| spine.shci_band_for_object_kind(object_kind, tier));
            if shci < min || shci > max {
                diags.push(InvariantDiagnostic {
                    code: "ERR_SHCI_COUPLING".to_string(),
                    json_pointer: "/invariantBindings/SHCI/value".to_string(),
                    submitted_value: Some(serde_json::json!(shci)),
                    expected: format!("SHCI in [{:.2}, {:.2}]", min, max),
                    remediation:
                        "Align SHCI with referenced history bundles or adjust CIC/MDI to more plausible values."
                            .to_string(),
                    severity: Severity::Error,
                    fix_order: 2,
                    interaction_effects: vec![InteractionEffect {
                        interaction_id: "XMIT_SHCI".to_string(),
                        description:
                            "SHCI must remain consistent with local history and CIC-driven coupling."
                                .to_string(),
                    }],
                });
            }
        }
    }
}

fn out_of_range_invariant(
    spec: &InvariantSpec,
    value: f64,
    min: f64,
    max: f64,
) -> InvariantDiagnostic {
    InvariantDiagnostic {
        code: format!("ERR_{}_RANGE", spec.abbreviation.to_uppercase()),
        json_pointer: format!("/invariantBindings/{}/value", spec.abbreviation),
        submitted_value: Some(serde_json::json!(value)),
        expected: format!(
            "{} in [{:.2}, {:.2}] for this objectKind and tier",
            spec.abbreviation, min, max
        ),
        remediation: format!(
            "Clamp {} into [{:.2}, {:.2}] or change tier/objectKind to one that permits {:.2}.",
            spec.abbreviation, min, max, value
        ),
        severity: Severity::Error,
        fix_order: 1,
        interaction_effects: vec![],
    }
}

fn out_of_range_metric(
    spec: &MetricSpec,
    value: f64,
    min: f64,
    max: f64,
) -> InvariantDiagnostic {
    InvariantDiagnostic {
        code: format!("ERR_{}_RANGE", spec.abbreviation.to_uppercase()),
        json_pointer: format!("/metricTargets/{}/target", spec.abbreviation),
        submitted_value: Some(serde_json::json!(value)),
        expected: format!(
            "{} target in [{:.2}, {:.2}] for this objectKind and tier",
            spec.abbreviation, min, max
        ),
        remediation: format!(
            "Adjust {} target into [{:.2}, {:.2}] or request a different archetype/tier.",
            spec.abbreviation, min, max
        ),
        severity: Severity::Error,
        fix_order: 1,
        interaction_effects: vec![],
    }
}

/// Lift an InvariantDiagnostic into the crate-wide ValidationError.
pub fn as_validation_errors(diags: Vec<InvariantDiagnostic>) -> Vec<ValidationError> {
    diags
        .into_iter()
        .map(|d| ValidationError {
            code: d.code,
            layer: ValidationLayer::Invariant,
            severity: d.severity,
            json_pointer: d.json_pointer,
            submitted_value: d.submitted_value,
            expected: d.expected,
            remediation: d.remediation,
            fix_order: d.fix_order,
            interaction_effects: d.interaction_effects,
        })
        .collect()
}
