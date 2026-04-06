//! Manifest routing tests.
//!
//! These tests verify that the manifest layer routes objectKinds to repos
//! based on tier and manifest rules, and that implicit deny is enforced.

use std::collections::HashMap;

use hpc_chat_director::manifests::{
    AuthoringHint, CiCheck, ManifestIndex, Policy, RepoManifest, RoutingError,
};
use hpc_chat_director::model::spine_types::{ObjectKind, Tier};

fn make_manifest(
    repo_name: &str,
    tier: Tier,
    kinds: Vec<ObjectKind>,
    implicit_deny: bool,
) -> RepoManifest {
    let mut default_paths = HashMap::new();
    default_paths.insert(
        ObjectKind::MoodContract,
        "contracts/mood/{id}.json".to_string(),
    );

    RepoManifest {
        repo_name: repo_name.to_string(),
        tier,
        allowed_object_kinds: kinds,
        schema_whitelist: vec!["moodContract.v1".to_string()],
        default_paths,
        policies: vec![Policy::OneFilePerRequest],
        ci_checks: vec![CiCheck::SchemaValidate],
        implicit_deny,
        authoring_hints: vec![AuthoringHint {
            rule_id: "AH-TIER".to_string(),
            description: "Tier routing test manifest".to_string(),
        }],
    }
}

#[test]
fn manifest_routing_basic_success() {
    let mut index = ManifestIndex::default();

    let m = make_manifest(
        "Horror.Place",
        Tier::Tier1Public,
        vec![ObjectKind::MoodContract],
        false,
    );
    index
        .insert_manifest(m);

    let explanation = index
        .explain_route(ObjectKind::MoodContract, Tier::Tier1Public)
        .expect("route should succeed");

    assert_eq!(explanation.repo_name, "Horror.Place");
    assert_eq!(explanation.requested_tier, Tier::Tier1Public);
    assert!(explanation
        .default_path
        .as_deref()
        .unwrap()
        .contains("contracts/mood"));
}

#[test]
fn manifest_routing_route_not_found() {
    let index = ManifestIndex::default();
    let err = index
        .explain_route(ObjectKind::MoodContract, Tier::Tier1Public)
        .err()
        .expect("route should fail");

    match err {
        RoutingError::RouteNotFound { .. } => {}
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn manifest_routing_implicit_deny() {
    let mut index = ManifestIndex::default();
    let m = make_manifest(
        "Horror.Place",
        Tier::Tier1Public,
        vec![], // no allowed kinds
        true,   // implicit deny
    );
    index
        .insert_manifest(m);

    let err = index
        .explain_route(ObjectKind::MoodContract, Tier::Tier1Public)
        .err()
        .expect("route should fail");

    match err {
        RoutingError::ImplicitDeny { repo_name, .. } => {
            assert_eq!(repo_name, "Horror.Place");
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
