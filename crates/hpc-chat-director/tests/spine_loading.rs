//! Tests for spine loading and invariant/metric queries.
//!
//! Ensures the schema spine loads correctly and derived metrics
//! compute as expected from base invariants.

use std::path::PathBuf;
use std::collections::HashMap;

use hpc_chat_director::SpineIndex;
use hpc_chat_director::model::spine_types::{ObjectKind, Tier};

/// Spine loads from the real repo and exposes basic queries.
#[test]
fn spine_loads_and_exposes_basic_queries() {
    // Adjust this root resolution if your workspace layout differs.
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..");

    let schema_spine = root
        .join("schemas")
        .join("core")
        .join("schema-spine-index-v1.json");
    let invariants_spine = root
        .join("schemas")
        .join("core")
        .join("invariants-spine.v1.json");
    let entertainment_spine = root
        .join("schemas")
        .join("core")
        .join("entertainment-metrics-spine.v1.json");

    let spine = SpineIndex::load_from_paths(
        &schema_spine,
        &invariants_spine,
        &entertainment_spine,
    )
    .expect("spine must load from repo schemas");

    // Basic invariant/metric lookup.
    assert!(spine.describe_invariant("CIC").is_some());
    assert!(spine.describe_metric("UEC").is_some());

    // ObjectKind profile is available for a v1 contract family.
    let mood_profile = spine.describe_object_kind(ObjectKind::MoodContract);
    assert!(mood_profile.is_some());
}

/// Canonical ranges for invariants are exposed and tier-aware.
#[test]
fn invariant_ranges_respect_tier_overrides() {
    let spine = load_repo_spine();

    let cic_spec = spine
        .describe_invariant("CIC")
        .expect("CIC spec must exist");

    // Canonical band and tier override should be consistent.
    let (base_min, base_max) =
        spine.range_for_invariant(&cic_spec, ObjectKind::MoodContract, Tier::Tier2Internal);
    assert!(base_min <= base_max);

    // Tier 1 public should narrow CIC ceiling as per spec.
    let (_t1_min, t1_max) =
        spine.range_for_invariant(&cic_spec, ObjectKind::MoodContract, Tier::Tier1Public);
    assert!(t1_max <= base_max);
}

/// Metric target bands are exposed as safe target bands per tier.
#[test]
fn metric_targets_respect_tier_bands() {
    let spine = load_repo_spine();

    let uec_spec = spine
        .describe_metric("UEC")
        .expect("UEC metric spec must exist");

    let (t1_min, t1_max) =
        spine.target_band_for_metric(&uec_spec, ObjectKind::MoodContract, Tier::Tier1Public);
    assert!(0.0 <= t1_min && t1_min <= t1_max && t1_max <= 1.0);
}

/// Contract family profiles are loaded and include required invariants.
#[test]
fn contract_families_have_required_invariants() {
    let spine = load_repo_spine();

    let mood_profile = spine
        .describe_object_kind(ObjectKind::MoodContract)
        .expect("moodContract family profile must exist");
    assert!(mood_profile.required_invariants.contains(&"CIC".to_string()));
}

/// Derived metrics (SPR, SHCI) stay within [0.0, 1.0] for plausible inputs.
#[test]
fn derived_metrics_are_in_valid_range() {
    let spine = load_repo_spine();

    let mut invariants = HashMap::new();
    invariants.insert("CIC".to_string(), 0.8);
    invariants.insert("AOS".to_string(), 0.6);
    invariants.insert("MDI".to_string(), 0.5);
    invariants.insert("LSG".to_string(), 0.4);
    invariants.insert("FCF".to_string(), 5.0);

    let derived = spine
        .compute_derived_metrics(&invariants)
        .expect("derived metrics must compute");

    if let Some(spr) = derived.spr {
        assert!((0.0..=1.0).contains(&spr));
    }
    if let Some(shci) = derived.shci {
        assert!((0.0..=1.0).contains(&shci));
    }
}

/// Safe defaults provide a Tier 1-safe band for moodContract.
#[test]
fn safe_defaults_for_mood_tier1_exist() {
    let spine = load_repo_spine();

    let defaults = spine
        .safe_defaults(ObjectKind::MoodContract, Tier::Tier1Public)
        .expect("safe defaults for moodContract Tier1Public must exist");

    assert!(defaults.invariants.contains_key("CIC"));
    let cic_band = defaults.invariants.get("CIC").unwrap();
    assert!(cic_band.0 <= cic_band.1);
}

/// ObjectKind profile API reports required invariants and metrics.
#[test]
fn describe_object_kind_reports_requirements() {
    let spine = load_repo_spine();

    let profile = spine
        .describe_object_kind(ObjectKind::MoodContract)
        .expect("moodContract profile must exist");

    assert_eq!(profile.kind, ObjectKind::MoodContract);
    assert!(!profile.required_invariants.is_empty());
    assert!(!profile.required_metrics.is_empty());
}

/// Helper: load the real spine from the repo instead of synthesizing JSON.
fn load_repo_spine() -> SpineIndex {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..");

    let schema_spine = root
        .join("schemas")
        .join("core")
        .join("schema-spine-index-v1.json");
    let invariants_spine = root
        .join("schemas")
        .join("core")
        .join("invariants-spine.v1.json");
    let entertainment_spine = root
        .join("schemas")
        .join("core")
        .join("entertainment-metrics-spine.v1.json");

    SpineIndex::load_from_paths(
        &schema_spine,
        &invariants_spine,
        &entertainment_spine,
    )
    .expect("failed to load spine from repo fixtures")
}
