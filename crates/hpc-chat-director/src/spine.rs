use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::Deserialize;

use crate::config::Config;
use crate::errors::ChatDirectorError;

#[derive(Debug, Clone, Deserialize)]
pub struct InvariantRange {
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TierOverride {
    pub tier: String,
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DriftSpec {
    pub allowed: bool,
    pub maxDeltaPerRelease: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InvariantSpec {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: String,
    pub class: String,
    pub range: InvariantRange,
    pub tierOverrides: Vec<TierOverride>,
    pub drift: DriftSpec,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DerivedMetricSpec {
    pub code: String,
    pub inputs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InteractionRule {
    pub ruleId: String,
    pub sourceMetric: String,
    pub targetMetric: String,
    pub relationship: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SafeBand {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SafeDefaultsBands {
    pub CIC: Option<SafeBand>,
    pub AOS: Option<SafeBand>,
    pub DET: Option<SafeBand>,
    pub UEC: Option<SafeBand>,
    pub ARR: Option<SafeBand>,
    pub SHCI: Option<SafeBand>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SafeDefaultsEntry {
    pub objectKind: String,
    pub tier: String,
    pub bands: SafeDefaultsBands,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InvariantsSpine {
    pub version: String,
    pub invariants: Vec<InvariantSpec>,
    pub derivedMetrics: Vec<DerivedMetricSpec>,
    pub interactionRules: Vec<InteractionRule>,
    pub safeDefaults: Vec<SafeDefaultsEntry>,
}

#[derive(Debug, Clone)]
pub struct ObjectKindProfile {
    pub object_kind: String,
    pub required_invariants: Vec<String>,
    pub allowed_metrics: Vec<String>,
    pub tiers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DefaultBands {
    pub cic: Option<SafeBand>,
    pub aos: Option<SafeBand>,
    pub det: Option<SafeBand>,
    pub uec: Option<SafeBand>,
    pub arr: Option<SafeBand>,
    pub shci: Option<SafeBand>,
}

#[derive(Debug, Clone)]
pub struct SuggestedRanges {
    pub object_kind: String,
    pub tier: String,
    pub cic: Option<SafeBand>,
    pub aos: Option<SafeBand>,
    pub det: Option<SafeBand>,
    pub uec: Option<SafeBand>,
    pub arr: Option<SafeBand>,
    pub shci: Option<SafeBand>,
}

#[derive(Debug, Clone)]
pub struct SpineIndex {
    invariants_spine: InvariantsSpine,
}

impl SpineIndex {
    pub fn load(config: &Config) -> Result<Self, ChatDirectorError> {
        let path = config
            .spine_root()
            .join("invariants-spine.v1.json");

        let mut file = File::open(&path)
            .map_err(|e| ChatDirectorError::Io(path.clone(), e))?;

        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|e| ChatDirectorError::Io(path.clone(), e))?;

        let invariants_spine: InvariantsSpine =
            serde_json::from_str(&buf).map_err(ChatDirectorError::InvalidSpine)?;

        Ok(SpineIndex { invariants_spine })
    }

    pub fn describe_object_kind(&self, object_kind: &str) -> ObjectKindProfile {
        // For v1, treat all invariants and tiers as available to every objectKind.
        let required_invariants = self
            .invariants_spine
            .invariants
            .iter()
            .map(|inv| inv.code.clone())
            .collect();

        let allowed_metrics = vec![
            "UEC".to_string(),
            "EMD".to_string(),
            "STCI".to_string(),
            "CDL".to_string(),
            "ARR".to_string(),
        ];

        let tiers = vec![
            "TIER1_PUBLIC".to_string(),
            "TIER2_MATURE".to_string(),
            "TIER3_RESEARCH".to_string(),
        ];

        ObjectKindProfile {
            object_kind: object_kind.to_string(),
            required_invariants,
            allowed_metrics,
            tiers,
        }
    }

    pub fn safe_defaults(
        &self,
        object_kind: &str,
        tier: &str,
    ) -> Option<DefaultBands> {
        let entry = self
            .invariants_spine
            .safeDefaults
            .iter()
            .find(|entry| entry.objectKind == object_kind && entry.tier == tier)?;

        Some(DefaultBands {
            cic: entry.bands.CIC.clone(),
            aos: entry.bands.AOS.clone(),
            det: entry.bands.DET.clone(),
            uec: entry.bands.UEC.clone(),
            arr: entry.bands.ARR.clone(),
            shci: entry.bands.SHCI.clone(),
        })
    }

    pub fn suggest_ranges(
        &self,
        object_kind: &str,
        tier: &str,
    ) -> SuggestedRanges {
        let defaults = self.safe_defaults(object_kind, tier);

        match defaults {
            Some(bands) => SuggestedRanges {
                object_kind: object_kind.to_string(),
                tier: tier.to_string(),
                cic: bands.cic,
                aos: bands.aos,
                det: bands.det,
                uec: bands.uec,
                arr: bands.arr,
                shci: bands.shci,
            },
            None => SuggestedRanges {
                object_kind: object_kind.to_string(),
                tier: tier.to_string(),
                cic: None,
                aos: None,
                det: None,
                uec: None,
                arr: None,
                shci: None,
            },
        }
    }

    pub fn interaction_rules(&self) -> &[InteractionRule] {
        &self.invariants_spine.interactionRules
    }

    pub fn invariant_specs(&self) -> &[InvariantSpec] {
        &self.invariants_spine.invariants
    }
}
