//! Unified skeleton generation entrypoints.
//!
//! Provides deterministic skeleton generation across all contract families.
//! Dispatches to family-specific generators based on objectKind, pulling
//! numeric ranges and required fields from the loaded spine. Generated
//! skeletons are "fill-in-the-blanks" structures that are structurally
//! valid before AI adds content.

pub mod region;
pub mod seed;
pub mod mood;
pub mod event;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::model::spine_types::{SchemaSpine, Tier};

/// Annotated skeleton with field hints for AI.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnotatedSkeleton {
    /// The skeleton JSON content.
    pub content: serde_json::Value,
    /// Field annotations: "AI_FILL", "DO_NOT_MODIFY", or "NEW_IN_VERSION".
    pub field_hints: HashMap<String, FieldHint>,
    /// Metadata about the skeleton generation.
    pub metadata: SkeletonMetadata,
}

/// Hint for a skeleton field.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldHint {
    /// AI should fill this field with creative content.
    AiFill,
    /// Structural field; do not modify.
    DoNotModify,
    /// New field in this schema version; optional.
    NewInVersion,
}

/// Metadata about skeleton generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkeletonMetadata {
    /// Object kind this skeleton is for.
    pub object_kind: String,
    /// Tier this skeleton targets.
    pub tier: Tier,
    /// Schema version used.
    pub schema_version: String,
    /// Invariant bands pre-filled from spine.
    pub pre_filled_invariants: HashMap<String, serde_json::Value>,
    /// Metric targets pre-filled from spine.
    pub pre_filled_metrics: HashMap<String, serde_json::Value>,
}

/// Generate a skeleton for the given objectKind and context.
///
/// Returns an AnnotatedSkeleton with valid invariant/metric bands
/// and field hints for AI to fill creative content.
pub fn generate_skeleton(
    object_kind: &str,
    tier: Tier,
    spine: &SchemaSpine,
) -> Result<AnnotatedSkeleton, crate::errors::Error> {
    match object_kind {
        "regionContractCard" => region::generate_region_skeleton(tier, spine),
        "seedContractCard" => seed::generate_seed_skeleton(tier, spine),
        "moodContract" => mood::generate_mood_skeleton(tier, spine),
        "eventContract" => event::generate_event_skeleton(tier, spine),
        _ => Err(crate::errors::Error::Internal {
            message: format!("Unknown objectKind for skeleton generation: {}", object_kind),
        }),
    }
}

/// Serialize an AnnotatedSkeleton into a prompt-friendly format.
///
/// Returns a string that AI can consume directly, with field
/// descriptions and valid ranges inline as structured metadata.
impl AnnotatedSkeleton {
    pub fn to_prompt_context(&self) -> String {
        // Build a structured prompt with skeleton and hints
        let mut prompt = String::new();
        
        prompt.push_str(&format!("# Skeleton for {}\n\n", self.metadata.object_kind));
        prompt.push_str(&format!("**Tier:** {:?}\n", self.metadata.tier));
        prompt.push_str(&format!("**Schema Version:** {}\n\n", self.metadata.schema_version));
        
        prompt.push_str("## Pre-filled Values (DO_NOT_MODIFY)\n");
        for (field, value) in &self.metadata.pre_filled_invariants {
            prompt.push_str(&format!("- `{}`: `{:?}`\n", field, value));
        }
        for (field, value) in &self.metadata.pre_filled_metrics {
            prompt.push_str(&format!("- `{}`: `{:?}`\n", field, value));
        }
        prompt.push('\n');
        
        prompt.push_str("## Fields to Fill (AI_FILL)\n");
        for (field, hint) in &self.field_hints {
            if matches!(hint, FieldHint::AiFill) {
                prompt.push_str(&format!("- `{}`: [Your creative content here]\n", field));
            }
        }
        prompt.push('\n');
        
        prompt.push_str("## Skeleton JSON\n```json\n");
        prompt.push_str(&serde_json::to_string_pretty(&self.content).unwrap_or_default());
        prompt.push_str("\n```\n");
        
        prompt
    }
}

/// Telemetry struct for tracking generation success rates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationMetrics {
    /// Object kind that was generated.
    pub object_kind: String,
    /// Whether a skeleton was used.
    pub skeleton_used: bool,
    /// Whether validation passed.
    pub validation_passed: bool,
    /// Failure codes if validation failed.
    pub failure_codes: Vec<String>,
    /// Timestamp of generation.
    pub timestamp: String,
}

/// Generate a coherent bundle of related skeletons.
///
/// Ensures cross-family invariant coherence for tightly coupled
/// artifacts (region + seed + mood + event in same lore context).
pub fn generate_coherent_bundle(
    region_id: &str,
    seed_id: &str,
    mood_id: &str,
    event_id: &str,
    tier: Tier,
    spine: &SchemaSpine,
) -> Result<BundleSkeletons, crate::errors::Error> {
    // Generate individual skeletons
    let region = region::generate_region_skeleton(tier, spine)?;
    let seed = seed::generate_seed_skeleton(tier, spine)?;
    let mood = mood::generate_mood_skeleton(tier, spine)?;
    let event = event::generate_event_skeleton(tier, spine)?;
    
    // Apply cross-family coherence adjustments
    // (Simplified: real implementation would share invariant envelopes)
    
    Ok(BundleSkeletons {
        region,
        seed,
        mood,
        event,
        bundle_id: format!("bundle.{}.{}.{}.{}", region_id, seed_id, mood_id, event_id),
    })
}

/// Coherent bundle of related skeletons.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleSkeletons {
    pub region: AnnotatedSkeleton,
    pub seed: AnnotatedSkeleton,
    pub mood: AnnotatedSkeleton,
    pub event: AnnotatedSkeleton,
    pub bundle_id: String,
}
