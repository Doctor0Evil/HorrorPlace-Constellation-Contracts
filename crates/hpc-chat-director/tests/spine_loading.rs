//! Tests for spine loading and invariant/metric queries.
//!
//! Ensures the schema spine loads correctly and derived metrics
//! compute as expected from base invariants.

use std::path::Path;
use hpc_chat_director::config::Config;
use hpc_chat_director::spine;
use hpc_chat_director::model::spine_types::{SchemaSpine, Tier};

/// Test that a minimal spine loads successfully.
#[test]
fn test_spine_loads_minimal() {
    // Create a temporary directory with a minimal spine
    let temp_dir = tempfile::tempdir().unwrap();
    let spine_path = temp_dir.path().join("schemas").join("core").join("schema-spine-index-v1.json");
    
    // Ensure parent directories exist
    std::fs::create_dir_all(spine_path.parent().unwrap()).unwrap();
    
    // Write minimal valid spine JSON
    let minimal_spine = r#"{
        "version": "v1",
        "$id": "schema://HorrorPlace-Constellation-Contracts/schema-spine-index-v1.json",
        "title": "Test Spine",
        "description": "Minimal spine for testing",
        "invariants": {
            "CIC": {
                "name": "CIC",
                "canonicalRange": { "min": 0.0, "max": 1.0 },
                "tierOverrides": {},
                "driftMode": "static",
                "compatibleWith": [],
                "description": "Contextual Integrity Coefficient",
                "requiredBy": []
            }
        },
        "metrics": {},
        "contractFamilies": [],
        "interactionRules": [],
        "safeDefaults": {}
    }"#;
    
    std::fs::write(&spine_path, minimal_spine).unwrap();
    
    // Load config and spine
    let config = Config::detect(temp_dir.path()).unwrap();
    let result = spine::load(&config);
    
    assert!(result.is_ok(), "Failed to load minimal spine: {:?}", result.err());
    
    let spine = result.unwrap();
    assert_eq!(spine.version, "v1");
    assert!(spine.invariants.contains_key("CIC"));
}

/// Test that invariant ranges are queried correctly.
#[test]
fn test_invariant_ranges() {
    let spine = load_test_spine();
    
    // Get CIC spec
    let cic = spine.invariants.get("CIC").unwrap();
    
    // Check canonical range
    assert_eq!(cic.canonical_range.min, 0.0);
    assert_eq!(cic.canonical_range.max, 1.0);
    
    // Check tier overrides (if any)
    // In test spine, no overrides, so should use canonical
}

/// Test that metric targets are queried correctly.
#[test]
fn test_metric_targets() {
    let spine = load_test_spine();
    
    // Get UEC spec if present
    if let Some(uec) = spine.metrics.get("UEC") {
        assert_eq!(uec.target_band.min, 0.0);
        assert_eq!(uec.target_band.max, 1.0);
    }
}

/// Test that contract families are loaded correctly.
#[test]
fn test_contract_families() {
    let spine = load_test_spine();
    
    // Check that mood contract family exists
    let mood_family = spine
        .contract_families
        .iter()
        .find(|f| f.name == "mood")
        .expect("mood contract family not found");
    
    assert!(mood_family.kinds.contains(&"moodContract".to_string()));
    assert!(mood_family.required_invariants.contains(&"CIC".to_string()));
}

/// Test derived metric computation (SPR, SHCI).
#[test]
fn test_derived_metrics() {
    let spine = load_test_spine();
    
    // Create a test invariant snapshot
    let mut invariants = std::collections::HashMap::new();
    invariants.insert("CIC".to_string(), 0.8);
    invariants.insert("AOS".to_string(), 0.6);
    invariants.insert("MDI".to_string(), 0.5);
    
    // Compute derived metrics
    let derived = spine.compute_derived(&invariants);
    
    // SPR and SHCI should be in valid range [0.0, 1.0]
    assert!((0.0..=1.0).contains(&derived.spr));
    assert!((0.0..=1.0).contains(&derived.shci));
    
    // With high CIC, SHCI should be relatively high
    assert!(derived.shci >= 0.5);
}

/// Test safe defaults query.
#[test]
fn test_safe_defaults() {
    let spine = load_test_spine();
    
    // Get safe defaults for moodContract at Tier 1
    let defaults = spine.safe_defaults("moodContract", Tier::T1);
    
    assert!(defaults.is_some());
    let defaults = defaults.unwrap();
    
    // Should have CIC default in safe range
    assert!(defaults.invariants.contains_key("CIC"));
}

/// Test objectKind profile query.
#[test]
fn test_describe_object_kind() {
    let spine = load_test_spine();
    
    let profile = spine.describe_object_kind("moodContract");
    
    assert!(profile.is_some());
    let profile = profile.unwrap();
    
    assert_eq!(profile.kind, "moodContract");
    assert!(!profile.required_invariants.is_empty());
}

/// Load a test spine with realistic data.
fn load_test_spine() -> SchemaSpine {
    // Use fixture data or generate minimal valid spine
    let temp_dir = tempfile::tempdir().unwrap();
    setup_test_spine_files(temp_dir.path());
    
    let config = Config::detect(temp_dir.path()).unwrap();
    spine::load(&config).expect("Failed to load test spine")
}

/// Set up test spine files in a temporary directory.
fn setup_test_spine_files(root: &Path) {
    let spine_dir = root.join("schemas").join("core");
    std::fs::create_dir_all(&spine_dir).unwrap();
    
    // Write schema-spine-index-v1.json
    let spine_index = r#"{
        "version": "v1",
        "$id": "schema://HorrorPlace-Constellation-Contracts/schema-spine-index-v1.json",
        "title": "Test Schema Spine",
        "description": "Test spine for unit tests",
        "invariants": {
            "CIC": {
                "name": "CIC",
                "canonicalRange": { "min": 0.0, "max": 1.0 },
                "tierOverrides": {},
                "driftMode": "static",
                "compatibleWith": ["ARR", "SHCI"],
                "description": "Contextual Integrity Coefficient",
                "requiredBy": ["mood", "event", "region", "seed"]
            },
            "AOS": {
                "name": "AOS",
                "canonicalRange": { "min": 0.0, "max": 1.0 },
                "tierOverrides": {},
                "driftMode": "slowly_varying",
                "compatibleWith": ["EMD"],
                "description": "Ambient Oscillation Score",
                "requiredBy": ["mood", "event", "region", "seed"]
            },
            "MDI": {
                "name": "MDI",
                "canonicalRange": { "min": 0.0, "max": 1.0 },
                "tierOverrides": {},
                "driftMode": "static",
                "compatibleWith": [],
                "description": "Mood Drift Index",
                "requiredBy": ["mood", "region", "seed"]
            },
            "DET": {
                "name": "DET",
                "canonicalRange": { "min": 0.0, "max": 10.0 },
                "tierOverrides": {
                    "T1": { "min": 0.0, "max": 7.0 }
                },
                "driftMode": "static",
                "compatibleWith": ["CDL", "ARR"],
                "description": "Dread Entropy Threshold",
                "requiredBy": ["mood", "event"]
            }
        },
        "metrics": {
            "UEC": {
                "name": "UEC",
                "targetBand": { "min": 0.0, "max": 1.0 },
                "tierAdjustments": {},
                "telemetryHook": null,
                "description": "Unnamed Entertainment Coefficient",
                "requiredBy": ["mood", "event", "region", "seed"]
            },
            "ARR": {
                "name": "ARR",
                "targetBand": { "min": 0.3, "max": 1.0 },
                "tierAdjustments": {},
                "telemetryHook": "session_end",
                "description": "Audience Retention Rating",
                "requiredBy": ["mood", "event", "region", "seed"]
            }
        },
        "contractFamilies": [
            {
                "name": "mood",
                "kinds": ["moodContract"],
                "requiredInvariants": ["CIC", "MDI", "AOS", "DET", "LSG", "HVF", "SHCI"],
                "optionalInvariants": [],
                "requiredMetrics": ["UEC", "EMD", "CDL", "ARR"],
                "optionalMetrics": ["STCI"],
                "allowedPhases": [1, 2, 3],
                "tierRestrictions": {}
            },
            {
                "name": "event",
                "kinds": ["eventContract"],
                "requiredInvariants": ["CIC", "LSG", "DET", "SHCI", "RWF"],
                "optionalInvariants": [],
                "requiredMetrics": ["UEC", "EMD", "STCI", "CDL", "ARR"],
                "optionalMetrics": [],
                "allowedPhases": [1, 2, 3],
                "tierRestrictions": {}
            },
            {
                "name": "region",
                "kinds": ["regionContractCard"],
                "requiredInvariants": ["CIC", "MDI", "AOS", "SHCI", "LSG", "HVF"],
                "optionalInvariants": [],
                "requiredMetrics": ["UEC", "ARR"],
                "optionalMetrics": [],
                "allowedPhases": [1, 2],
                "tierRestrictions": {}
            },
            {
                "name": "seed",
                "kinds": ["seedContractCard"],
                "requiredInvariants": ["CIC", "MDI", "AOS", "LSG", "HVF", "SHCI"],
                "optionalInvariants": [],
                "requiredMetrics": ["UEC", "ARR"],
                "optionalMetrics": [],
                "allowedPhases": [1, 2],
                "tierRestrictions": {}
            }
        ],
        "interactionRules": [
            {
                "id": "XMIT_001",
                "sourceMetric": "DET",
                "targetMetric": "CDL",
                "effectType": "amplify",
                "condition": { "sourceThreshold": 8.0 },
                "description": "High DET raises CDL floor"
            }
        ],
        "safeDefaults": {
            "moodContract": {
                "byTier": {
                    "T1": {
                        "invariants": {
                            "CIC": { "min": 0.3, "max": 0.7 },
                            "DET": { "min": 0.0, "max": 7.0 }
                        },
                        "metrics": {
                            "ARR": { "min": 0.5, "max": 1.0 }
                        }
                    }
                }
            }
        }
    }"#;
    
    std::fs::write(spine_dir.join("schema-spine-index-v1.json"), spine_index).unwrap();
    
    // Write invariants-spine.v1.json (could be separate or merged)
    // For this test, we use the merged format above
}
