//! Typed representations of the schema spine index.
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

impl SchemaSpine {
    pub fn find_invariant(&self, name: &str) -> Option<&InvariantSpec> {
        self.invariants.get(name)
    }

    pub fn find_metric(&self, name: &str) -> Option<&MetricSpec> {
        self.metrics.get(name)
    }

    pub fn describe_object_kind(&self, kind: &str) -> Option<ObjectKindProfile> {
        let families: Vec<&ContractFamily> = self
            .contract_families
            .iter()
            .filter(|f| f.kinds.iter().any(|k| k == kind))
            .collect();

        if families.is_empty() {
            return None;
        }

        let mut required_invariants = Vec::new();
        let mut required_metrics = Vec::new();
        let mut phases = Vec::new();

        for fam in families {
            for inv in &fam.required_invariants {
                if !required_invariants.contains(inv) {
                    required_invariants.push(inv.clone());
                }
            }
            for met in &fam.required_metrics {
                if !required_metrics.contains(met) {
                    required_metrics.push(met.clone());
                }
            }
            for phase in &fam.allowed_phases {
                if !phases.contains(phase) {
                    phases.push(*phase);
                }
            }
        }

        Some(ObjectKindProfile {
            object_kind: kind.to_string(),
            required_invariants,
            required_metrics,
            allowed_phases: phases,
        })
    }

    pub fn safe_defaults_for(&self, object_kind: &str, tier: Tier) -> Option<DefaultBands> {
        let key = object_kind.to_string();
        let by_tier = self.safe_defaults.get(&key)?;
        by_tier.by_tier.get(&tier).cloned()
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
    Static,
    SlowlyVarying,
    PlayerReactive,
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
    pub min_delta: Option<f64>,
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

impl ContractFamily {
    pub fn required_invariants(&self) -> Vec<String> {
        self.required_invariants.clone()
    }

    pub fn required_metrics(&self) -> Vec<String> {
        self.required_metrics.clone()
    }
}

/// Cross-metric interaction rule (XMIT).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionRule {
    pub id: String,
    pub source_metric: String,
    pub target_metric: String,
    pub effect_type: EffectType,
    pub condition: InteractionCondition,
    pub description: String,
}

/// Effect type for interaction rules.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectType {
    Amplify,
    Suppress,
    Gate,
}

/// Condition for applying an interaction rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionCondition {
    pub source_threshold: Option<f64>,
    pub source_max: Option<f64>,
    #[serde(default)]
    pub archetypes: Vec<String>,
    #[serde(default)]
    pub tile_classes: Vec<String>,
}

/// Safe default bands per tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierDefaultBands {
    pub by_tier: HashMap<Tier, DefaultBands>,
}

/// Conservative default bands for validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultBands {
    pub invariants: HashMap<String, NumericRange>,
    pub metrics: HashMap<String, NumericRange>,
}

/// Tier restriction for a contract family.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierRestriction {
    pub allowed: bool,
    pub notes: Option<String>,
}

/// Constellation tier classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tier {
    T1,
    T2,
    T3,
}

/// Authoring phase in the pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Phase {
    Schema0,
    Registry1,
    Bundles2,
    LuaPolicy3,
    Adapters4,
}

/// Compact profile for one object kind.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectKindProfile {
    pub object_kind: String,
    pub required_invariants: Vec<String>,
    pub required_metrics: Vec<String>,
    pub allowed_phases: Vec<Phase>,
}
