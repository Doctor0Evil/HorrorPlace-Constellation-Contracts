// crates/hpc-chat-director/src/validate/invariants.rs

//! Invariant and metric enforcement logic.
//!
//! Translates the invariant and entertainment-metric spines into
//! concrete numeric enforcement rules. Validates ranges, applies
//! cross-metric interactions (XMIT rules), and emits structured
//! diagnostics and explanation records.

use std::collections::HashMap;

use serde_json::Value;

use crate::errors::ChatDirectorError;
use crate::model::response_types::AiAuthoringResponse;
use crate::spine::{InteractionRule, InvariantSpec, SafeBand, SpineIndex};

#[derive(Debug, Clone)]
pub struct InteractionEffect {
    pub rule_id: String,
    pub source_metric: String,
    pub target_metric: String,
    pub adjusted_min: f64,
    pub adjusted_max: f64,
}

/// Invariant-level diagnostic with cross-metric effects and fix ordering.
#[derive(Debug, Clone)]
pub struct InvariantDiagnostic {
    pub error_code: String,
    pub field_path: String,
    pub submitted_value: f64,
    pub expected_min: f64,
    pub expected_max: f64,
    pub remediation: String,
    pub fix_order: u8,
    pub interaction_effects: Vec<InteractionEffect>,
}

/// All invariant/metric diagnostics for a single artifact.
#[derive(Debug, Default)]
pub struct InvariantResult {
    pub diagnostics: Vec<InvariantDiagnostic>,
}

/// Entry point: enforce invariants and entertainment metrics for a response.
pub fn validate_invariants(
    spine: &SpineIndex,
    response: &AiAuthoringResponse,
) -> Result<InvariantResult, ChatDirectorError> {
    let mut diagnostics = Vec::new();

    // Pass 1: raw band checks from spine defaults.
    let raw_bands = build_raw_bands(spine, response);
    let mut post_xmit_bands = raw_bands.clone();

    // Pass 2: apply XMIT interaction rules to adjust bands.
    let effects = apply_all_xmit_rules(spine, &raw_bands, &mut post_xmit_bands);

    // Pass 3: validate submitted values against post-XMIT bands.
    let content = &response.content;
    let values = extract_invariant_values(content);

    for (code, value) in values {
        if let Some(band) = post_xmit_bands.get(&code) {
            if value < band.min || value > band.max {
                let error_code = format!("ERR{}_RANGE", code);
                let remediation = format!(
                    "Adjust {} from {:.3} to within [{:.3}, {:.3}].",
                    code, value, band.min, band.max
                );

                let rule_effects: Vec<InteractionEffect> = effects
                    .iter()
                    .filter(|e| e.target_metric == code)
                    .cloned()
                    .collect();

                diagnostics.push(InvariantDiagnostic {
                    error_code,
                    field_path: format!("/invariants/{}", code),
                    submitted_value: value,
                    expected_min: band.min,
                    expected_max: band.max,
                    remediation,
                    fix_order: 2,
                    interaction_effects: rule_effects,
                });
            }
        }
    }

    Ok(InvariantResult { diagnostics })
}

fn build_raw_bands(
    spine: &SpineIndex,
    response: &AiAuthoringResponse,
) -> HashMap<String, SafeBand> {
    let mut bands = HashMap::new();

    // Use safe defaults for the given objectKind and tier.
    if let Some(defaults) = spine.safe_defaults(&response.objectKind, &response.tier) {
        if let Some(band) = defaults.cic {
            bands.insert("CIC".to_string(), band);
        }
        if let Some(band) = defaults.aos {
            bands.insert("AOS".to_string(), band);
        }
        if let Some(band) = defaults.det {
            bands.insert("DET".to_string(), band);
        }
        if let Some(band) = defaults.uec {
            bands.insert("UEC".to_string(), band);
        }
        if let Some(band) = defaults.arr {
            bands.insert("ARR".to_string(), band);
        }
        if let Some(band) = defaults.shci {
            bands.insert("SHCI".to_string(), band);
        }
    }

    // Fall back to global ranges if safeDefaults missing.
    for inv in spine.invariant_specs() {
        if !bands.contains_key(&inv.code) {
            let band = SafeBand {
                min: inv.range.min,
                max: inv.range.max,
            };
            bands.insert(inv.code.clone(), band);
        }
    }

    bands
}

fn apply_all_xmit_rules(
    spine: &SpineIndex,
    raw_bands: &HashMap<String, SafeBand>,
    post_xmit_bands: &mut HashMap<String, SafeBand>,
) -> Vec<InteractionEffect> {
    let mut effects = Vec::new();

    for rule in spine.interaction_rules() {
        if let Some(effect) = apply_single_xmit_rule(rule, raw_bands, post_xmit_bands) {
            effects.push(effect);
        }
    }

    effects
}

fn apply_single_xmit_rule(
    rule: &InteractionRule,
    raw_bands: &HashMap<String, SafeBand>,
    post_xmit_bands: &mut HashMap<String, SafeBand>,
) -> Option<InteractionEffect> {
    let source = raw_bands.get(&rule.sourceMetric)?;
    let target = post_xmit_bands.get(&rule.targetMetric)?;

    // For v1, treat interaction as clamping target band to intersection with source band.
    let adjusted_min = source.min.max(target.min);
    let adjusted_max = source.max.min(target.max);

    let effect = InteractionEffect {
        rule_id: rule.ruleId.clone(),
        source_metric: rule.sourceMetric.clone(),
        target_metric: rule.targetMetric.clone(),
        adjusted_min,
        adjusted_max,
    };

    post_xmit_bands.insert(
        rule.targetMetric.clone(),
        SafeBand {
            min: adjusted_min,
            max: adjusted_max,
        },
    );

    Some(effect)
}

fn extract_invariant_values(content: &Value) -> Vec<(String, f64)> {
    let mut values = Vec::new();

    if let Some(obj) = content.get("invariants").and_then(|v| v.as_object()) {
        for (key, v) in obj {
            if let Some(num) = v.as_f64() {
                values.push((key.clone(), num));
            }
        }
    }

    values
}
