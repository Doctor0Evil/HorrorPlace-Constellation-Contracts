//! Typed representations of the schema spine index.
//!
//! The spine is the single source of truth for invariants, metrics,
//! and contract families. All numeric ranges and governance rules
//! are defined here, never hardcoded in Rust.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::model::SchemaBacked;

/// Root container for the schema spine index.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaSpine {
    /// Spine schema version.
    pub version: String,
    /// Canonical schema URI for this spine.
    #[serde(rename = "$id")]
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Description of the spine's purpose.
    pub description: String,
    /// Map of invariant definitions by name.
    pub invariants: HashMap<String, InvariantSpec>,
    /// Map of entertainment metric definitions by name.
    pub metrics: HashMap<String, MetricSpec>,
    /// Contract families grouping related schemas.
    pub contract_families: Vec<ContractFamily>,
    /// Cross-metric interaction rules (XMIT).
    #[serde(default)]
    pub interaction_rules: Vec<InteractionRule>,
    /// Safe default bands per objectKind × tier.
    #[serde(default)]
    pub safe_defaults: HashMap<String, TierDefaultBands>,
}

impl SchemaBacked for SchemaSpine {
    fn schema_uri() -> &'static str {
        "schema://HorrorPlace-Constellation-Contracts/schema-spine-index-v1.json"
    }

    fn schema_version() -> &'static str {
        "v1"
    }
}

/// Definition of a single invariant.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvariantSpec {
    /// Invariant name (e.g., "CIC", "DET").
    pub name: String,
    /// Canonical numeric range.
    pub canonical_range: NumericRange,
    /// Tier-specific overrides.
    #[serde(default)]
    pub tier_overrides: HashMap<Tier, NumericRange>,
    /// Whether this invariant may drift over time.
    #[serde(default)]
    pub drift_mode: DriftMode,
    /// Metrics this invariant interacts with.
    #[serde(default)]
    pub compatible_with: Vec<String>,
    /// Prompt-safe description.
    pub description: String,
    /// Whether this invariant is required for each contract family.
    #[serde(default)]
    pub required_by: Vec<String>,
}

/// Numeric range for invariants and metrics.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumericRange {
    pub min: f64,
    pub max: f64,
}

/// Drift mode for invariants.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftMode {
    /// Invariant is static; never changes.
    Static,
    /// Invariant may shift slowly over time.
    SlowlyVarying,
    /// Invariant responds to player telemetry.
    PlayerReactive,
    /// Invariant is derived from other values.
    Derived,
}

/// Definition of an entertainment metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricSpec {
    /// Metric name (e.g., "UEC", "ARR").
    pub name: String,
    /// Target band for this metric.
    pub target_band: NumericRange,
    /// Tier-specific target adjustments.
    #[serde(default)]
    pub tier_adjustments: HashMap<Tier, MetricAdjustment>,
    /// Whether this metric is telemetry-hooked.
    #[serde(default)]
    pub telemetry_hook: Option<String>,
    /// Prompt-safe description.
    pub description: String,
    /// Whether this metric is required for each contract family.
    #[serde(default)]
    pub required_by: Vec<String>,
}

/// Adjustment to metric targets per tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricAdjustment {
    /// Optional min adjustment.
    pub min_delta: Option<f64>,
    /// Optional max adjustment.
    pub max_delta: Option<f64>,
}

/// Grouping of related contract schemas.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractFamily {
    /// Family name (e.g., "mood", "event").
    pub name: String,
    /// Object kinds belonging to this family.
    pub kinds: Vec<String>,
    /// Required invariant bindings for this family.
    pub required_invariants: Vec<String>,
    /// Optional invariant bindings.
    #[serde(default)]
    pub optional_invariants: Vec<String>,
    /// Required metric targets for this family.
    pub required_metrics: Vec<String>,
    /// Optional metric targets.
    #[serde(default)]
    pub optional_metrics: Vec<String>,
    /// Allowed phases for this family.
    pub allowed_phases: Vec<Phase>,
    /// Tier restrictions per object kind.
    #[serde(default)]
    pub tier_restrictions: HashMap<String, TierRestriction>,
}

/// Cross-metric interaction rule (XMIT).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionRule {
    /// Rule ID (e.g., "XMIT_001").
    pub id: String,
    /// Source metric name.
    pub source_metric: String,
    /// Target metric name.
    pub target_metric: String,
    /// Effect type: amplify, suppress, or gate.
    pub effect_type: EffectType,
    /// Condition for applying this rule.
    pub condition: InteractionCondition,
    /// Effect description.
    pub description: String,
}

/// Effect type for interaction rules.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectType {
    /// Amplify: increase target metric band.
    Amplify,
    /// Suppress: decrease target metric band.
    Suppress,
    /// Gate: enforce minimum/maximum threshold.
    Gate,
}

/// Condition for applying an interaction rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionCondition {
    /// Source metric must exceed this value.
    pub source_threshold: Option<f64>,
    /// Source metric must be below this value.
    pub source_max: Option<f64>,
    /// Target archetype must match one of these.
    #[serde(default)]
    pub archetypes: Vec<String>,
    /// Target tile class must match one of these.
    #[serde(default)]
    pub tile_classes: Vec<String>,
}

/// Safe default bands per tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierDefaultBands {
    /// Defaults per tier.
    pub by_tier: HashMap<Tier, DefaultBands>,
}

/// Conservative default bands for validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultBands {
    /// Invariant defaults.
    pub invariants: HashMap<String, NumericRange>,
    /// Metric defaults.
    pub metrics: HashMap<String, NumericRange>,
}

/// Tier restriction for a contract family.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierRestriction {
    /// Whether this tier is allowed.
    pub allowed: bool,
    /// Optional notes explaining restrictions.
    pub notes: Option<String>,
}

/// Constellation tier classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tier {
    /// Tier 1: Public, contract-only surfaces.
    T1,
    /// Tier 2: Private vaults for seeds and bundles.
    T2,
    /// Tier 3: Private labs for experimental work.
    T3,
}

/// Authoring phase in the pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Phase {
    /// Phase 0: Schema definitions only.
    Schema0,
    /// Phase 1: Registry entries.
    Registry1,
    /// Phase 2: Full contract cards (bundles).
    Bundles2,
    /// Phase 3: Lua policy bindings.
    LuaPolicy3,
    /// Phase 4: Engine adapters.
    Adapters4,
}
