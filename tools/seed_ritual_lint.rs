use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::Value;

/// Severity of a lint finding.
#[derive(Debug)]
enum Severity {
    Warning,
    Error,
}

/// Simple lint finding structure.
#[derive(Debug)]
struct Finding {
    severity: Severity,
    code: &'static str,
    message: String,
    ritual_id: String,
}

/// SeedContractV1 subset needed for linting.
/// This should mirror the core fields from schemas/seedcontractv1.json
/// but stays minimal on purpose.
#[derive(Debug, Deserialize)]
struct SeedContract {
    #[serde(rename = "seedid")]
    seed_id: String,
    #[serde(rename = "stage")]
    stage: String,
    #[serde(rename = "metrictargets")]
    metric_targets: MetricTargets,
}

#[derive(Debug, Deserialize)]
struct MetricTargets {
    #[serde(rename = "UECdelta")]
    uec_delta: f64,
    #[serde(rename = "EMDdelta")]
    emd_delta: f64,
    #[serde(rename = "STCIdelta")]
    stci_delta: f64,
    #[serde(rename = "CDLdelta")]
    cdl_delta: f64,
    #[serde(rename = "ARRmin")]
    arr_min: f64,
    #[serde(rename = "ARRmax")]
    arr_max: f64,
}

/// Seed-Ritual contract v1 subset for linting.
/// Mirrors schemas/seed-ritual-contract-v1.json but only includes fields
/// needed for cross-checks and governance.
#[derive(Debug, Deserialize)]
struct SeedRitual {
    #[serde(rename = "ritualId")]
    ritual_id: String,
    version: String,
    #[serde(rename = "consentTier")]
    consent_tier: String,
    #[serde(rename = "gxiCap")]
    gxi_cap: f64,
    #[serde(rename = "explicitViolenceForbidden")]
    explicit_violence_forbidden: bool,
    #[serde(rename = "stageBindings")]
    stage_bindings: Vec<StageBinding>,
    #[serde(rename = "stageMetricBands")]
    stage_metric_bands: StageMetricBands,
    #[serde(rename = "stageIntensityEnvelopes")]
    stage_intensity_envelopes: StageIntensityEnvelopes,
}

#[derive(Debug, Deserialize)]
struct StageBinding {
    stage: String,
    #[serde(rename = "seedId")]
    seed_id: String,
    #[serde(default)]
    weight: Option<f64>,
    #[serde(default)]
    priority: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct StageMetricBands {
    probe: StageMetricBand,
    evidence: StageMetricBand,
    confrontation: StageMetricBand,
    aftermath: StageMetricBand,
    residual: StageMetricBand,
}

#[derive(Debug, Deserialize)]
struct StageMetricBand {
    #[serde(rename = "UEC_min")]
    uec_min: f64,
    #[serde(rename = "EMD_min")]
    emd_min: f64,
    #[serde(rename = "STCI_min")]
    stci_min: f64,
    #[serde(rename = "CDL_min")]
    cdl_min: f64,
    #[serde(rename = "ARR_min")]
    arr_min: f64,
    #[serde(rename = "ARR_max")]
    arr_max: f64,
}

#[derive(Debug, Deserialize)]
struct StageIntensityEnvelopes {
    probe: StageIntensityBand,
    evidence: StageIntensityBand,
    confrontation: StageIntensityBand,
    aftermath: StageIntensityBand,
    residual: StageIntensityBand,
}

#[derive(Debug, Deserialize)]
struct StageIntensityBand {
    #[serde(rename = "audioIntensity")]
    audio_intensity: MinMax,
    #[serde(rename = "visualIntensity")]
    visual_intensity: MinMax,
    #[serde(default)]
    #[serde(rename = "hapticIntensity")]
    haptic_intensity: Option<MinMax>,
    #[serde(default)]
    #[serde(rename = "bciIntensityHint")]
    bci_intensity_hint: Option<MinMax>,
}

#[derive(Debug, Deserialize)]
struct MinMax {
    min: f64,
    max: f64,
}

/// Simple policy structure for consent tiers and GXI caps.
/// In a full implementation this would be loaded from a governance profile.
struct ConsentPolicy {
    gxi_min: f64,
    gxi_max: f64,
    require_explicit_forbidden: bool,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: seed_ritual_lint <rituals_dir> <seeds_dir>\n\
             - rituals_dir: directory containing Seed-Ritual JSON files\n\
             - seeds_dir: directory containing SeedContractV1 JSON files"
        );
        std::process::exit(1);
    }

    let rituals_dir = Path::new(&args[1]);
    let seeds_dir = Path::new(&args[2]);

    let seed_index = match load_seeds(seeds_dir) {
        Ok(index) => index,
        Err(err) => {
            eprintln!("ERROR: failed to load seeds: {err}");
            std::process::exit(1);
        }
    };

    let mut all_findings: Vec<Finding> = Vec::new();

    match walk_dir_json(rituals_dir) {
        Ok(files) => {
            for path in files {
                match lint_ritual_file(&path, &seed_index) {
                    Ok(findings) => all_findings.extend(findings),
                    Err(err) => {
                        eprintln!("ERROR: failed to lint ritual file {:?}: {err}", path);
                        std::process::exit(1);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("ERROR: failed to enumerate rituals: {err}");
            std::process::exit(1);
        }
    }

    let mut error_count = 0usize;
    for f in &all_findings {
        match f.severity {
            Severity::Warning => {
                eprintln!(
                    "WARN [{}] ritual={} {}",
                    f.code, f.ritual_id, f.message
                );
            }
            Severity::Error => {
                eprintln!(
                    "ERROR [{}] ritual={} {}",
                    f.code, f.ritual_id, f.message
                );
                error_count += 1;
            }
        }
    }

    if error_count > 0 {
        eprintln!(
            "seed_ritual_lint: {} error(s) detected, failing CI.",
            error_count
        );
        std::process::exit(1);
    } else {
        println!("seed_ritual_lint: all rituals passed lint checks.");
    }
}

/// Load all SeedContractV1 JSON files under a directory into an index keyed by seedId.
fn load_seeds(dir: &Path) -> Result<HashMap<String, SeedContract>, String> {
    let mut index = HashMap::new();
    let files = walk_dir_json(dir)?;
    for path in files {
        let data = fs::read_to_string(&path)
            .map_err(|e| format!("failed to read seed {:?}: {e}", path))?;
        let value: Value = serde_json::from_str(&data)
            .map_err(|e| format!("failed to parse seed {:?} as JSON: {e}", path))?;

        // Quick check: ensure this looks like a SeedContractV1 by presence of key fields.
        if !value.get("seedid").is_some() {
            continue;
        }

        let seed: SeedContract = serde_json::from_value(value)
            .map_err(|e| format!("failed to deserialize seed {:?}: {e}", path))?;
        index.insert(seed.seed_id.clone(), seed);
    }
    Ok(index)
}

/// Collect all .json files under a directory (non-recursive or recursive as needed).
fn walk_dir_json(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    if !root.exists() {
        return Ok(files);
    }
    for entry in fs::read_dir(root)
        .map_err(|e| format!("failed to read dir {:?}: {e}", root))?
    {
        let entry = entry.map_err(|e| format!("failed to read dir entry: {e}"))?;
        let path = entry.path();
        if path.is_dir() {
            let mut nested = walk_dir_json(&path)?;
            files.append(&mut nested);
        } else if let Some(ext) = path.extension() {
            if ext == "json" {
                files.push(path);
            }
        }
    }
    Ok(files)
}

/// Lint a single Seed-Ritual file against Seeds and simple governance policies.
fn lint_ritual_file(
    path: &Path,
    seed_index: &HashMap<String, SeedContract>,
) -> Result<Vec<Finding>, String> {
    let mut findings = Vec::new();

    let data = fs::read_to_string(path)
        .map_err(|e| format!("failed to read ritual {:?}: {e}", path))?;
    let value: Value = serde_json::from_str(&data)
        .map_err(|e| format!("failed to parse ritual {:?} as JSON: {e}", path))?;

    // Quick recognition check.
    if !value.get("ritualId").is_some() {
        return Ok(findings);
    }

    let ritual: SeedRitual = serde_json::from_value(value)
        .map_err(|e| format!("failed to deserialize ritual {:?}: {e}", path))?;
    let ritual_id = ritual.ritual_id.clone();

    // Consent/GXI policy checks.
    let policy = consent_policy_for_tier(&ritual.consent_tier);
    findings.extend(check_gxi_and_consent(&ritual, &policy));

    // Stage → Seed mapping checks.
    findings.extend(check_stage_seed_compatibility(
        &ritual,
        seed_index,
    ));

    // Cross-stage intensity continuity checks.
    findings.extend(check_intensity_continuity(&ritual));

    // Stage-level metric band sanity checks.
    findings.extend(check_stage_metric_bands(&ritual));

    // Attach ritualId to all findings.
    for f in &mut findings {
        let Finding { ritual_id: ref mut rid, .. } = f;
        *rid = ritual_id.clone();
    }

    Ok(findings)
}

fn consent_policy_for_tier(tier: &str) -> ConsentPolicy {
    match tier {
        "Tier1Public" => ConsentPolicy {
            gxi_min: 0.0,
            gxi_max: 3.0,
            require_explicit_forbidden: true,
        },
        "Tier2Internal" => ConsentPolicy {
            gxi_min: 0.0,
            gxi_max: 6.0,
            require_explicit_forbidden: true,
        },
        "Tier3Vault" => ConsentPolicy {
            gxi_min: 0.0,
            gxi_max: 10.0,
            require_explicit_forbidden: false,
        },
        _ => ConsentPolicy {
            gxi_min: 0.0,
            gxi_max: 10.0,
            require_explicit_forbidden: true,
        },
    }
}

/// Check that gxiCap and explicitViolenceForbidden agree with consent-tier policy.
fn check_gxi_and_consent(ritual: &SeedRitual, policy: &ConsentPolicy) -> Vec<Finding> {
    let mut findings = Vec::new();

    if ritual.gxi_cap < policy.gxi_min || ritual.gxi_cap > policy.gxi_max {
        findings.push(Finding {
            severity: Severity::Error,
            code: "GXI_CAP_OUT_OF_POLICY",
            message: format!(
                "gxiCap={} is outside policy [{}, {}] for consentTier={}",
                ritual.gxi_cap, policy.gxi_min, policy.gxi_max, ritual.consent_tier
            ),
            ritual_id: String::new(),
        });
    }

    if policy.require_explicit_forbidden && !ritual.explicit_violence_forbidden {
        findings.push(Finding {
            severity: Severity::Error,
            code: "EXPLICIT_FORBIDDEN_REQUIRED",
            message: format!(
                "explicitViolenceForbidden=false but consentTier={} requires it to be true",
                ritual.consent_tier
            ),
            ritual_id: String::new(),
        });
    }

    findings
}

/// Check that Seeds exist, roughly match stage semantics, and can plausibly meet metric floors.
fn check_stage_seed_compatibility(
    ritual: &SeedRitual,
    seed_index: &HashMap<String, SeedContract>,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    for binding in &ritual.stage_bindings {
        let mut stage_findings: Vec<Finding> = Vec::new();

        let seed = match seed_index.get(&binding.seed_id) {
            Some(s) => s,
            None => {
                stage_findings.push(Finding {
                    severity: Severity::Error,
                    code: "SEED_NOT_FOUND",
                    message: format!(
                        "stage={} references missing seedId={}",
                        binding.stage, binding.seed_id
                    ),
                    ritual_id: String::new(),
                });
                findings.extend(stage_findings);
                continue;
            }
        };

        // Simple stage-name compatibility check (string prefix-based).
        if !seed.stage.to_lowercase().contains(&binding.stage.to_lowercase()) {
            stage_findings.push(Finding {
                severity: Severity::Warning,
                code: "STAGE_NAME_MISMATCH",
                message: format!(
                    "ritual stage={} bound to seedId={} with seed.stage='{}' (check mapping policy)",
                    binding.stage, binding.seed_id, seed.stage
                ),
                ritual_id: String::new(),
            });
        }

        // Metric floor plausibility: if ritual expects a positive push but seed delta is <= 0, warn.
        let band = match binding.stage.as_str() {
            "probe" => &ritual.stage_metric_bands.probe,
            "evidence" => &ritual.stage_metric_bands.evidence,
            "confrontation" => &ritual.stage_metric_bands.confrontation,
            "aftermath" => &ritual.stage_metric_bands.aftermath,
            "residual" => &ritual.stage_metric_bands.residual,
            _ => {
                stage_findings.push(Finding {
                    severity: Severity::Warning,
                    code: "UNKNOWN_STAGE",
                    message: format!(
                        "ritual stage='{}' is not in the canonical five-stage set",
                        binding.stage
                    ),
                    ritual_id: String::new(),
                });
                findings.extend(stage_findings);
                continue;
            }
        };

        if band.uec_min > 0.0 && seed.metric_targets.uec_delta <= 0.0 {
            stage_findings.push(Finding {
                severity: Severity::Warning,
                code: "UNDERPOWERED_UEC",
                message: format!(
                    "stage={} has UEC_min={} but seedId={} has UECdelta={}",
                    binding.stage, band.uec_min, binding.seed_id, seed.metric_targets.uec_delta
                ),
                ritual_id: String::new(),
            });
        }

        if band.emd_min > 0.0 && seed.metric_targets.emd_delta <= 0.0 {
            stage_findings.push(Finding {
                severity: Severity::Warning,
                code: "UNDERPOWERED_EMD",
                message: format!(
                    "stage={} has EMD_min={} but seedId={} has EMDdelta={}",
                    binding.stage, band.emd_min, binding.seed_id, seed.metric_targets.emd_delta
                ),
                ritual_id: String::new(),
            });
        }

        if band.stci_min > 0.0 && seed.metric_targets.stci_delta <= 0.0 {
            stage_findings.push(Finding {
                severity: Severity::Warning,
                code: "UNDERPOWERED_STCI",
                message: format!(
                    "stage={} has STCI_min={} but seedId={} has STCIdelta={}",
                    binding.stage, band.stci_min, binding.seed_id, seed.metric_targets.stci_delta
                ),
                ritual_id: String::new(),
            });
        }

        if band.cdl_min > 0.0 && seed.metric_targets.cdl_delta <= 0.0 {
            stage_findings.push(Finding {
                severity: Severity::Warning,
                code: "UNDERPOWERED_CDL",
                message: format!(
                    "stage={} has CDL_min={} but seedId={} has CDLdelta={}",
                    binding.stage, band.cdl_min, binding.seed_id, seed.metric_targets.cdl_delta
                ),
                ritual_id: String::new(),
            });
        }

        // ARR band compatibility check.
        if seed.metric_targets.arr_min > band.arr_max
            || seed.metric_targets.arr_max < band.arr_min
        {
            stage_findings.push(Finding {
                severity: Severity::Error,
                code: "ARR_BAND_INCOMPATIBLE",
                message: format!(
                    "stage={} ritual ARR[{},{}] incompatible with seedId={} ARR[{},{}]",
                    binding.stage,
                    band.arr_min,
                    band.arr_max,
                    binding.seed_id,
                    seed.metric_targets.arr_min,
                    seed.metric_targets.arr_max
                ),
                ritual_id: String::new(),
            });
        }

        findings.extend(stage_findings);
    }

    findings
}

/// Check monotone / staircase continuity of intensity bands across stages.
fn check_intensity_continuity(ritual: &SeedRitual) -> Vec<Finding> {
    let mut findings = Vec::new();

    let order = [
        ("probe", &ritual.stage_intensity_envelopes.probe),
        ("evidence", &ritual.stage_intensity_envelopes.evidence),
        ("confrontation", &ritual.stage_intensity_envelopes.confrontation),
        ("aftermath", &ritual.stage_intensity_envelopes.aftermath),
        ("residual", &ritual.stage_intensity_envelopes.residual),
    ];

    // Rising shape into confrontation for audio/visual max, allowance for drop afterward.
    for w in order.windows(2) {
        let (s1_name, s1) = w[0];
        let (s2_name, s2) = w[1];

        // Only enforce strict non-decrease up through confrontation.
        let enforce_monotone = s1_name != "confrontation" && s2_name != "aftermath";

        if enforce_monotone {
            if s2.audio_intensity.max + 1e-6 < s1.audio_intensity.max {
                findings.push(Finding {
                    severity: Severity::Warning,
                    code: "AUDIO_INTENSITY_NON_MONOTONE",
                    message: format!(
                        "audioIntensity.max decreases from stage={} ({}) to stage={} ({})",
                        s1_name,
                        s1.audio_intensity.max,
                        s2_name,
                        s2.audio_intensity.max
                    ),
                    ritual_id: String::new(),
                });
            }

            if s2.visual_intensity.max + 1e-6 < s1.visual_intensity.max {
                findings.push(Finding {
                    severity: Severity::Warning,
                    code: "VISUAL_INTENSITY_NON_MONOTONE",
                    message: format!(
                        "visualIntensity.max decreases from stage={} ({}) to stage={} ({})",
                        s1_name,
                        s1.visual_intensity.max,
                        s2_name,
                        s2.visual_intensity.max
                    ),
                    ritual_id: String::new(),
                });
            }
        }
    }

    findings
}

/// Check that stage metric bands are sane per stage (min <= max, ARR band non-empty, etc.).
fn check_stage_metric_bands(ritual: &SeedRitual) -> Vec<Finding> {
    let mut findings = Vec::new();

    let stages = [
        ("probe", &ritual.stage_metric_bands.probe),
        ("evidence", &ritual.stage_metric_bands.evidence),
        ("confrontation", &ritual.stage_metric_bands.confrontation),
        ("aftermath", &ritual.stage_metric_bands.aftermath),
        ("residual", &ritual.stage_metric_bands.residual),
    ];

    for (name, band) in stages {
        if band.arr_min > band.arr_max {
            findings.push(Finding {
                severity: Severity::Error,
                code: "ARR_BAND_INVERTED",
                message: format!(
                    "stage={} has ARR_min={} > ARR_max={}",
                    name, band.arr_min, band.arr_max
                ),
                ritual_id: String::new(),
            });
        }
        for (metric_name, value) in &[
            ("UEC_min", band.uec_min),
            ("EMD_min", band.emd_min),
            ("STCI_min", band.stci_min),
            ("CDL_min", band.cdl_min),
        ] {
            if *value < 0.0 || *value > 1.0 {
                findings.push(Finding {
                    severity: Severity::Error,
                    code: "METRIC_MIN_OUT_OF_RANGE",
                    message: format!(
                        "stage={} has {}={} outside [0,1]",
                        name, metric_name, value
                    ),
                    ritual_id: String::new(),
                });
            }
        }
    }

    findings
}
