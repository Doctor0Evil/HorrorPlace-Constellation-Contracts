// crates/hpc-chat-director/src/validate/invariants.rs

use std::collections::HashMap;

use crate::model::spine_types::{ObjectKind, Tier};
use crate::spine::SpineIndex;
use crate::validate::{Diagnostic, Severity, ValidationLayer};
use serde_json::Value;

/// Basic scalar range (inclusive) used for invariant and metric checks.
#[derive(Debug, Clone)]
pub struct ScalarRange {
    pub min: f64,
    pub max: f64,
}

/// Identifies a JSON field to validate and how.
#[derive(Debug, Clone)]
pub enum InvariantKind {
    Structural,
    Metric,
    Derived,
}

/// One row of the enforcement table (range-level checks only).
#[derive(Debug, Clone)]
pub struct InvariantRule {
    pub code: &'static str,
    pub name: &'static str,
    pub kind: InvariantKind,
    pub json_pointer: &'static str,
    pub base_range: ScalarRange,
    pub families: &'static [ObjectKind],
}

/// Types of cross-metric interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionType {
    Amplify,
    Suppress,
    Gate,
}

/// A single XMIT_* cross-metric rule.
#[derive(Debug, Clone)]
pub struct InteractionRule {
    pub id: &'static str,
    pub code: &'static str,
    pub metric_a: &'static str,
    pub metric_b: &'static str,
    pub interaction_type: InteractionType,
}

/// Public entry point: enforce all invariants and interactions for a payload.
pub fn validate_invariants(
    spine: &SpineIndex,
    object_kind: ObjectKind,
    tier: Tier,
    payload: &Value,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // 1. Range-level checks (CIC, MDI, AOS, DET, etc.).
    let rules = base_invariant_rules();
    for rule in rules.iter().filter(|r| r.families.contains(&object_kind)) {
        apply_invariant_rule(spine, rule, object_kind, tier, payload, &mut diagnostics);
    }

    // 2. Cross-metric interactions (XMIT_001..XMIT_005).
    let interactions = interaction_rules();
    for ir in &interactions {
        apply_interaction_rule(spine, ir, object_kind, tier, payload, &mut diagnostics);
    }

    diagnostics
}

/// Static table: core invariant and metric range rules.
/// All numeric defaults here are fallbacks; spine overrides are applied first.
fn base_invariant_rules() -> &'static [InvariantRule] {
    use InvariantKind::{Derived, Metric, Structural};

    const ALL_FAMILIES: &[ObjectKind] = &[
        ObjectKind::MoodContract,
        ObjectKind::EventContract,
        ObjectKind::RegionContractCard,
        ObjectKind::SeedContractCard,
    ];

    const MOOD_REGION_SEED: &[ObjectKind] = &[
        ObjectKind::MoodContract,
        ObjectKind::RegionContractCard,
        ObjectKind::SeedContractCard,
    ];

    const MOOD_EVENT: &[ObjectKind] = &[
        ObjectKind::MoodContract,
        ObjectKind::EventContract,
    ];

    const EVENT_ONLY: &[ObjectKind] = &[ObjectKind::EventContract];

    const REGION_ONLY: &[ObjectKind] = &[ObjectKind::RegionContractCard];

    &[
        InvariantRule {
            code: "ERR_CIC_RANGE",
            name: "CIC",
            kind: Structural,
            json_pointer: "/invariantBindings/CIC/value",
            base_range: ScalarRange { min: 0.0, max: 1.0 },
            families: ALL_FAMILIES,
        },
        InvariantRule {
            code: "ERR_MDI_RANGE",
            name: "MDI",
            kind: Structural,
            json_pointer: "/invariantBindings/MDI/value",
            base_range: ScalarRange { min: 0.0, max: 1.0 },
            families: MOOD_REGION_SEED,
        },
        InvariantRule {
            code: "ERR_AOS_RANGE",
            name: "AOS",
            kind: Structural,
            json_pointer: "/invariantBindings/AOS/value",
            base_range: ScalarRange { min: 0.0, max: 1.0 },
            families: MOOD_REGION_SEED,
        },
        InvariantRule {
            code: "ERR_LSG_RANGE",
            name: "LSG",
            kind: Structural,
            json_pointer: "/invariantBindings/LSG/value",
            base_range: ScalarRange { min: 0.0, max: 1.0 },
            families: ALL_FAMILIES,
        },
        InvariantRule {
            code: "ERR_HVF_RANGE",
            name: "HVF",
            kind: Structural,
            json_pointer: "/invariantBindings/HVF/value",
            base_range: ScalarRange { min: 0.0, max: 1.0 },
            families: MOOD_REGION_SEED,
        },
        InvariantRule {
            code: "ERR_DET_RANGE",
            name: "DET",
            kind: Structural,
            json_pointer: "/invariantBindings/DET/value",
            base_range: ScalarRange { min: 0.0, max: 10.0 },
            families: MOOD_EVENT,
        },
        InvariantRule {
            code: "ERR_FCF_RANGE",
            name: "FCF",
            kind: Structural,
            json_pointer: "/invariantBindings/FCF/value",
            base_range: ScalarRange { min: 0.0, max: 10.0 },
            families: MOOD_EVENT,
        },
        InvariantRule {
            code: "ERR_RRM_RANGE",
            name: "RRM",
            kind: Structural,
            json_pointer: "/invariantBindings/RRM/value",
            base_range: ScalarRange { min: 0.0, max: 1.0 },
            families: REGION_ONLY,
        },
        InvariantRule {
            code: "ERR_SPR_DERIVED",
            name: "SPR",
            kind: Derived,
            json_pointer: "/invariantBindings/SPR/value",
            base_range: ScalarRange { min: 0.0, max: 1.0 },
            families: ALL_FAMILIES,
        },
        InvariantRule {
            code: "ERR_SHCI_COUPLING",
            name: "SHCI",
            kind: Derived,
            json_pointer: "/invariantBindings/SHCI/value",
            base_range: ScalarRange { min: 0.0, max: 1.0 },
            families: ALL_FAMILIES,
        },
        InvariantRule {
            code: "ERR_RWF_FLOOR",
            name: "RWF",
            kind: Derived,
            json_pointer: "/invariantBindings/RWF/value",
            base_range: ScalarRange { min: 0.0, max: 1.0 },
            families: EVENT_ONLY,
        },
        InvariantRule {
            code: "ERR_UEC_RANGE",
            name: "UEC",
            kind: Metric,
            json_pointer: "/metricTargets/UEC/target",
            base_range: ScalarRange { min: 0.0, max: 1.0 },
            families: ALL_FAMILIES,
        },
        InvariantRule {
            code: "ERR_EMD_RANGE",
            name: "EMD",
            kind: Metric,
            json_pointer: "/metricTargets/EMD/target",
            base_range: ScalarRange { min: -1.0, max: 1.0 },
            families: MOOD_EVENT,
        },
        InvariantRule {
            code: "ERR_STCI_RANGE",
            name: "STCI",
            kind: Metric,
            json_pointer: "/metricTargets/STCI/target",
            base_range: ScalarRange { min: 0.0, max: 1.0 },
            families: EVENT_ONLY,
        },
        InvariantRule {
            code: "ERR_CDL_RANGE",
            name: "CDL",
            kind: Metric,
            json_pointer: "/metricTargets/CDL/target",
            base_range: ScalarRange { min: 0.0, max: 1.0 },
            families: MOOD_EVENT,
        },
        InvariantRule {
            code: "ERR_ARR_FLOOR",
            name: "ARR",
            kind: Metric,
            json_pointer: "/metricTargets/ARR/target",
            base_range: ScalarRange { min: 0.0, max: 1.0 },
            families: ALL_FAMILIES,
        },
    ]
}

/// Static table: cross-metric interaction rules XMIT_001..XMIT_005.
fn interaction_rules() -> &'static [InteractionRule] {
    &[
        InteractionRule {
            id: "XMIT_001",
            code: "ERR_XMIT_001",
            metric_a: "DET",
            metric_b: "CDL",
            interaction_type: InteractionType::Amplify,
        },
        InteractionRule {
            id: "XMIT_002",
            code: "ERR_XMIT_002",
            metric_a: "CIC",
            metric_b: "SHCI",
            interaction_type: InteractionType::Amplify,
        },
        InteractionRule {
            id: "XMIT_003",
            code: "ERR_XMIT_003",
            metric_a: "DET",
            metric_b: "ARR",
            interaction_type: InteractionType::Suppress,
        },
        InteractionRule {
            id: "XMIT_004",
            code: "ERR_XMIT_004",
            metric_a: "AOS",
            metric_b: "EMD",
            interaction_type: InteractionType::Amplify,
        },
        InteractionRule {
            id: "XMIT_005",
            code: "ERR_XMIT_005",
            metric_a: "LSG",
            metric_b: "HVF",
            interaction_type: InteractionType::Gate,
        },
    ]
}

fn apply_invariant_rule(
    spine: &SpineIndex,
    rule: &InvariantRule,
    object_kind: ObjectKind,
    tier: Tier,
    payload: &Value,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let value = match payload.pointer(rule.json_pointer) {
        Some(Value::Number(n)) => match n.as_f64() {
            Some(v) => v,
            None => return,
        },
        _ => return,
    };

    // Ask spine for the effective range (including tier overrides) if available.
    let effective_range = spine
        .effective_range(rule.name, object_kind, tier)
        .unwrap_or(rule.base_range.clone());

    if value < effective_range.min || value > effective_range.max {
        diagnostics.push(Diagnostic {
            code: rule.code.to_string(),
            layer: ValidationLayer::Invariant,
            severity: Severity::Error,
            json_pointer: rule.json_pointer.to_string(),
            submitted_value: Some(Value::from(value)),
            expected: format!(
                "{} <= {} <= {} for {:?} at {:?}",
                effective_range.min, rule.name, effective_range.max, object_kind, tier
            ),
            remediation: String::from("Adjust value into the allowed range or change tier."),
            fix_order: 1,
        });
    }
}

fn apply_interaction_rule(
    _spine: &SpineIndex,
    rule: &InteractionRule,
    _object_kind: ObjectKind,
    _tier: Tier,
    payload: &Value,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // Simple helper to read invariant/metric values by abbreviation.
    fn read_scalar(payload: &Value, abbrev: &str) -> Option<f64> {
        // First look in invariantBindings.
        let inv_ptr = format!("/invariantBindings/{}/value", abbrev);
        if let Some(Value::Number(n)) = payload.pointer(&inv_ptr) {
            return n.as_f64();
        }
        // Then in metricTargets.
        let met_ptr = format!("/metricTargets/{}/target", abbrev);
        if let Some(Value::Number(n)) = payload.pointer(&met_ptr) {
            return n.as_f64();
        }
        None
    }

    let a = match read_scalar(payload, rule.metric_a) {
        Some(v) => v,
        None => return,
    };
    let b = match read_scalar(payload, rule.metric_b) {
        Some(v) => v,
        None => return,
    };

    match rule.id {
        // XMIT_001: DET > 8.0 => CDL floor 0.4
        "XMIT_001" => {
            if rule.metric_a == "DET" && a > 8.0 && b < 0.4 {
                diagnostics.push(Diagnostic {
                    code: rule.code.to_string(),
                    layer: ValidationLayer::Invariant,
                    severity: Severity::Error,
                    json_pointer: "/metricTargets/CDL/target".to_string(),
                    submitted_value: Some(Value::from(b)),
                    expected: "CDL >= 0.4 when DET > 8.0".to_string(),
                    remediation: "Increase CDL to at least 0.4 or lower DET to 8.0 or below."
                        .to_string(),
                    fix_order: 2,
                });
            }
        }
        // XMIT_002: CIC > 0.7 widens SHCI band; here we only check SHCI is not absurdly out of [0.0, 1.0].
        "XMIT_002" => {
            if rule.metric_a == "CIC" && a > 0.7 && (b < 0.0 || b > 1.0) {
                diagnostics.push(Diagnostic {
                    code: rule.code.to_string(),
                    layer: ValidationLayer::Invariant,
                    severity: Severity::Warning,
                    json_pointer: "/invariantBindings/SHCI/value".to_string(),
                    submitted_value: Some(Value::from(b)),
                    expected: "SHCI in [0.0, 1.0] with widened tolerance when CIC > 0.7"
                        .to_string(),
                    remediation: "Clamp SHCI into [0.0, 1.0] or lower CIC to 0.7 or below."
                        .to_string(),
                    fix_order: 3,
                });
            }
        }
        // XMIT_003: DET > 9.0 may suppress ARR floor; still enforce suppressed minimum.
        "XMIT_003" => {
            if rule.metric_a == "DET" && a > 9.0 {
                let suppressed_floor = 0.2;
                if b < suppressed_floor {
                    diagnostics.push(Diagnostic {
                        code: rule.code.to_string(),
                        layer: ValidationLayer::Invariant,
                        severity: Severity::Warning,
                        json_pointer: "/metricTargets/ARR/target".to_string(),
                        submitted_value: Some(Value::from(b)),
                        expected: format!(
                            "ARR >= {} when DET > 9.0 (suppressed floor)",
                            suppressed_floor
                        ),
                        remediation: "Raise ARR toward the suppressed floor or reduce DET below 9.0."
                            .to_string(),
                        fix_order: 3,
                    });
                }
            }
        }
        // XMIT_004: AOS > 0.6 amplifies EMD targets.
        "XMIT_004" => {
            if rule.metric_a == "AOS" && a > 0.6 && (b < -1.0 || b > 1.0) {
                diagnostics.push(Diagnostic {
                    code: rule.code.to_string(),
                    layer: ValidationLayer::Invariant,
                    severity: Severity::Error,
                    json_pointer: "/metricTargets/EMD/target".to_string(),
                    submitted_value: Some(Value::from(b)),
                    expected: "EMD within amplified band [-1.0, 1.0] when AOS > 0.6".to_string(),
                    remediation: "Clamp EMD into [-1.0, 1.0] and reconsider AOS if necessary."
                        .to_string(),
                    fix_order: 2,
                });
            }
        }
        // XMIT_005: LSG < 0.2 => HVF >= 0.3
        "XMIT_005" => {
            if rule.metric_a == "LSG" && a < 0.2 && b < 0.3 {
                diagnostics.push(Diagnostic {
                    code: rule.code.to_string(),
                    layer: ValidationLayer::Invariant,
                    severity: Severity::Error,
                    json_pointer: "/invariantBindings/HVF/value".to_string(),
                    submitted_value: Some(Value::from(b)),
                    expected: "HVF >= 0.3 when LSG < 0.2".to_string(),
                    remediation:
                        "Increase HVF to at least 0.3 or raise LSG above 0.2 for this region."
                            .to_string(),
                    fix_order: 2,
                });
            }
        }
        _ => {}
    }
}
