//! Generates `seedContractCard` skeletons.
//!
//! Seeds declare invariant gradients over space and time, SHCI coupling
//! to historical events, and UEC/ARR trajectory templates.

use crate::generate::{AnnotatedSkeleton, FieldHint, SkeletonMetadata};
use crate::model::manifest_types::Tier;
use crate::model::spine_types::SchemaSpine;
use serde_json::json;
use std::collections::HashMap;

/// Generates a seed skeleton for the given tier.
pub fn generate_seed_skeleton(tier: Tier, spine: &SchemaSpine) -> AnnotatedSkeleton {
    let mut content = serde_json::Map::new();

    // Structural fields
    content.insert("schemaRef".into(), json!("schema://Horror.Place/seedcontract_v1.json"));
    content.insert("id".into(), json!("seed.TODO_UUID"));
    content.insert("bundleref".into(), json!("bundle.TODO_BUNDLE_ID"));
    content.insert("archetype".into(), json!("TODO_ARCHETYPE"));
    content.insert("intensityBand".into(), json!({ "min": 0, "max": 10 }));
    content.insert("safetyTier".into(), json!("TODO_SAFETY"));

    // Invariant bindings
    let mut invariants = serde_json::Map::new();
    invariants.insert("CIC".into(), json!({ "min": 0.4, "max": 0.8 }));
    invariants.insert("MDI".into(), json!({ "min": 0.2, "max": 0.9 }));
    invariants.insert("AOS".into(), json!({ "min": 0.1, "max": 0.6 }));
    invariants.insert("LSG".into(), json!({ "min": 0.3, "max": 0.7 }));
    invariants.insert("HVF".into(), json!({ "min": 0.5, "max": 0.9 }));
    invariants.insert("SHCI".into(), json!({ "min": 0.2, "max": 0.8 }));
    content.insert("invariants".into(), json!(invariants));

    // Metric intent
    let mut metrics = serde_json::Map::new();
    metrics.insert("UEC".into(), json!({ "targetMin": 0.6, "targetMax": 1.0 }));
    metrics.insert("ARR".into(), json!({ "targetMin": 0.5, "targetMax": 0.9 }));
    content.insert("metricsIntent".into(), json!(metrics));

    // Pacing/Telemetry
    content.insert("pacingTemplate".into(), json!("slow_burn")); // Default
    content.insert("telemetryHooks".into(), json!(vec!["session_start", "region_entry", "intensity_spike"]));

    // Registry readiness
    content.insert("registryReady".into(), json!(false));

    // Build hints
    let mut hints = HashMap::new();
    hints.insert("id".to_string(), FieldHint::AiFill);
    hints.insert("bundleref".to_string(), FieldHint::AiFill);
    hints.insert("archetype".to_string(), FieldHint::AiFill);
    hints.insert("intensityBand".to_string(), FieldHint::AiFill);
    hints.insert("safetyTier".to_string(), FieldHint::AiFill);
    hints.insert("invariants".to_string(), FieldHint::DoNotModify);
    hints.insert("metricsIntent".to_string(), FieldHint::AiFill);
    hints.insert("pacingTemplate".to_string(), FieldHint::AiFill);
    hints.insert("telemetryHooks".to_string(), FieldHint::AiFill);
    hints.insert("registryReady".to_string(), FieldHint::DoNotModify);

    let metadata = SkeletonMetadata {
        object_kind: "seedContractCard".to_string(),
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
