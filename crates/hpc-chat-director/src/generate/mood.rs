//! Generates `moodContract` skeletons.
//!
//! Includes DreadForge patterns, tileClass-specific bands, and
//! required hooks lists.

use crate::generate::{AnnotatedSkeleton, FieldHint, SkeletonMetadata};
use crate::model::manifest_types::Tier;
use crate::model::spine_types::SchemaSpine;
use serde_json::json;
use std::collections::HashMap;

/// Generates a mood skeleton for the given tier and optional archetype/tileClass.
pub fn generate_mood_skeleton(
    tier: Tier,
    spine: &SchemaSpine,
    archetype: Option<&str>,
    tile_class: Option<&str>,
) -> AnnotatedSkeleton {
    let tc = tile_class.unwrap_or("spawn");
    let arch = archetype.unwrap_or("sanctuary");

    let (cic_min, cic_max, lsg_min, lsg_max, det_max, arr_min) = match tc {
        "spawn" => (0.3, 0.7, 0.6, 1.0, 3.0, 0.5),
        "battlefront" => (0.5, 1.0, 0.2, 0.6, 9.5, 0.4),
        "liminal" => (0.2, 0.6, 0.0, 0.4, 7.0, 0.3),
        _ => (0.2, 0.8, 0.1, 0.9, 8.0, 0.3), // Default fallback
    };

    let mut content = serde_json::Map::new();

    // Structural fields
    content.insert("schemaRef".into(), json!("schema://Horror.Place/moodcontract_v1.json"));
    content.insert("id".into(), json!("mood.TODO_UUID"));
    content.insert("moodName".into(), json!("TODO_NAME"));
    content.insert("tileClass".into(), json!(tc));
    content.insert("archetype".into(), json!(arch));

    // Invariant bindings (TileClass specific)
    let mut invariants = serde_json::Map::new();
    invariants.insert("CIC".into(), json!({ "min": cic_min, "max": cic_max }));
    invariants.insert("MDI".into(), json!({ "min": 0.1, "max": 0.8 }));
    invariants.insert("AOS".into(), json!({ "min": 0.1, "max": 0.9 }));
    invariants.insert("DET".into(), json!({ "min": 0.0, "max": det_max }));
    invariants.insert("LSG".into(), json!({ "min": lsg_min, "max": lsg_max }));
    invariants.insert("HVF".into(), json!({ "min": 0.2, "max": 0.9 }));
    invariants.insert("SHCI".into(), json!({ "min": 0.1, "max": 0.8 }));
    content.insert("invariants".into(), json!(invariants));

    // Metric targets
    let mut metrics = serde_json::Map::new();
    metrics.insert("UEC".into(), json!({ "targetMin": 0.4, "targetMax": 1.0 }));
    metrics.insert("EMD".into(), json!({ "targetMin": -0.2, "targetMax": 0.5 }));
    metrics.insert("CDL".into(), json!({ "targetMin": 0.0, "targetMax": 0.8 }));
    metrics.insert("ARR".into(), json!({ "targetMin": arr_min, "targetMax": 1.0 }));
    content.insert("metricsIntent".into(), json!(metrics));

    // Required hooks
    content.insert("requiredHooks".into(), json!(vec!["on_entry", "on_intensity_change", "on_exit"]));

    // DreadForge pattern (simplified)
    content.insert("dreadForgePattern".into(), json!({
        "stages": ["outer", "threshold", "locus", "rupture", "fallout"],
        "modulation": {}
    }));

    // Registry readiness
    content.insert("registryReady".into(), json!(false));

    // Build hints
    let mut hints = HashMap::new();
    hints.insert("id".to_string(), FieldHint::AiFill);
    hints.insert("moodName".to_string(), FieldHint::AiFill);
    hints.insert("tileClass".to_string(), FieldHint::DoNotModify);
    hints.insert("archetype".to_string(), FieldHint::AiFill);
    hints.insert("invariants".to_string(), FieldHint::DoNotModify);
    hints.insert("metricsIntent".to_string(), FieldHint::AiFill);
    hints.insert("requiredHooks".to_string(), FieldHint::AiFill);
    hints.insert("dreadForgePattern".to_string(), FieldHint::AiFill);
    hints.insert("registryReady".to_string(), FieldHint::DoNotModify);

    let metadata = SkeletonMetadata {
        object_kind: "moodContract".to_string(),
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
