//! CLI subcommand: `hpc-chat-director plan`.
//!
//! Normalizes a natural-language prompt into a structured AiAuthoringRequest,
//! resolving schema refs, target paths, and manifest-derived defaults.

use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::model::request_types::{AiAuthoringRequest, RequestDefaults};
use crate::model::spine_types::{Phase, Tier};
use crate::errors::Error;

/// Result of planning: normalized request plus generation guidance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanResult {
    /// The normalized authoring request.
    pub request: AiAuthoringRequest,
    /// Optional guidance for generation (archetypes, ranges, pitfalls).
    #[serde(default)]
    pub generation_guide: Option<GenerationGuide>,
    /// Disambiguation candidates if intent was ambiguous.
    #[serde(default)]
    pub candidate_kinds: Vec<String>,
}

/// Guidance for AI generation, derived from spine and manifests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationGuide {
    /// Allowed invariant ranges for this objectKind/tier.
    pub invariant_ranges: std::collections::HashMap<String, serde_json::Value>,
    /// Allowed metric targets.
    pub metric_targets: std::collections::HashMap<String, serde_json::Value>,
    /// Suggested archetypes for this contract family.
    #[serde(default)]
    pub suggested_archetypes: Vec<String>,
    /// Common validation pitfalls to avoid.
    #[serde(default)]
    pub common_pitfalls: Vec<String>,
    /// Example IDs from the registry for referencedIds.
    #[serde(default)]
    pub example_referenced_ids: Vec<String>,
}

/// Run the plan subcommand.
pub fn run(
    prompt: &str,
    target_repo_override: Option<&str>,
    config: &crate::config::Config,
    spine: &crate::model::spine_types::SchemaSpine,
    manifests: &[crate::model::manifest_types::RepoManifest],
) -> Result<PlanResult, Error> {
    // 1. Parse intent and extract objectKind candidates
    let (object_kind, candidate_kinds) = extract_object_kind_from_prompt(prompt, spine)?;

    // 2. Resolve target repo and path from manifests
    let (target_repo, target_path, tier) = resolve_target(
        &object_kind,
        target_repo_override,
        manifests,
        config,
    )?;

    // 3. Determine phase from objectKind and tier (default to Bundles2 for v1)
    let phase = determine_default_phase(&object_kind, tier);

    // 4. Build request with defaults from spine
    let defaults = RequestDefaults::from_spine_and_manifests(
        &object_kind,
        tier,
        spine,
        manifests,
    );

    let request = AiAuthoringRequest {
        schema_ref: spine.schema_uri_for(&object_kind).unwrap_or_default(),
        intent: prompt.to_string(),
        object_kind: object_kind.clone(),
        candidate_kinds: candidate_kinds.clone(),
        target_repo,
        target_path,
        phase,
        tier,
        referenced_ids: Vec::new(), // AI to fill
        shci_bands: None, // AI to fill
        intended_invariants: defaults.invariant_defaults.clone(),
        intended_metrics: defaults.metric_defaults.clone(),
        extra_guidance: None,
        agent_profile_id: None, // Could be populated from env
    };

    // 5. Build generation guide if requested
    let generation_guide = if spine.describe_object_kind(&object_kind).is_some() {
        Some(GenerationGuide {
            invariant_ranges: spine.invariant_ranges_for(&object_kind, tier),
            metric_targets: spine.metric_targets_for(&object_kind, tier),
            suggested_archetypes: spine.suggested_archetypes_for(&object_kind),
            common_pitfalls: common_pitfalls_for(&object_kind),
            example_referenced_ids: Vec::new(), // Could query registry
        })
    } else {
        None
    };

    Ok(PlanResult {
        request,
        generation_guide,
        candidate_kinds,
    })
}

/// Extract objectKind from prompt; return candidates if ambiguous.
fn extract_object_kind_from_prompt(
    prompt: &str,
    spine: &crate::model::spine_types::SchemaSpine,
) -> Result<(String, Vec<String>), Error> {
    let prompt_lower = prompt.to_lowercase();
    
    // Simple keyword matching; real version would use NLP or intent classifier
    let candidates: Vec<String> = spine
        .contract_families
        .iter()
        .flat_map(|f| &f.kinds)
        .filter(|kind| {
            let kind_lower = kind.to_lowercase();
            prompt_lower.contains(&kind_lower) || 
            prompt_lower.contains(&kind_lower.replace("contract", ""))
        })
        .cloned()
        .collect();

    match candidates.len() {
        0 => Err(Error::Internal {
            message: format!("Could not determine objectKind from prompt: {}", prompt),
        }),
        1 => Ok((candidates[0].clone(), Vec::new())),
        _ => {
            // Return first as default, but include all as candidates
            Ok((candidates[0].clone(), candidates))
        }
    }
}

/// Resolve target repo, path, and tier from manifests.
fn resolve_target(
    object_kind: &str,
    repo_override: Option<&str>,
    manifests: &[crate::model::manifest_types::RepoManifest],
    config: &crate::config::Config,
) -> Result<(String, String, Tier), Error> {
    // Use override if provided
    if let Some(repo) = repo_override {
        if let Some(manifest) = manifests.iter().find(|m| m.repo == repo) {
            if manifest.allows_object_kind(object_kind) {
                let path = manifest.default_path_for(object_kind)
                    .unwrap_or("{id}.json")
                    .replace("{id}", "TODO_ID");
                return Ok((repo.to_string(), path, manifest.tier));
            }
        }
        return Err(Error::Config {
            message: format!("Repo '{}' does not accept objectKind '{}'", repo, object_kind),
            path: config.root.clone(),
        });
    }

    // Find first manifest that accepts this objectKind
    for manifest in manifests {
        if manifest.allows_object_kind(object_kind) {
            let path = manifest.default_path_for(object_kind)
                .unwrap_or("{id}.json")
                .replace("{id}", "TODO_ID");
            return Ok((manifest.repo.clone(), path, manifest.tier));
        }
    }

    Err(Error::Config {
        message: format!("No repo manifest accepts objectKind '{}'", object_kind),
        path: config.root.clone(),
    })
}

/// Determine default phase for objectKind and tier.
fn determine_default_phase(object_kind: &str, tier: Tier) -> Phase {
    // v1 defaults: all four contract families are Phase 2 (Bundles)
    // Phase 1 for registry entries only
    match tier {
        Tier::T1 => Phase::Bundles2, // Public contracts
        Tier::T2 => Phase::Bundles2, // Vault contracts
        Tier::T3 => Phase::Bundles2, // Lab contracts
    }
}

/// Return common validation pitfalls for an objectKind.
fn common_pitfalls_for(object_kind: &str) -> Vec<String> {
    match object_kind {
        "moodContract" => vec![
            "DET must be within tier-specific range (Tier 1: 0-7, Tier 2: 0-10)".into(),
            "ARR floor is 0.5 for spawn tileClass, 0.4 for battlefront".into(),
            "SHCI must reference valid invariant bundle from Black-Archivum".into(),
        ],
        "eventContract" => vec![
            "Stage modulation deltas must sum to zero across all stages".into(),
            "preconditions must reference valid invariant names".into(),
            "telemetryHooks cannot be empty for intensity_band >= 7".into(),
        ],
        "regionContractCard" => vec![
            "spatialGradients must define decay and center for each invariant".into(),
            "registryReady must be false for new regions".into(),
            "SHCI coupling must reference valid historical events".into(),
        ],
        "seedContractCard" => vec![
            "intensityBand must be 0-10; values >= 8 require deadledgerref".into(),
            "pacingTemplate must be one of: slow_burn, sudden_rupture, escalating".into(),
            "bundleref must point to valid Black-Archivum invariant bundle".into(),
        ],
        _ => Vec::new(),
    }
}
