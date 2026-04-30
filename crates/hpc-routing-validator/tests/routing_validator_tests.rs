use hpc_routing_validator::{
    validate_routing_system, MathConstraint, ObjectKindEntry, RepoManifest, RepoName, RepoPolicies,
    RoutingErrorCode, RoutingSpine, Tier,
};

fn make_minimal_spine() -> RoutingSpine {
    RoutingSpine {
        routingSpineVersion: "1.0.0".to_string(),
        schemaRef: "schemas/hpc-routing-spine.schema.json".to_string(),
        updatedAt: "2026-04-30T00:00:00Z".to_string(),
        mathConstraints: vec![MathConstraint {
            id: "sprFromInvariants.v1".to_string(),
            description: "SPR derived from invariants".to_string(),
            appliesTo: vec!["moodContract".to_string()],
        }],
        objectKinds: vec![ObjectKindEntry {
            name: "moodContract".to_string(),
            description: "Mood contract".to_string(),
            allowedTiers: vec![Tier::T1Core],
            routes: vec![],
        }],
    }
}

fn make_minimal_manifest() -> RepoManifest {
    RepoManifest {
        schemaVersion: "1.0.0".to_string(),
        schemaRef: "https://horror.place/constellation/schemas/core/repo-manifest-v1.json".to_string(),
        repoName: RepoName::HorrorPlace,
        tier: Tier::T1Core,
        kind: "SCHEMA_SPINE_REPO".to_string(),
        description: "Test manifest".to_string(),
        visibility: "public".to_string(),
        implicitDeny: false,
        routingSpineRef: Some("spine/hpc-routing-spine-v1.json".to_string()),
        allowedObjectKinds: vec!["moodContract".to_string()],
        defaultPaths: Default::default(),
        schemaWhitelist: vec![],
        policies: RepoPolicies::default(),
    }
}

#[test]
fn test_valid_routing_system_returns_no_errors() {
    let mut spine = make_minimal_spine();
    let manifest = make_minimal_manifest();

    // Add a valid route for moodContract T1-core -> Horror.Place
    spine.objectKinds[0].routes.push(
        serde_json::from_value(serde_json::json!({
            "id": "moodContract_T1_core_Horror.Place",
            "tier": "T1-core",
            "repo": "Horror.Place",
            "schemaRef": "schemas/contracts/mood-contract-v1.json",
            "aiAuthoringKind": "moodContract",
            "defaultTargetPath": "schemas/contracts/moods/",
            "registryRef": "registry/registry-moods.ndjson",
            "invariants": ["CIC", "AOS"],
            "metrics": ["UEC", "EMD"],
            "constraints": ["sprFromInvariants.v1"],
            "phase": "Phase0Schema",
            "additionalPropertiesRequired": true
        })).unwrap()
    );

    let errors = validate_routing_system(&[manifest], &spine);
    assert!(errors.is_empty(), "Expected no errors but got: {:?}", errors);
}

#[test]
fn test_duplicate_route_conflict_detected() {
    let mut spine = make_minimal_spine();

    // Add two routes for the same (objectKind, tier)
    spine.objectKinds[0].routes.push(
        serde_json::from_value(serde_json::json!({
            "id": "moodContract_T1_core_Horror.Place",
            "tier": "T1-core",
            "repo": "Horror.Place",
            "schemaRef": "schemas/contracts/mood-contract-v1.json",
            "aiAuthoringKind": "moodContract",
            "defaultTargetPath": "schemas/contracts/moods/",
            "registryRef": "registry/registry-moods.ndjson",
            "invariants": [],
            "metrics": [],
            "constraints": [],
            "phase": "Phase0Schema",
            "additionalPropertiesRequired": true
        })).unwrap()
    );
    spine.objectKinds[0].routes.push(
        serde_json::from_value(serde_json::json!({
            "id": "moodContract_T1_core_HorrorPlace_Spectral_Foundry",
            "tier": "T1-core",
            "repo": "HorrorPlace-Spectral-Foundry",
            "schemaRef": "schemas/contracts/mood-contract-v1.json",
            "aiAuthoringKind": "moodContract",
            "defaultTargetPath": "moods/",
            "registryRef": "registry/registry-moods.ndjson",
            "invariants": [],
            "metrics": [],
            "constraints": [],
            "phase": "Phase2Contracts",
            "additionalPropertiesRequired": true
        })).unwrap()
    );

    let manifest = make_minimal_manifest();
    let errors = validate_routing_system(&[manifest], &spine);

    assert!(!errors.is_empty());
    assert_eq!(errors[0].code, RoutingErrorCode::ERR_ROUTE_CONFLICT);
    assert!(errors[0].message.contains("moodContract"));
}

#[test]
fn test_implicit_deny_violation_detected() {
    let mut spine = make_minimal_spine();
    let mut manifest = make_minimal_manifest();

    // Set implicitDeny = true and empty allowedObjectKinds
    manifest.implicitDeny = true;
    manifest.allowedObjectKinds = vec![];

    spine.objectKinds[0].routes.push(
        serde_json::from_value(serde_json::json!({
            "id": "moodContract_T1_core_Horror.Place",
            "tier": "T1-core",
            "repo": "Horror.Place",
            "schemaRef": "schemas/contracts/mood-contract-v1.json",
            "aiAuthoringKind": "moodContract",
            "defaultTargetPath": "schemas/contracts/moods/",
            "registryRef": "registry/registry-moods.ndjson",
            "invariants": [],
            "metrics": [],
            "constraints": [],
            "phase": "Phase0Schema",
            "additionalPropertiesRequired": true
        })).unwrap()
    );

    let errors = validate_routing_system(&[manifest], &spine);

    assert!(!errors.is_empty());
    assert_eq!(errors[0].code, RoutingErrorCode::ERR_MANIFEST_IMPLICIT_DENY);
    assert!(errors[0].message.contains("implicitDeny"));
}

#[test]
fn test_missing_constraint_detected() {
    let mut spine = make_minimal_spine();
    let manifest = make_minimal_manifest();

    // Route references a constraint that doesn't exist in mathConstraints
    spine.objectKinds[0].routes.push(
        serde_json::from_value(serde_json::json!({
            "id": "moodContract_T1_core_Horror.Place",
            "tier": "T1-core",
            "repo": "Horror.Place",
            "schemaRef": "schemas/contracts/mood-contract-v1.json",
            "aiAuthoringKind": "moodContract",
            "defaultTargetPath": "schemas/contracts/moods/",
            "registryRef": "registry/registry-moods.ndjson",
            "invariants": [],
            "metrics": [],
            "constraints": ["nonExistentConstraint.v1"],
            "phase": "Phase0Schema",
            "additionalPropertiesRequired": true
        })).unwrap()
    );

    let errors = validate_routing_system(&[manifest], &spine);

    assert!(!errors.is_empty());
    assert_eq!(errors[0].code, RoutingErrorCode::ERR_CONSTRAINT_MISSING);
    assert!(errors[0].message.contains("nonExistentConstraint.v1"));
}
