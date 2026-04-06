//! Integration tests for manifest-based routing and tier policy enforcement.
//!
//! These tests verify that CHAT_DIRECTOR correctly routes artifacts to
//! their canonical repos based on objectKind, tier, and manifest rules,
//! and that policy violations produce structured diagnostics.

use std::path::PathBuf;
use tempfile::TempDir;
use hpc_chat_director::{ChatDirector, config::Config};
use hpc_chat_director::model::{
    request_types::AiAuthoringRequest,
    response_types::AiAuthoringResponse,
    spine_types::{Phase, Tier},
};
use serde_json::json;

/// Test that valid routing succeeds for Tier 1 moodContract.
#[test]
fn test_routing_tier1_mood_contract() {
    let (_temp_dir, director) = setup_test_environment();

    let req = AiAuthoringRequest {
        schema_ref: "schema://Horror.Place/moodcontract_v1.json".into(),
        intent: "Create a liminal mood for a backrooms region".into(),
        object_kind: "moodContract".into(),
        candidate_kinds: vec![],
        target_repo: "Horror.Place".into(),
        target_path: "moods/liminal_backrooms_v1.json".into(),
        phase: Phase::Bundles2,
        tier: Tier::T1,
        referenced_ids: vec!["region.backrooms_v1".into()],
        shci_bands: None,
        intended_invariants: Default::default(),
        intended_metrics: Default::default(),
        extra_guidance: None,
        agent_profile_id: None,
    };

    let resp = AiAuthoringResponse {
        schema_ref: req.schema_ref.clone(),
        artifact: json!({
            "id": "mood.liminal_backrooms.v1",
            "tileClass": "liminal",
            "archetype": "sanctuary",
            "invariants": {
                "CIC": 0.5, "MDI": 0.6, "AOS": 0.7, "DET": 5.0,
                "LSG": 0.3, "HVF": 0.6, "SHCI": 0.5
            },
            "metricsIntent": {
                "UEC": 0.6, "EMD": 0.2, "CDL": 0.5, "ARR": 0.7
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
        target_path: "moods/liminal_backrooms_v1.json".into(),
        registry_diffs: vec![],
        additional_artifacts: vec![],
        generated_by: hpc_chat_director::model::response_types::GeneratedBy {
            agent_id: "test-agent".into(),
            model_version: Some("test-v1".into()),
            session_id: None,
        },
    };

    let result = director.validate_response(&req, &resp);
    assert!(result.is_ok(), "Valid Tier 1 moodContract should pass routing: {:?}", result.err());
}

/// Test that routing fails when objectKind is not allowed in target repo.
#[test]
fn test_routing_object_kind_not_allowed() {
    let (_temp_dir, director) = setup_test_environment();

    let req = AiAuthoringRequest {
        schema_ref: "schema://Horror.Place/eventcontract_v1.json".into(),
        intent: "Create a raw narrative event".into(),
        object_kind: "eventContract".into(),
        candidate_kinds: vec![],
        target_repo: "HorrorPlace-Constellation-Contracts".into(), // This repo only accepts schemas/contracts, not events
        target_path: "events/raw_narrative_v1.json".into(),
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
        artifact: json!({"id": "event.raw_narrative.v1"}),
        envelope: hpc_chat_director::model::response_types::PrismEnvelope {
            envelope_version: "v1".into(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            schema_ref: req.schema_ref.clone(),
            deadledger_ref: None,
            zkp_commitment: None,
            prisma_meta: None,
            signature: None,
        },
        target_repo: "HorrorPlace-Constellation-Contracts".into(),
        target_path: "events/raw_narrative_v1.json".into(),
        registry_diffs: vec![],
        additional_artifacts: vec![],
        generated_by: hpc_chat_director::model::response_types::GeneratedBy {
            agent_id: "test-agent".into(),
            model_version: None,
            session_id: None,
        },
    };

    let result = director.validate_response(&req, &resp);
    assert!(result.is_err(), "EventContract should not be allowed in Constellation-Contracts");
    
    let err = result.unwrap_err();
    assert!(err.message.contains("not allowed"), "Error should mention objectKind not allowed");
}

/// Test that Tier 1 policy blocks raw narrative content.
#[test]
fn test_tier1_blocks_raw_narrative() {
    let (_temp_dir, director) = setup_test_environment();

    let req = AiAuthoringRequest {
        schema_ref: "schema://Horror.Place/regioncontract_v1.json".into(),
        intent: "Create a region with raw lore".into(),
        object_kind: "regionContractCard".into(),
        candidate_kinds: vec![],
        target_repo: "Horror.Place".into(),
        target_path: "regions/with_lore_v1.json".into(),
        phase: Phase::Bundles2,
        tier: Tier::T1,
        referenced_ids: vec![],
        shci_bands: None,
        intended_invariants: Default::default(),
        intended_metrics: Default::default(),
        extra_guidance: None,
        agent_profile_id: None,
    };

    // Artifact with forbidden "rawNarrative" field
    let resp = AiAuthoringResponse {
        schema_ref: req.schema_ref.clone(),
        artifact: json!({
            "id": "region.with_lore.v1",
            "rawNarrative": "This is forbidden content" // Should trigger Tier 1 policy
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
        target_path: "regions/with_lore_v1.json".into(),
        registry_diffs: vec![],
        additional_artifacts: vec![],
        generated_by: hpc_chat_director::model::response_types::GeneratedBy {
            agent_id: "test-agent".into(),
            model_version: None,
            session_id: None,
        },
    };

    let result = director.validate_response(&req, &resp);
    // Should fail due to Tier 1 policy (no raw narrative)
    assert!(result.is_err(), "Tier 1 should block raw narrative content");
}

/// Test that RWF gating redirects low-confidence artifacts.
#[test]
fn test_rwf_gating_redirects_to_staging() {
    let (_temp_dir, director) = setup_test_environment();

    let req = AiAuthoringRequest {
        schema_ref: "schema://Horror.Place/seedcontract_v1.json".into(),
        intent: "Create experimental seed".into(),
        object_kind: "seedContractCard".into(),
        candidate_kinds: vec![],
        target_repo: "Horror.Place".into(), // Tier 1 requires high RWF
        target_path: "seeds/experimental_v1.json".into(),
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
        artifact: json!({"id": "seed.experimental.v1"}),
        envelope: hpc_chat_director::model::response_types::PrismEnvelope {
            envelope_version: "v1".into(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            schema_ref: req.schema_ref.clone(),
            deadledger_ref: None,
            zkp_commitment: None,
            prisma_meta: Some(hpc_chat_director::model::response_types::PrismMeta {
                rwf: Some(0.3), // Below Tier 1 minimum (typically 0.8)
                review_status: None,
                generation_phase: None,
                telemetry_hooks: vec![],
            }),
            signature: None,
        },
        target_repo: "Horror.Place".into(),
        target_path: "seeds/experimental_v1.json".into(),
        registry_diffs: vec![],
        additional_artifacts: vec![],
        generated_by: hpc_chat_director::model::response_types::GeneratedBy {
            agent_id: "test-agent".into(),
            model_version: None,
            session_id: None,
        },
    };

    let result = director.validate_response(&req, &resp);
    assert!(result.is_err(), "Low RWF should fail Tier 1 routing");
    
    // Error should suggest alternative repo
    let err_msg = format!("{:?}", result.err());
    assert!(
        err_msg.contains("suggested") || err_msg.contains("alternative"),
        "Error should suggest alternative repo for low-RWF artifact"
    );
}

/// Test that cross-repo references respect manifest policies.
#[test]
fn test_cross_repo_reference_validation() {
    let (_temp_dir, director) = setup_test_environment();

    let req = AiAuthoringRequest {
        schema_ref: "schema://Horror.Place/eventcontract_v1.json".into(),
        intent: "Create event referencing vault content".into(),
        object_kind: "eventContract".into(),
        candidate_kinds: vec![],
        target_repo: "Horror.Place".into(), // Public Tier 1
        target_path: "events/vault_ref_v1.json".into(),
        phase: Phase::Bundles2,
        tier: Tier::T1,
        referenced_ids: vec![
            "event.vault_secret_001".into(), // From private vault repo
        ],
        shci_bands: None,
        intended_invariants: Default::default(),
        intended_metrics: Default::default(),
        extra_guidance: None,
        agent_profile_id: None,
    };

    let resp = AiAuthoringResponse {
        schema_ref: req.schema_ref.clone(),
        artifact: json!({"id": "event.vault_ref.v1"}),
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
        target_path: "events/vault_ref_v1.json".into(),
        registry_diffs: vec![],
        additional_artifacts: vec![],
        generated_by: hpc_chat_director::model::response_types::GeneratedBy {
            agent_id: "test-agent".into(),
            model_version: None,
            session_id: None,
        },
    };

    let result = director.validate_response(&req, &resp);
    // Should fail or warn about cross-repo reference from public to private
    // Exact behavior depends on manifest cross_ref_policy configuration
    // This test documents the expected behavior for future refinement
    println!("Cross-ref validation result: {:?}", result);
}

/// Set up a minimal test environment with spine and manifests.
fn setup_test_environment() -> (TempDir, ChatDirector) {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create minimal spine and manifest structure
    // (In real tests, use fixtures from HorrorPlace-Constellation-Contracts)
    create_test_spine(root);
    create_test_manifests(root);

    let config = Config::detect(root).unwrap();
    let director = ChatDirector::load_environment(root).unwrap();
    
    (temp_dir, director)
}

/// Create a minimal schema spine for testing.
fn create_test_spine(root: &std::path::Path) {
    let spine_dir = root.join("schemas").join("core");
    std::fs::create_dir_all(&spine_dir).unwrap();
    
    // Write minimal schema-spine-index-v1.json
    let spine_content = r#"{
        "version": "v1",
        "$id": "schema://HorrorPlace-Constellation-Contracts/schema-spine-index-v1.json",
        "title": "Test Spine",
        "description": "Minimal spine for manifest routing tests",
        "invariants": {
            "CIC": {
                "name": "CIC",
                "canonicalRange": {"min": 0.0, "max": 1.0},
                "tierOverrides": {},
                "driftMode": "static",
                "compatibleWith": [],
                "description": "Contextual Integrity Coefficient",
                "requiredBy": []
            }
        },
        "metrics": {},
        "contractFamilies": [
            {
                "name": "mood",
                "kinds": ["moodContract"],
                "requiredInvariants": ["CIC"],
                "optionalInvariants": [],
                "requiredMetrics": [],
                "optionalMetrics": [],
                "allowedPhases": [2],
                "tierRestrictions": {}
            }
        ],
        "interactionRules": [],
        "safeDefaults": {}
    }"#;
    
    std::fs::write(
        spine_dir.join("schema-spine-index-v1.json"),
        spine_content
    ).unwrap();
}

/// Create minimal repo manifests for testing.
fn create_test_manifests(root: &std::path::Path) {
    let manifests_dir = root.join("manifests");
    std::fs::create_dir_all(&manifests_dir).unwrap();
    
    // Horror.Place manifest (Tier 1, contracts only)
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
            "tierRationale": "Tier 1 is public contract-only; no raw narrative.",
            "defaultStagingRepo": "HorrorPlace-Atrocity-Seeds"
        }
    }"#;
    
    std::fs::write(
        manifests_dir.join("repo-manifest.hpc.horror-place.json"),
        horror_place
    ).unwrap();
    
    // Constellation-Contracts manifest (Tier 1, schemas only)
    let contracts = r#"{
        "repo": "HorrorPlace-Constellation-Contracts",
        "tier": "T1",
        "allowedObjectKinds": [],
        "allowedSchemas": ["schema://HorrorPlace-Constellation-Contracts/*"],
        "defaultTargetPaths": {},
        "rules": {
            "oneFilePerRequest": true,
            "requireDeadledgerRef": false
        },
        "authoringHints": {
            "tierRationale": "This repo holds schemas and contracts only, not content."
        }
    }"#;
    
    std::fs::write(
        manifests_dir.join("repo-manifest.hpc.constellation-contracts.json"),
        contracts
    ).unwrap();
}
