// crates/hpc-chat-director/src/validate/invariants.rs

//! Invariant and metric enforcement logic.
//!
//! Translates the invariant and entertainment-metric spines into
//! concrete numeric enforcement rules. Validates ranges, applies
//! cross-metric interactions (XMIT rules), and emits structured
//! diagnostics and explanation records.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::response_types::AiAuthoringResponse;
use crate::model::spine_types::{ObjectKind, SchemaSpine, Tier};
use crate::validate::{Diagnostic, Severity, ValidationLayer};

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

/// Result of validating a single invariant or metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvariantResult {
    /// Metric/invariant name.
    pub metric: String,
    /// Submitted value.
    pub submitted_value: f64,
    /// Valid band after XMIT adjustments.
    pub band: InvariantBand,
    /// Whether the value passed validation.
    pub passed: bool,
    /// Cross-metric interaction effects that affected this metric.
    pub interaction_effects: Vec<InteractionEffect>,
}

/// Numeric band constraint for a metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvariantBand {
    pub metric: String,
    pub min: f64,
    pub max: f64,
    pub error_code: String,
}

/// Cross-metric interaction effect record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionEffect {
    pub rule_id: String,
    pub source_metric: String,
    pub target_metric: String,
    pub effect_type: String,
    pub adjusted_band: InvariantBand,
    pub reason: String,
}

/// Structured explanation for an invariant failure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvariantExplanation {
    pub code: String,
    pub layer: String,
    pub severity: String,
    pub json_pointer: String,
    pub submitted_value: String,
    pub expected: String,
    pub remediation: String,
}

/// Public entry point used by validatemod.rs.
///
/// This is the machine-enforceable horror logic:
/// 1. Derive effective bands from the spine (including tier overrides).
/// 2. Apply cross-metric XMIT rules to adjust those bands.
/// 3. Compare submitted values to post-XMIT bands and emit Diagnostics.
pub fn validate_invariants(
    spine: &SchemaSpine,
    object_kind: ObjectKind,
    tier: Tier,
    resp: &AiAuthoringResponse,
) -> Result<Vec<Diagnostic>, crate::errors::ValidationError> {
    let artifact = &resp.payload;

    // Extract scalar values keyed by invariant / metric abbreviation.
    let values = extract_metric_values(artifact)?;

    // Build default bands from the spine for this objectKind and tier.
    let mut bands = get_default_bands(object_kind, spine, tier)?;

    // Pass 1: collect pre-XMIT out-of-band values (for telemetry/debug only).
    let mut pre_xmit_failures: Vec<InvariantResult> = Vec::new();
    for (metric, value) in &values {
        if let Some(band) = bands.get(metric) {
            if value < &band.min || value > &band.max {
                pre_xmit_failures.push(InvariantResult {
                    metric: metric.clone(),
                    submitted_value: *value,
                    band: band.clone(),
                    passed: false,
                    interaction_effects: Vec::new(),
                });
            }
        }
    }

    // Pass 2: apply spine-defined XMIT interaction rules to mutate bands.
    let mut interaction_effects = Vec::new();
    for rule in &spine.interaction_rules {
        if let Some(effect) = apply_xmit_rule(rule, &values, &mut bands) {
            interaction_effects.push(effect);
        }
    }

    // Pass 3: re-check values against adjusted bands and emit Diagnostics.
    let mut diagnostics = Vec::new();
    for (metric, value) in &values {
        if let Some(band) = bands.get(metric) {
            let passed = value >= &band.min && value <= &band.max;

            // Attach interaction effects for this metric.
            let effects: Vec<InteractionEffect> = interaction_effects
                .iter()
                .filter(|e| e.target_metric == *metric)
                .cloned()
                .collect();

            if !passed {
                let result = InvariantResult {
                    metric: metric.clone(),
                    submitted_value: *value,
                    band: band.clone(),
                    passed,
                    interaction_effects: effects.clone(),
                };
                let explanation = explain_invariant_failure(&result);

                diagnostics.push(Diagnostic {
                    code: band.error_code.clone(),
                    layer: ValidationLayer::Invariant,
                    severity: Severity::Error,
                    json_pointer: explanation.json_pointer,
                    submitted_value: Some(Value::from(result.submitted_value)),
                    expected: explanation.expected,
                    remediation: explanation.remediation,
                    fix_order: 1,
                });
            }
        }
    }

    // Also run the static per-field rules for contracts that still use invariantBindings/metricTargets.
    let static_rules = base_invariant_rules();
    for rule in static_rules
        .iter()
        .filter(|r| r.families.contains(&object_kind))
    {
        apply_static_invariant_rule(spine, rule, object_kind, tier, artifact, &mut diagnostics);
    }

    // Optionally, we could down-grade pre-XMIT-only failures into softDiagnostics via CLI,
    // but here we just return the hard errors.
    Ok(diagnostics)
}

/// Extract invariant/metric values from a contract artifact.
///
/// This expects the v1 shapes:
/// - invariantBindings.{ABBREV}.value for invariants
/// - metricTargets.{ABBREV}.target for metrics
fn extract_metric_values(
    artifact: &Value,
) -> Result<HashMap<String, f64>, crate::errors::ValidationError> {
    let mut values = HashMap::new();

    // invariantBindings block
    if let Some(invariants) = artifact.get("invariantBindings").and_then(|v| v.as_object()) {
        for (abbr, v) in invariants {
            if let Some(obj) = v.as_object() {
                if let Some(num) = obj.get("value").and_then(|x| x.as_f64()) {
                    values.insert(abbr.clone(), num);
                }
            }
        }
    }

    // metricTargets block
    if let Some(metrics) = artifact.get("metricTargets").and_then(|v| v.as_object()) {
        for (abbr, v) in metrics {
            if let Some(obj) = v.as_object() {
                if let Some(num) = obj.get("target").and_then(|x| x.as_f64()) {
                    values.insert(abbr.clone(), num);
                }
            }
        }
    }

    Ok(values)
}

/// Get default bands for metrics from spine for given objectKind and tier.
fn get_default_bands(
    object_kind: ObjectKind,
    spine: &SchemaSpine,
    tier: Tier,
) -> Result<HashMap<String, InvariantBand>, crate::errors::ValidationError> {
    let mut bands = HashMap::new();

    // Safe default bands per objectKind/tier where provided.
    if let Some(defaults) = spine.safe_defaults.get(&object_kind) {
        if let Some(tier_defaults) = defaults.by_tier.get(&tier) {
            // Invariant defaults.
            for (name, range) in &tier_defaults.invariants {
                bands.insert(
                    name.clone(),
                    InvariantBand {
                        metric: name.clone(),
                        min: range.min,
                        max: range.max,
                        error_code: format!("ERR_{}_RANGE", name.to_uppercase()),
                    },
                );
            }
            // Metric defaults.
            for (name, range) in &tier_defaults.metrics {
                bands.insert(
                    name.clone(),
                    InvariantBand {
                        metric: name.clone(),
                        min: range.min,
                        max: range.max,
                        error_code: format!("ERR_{}_RANGE", name.to_uppercase()),
                    },
                );
            }
        }
    }

    // Fallback: canonical invariant ranges with tier overrides.
    for (name, spec) in &spine.invariants {
        if !bands.contains_key(name) {
            let range = spec
                .tier_overrides
                .get(&tier)
                .unwrap_or(&spec.canonical_range);
            bands.insert(
                name.clone(),
                InvariantBand {
                    metric: name.clone(),
                    min: range.min,
                    max: range.max,
                    error_code: format!("ERR_{}_RANGE", name.to_uppercase()),
                },
            );
        }
    }

    // Fallback: canonical metric target bands.
    for (name, spec) in &spine.metrics {
        if !bands.contains_key(name) {
            bands.insert(
                name.clone(),
                InvariantBand {
                    metric: name.clone(),
                    min: spec.target_band.min,
                    max: spec.target_band.max,
                    error_code: format!("ERR_{}_RANGE", name.to_uppercase()),
                },
            );
        }
    }

    Ok(bands)
}

/// Static table: core invariant and metric range rules for v1 artifacts
/// that still use invariantBindings/metricTargets paths directly.
///
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

    const MOOD_EVENT: &[ObjectKind] = &[ObjectKind::MoodContract, ObjectKind::EventContract];

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

fn apply_static_invariant_rule(
    spine: &SchemaSpine,
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
        .unwrap_or(ScalarRange {
            min: rule.base_range.min,
            max: rule.base_range.max,
        });

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
            remediation: "Adjust value into the allowed range or change tier.".to_string(),
            fix_order: 1,
        });
    }
}

/// Apply a single spine-defined XMIT interaction rule; returns an InteractionEffect if triggered.
///
/// The concrete semantics (thresholds, effect type, target metric) come from the spine.
fn apply_xmit_rule(
    rule: &crate::model::spine_types::InteractionRule,
    values: &HashMap<String, f64>,
    bands: &mut HashMap<String, InvariantBand>,
) -> Option<InteractionEffect> {
    let source_value = values.get(&rule.source_metric)?;

    // Threshold checks.
    if let Some(threshold) = rule.condition.source_threshold {
        if *source_value <= threshold {
            return None;
        }
    }
    
