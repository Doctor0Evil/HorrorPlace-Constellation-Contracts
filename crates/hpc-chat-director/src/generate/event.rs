//! Generates `eventContract` skeletons.
//!
//! Events are bound to CIC/LSG/DET, SHCI, and RWF, with stage-modulated
//! metrics and preconditions/telemetry hooks.

use crate::generate::{AnnotatedSkeleton, FieldHint, SkeletonMetadata};
use crate::model::manifest_types::Tier;
use crate::model::spine_types::SchemaSpine;
use serde_json::json;
use std::collections::HashMap;

/// Generates an event skeleton for the given tier.
pub fn generate_event_skeleton(tier: Tier, spine: &SchemaSpine) -> AnnotatedSkeleton {
    let mut content = serde_json::Map::new();

    // Structural fields
    content.insert("schemaRef".into(), json!("schema://Horror.Place/eventcontract_v1.json"));
    content.insert("id".into(), json!("event.TODO_UUID"));
    content.insert("eventName".into(), json!("TODO_NAME"));
    content.insert("archetype".into(), json!("TODO_ARCHETYPE"));
    content.insert("preconditions".into(), json!(vec![])); // AI to fill
    content.insert("telemetryHooks".into(), json!(vec!["trigger", "peak", "resolve"]));
    content.insert("intensityBand".into(), json!({ "min": 0, "max": 10 }));
    content.insert("safetyTier".into(), json!("TODO_SAFETY"));

    // Invariant bindings
    let mut invariants = serde_json::Map::new();
    invariants.insert("CIC".into(), json!({ "min": 0.4, "max": 1.0 }));
    invariants.insert("LSG".into(), json!({ "min": 0.1, "max": 0.6 }));
    invariants.insert("DET".into(), json!({ "min": 0.0, "max": 9.0 }));
    invariants.insert("SHCI".into(), json!({ "min": 0.2, "max": 0.9 }));
    invariants.insert("RWF".into(), json!({ "min": 0.5, "max": 1.0 }));
    content.insert("invariants".into(), json!(invariants));

    // Stage modulation grid
    let stages = vec![
        ("outer", json!({"uecDelta": 0.0, "cicMod": 1.0, "detMod": 1.0})),
        ("threshold", json!({"uecDelta": 0.1, "cicMod": 1.0, "detMod": 1.1})),
        ("locus", json!({"uecDelta": 0.3, "cicMod": 1.1, "detMod": 1.3})),
        ("rupture", json!({"uecDelta": 0.5, "cicMod": 1.0, "detMod": 1.5})),
        ("fallout", json!({"uecDelta": -0.2, "cicMod": 0.9, "detMod": 0.7})),
    ];
    let stage_grid: serde_json::Map<String, serde_json::Value> = stages
        .into_iter()
        .map(|(name, mod_data)| (name.to_string(), mod_data))
        .collect();
    content.insert("stageModulation".into(), json!(stage_grid));

    // Metric intent
    let mut metrics = serde_json::Map::new();
    metrics.insert("UEC".into(), json!({ "targetMin": 0.5, "targetMax": 1.0 }));
    metrics.insert("EMD".into(), json!({ "targetMin": 0.1, "targetMax": 0.8 }));
    metrics.insert("CDL".into(), json!({ "targetMin": 0.2, "targetMax": 0.9 }));
    metrics.insert("ARR".into(), json!({ "targetMin": 0.4, "targetMax": 1.0 }));
    content.insert("metricsIntent".into(), json!(metrics));

    // Registry readiness
    content.insert("registryReady".into(), json!(false));

    // Build hints
    let mut hints = HashMap::new();
    hints.insert("id".to_string(), FieldHint::AiFill);
    hints.insert("eventName".to_string(), FieldHint::AiFill);
    hints.insert("archetype".to_string(), FieldHint::AiFill);
    hints.insert("preconditions".to_string(), FieldHint::AiFill);
    hints.insert("intensityBand".to_string(), FieldHint::AiFill);
    hints.insert("safetyTier".to_string(), FieldHint::AiFill);
    hints.insert("invariants".to_string(), FieldHint::DoNotModify);
    hints.insert("stageModulation".to_string(), FieldHint::AiFill);
    hints.insert("metricsIntent".to_string(), FieldHint::AiFill);
    hints.insert("registryReady".to_string(), FieldHint::DoNotModify);

    let metadata = SkeletonMetadata {
        object_kind: "eventContract".to_string(),
        tier,
        schema_version: "v1".to_string(),
        pre_filled_invariants: invariants.into_iter().map(|(k, v)| (k, v["min"].as_f64().unwrap_or(0.0).into())).collect(),
        pre_filled_metrics: metrics.into_iter().map(|(k, v)| (k, v["targetMin"].as_f64().unwrap_or(0.0).into())).collect(),
    };

    AnnotatedSkeleton {
        content: serde_json::Value::Object(content),
        field_hints: hints,
        meta: metadata,
    }
}
