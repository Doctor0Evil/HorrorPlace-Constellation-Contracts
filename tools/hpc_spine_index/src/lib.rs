// tools/hpc_spine_index/src/lib.rs
// Typed spine index for invariants + entertainment metrics.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvariantDef {
    pub code: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub schema_ref: &'static str,
    pub target_repo: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDef {
    pub code: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub schema_ref: &'static str,
    pub target_repo: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpineIndex {
    pub invariants: Vec<InvariantDef>,
    pub metrics: Vec<MetricDef>,
}

pub fn default_spine() -> SpineIndex {
    SpineIndex {
        invariants: vec![
            InvariantDef {
                code: "CIC",
                name: "Catastrophic Imprint Coefficient",
                description: "Strength of coupling to core atrocity.",
                schema_ref: "schema:Horror.Place/core/schemas/invariantsv1",
                target_repo: "Horror.Place",
            },
            InvariantDef {
                code: "MDI",
                name: "Mythic Density Index",
                description: "Concentration of folklore and legend.",
                schema_ref: "schema:Horror.Place/core/schemas/invariantsv1",
                target_repo: "Horror.Place",
            },
            InvariantDef {
                code: "AOS",
                name: "Archival Opacity Score",
                description: "Degree of archival gaps and contradictions.",
                schema_ref: "schema:Horror.Place/core/schemas/invariantsv1",
                target_repo: "Horror.Place",
            },
            InvariantDef {
                code: "RRM",
                name: "Ritual Residue Map",
                description: "Local strength of repeated rituals and drills.",
                schema_ref: "schema:Horror.Place/core/schemas/invariantsv1",
                target_repo: "Horror.Place",
            },
            InvariantDef {
                code: "FCF",
                name: "Folkloric Convergence Factor",
                description: "Alignment of multiple folklore strands.",
                schema_ref: "schema:Horror.Place/core/schemas/invariantsv1",
                target_repo: "Horror.Place",
            },
            InvariantDef {
                code: "SPR",
                name: "Spectral Plausibility Rating",
                description: "Derived plausibility of spectral phenomena.",
                schema_ref: "schema:Horror.Place/core/schemas/invariantsv1",
                target_repo: "Horror.Place",
            },
            InvariantDef {
                code: "RWF",
                name: "Reliability Weighting Factor",
                description: "Source credibility for local history.",
                schema_ref: "schema:Horror.Place/core/schemas/invariantsv1",
                target_repo: "Horror.Place",
            },
            InvariantDef {
                code: "DET",
                name: "Dread Exposure Threshold",
                description: "Local tolerance before enforced cooldown.",
                schema_ref: "schema:Horror.Place/core/schemas/invariantsv1",
                target_repo: "Horror.Place",
            },
            InvariantDef {
                code: "HVF",
                name: "Haunt Vector Field",
                description: "Magnitude/direction of spectral flow.",
                schema_ref: "schema:Horror.Place/core/schemas/invariantsv1",
                target_repo: "Horror.Place",
            },
            InvariantDef {
                code: "LSG",
                name: "Liminal Stress Gradient",
                description: "Stress gradient at thresholds and borders.",
                schema_ref: "schema:Horror.Place/core/schemas/invariantsv1",
                target_repo: "Horror.Place",
            },
            InvariantDef {
                code: "SHCI",
                name: "Spectral-History Coupling Index",
                description: "Constraint strength of entities to local history.",
                schema_ref: "schema:Horror.Place/core/schemas/invariantsv1",
                target_repo: "Horror.Place",
            },
        ],
        metrics: vec![
            MetricDef {
                code: "UEC",
                name: "Uncertainty Engagement Coefficient",
                description: "Engagement driven by uncertainty, not shock.",
                schema_ref: "schema:Horror.Place/core/schemas/entertainmentmetricsv1",
                target_repo: "Horror.Place",
            },
            MetricDef {
                code: "EMD",
                name: "Evidential Mystery Density",
                description: "Density of unresolved but grounded clues.",
                schema_ref: "schema:Horror.Place/core/schemas/entertainmentmetricsv1",
                target_repo: "Horror.Place",
            },
            MetricDef {
                code: "STCI",
                name: "Safe-Threat Contrast Index",
                description: "Contrast between safety and threat states.",
                schema_ref: "schema:Horror.Place/core/schemas/entertainmentmetricsv1",
                target_repo: "Horror.Place",
            },
            MetricDef {
                code: "CDL",
                name: "Cognitive Dissonance Load",
                description: "Mental strain from conflicting evidence.",
                schema_ref: "schema:Horror.Place/core/schemas/entertainmentmetricsv1",
                target_repo: "Horror.Place",
            },
            MetricDef {
                code: "ARR",
                name: "Ambiguous Resolution Ratio",
                description: "Fraction of arcs that end ambiguously.",
                schema_ref: "schema:Horror.Place/core/schemas/entertainmentmetricsv1",
                target_repo: "Horror.Place",
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spine_has_core_codes() {
        let s = default_spine();
        assert!(s.invariants.iter().any(|i| i.code == "CIC"));
        assert!(s.metrics.iter().any(|m| m.code == "UEC"));
    }
}
