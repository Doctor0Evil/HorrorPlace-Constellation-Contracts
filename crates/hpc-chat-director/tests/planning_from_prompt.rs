//! Tests for prompt normalization and AiAuthoringRequest generation.
//!
//! These tests verify that `plan_from_prompt` correctly:
//! - Extracts objectKind from natural language
//! - Resolves target repo and path from manifests
//! - Injects spine-derived defaults
//! - Handles ambiguous intents with candidateKinds

use std::path::Path;
use tempfile::TempDir;
use hpc_chat_director::{ChatDirector, config::Config};
use hpc_chat_director::model::spine_types::{Phase, Tier};

/// Test that a clear prompt produces a single objectKind.
#[test]
fn test_clear_prompt_single_object_kind() {
    let (_temp_dir, director) = setup_test_environment();

    let result = director.plan_from_prompt(
        "Create a mood contract for a liminal backrooms region",
        None,
    );

    assert!(result.is_ok(), "Clear prompt should produce valid request: {:?}", result.err());
    let req = result.unwrap();
    
    assert_eq!(req.object_kind, "moodContract");
    assert!(req.candidate_kinds.is_empty(), "Unambiguous prompt should have no candidates");
    assert_eq!(req.phase, Phase::Bundles2); // Default for v1
}

/// Test that ambiguous prompts produce candidateKinds.
#[test]
fn test_ambiguous_prompt_produces_candidates() {
    let (_temp_dir, director) = setup_test_environment();

    let result = director.plan_from_prompt(
        "Create a horror atmosphere for a new region",
        None,
    );

    assert!(result.is_ok(), "Ambiguous prompt should still produce a request");
    let req = result.unwrap();
    
    // Should have at least moodContract and regionContractCard as candidates
    assert!(
        req.candidate_kinds.contains(&"moodContract".to_string()) ||
        req.candidate_kinds.contains(&"regionContractCard".to_string()),
        "Ambiguous prompt should suggest candidate objectKinds"
    );
}

/// Test that target repo is resolved from manifests.
#[test]
fn test_target_repo_resolution() {
    let (_temp_dir, director) = setup_test_environment();

    let result = director.plan_from_prompt(
        "Create a seed contract for experimental PCG",
        None,
    );

    assert!(result.is_ok());
    let req = result.unwrap();
    
    // Should route to a repo that accepts seedContractCard
    assert!(
        req.target_repo == "Horror.Place" || 
        req.target_repo.contains("Atrocity") ||
        req.target_repo.contains("Seeds"),
        "Seed contract should route to appropriate repo, got: {}",
        req.target_repo
    );
}

/// Test that spine-derived defaults are injected.
#[test]
fn test_spine_defaults_injection() {
    let (_temp_dir, director) = setup_test_environment();

    let result = director.plan_from_prompt(
        "Create a Tier 1 mood contract with safe defaults",
        None,
    );

    assert!(result.is_ok());
    let req = result.unwrap();
    
    // Should have some intended metrics from spine safe_defaults
    assert!(
        !req.intended_invariants.is_empty() || !req.intended_metrics.is_empty(),
        "Spine defaults should be injected into request"
    );
    
    // Tier should match request
    assert_eq!(req.tier, Tier::T1);
}

/// Test that explicit target_repo override is respected.
#[test]
fn test_target_repo_override() {
    let (_temp_dir, director) = setup_test_environment();

    let result = director.plan_from_prompt(
        "Create an event contract",
        Some("HorrorPlace-Atrocity-Seeds"),
    );

    assert!(result.is_ok());
    let req = result.unwrap();
    
    assert_eq!(req.target_repo, "HorrorPlace-Atrocity-Seeds");
}

/// Test that phase is inferred from objectKind and tier.
#[test]
fn test_phase_inference() {
    let (_temp_dir, director) = setup_test_environment();

    let result = director.plan_from_prompt(
        "Create a region contract for Tier 2 vault",
        None,
    );

    assert!(result.is_ok());
    let req = result.unwrap();
    
    // For v1, all four contract families default to Phase 2 (Bundles)
    assert_eq!(req.phase, Phase::Bundles2);
}

/// Test that referencedIds are preserved from prompt context.
#[test]
fn test_referenced_ids_preservation() {
    let (_temp_dir, director) = setup_test_environment();

    // Prompt mentioning a specific region ID
    let result = director.plan_from_prompt(
        "Create a mood for region.backrooms_v1",
        None,
    );

    assert!(result.is_ok());
    let req = result.unwrap();
    
    // Should extract region ID as referencedIds
    assert!(
        req.referenced_ids.iter().any(|id| id.contains("backrooms")),
        "Referenced region ID should be extracted: {:?}",
        req.referenced_ids
    );
}

/// Test that invalid objectKind produces structured error.
#[test]
fn test_unknown_object_kind_error() {
    let (_temp_dir, director) = setup_test_environment();

    let result = director.plan_from_prompt(
        "Create a fictionalContract that doesn't exist",
        None,
    );

    assert!(result.is_err(), "Unknown objectKind should produce error");
    
    let err = result.unwrap_err();
    assert!(
        err.message.contains("objectKind") || err.message.contains("not found"),
        "Error should mention unknown objectKind"
    );
}

/// Set up minimal test environment for planning tests.
fn setup_test_environment() -> (TempDir, ChatDirector) {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    create_test_spine(root);
    create_test_manifests(root);

    let config = Config::detect(root).unwrap();
    let director = ChatDirector::load_environment(root).unwrap();
    
    (temp_dir, director)
}

/// Create minimal schema spine for testing.
fn create_test_spine(root: &Path) {
    let spine_dir = root.join("schemas").join("core");
    std::fs::create_dir_all(&spine_dir).unwrap();
    
    let spine_content = r#"{
        "version": "v1",
        "$id": "schema://HorrorPlace-Constellation-Contracts/schema-spine-index-v1.json",
        "title": "Test Spine",
        "description": "Minimal spine for planning tests",
        "invariants": {
            "CIC": {
                "name": "CIC",
                "canonicalRange": {"min": 0.0, "max": 1.0},
                "tierOverrides": {"T1": {"min": 0.3, "max": 0.7}},
                "driftMode": "static",
                "compatibleWith": ["ARR"],
                "description": "Contextual Integrity Coefficient",
                "requiredBy": ["mood", "event", "region", "seed"]
            },
            "DET": {
                "name": "DET",
                "canonicalRange": {"min": 0.0, "max": 10.0},
                "tierOverrides": {"T1": {"min": 0.0, "max": 7.0}},
                "driftMode": "static",
                "compatibleWith": ["CDL"],
                "description": "Dread Entropy Threshold",
                "requiredBy": ["mood", "event"]
            }
        },
        "metrics": {
            "ARR": {
                "name": "ARR",
                "targetBand": {"min": 0.5, "max": 1.0},
                "tierAdjustments": {},
                "telemetryHook": null,
                "description": "Audience Retention Rating",
                "requiredBy": ["mood", "event", "region", "seed"]
            }
        },
        "contractFamilies": [
            {
                "name": "mood",
                "kinds": ["moodContract"],
                "requiredInvariants": ["CIC", "DET"],
                "optionalInvariants": [],
                "requiredMetrics": ["ARR"],
                "optionalMetrics": [],
                "allowedPhases": [1, 2, 3],
                "tierRestrictions": {}
            },
            {
                "name": "event",
                "kinds": ["eventContract"],
                "requiredInvariants": ["CIC", "DET"],
                "optionalInvariants": [],
                "requiredMetrics": ["ARR"],
                "optionalMetrics": [],
                "allowedPhases": [1, 2, 3],
                "tierRestrictions": {}
            },
            {
                "name": "region",
                "kinds": ["regionContractCard"],
                "requiredInvariants": ["CIC"],
                "optionalInvariants": [],
                "requiredMetrics": ["ARR"],
                "optionalMetrics": [],
                "allowedPhases": [1, 2],
                "tierRestrictions": {}
            },
            {
                "name": "seed",
                "kinds": ["seedContractCard"],
                "requiredInvariants": ["CIC"],
                "optionalInvariants": [],
                "requiredMetrics": ["ARR"],
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
                "condition": {"sourceThreshold": 8.0},
                "description": "High DET raises CDL floor"
            }
        ],
        "safeDefaults": {
            "moodContract": {
                "byTier": {
                    "T1": {
                        "invariants": {
                            "CIC": {"min": 0.3, "max": 0.7},
                            "DET": {"min": 0.0, "max": 7.0}
                        },
                        "metrics": {
                            "ARR": {"min": 0.5, "max": 1.0}
                        }
                    }
                }
            }
        }
    }"#;
    
    std::fs::write(
        spine_dir.join("schema-spine-index-v1.json"),
        spine_content
    ).unwrap();
}

/// Create minimal repo manifests for testing.
fn create_test_manifests(root: &Path) {
    let manifests_dir = root.join("manifests");
    std::fs::create_dir_all(&manifests_dir).unwrap();
    
    // Horror.Place manifest
    let horror_place = r#"{
        "repo": "Horror.Place",
        "tier": "T1",
        "allowedObjectKinds": ["moodContract", "eventContract", "regionContractCard", "seedContractCard"],
        "allowedSchemas": ["schema://Horror.Place/*"],
        "defaultTargetPaths": {
            "moodContract": "moods/{id}.json",
            "eventContract": "events/{id}.json",
            "regionContractCard": "regions/{id}.json",
            "seedContractCard": "seeds/{id}.json"
        },
        "rules": {
            "oneFilePerRequest": true,
            "requireDeadledgerRef": false,
            "minRwfForTier": 0.8
        },
        "authoringHints": {
            "tierRationale": "Tier 1 is public contract-only.",
            "defaultStagingRepo": "HorrorPlace-Atrocity-Seeds"
        }
    }"#;
    
    std::fs::write(
        manifests_dir.join("repo-manifest.hpc.horror-place.json"),
        horror_place
    ).unwrap();
    
    // Atrocity-Seeds manifest
    let atrocity = r#"{
        "repo": "HorrorPlace-Atrocity-Seeds",
        "tier": "T2",
        "allowedObjectKinds": ["eventContract", "regionContractCard", "seedContractCard"],
        "allowedSchemas": ["schema://Horror.Place/*"],
        "defaultTargetPaths": {
            "eventContract": "events/{id}.json",
            "regionContractCard": "regions/{id}.json",
            "seedContractCard": "seeds/{id}.json"
        },
        "rules": {
            "oneFilePerRequest": true,
            "requireDeadledgerRef": true
        },
        "authoringHints": {
            "tierRationale": "Tier 2 vault for implication-only seeds.",
            "deadledgerRationale": "High-intensity seeds require Dead-Ledger attestation."
        }
    }"#;
    
    std::fs::write(
        manifests_dir.join("repo-manifest.hpc.atrocity-seeds.json"),
        atrocity
    ).unwrap();
}
