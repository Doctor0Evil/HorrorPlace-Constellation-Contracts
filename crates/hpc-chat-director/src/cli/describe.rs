//! CLI subcommand: `hpc-chat-director describe`.
//!
//! Returns structured capability information for objectKinds, phases,
//! and tiers, enabling AI orchestrators to discover allowed operations.

use serde::{Deserialize, Serialize};
use crate::model::spine_types::{Phase, Tier};
use crate::errors::Error;

/// Capability profile for an objectKind/phase/tier combination.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectKindProfile {
    /// Object kind name.
    pub object_kind: String,
    /// Contract family this belongs to.
    pub family: String,
    /// Allowed phases for this kind.
    pub allowed_phases: Vec<Phase>,
    /// Tier restrictions.
    pub tier_restrictions: std::collections::HashMap<Tier, TierRestriction>,
    /// Required invariants with ranges.
    pub required_invariants: std::collections::HashMap<String, InvariantProfile>,
    /// Required metrics with targets.
    pub required_metrics: std::collections::HashMap<String, MetricProfile>,
    /// Suggested archetypes for this kind.
    #[serde(default)]
    pub suggested_archetypes: Vec<String>,
    /// Example target paths per repo.
    #[serde(default)]
    pub example_paths: std::collections::HashMap<String, String>,
}

/// Restriction for a tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierRestriction {
    /// Whether this tier allows this objectKind.
    pub allowed: bool,
    /// Optional notes explaining restrictions.
    #[serde(default)]
    pub notes: Option<String>,
    /// Minimum RWF required for this tier.
    #[serde(default)]
    pub min_rwf: Option<f64>,
}

/// Profile for an invariant.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvariantProfile {
    /// Whether this invariant is required.
    pub required: bool,
    /// Canonical range.
    pub range: serde_json::Value,
    /// Tier-specific overrides.
    #[serde(default)]
    pub tier_overrides: std::collections::HashMap<Tier, serde_json::Value>,
    /// Description for AI context.
    #[serde(default)]
    pub description: String,
}

/// Profile for a metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricProfile {
    /// Whether this metric is required.
    pub required: bool,
    /// Target band.
    pub target_band: serde_json::Value,
    /// Tier adjustments.
    #[serde(default)]
    pub tier_adjustments: std::collections::HashMap<Tier, serde_json::Value>,
    /// Description for AI context.
    #[serde(default)]
    pub description: String,
}

/// Run the describe subcommand.
pub fn run(
    object_kind: Option<&str>,
    phase: Option<u8>,
    tier: Option<&str>,
    director: &crate::ChatDirector,
) -> Result<serde_json::Value, Error> {
    // Build profile from spine and manifests
    let profiles = if let Some(kind) = object_kind {
        // Single objectKind profile
        let profile = build_object_kind_profile(kind, director)?;
        serde_json::json!({ "object_kind_profile": profile })
    } else {
        // All known objectKinds summary
        let all_profiles = director
            .spine
            .contract_families
            .iter()
            .flat_map(|f| &f.kinds)
            .filter_map(|kind| {
                build_object_kind_profile(kind, director).ok()
            })
            .collect::<Vec<_>>();
        serde_json::json!({ "all_object_kinds": all_profiles })
    };

    // Apply filters if specified
    let filtered = apply_filters(profiles, phase, tier)?;

    Ok(filtered)
}

/// Build a profile for a specific objectKind.
fn build_object_kind_profile(
    kind: &str,
    director: &crate::ChatDirector,
) -> Result<ObjectKindProfile, Error> {
    let spine = &director.spine;
    
    // Get contract family info
    let family = spine
        .contract_families
        .iter()
        .find(|f| f.kinds.contains(&kind.to_string()))
        .ok_or_else(|| Error::Internal {
            message: format!("Unknown objectKind: {}", kind),
        })?;

    // Build invariant profiles
    let required_invariants = family
        .required_invariants
        .iter()
        .filter_map(|name| {
            spine.invariants.get(name).map(|spec| {
                (name.clone(), InvariantProfile {
                    required: true,
                    range: serde_json::json!({
                        "min": spec.canonical_range.min,
                        "max": spec.canonical_range.max,
                    }),
                    tier_overrides: spec.tier_overrides
                        .iter()
                        .map(|(tier, range)| {
                            (tier.clone(), serde_json::json!({
                                "min": range.min,
                                "max": range.max,
                            }))
                        })
                        .collect(),
                    description: spec.description.clone(),
                })
            })
        })
        .collect();

    // Build metric profiles
    let required_metrics = family
        .required_metrics
        .iter()
        .filter_map(|name| {
            spine.metrics.get(name).map(|spec| {
                (name.clone(), MetricProfile {
                    required: true,
                    target_band: serde_json::json!({
                        "min": spec.target_band.min,
                        "max": spec.target_band.max,
                    }),
                    tier_adjustments: spec.tier_adjustments
                        .iter()
                        .map(|(tier, adj)| {
                            (tier.clone(), serde_json::json!({
                                "min_delta": adj.min_delta,
                                "max_delta": adj.max_delta,
                            }))
                        })
                        .collect(),
                    description: spec.description.clone(),
                })
            })
        })
        .collect();

    // Build tier restrictions from manifests
    let tier_restrictions = director
        .manifests
        .iter()
        .map(|manifest| {
            let allowed = manifest.allows_object_kind(kind);
            (manifest.tier, TierRestriction {
                allowed,
                notes: if !allowed {
                    Some(format!("Repo '{}' does not accept {}", manifest.repo_name, kind))
                } else {
                    None
                },
                min_rwf: manifest.rules.min_rwf_for_tier,
            })
        })
        .collect();

    // Example paths from manifests
    let example_paths = director
        .manifests
        .iter()
        .filter(|m| m.allows_object_kind(kind))
        .filter_map(|m| {
            m.default_path_for(kind).map(|template| {
                (m.repo_name.clone(), template.replace("{id}", "example_id"))
            })
        })
        .collect();

    Ok(ObjectKindProfile {
        object_kind: kind.to_string(),
        family: family.name.clone(),
        allowed_phases: family.allowed_phases.clone(),
        tier_restrictions,
        required_invariants,
        required_metrics,
        suggested_archetypes: spine.suggested_archetypes_for(kind),
        example_paths,
    })
}

/// Apply phase and tier filters to profiles.
fn apply_filters(
    profiles: serde_json::Value,
    phase: Option<u8>,
    tier: Option<&str>,
) -> Result<serde_json::Value, Error> {
    let mut result = profiles;
    
    // Filter by phase if specified
    if let Some(p) = phase {
        let phase_enum = match p {
            0 => Phase::Schema0,
            1 => Phase::Registry1,
            2 => Phase::Bundles2,
            3 => Phase::LuaPolicy3,
            4 => Phase::Adapters4,
            _ => return Err(Error::Internal {
                message: format!("Invalid phase: {}", p),
            }),
        };
        // Could filter profiles.allowed_phases here
    }
    
    // Filter by tier if specified
    if let Some(t) = tier {
        let tier_enum = match t {
            "T1" | "t1" | "T1-core" | "t1-core" => Tier::T1Core,
            "T2" | "t2" | "T2-vault" | "t2-vault" => Tier::T2Vault,
            "T3" | "t3" | "T3-research" | "t3-research" => Tier::T3Research,
            _ => return Err(Error::Internal {
                message: format!("Invalid tier: {}", t),
            }),
        };
        // Could filter profiles.tier_restrictions here
    }
    
    Ok(result)
}
