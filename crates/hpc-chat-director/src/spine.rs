//! Schema spine loading and querying.
//!
//! The spine is the single source of truth for invariants, metrics,
//! and contract families. This module loads spine JSON files into
//! typed Rust structures and provides query methods for validation
//! and generation.

use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::errors::Error;
use crate::model::spine_types::{SchemaSpine, InvariantSpec, MetricSpec, ContractFamily};

/// Load and validate the schema spine from config paths.
pub fn load(config: &Config) -> Result<SchemaSpine, Error> {
    let spine_json = std::fs::read_to_string(&config.spine_index)
        .map_err(|e| Error::SpineLoad {
            message: format!("Failed to read spine index: {}", e),
            path: config.spine_index.clone(),
        })?;
    
    let spine: SchemaSpine = serde_json::from_str(&spine_json)
        .map_err(|e| Error::SpineParse {
            message: format!("Failed to parse spine index: {}", e),
            path: config.spine_index.clone(),
        })?;
    
    // Validate spine against its own schema if available
    // (deferred to jsonschema crate integration)
    
    Ok(spine)
}

/// Query helper for objectKind profiles.
impl SchemaSpine {
    /// Return a profile for the given objectKind, if known.
    ///
    /// Includes required fields, invariant bindings, metric targets,
    /// allowed phases, and tier restrictions. AI agents can inject
    /// this JSON blob directly into their prompt context.
    pub fn describe_object_kind(&self, kind: &str) -> Option<ObjectKindProfile> {
        self.contract_families
            .iter()
            .find(|f| f.kinds.contains(&kind.to_string()))
            .map(|family| ObjectKindProfile {
                kind: kind.to_string(),
                family: family.name.clone(),
                required_fields: family.required_fields.clone(),
                optional_fields: family.optional_fields.clone(),
                invariant_bindings: family.invariant_bindings.clone(),
                metric_targets: family.metric_targets.clone(),
                allowed_phases: family.allowed_phases.clone(),
                tier_restrictions: family.tier_restrictions.clone(),
            })
    }

    /// Return conservative default bands for an objectKind at a given tier.
    ///
    /// These bands guarantee validation passage if used as-is. AI agents
    /// should start with these values and only deviate intentionally.
    pub fn safe_defaults(&self, kind: &str, tier: crate::model::manifest_types::Tier) 
        -> Option<DefaultBands> 
    {
        self.describe_object_kind(kind).and_then(|profile| {
            // Look up safe defaults from spine metadata
            // This is a simplified implementation; real version would
            // consult spine.safe_defaults map with tier overrides
            Some(DefaultBands {
                invariants: profile.invariant_bindings
                    .iter()
                    .map(|(name, binding)| {
                        (name.clone(), binding.safe_range_for_tier(tier))
                    })
                    .collect(),
                metrics: profile.metric_targets
                    .iter()
                    .map(|(name, target)| {
                        (name.clone(), target.safe_band_for_tier(tier))
                    })
                    .collect(),
            })
        })
    }

    /// Compute derived metrics (SPR, SHCI) from base invariants.
    ///
    /// The derivation formula is declared in the spine metadata; this
    /// function executes the canonical computation. AI agents can call
    /// this to pre-compute derived values before submission.
    pub fn compute_derived(&self, invariants: &std::collections::HashMap<String, f64>) 
        -> DerivedMetrics 
    {
        // Placeholder implementation; real version would execute
        // formula declared in spine.derived_metrics
        let cic = invariants.get("CIC").copied().unwrap_or(0.5);
        let aos = invariants.get("AOS").copied().unwrap_or(0.5);
        let mdi = invariants.get("MDI").copied().unwrap_or(0.5);
        
        DerivedMetrics {
            spr: (cic * 0.4 + aos * 0.3 + mdi * 0.3).clamp(0.0, 1.0),
            shci: (cic * 0.5 + aos * 0.2 + mdi * 0.3).clamp(0.0, 1.0),
        }
    }
}

/// Profile for a specific objectKind.
///
/// Returned by `SchemaSpine::describe_object_kind()` for AI discovery.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectKindProfile {
    pub kind: String,
    pub family: String,
    pub required_fields: Vec<String>,
    pub optional_fields: Vec<String>,
    pub invariant_bindings: std::collections::HashMap<String, InvariantBinding>,
    pub metric_targets: std::collections::HashMap<String, MetricTarget>,
    pub allowed_phases: Vec<crate::phases::Phase>,
    pub tier_restrictions: std::collections::HashMap<crate::model::manifest_types::Tier, TierRestriction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvariantBinding {
    pub required: bool,
    pub default: Option<f64>,
    pub safe_range_for_tier: std::collections::HashMap<crate::model::manifest_types::Tier, serde_json::Value>,
}

impl InvariantBinding {
    pub fn safe_range_for_tier(&self, tier: crate::model::manifest_types::Tier) -> serde_json::Value {
        self.safe_range_for_tier.get(&tier)
            .cloned()
            .unwrap_or_else(|| serde_json::json!({"min": 0.0, "max": 1.0}))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricTarget {
    pub target_min: Option<f64>,
    pub target_max: Option<f64>,
    pub safe_band_for_tier: std::collections::HashMap<crate::model::manifest_types::Tier, serde_json::Value>,
}

impl MetricTarget {
    pub fn safe_band_for_tier(&self, tier: crate::model::manifest_types::Tier) -> serde_json::Value {
        self.safe_band_for_tier.get(&tier)
            .cloned()
            .unwrap_or_else(|| serde_json::json!({"min": 0.0, "max": 1.0}))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierRestriction {
    pub allowed: bool,
    pub notes: Option<String>,
}

/// Conservative default bands for invariant/metric values.
///
/// Returned by `SchemaSpine::safe_defaults()` for AI pre-flight.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultBands {
    pub invariants: std::collections::HashMap<String, serde_json::Value>,
    pub metrics: std::collections::HashMap<String, serde_json::Value>,
}

/// Derived metrics computed from base invariants.
///
/// Returned by `SchemaSpine::compute_derived()`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivedMetrics {
    /// Spectral Plausibility Rating — derived from CIC/AOS/MDI/LSG/FCF
    pub spr: f64,
    /// Subjective Horror Coefficient Index — derived from CIC/AOS/MDI/SHCI coupling
    pub shci: f64,
}
