//! Typed representations of AI authoring requests.
//!
//! Mirrors `ai-authoring-request-v1.json` schema for structured prompts.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::model::spine_types::{Phase, Tier};

/// Normalized authoring intent from AI-chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiAuthoringRequest {
    /// Canonical schema URI for this request type.
    #[serde(rename = "$schema")]
    pub schema_ref: String,
    /// High-level intent description.
    pub intent: String,
    /// Target object kind (e.g., "moodContract").
    pub object_kind: String,
    /// Candidate kinds if intent is ambiguous.
    #[serde(default)]
    pub candidate_kinds: Vec<String>,
    /// Target repository name.
    pub target_repo: String,
    /// Target filesystem path within repo.
    pub target_path: String,
    /// Authoring phase.
    pub phase: Phase,
    /// Tier classification.
    pub tier: Tier,
    /// Referenced entity IDs from other contracts.
    #[serde(default)]
    pub referenced_ids: Vec<String>,
    /// SHCI band constraints if applicable.
    #[serde(default)]
    pub shci_bands: Option<NumericBand>,
    /// Intended invariant values for pre-commitment.
    #[serde(default)]
    pub intended_invariants: HashMap<String, f64>,
    /// Intended metric targets for pre-commitment.
    #[serde(default)]
    pub intended_metrics: HashMap<String, f64>,
    /// Optional extra guidance for generation.
    #[serde(default)]
    pub extra_guidance: Option<String>,
    /// Agent profile ID for provenance.
    #[serde(default)]
    pub agent_profile_id: Option<String>,
}

/// Numeric band with min/max bounds.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumericBand {
    pub min: f64,
    pub max: f64,
}

/// Default values injected when AI omits optional fields.
#[derive(Debug, Clone)]
pub struct RequestDefaults {
    /// Source of each default value.
    pub sources: HashMap<String, DefaultSource>,
}

/// Source of a default value.
#[derive(Debug, Clone)]
pub enum DefaultSource {
    /// From spine safe_defaults.
    SpineSafeDefault,
    /// From tier policy.
    TierPolicy,
    /// From manifest authoring hint.
    ManifestHint,
    /// Fallback hardcoded value.
    Fallback,
}

impl RequestDefaults {
    /// Explain why each default was chosen.
    pub fn explain(&self) -> HashMap<String, String> {
        self.sources
            .iter()
            .map(|(field, source)| {
                let explanation = match source {
                    DefaultSource::SpineSafeDefault => {
                        format!("Default for '{}' from spine safe_defaults", field)
                    }
                    DefaultSource::TierPolicy => {
                        format!("Default for '{}' from tier policy", field)
                    }
                    DefaultSource::ManifestHint => {
                        format!("Default for '{}' from manifest hint", field)
                    }
                    DefaultSource::Fallback => {
                        format!("Fallback default for '{}'", field)
                    }
                };
                (field.clone(), explanation)
            })
            .collect()
    }
}
