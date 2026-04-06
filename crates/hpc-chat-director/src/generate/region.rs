//! Generates `regionContractCard` skeletons.
//!
//! Region skeletons are the PCG substrate for events and seeds. They
//! define topology, spatial gradients of invariants, and SHCI coupling
//! to the region's history.

use crate::generate::{AnnotatedSkeleton, FieldHint, SkeletonMetadata};
use crate::model::manifest_types::Tier;
use crate::model::spine_types::SchemaSpine;
use serde_json::json;
use std::collections::HashMap;

/// Generates a region skeleton for the given tier.
///
/// Includes topology-aware defaults for LSG/HVF based on archetype.
pub fn generate_region_skeleton(tier: Tier, spine: &SchemaSpine) -> AnnotatedSkeleton {
    // Default to "marsh" archetype if not specified; real CLI would pass this
    let archetype = "marsh";
    let cic_range = (0.3, 0.7);
    let lsg_range = (0.2, 0.5);
    let hvf_range = (0.4, 0.7);

    let mut content = serde_json::Map::new();

    // Structural fields
    content.insert("schemaRef".into(), json!("schema://Horror.Place/regioncontract_v1.json"));
    content.insert("id".into(), json!("region.TODO_UUID"));
    content.insert("regionName".into(), json!("TODO_NAME"));
    content.insert("topology".into(), json!("TODO_TOPOLOGY"));
    content.insert("tileClass".into(), json!("TODO_TILE"));

    // Invariant bindings
    let mut invariants = serde_json::Map::new();
    invariants.insert("CIC".into(), json!({ "min": cic_range.0, "max": cic_range.1 }));
    invariants.insert("MDI".into(), json!({ "min": 0.0, "max": 1.0 }));
    invariants.insert("AOS".into(), json!({ "min": 0.0, "max": 1.0 }));
    invariants.insert("RRM".into(), json!({ "min": 0.0, "max": 1.0 }));
    invariants.insert("FCF".into(), json!({ "min": 0.0, "max": 10.0 }));
    invariants.insert("LSG".into(), json!({ "min": lsg_range.0, "max": lsg_range.1 }));
    invariants.insert("HVF".into(), json!({ "min": hvf_range.0, "max": hvf_range.1 }));
    invariants.insert("SHCI".into(), json!({ "min": 0.0, "max": 1.0 }));
    content.insert("invariants".into(), json!(invariants));

    // Metric targets
    let mut metrics = serde_json::Map::new();
    metrics.insert("UEC".into(), json!({ "targetMin": 0.0, "targetMax": 1.0 }));
    metrics.insert("ARR".into(), json!({ "targetMin": 0.5, "targetMax": 1.0 }));
    content.insert("metricTargets".into(), json!(metrics));

    // Spatial gradients
    content.insert("spatialGradients".into(), json!({
        "CIC": { "decay": 0.05, "center": 0.6 },
        "LSG": { "decay": 0.1, "center": 0.3 }
    }));

    // Registry readiness
    content.insert("registryReady".into(), json!(false));

    // Build hints
    let mut hints = HashMap::new();
    hints.insert("id".to_string(), FieldHint::AiFill);
    hints.insert("regionName".to_string(), FieldHint::AiFill);
    hints.insert("topology".to_string(), FieldHint::AiFill);
    hints.insert("tileClass".to_string(), FieldHint::AiFill);
    hints.insert("invariants".to_string(), FieldHint::DoNotModify);
    hints.insert("metricTargets".to_string(), FieldHint::AiFill);
    hints.insert("spatialGradients".to_string(), FieldHint::AiFill);
    hints.insert("registryReady".to_string(), FieldHint::DoNotModify);

    let metadata = SkeletonMetadata {
        object_kind: "regionContractCard".to_string(),
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
