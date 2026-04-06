//! Tests for the full validation pipeline: schema → invariants → manifests → envelope.
//!
//! These tests verify that CHAT_DIRECTOR correctly validates AI-generated
//! responses against all constraint layers and produces structured diagnostics.

use std::path::Path;
use tempfile::TempDir;
use hpc_chat_director::{ChatDirector, config::Config, errors::ValidationError};
use hpc_chat_director::model::{
    request_types::AiAuthoringRequest,
    response_types::AiAuthoringResponse,
    spine_types::{Phase, Tier},
};
use serde_json::json;

/// Test that valid response passes all validation layers.
#[test]
fn test_valid_response_passes_all_layers() {
    let (_temp_dir, director) = setup_test_environment();

    let req = AiAuthoringRequest {
        schema_ref: "schema://Horror.Place/moodcontract_v1.json".into(),
        intent: "Create a valid mood contract".into(),
        object_kind: "moodContract".into(),
        candidate_kinds: vec![],
        target_repo: "Horror.Place".into(),
        target_path: "moods/valid_v1.json".into(),
        phase: Phase::Bundles2,
        tier: Tier::T1,
        referenced_ids: vec![],
        shci_bands: None,
        intended_invariants: Default::default(),
        intended_metrics: Default::default(),
        extra_guidance: None,
        agent_profile_id: None,
    };

    let resp = AiAuthoringResponse {
        schema_ref: req.schema_ref.clone(),
        artifact: json!({
            "id": "mood.valid.v1",
            "tileClass": "spawn",
            "archetype": "sanctuary",
            "invariants": {
                "CIC": 0.5, "MDI": 0.4, "AOS": 0.3, "DET": 4.0,
                "LSG": 0.7, "HVF": 0.3, "SHCI": 0.4
            },
            "metricsIntent": {
                "UEC": 0.6, "EMD": 0.1, "CDL": 0.3, "ARR": 0.7
            },
            "requiredHooks": ["on_entry", "on_exit"],
            "dreadForgePattern": {
                "stages": ["outer", "threshold", "locus", "rupture", "fallout"],
                "modulation": {}
            }
        }),
        envelope: hpc_chat_director::model::response_types::PrismEnvelope {
            envelope_version: "v1".into(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            schema_ref: req.schema_ref.clone(),
            deadledger_ref: None,
            zkp_commitment: None,
            prisma_meta: None,
            signature: None,
        },
        target_repo: "Horror.Place".into(),
        target_path: "moods/valid_v1.json".into(),
        registry_diffs: vec![],
        additional_artifacts: vec![],
        generated_by: hpc_chat_director::model::response_types::GeneratedBy {
            agent_id: "test-agent".into(),
            model_version: Some("test-v1".into()),
            session_id: None,
        },
    };

    let result = director.validate_response(&req, &resp);
    assert!(result.is_ok(), "Valid response should pass all layers: {:?}", result.err());
}

/// Test that schema validation failures produce structured errors.
#[test]
fn test_schema_validation_failure() {
    let (_temp_dir, director) = setup_test_environment();

    let req = AiAuthoringRequest {
        schema_ref: "schema://Horror.Place/moodcontract_v1.json".into(),
        intent: "Create mood with invalid schema".into(),
        object_kind: "moodContract".into(),
        candidate_kinds: vec![],
        target_repo: "Horror.Place".into(),
        target_path: "moods/invalid_schema_v1.json".into(),
        phase: Phase::Bundles2,
        tier: Tier::T1,
        referenced_ids: vec![],
        shci_bands: None,
        intended_invariants: Default::default(),
        intended_metrics: Default::default(),
        extra_guidance: None,
        agent_profile_id: None,
    };

    // Response with missing required field
    let resp = AiAuthoringResponse {
        schema_ref: req.schema_ref.clone(),
        artifact: json!({
            "id": "mood.invalid_schema.v1"
            // Missing required fields: tileClass, archetype, invariants, etc.
        }),
        envelope: hpc_chat_director::model::response_types::PrismEnvelope {
            envelope_version: "v1".into(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            schema_ref: req.schema_ref.clone(),
            deadledger_ref: None,
            zkp_commitment: None,
            prisma_meta: None,
            signature: None,
        },
        target_repo: "Horror.Place".into(),
        target_path: "moods/invalid_schema_v1.json".into(),
        registry_diffs: vec![],
        additional_artifacts: vec![],
        generated_by: hpc_chat_director::model::response_types::GeneratedBy {
            agent_id: "test-agent".into(),
            model_version: None,
            session_id: None,
        },
    };

    let result = director.validate_response(&req, &resp);
    assert!(result.is_err(), "Missing required fields should fail schema validation");
    
    let err = result.unwrap_err();
    assert_eq!(err.code, "SCHEMA_VALIDATION_FAILED");
    assert!(!err.json_pointer.is_empty(), "Error should include JSON Pointer");
}

/// Test that invariant range violations produce structured diagnostics.
#[test]
fn test_invariant_range_violation() {
    let (_temp_dir, director) = setup_test_environment();

    let req = AiAuthoringRequest {
        schema_ref: "schema://Horror.Place/moodcontract_v1.json".into(),
        intent: "Create mood with DET out of range".into(),
        object_kind: "moodContract".into(),
        candidate_kinds: vec![],
        target_repo: "Horror.Place".into(),
        target_path: "moods/high_det_v1.json".into(),
        phase: Phase::Bundles2,
        tier: Tier::T1,
        referenced_ids: vec![],
        shci_bands: None,
        intended_invariants: Default::default(),
        intended_metrics: Default::default(),
        extra_guidance: None,
        agent_profile_id: None,
    };

    let resp = AiAuthoringResponse {
        schema_ref: req.schema_ref.clone(),
        artifact: json!({
            "id": "mood.high_det.v1",
            "tileClass": "spawn",
            "archetype": "sanctuary",
            "invariants": {
                "CIC": 0.5, "MDI": 0.4, "AOS": 0.3,
                "DET": 9.5,  // Exceeds Tier 1 max of 7.0
                "LSG": 0.7, "HVF": 0.3, "SHCI": 0.4
            },
            "metricsIntent": {
                "UEC": 0.6, "EMD": 0.1, "CDL": 0.3, "ARR": 0.7
            }
        }),
        envelope: hpc_chat_director::model::response_types::PrismEnvelope {
            envelope_version: "v1".into(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            schema_ref: req.schema_ref.clone(),
            deadledger_ref: None,
            zkp_commitment: None,
            prisma_meta: None,
            signature: None,
        },
        target_repo: "Horror.Place".into(),
        target_path: "moods/high_det_v1.json".into(),
        registry_diffs: vec![],
        additional_artifacts: vec![],
        generated_by: hpc_chat_director::model::response_types::GeneratedBy {
            agent_id: "test-agent".into(),
            model_version: None,
            session_id: None,
        },
    };

    let result = director.validate_response(&req, &resp);
    assert!(result.is_err(), "DET > 7.0 should fail Tier 1 invariant check");
    
    let err = result.unwrap_err();
    assert!(
        err.code.contains("DET") || err.code.contains("RANGE"),
        "Error code should reference DET or range: {}",
        err.code
    );
    assert!(
        err.message.contains("7.0") || err.message.contains("range"),
        "Error message should mention Tier 1 DET limit"
    );
}

/// Test that cross-metric interactions (XMIT rules) are enforced.
#[test]
fn test_xmit_interaction_enforcement() {
    let (_temp_dir, director) = setup_test_environment();

    let req = AiAuthoringRequest {
        schema_ref: "schema://Horror.Place/moodcontract_v1.json".into(),
        intent: "Create mood with high DET and low CDL".into(),
        object_kind: "moodContract".into(),
        candidate_kinds: vec![],
        target_repo: "Horror.Place".into(),
        target_path: "moods/xmit_test_v1.json".into(),
        phase: Phase::Bundles2,
        tier: Tier::T1,
        referenced_ids: vec![],
        shci_bands: None,
        intended_invariants: Default::default(),
        intended_metrics: Default::default(),
        extra_guidance: None,
        agent_profile_id: None,
    };

    let resp = AiAuthoringResponse {
        schema_ref: req.schema_ref.clone(),
        artifact: json!({
            "id": "mood.xmit_test.v1",
            "tileClass": "battlefront",
            "archetype": "combat",
            "invariants": {
                "CIC": 0.8, "MDI": 0.7, "AOS": 0.8,
                "DET": 8.5,  // Triggers XMIT_001: CDL floor raised to 0.4
                "LSG": 0.4, "HVF": 0.7, "SHCI": 0.6
            },
            "metricsIntent": {
                "UEC": 0.7, "EMD": 0.3,
                "CDL": 0.2,  // Below adjusted floor of 0.4 due to high DET
                "ARR": 0.6
            }
        }),
        envelope: hpc_chat_director::model::response_types::PrismEnvelope {
            envelope_version: "v1".into(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            schema_ref: req.schema_ref.clone(),
            deadledger_ref: None,
            zkp_commitment: None,
            prisma_meta: None,
            signature: None,
        },
        target_repo: "Horror.Place".into(),
        target_path: "moods/xmit_test_v1.json".into(),
        registry_diffs: vec![],
        additional_artifacts: vec![],
        generated_by: hpc_chat_director::model::response_types::GeneratedBy {
            agent_id: "test-agent".into(),
            model_version: None,
            session_id: None,
        },
    };

    let result = director.validate_response(&req, &resp);
    // XMIT_001 should raise CDL floor to 0.4 when DET > 8.0
    // CDL=0.2 should fail the adjusted check
    assert!(result.is_err(), "CDL below XMIT-adjusted floor should fail");
    
    let err = result.unwrap_err();
    assert!(
        err.code.contains("CDL") || err.code.contains("XMIT"),
        "Error should reference CDL or XMIT interaction"
    );
}

/// Test that envelope structural validation catches missing fields.
#[test]
fn test_envelope_validation_missing_timestamp() {
    let (_temp_dir, director) = setup_test_environment();

    let req = AiAuthoringRequest {
        schema_ref: "schema://Horror.Place/moodcontract_v1.json".into(),
        intent: "Create mood with invalid envelope".into(),
        object_kind: "moodContract".into(),
        candidate_kinds: vec![],
        target_repo: "Horror.Place".into(),
        target_path: "moods/bad_envelope_v1.json".into(),
        phase: Phase::Bundles2,
        tier: Tier::T1,
        referenced_ids: vec![],
        shci_bands: None,
        intended_invariants: Default::default(),
        intended_metrics: Default::default(),
        extra_guidance: None,
        agent_profile_id: None,
    };

    let resp = AiAuthoringResponse {
        schema_ref: req.schema_ref.clone(),
        artifact: json!({"id": "mood.bad_envelope.v1"}),
        envelope: hpc_chat_director::model::response_types::PrismEnvelope {
            envelope_version: "v1".into(),
            timestamp: "".into(),  // Empty timestamp should fail
            schema_ref: req.schema_ref.clone(),
            deadledger_ref: None,
            zkp_commitment: None,
            prisma_meta: None,
            signature: None,
        },
        target_repo: "Horror.Place".into(),
        target_path: "moods/bad_envelope_v1.json".into(),
        registry_diffs: vec![],
        additional_artifacts: vec![],
        generated_by: hpc_chat_director::model::response_types::GeneratedBy {
            agent_id: "test-agent".into(),
            model_version: None,
            session_id: None,
        },
    };

    let result = director.validate_response(&req, &resp);
    assert!(result.is_err(), "Empty timestamp should fail envelope validation");
    
    let err = result.unwrap_err();
    assert!(
        err.code.contains("ENVELOPE") || err.code.contains("TIMESTAMP"),
        "Error should reference envelope or timestamp"
    );
}

/// Test that validation errors include remediation hints.
#[test]
fn test_validation_error_includes_remediation() {
    let (_temp_dir, director) = setup_test_environment();

    let req = AiAuthoringRequest {
        schema_ref: "schema://Horror.Place/moodcontract_v1.json".into(),
        intent: "Create mood to test remediation".into(),
        object_kind: "moodContract".into(),
        candidate_kinds: vec![],
        target_repo: "Horror.Place".into(),
        target_path: "moods/remediation_test_v1.json".into(),
        phase: Phase::Bundles2,
        tier: Tier::T1,
        referenced_ids: vec![],
        shci_bands: None,
        intended_invariants: Default::default(),
        intended_metrics: Default::default(),
        extra_guidance: None,
        agent_profile_id: None,
    };

    let resp = AiAuthoringResponse {
        schema_ref: req.schema_ref.clone(),
        artifact: json!({
            "id": "mood.remediation_test.v1",
            "invariants": {"DET": 12.0}  // Way out of range
        }),
        envelope: hpc_chat_director::model::response_types::PrismEnvelope {
            envelope_version: "v1".into(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            schema_ref: req.schema_ref.clone(),
            deadledger_ref: None,
            zkp_commitment: None,
            prisma_meta: None,
            signature: None,
        },
        target_repo: "Horror.Place".into(),
        target_path: "moods/remediation_test_v1.json".into(),
        registry_diffs: vec![],
        additional_artifacts: vec![],
        generated_by: hpc_chat_director::model::response_types::GeneratedBy {
            agent_id: "test-agent".into(),
            model_version: None,
            session_id: None,
        },
    };

    let result = director.validate_response(&req, &resp);
    assert!(result.is_err());
    
    let err = result.unwrap_err();
    assert!(
        err.remediation.is_some(),
        "ValidationError should include remediation hint"
    );
    
    let remediation = err.remediation.as_ref().unwrap();
    assert!(
        !remediation.suggestion.is_empty(),
        "Remediation should include actionable suggestion"
    );
}

/// Set up minimal test environment for validation tests.
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
        "description": "Minimal spine for validation tests",
        "invariants": {
            "CIC": {
                "name": "CIC",
                "canonicalRange": {"min": 0.0, "max": 1.0},
                "tierOverrides": {"T1": {"min": 0.3, "max": 0.7}},
                "driftMode": "static",
                "compatibleWith": [],
                "description": "Contextual Integrity Coefficient",
                "requiredBy": ["mood"]
            },
            "DET": {
                "name": "DET",
                "canonicalRange": {"min": 0.0, "max": 10.0},
                "tierOverrides": {"T1": {"min": 0.0, "max": 7.0}},
                "driftMode": "static",
                "compatibleWith": ["CDL"],
                "description": "Dread Entropy Threshold",
                "requiredBy": ["mood"]
            },
            "CDL": {
                "name": "CDL",
                "canonicalRange": {"min": 0.0, "max": 1.0},
                "tierOverrides": {},
                "driftMode": "static",
                "compatibleWith": [],
                "description": "Cognitive Dissonance Level",
                "requiredBy": ["mood"]
            }
        },
        "metrics": {
            "ARR": {
                "name": "ARR",
                "targetBand": {"min": 0.5, "max": 1.0},
                "tierAdjustments": {},
                "telemetryHook": null,
                "description": "Audience Retention Rating",
                "requiredBy": ["mood"]
            }
        },
        "contractFamilies": [
            {
                "name": "mood",
                "kinds": ["moodContract"],
                "requiredInvariants": ["CIC", "DET", "CDL"],
                "optionalInvariants": [],
                "requiredMetrics": ["ARR"],
                "optionalMetrics": [],
                "allowedPhases": [1, 2, 3],
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
                "description": "High DET raises CDL floor to 0.4"
            }
        ],
        "safeDefaults": {}
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
    
    let horror_place = r#"{
        "repo": "Horror.Place",
        "tier": "T1",
        "allowedObjectKinds": ["moodContract"],
        "allowedSchemas": ["schema://Horror.Place/*"],
        "defaultTargetPaths": {"moodContract": "moods/{id}.json"},
        "rules": {
            "oneFilePerRequest": true,
            "requireDeadledgerRef": false,
            "minRwfForTier": 0.8
        },
        "authoringHints": {
            "tierRationale": "Tier 1 is public contract-only."
        }
    }"#;
    
    std::fs::write(
        manifests_dir.join("repo-manifest.hpc.horror-place.json"),
        horror_place
    ).unwrap();
}
