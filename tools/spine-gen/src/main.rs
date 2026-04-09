use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// ---------------- Schema spine types ----------------

#[derive(Debug, Serialize, Deserialize)]
struct SchemaSpineIndex {
    id: String,
    version: String,
    schemas: Vec<SchemaEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SchemaEntry {
    schemaId: String,
    kind: String,
    spineCategory: String,
    consumers: Vec<SchemaConsumer>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SchemaConsumer {
    repo: String,
    paths: Vec<String>,
}

// ---------------- Invariants spine types ----------------

#[derive(Debug, Serialize, Deserialize)]
struct InvariantsSpine {
    id: String,
    version: String,
    invariants: Vec<InvariantEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InvariantEntry {
    code: String,
    name: String,
    description: String,
    range: MinMax,
    derivedFrom: Vec<String>,
    consumers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct MinMax {
    min: f64,
    max: f64,
}

// ---------------- Entertainment metrics spine types ----------------

#[derive(Debug, Serialize, Deserialize)]
struct MetricsSpine {
    id: String,
    version: String,
    metrics: Vec<MetricEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MetricEntry {
    code: String,
    name: String,
    description: String,
    range: MinMax,
    #[serde(default)]
    recommendedBands: Vec<RecommendedBand>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RecommendedBand {
    objectKind: String,
    tier: String,
    min: f64,
    max: f64,
}

// ---------------- Aggregation helpers ----------------

#[derive(Debug, Default, Clone, Copy)]
struct RangeAgg {
    min: f64,
    max: f64,
    count: u64,
}

impl RangeAgg {
    fn new() -> Self {
        RangeAgg {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            count: 0,
        }
    }

    fn update(&mut self, val_min: f64, val_max: f64) {
        if val_min < self.min {
            self.min = val_min;
        }
        if val_max > self.max {
            self.max = val_max;
        }
        self.count += 1;
    }

    fn to_minmax(self, default_min: f64, default_max: f64) -> MinMax {
        if self.count == 0 {
            MinMax {
                min: default_min,
                max: default_max,
            }
        } else {
            MinMax {
                min: self.min,
                max: self.max,
            }
        }
    }
}

// ---------------- Main ----------------

fn main() -> Result<()> {
    let repo_root = locate_repo_root()?;
    let schemas_dir = repo_root.join("schemas");

    // 1) Build schema spine index
    let schema_spine = build_schema_spine(&repo_root, &schemas_dir)?;
    write_pretty_json(
        &repo_root.join("schemas/spine/schema-spine-index-v1.json"),
        &schema_spine,
    )?;

    // 2) Scan contracts to build invariants + metrics spines
    // You can adjust or add roots here as needed.
    let contract_roots = vec![
        repo_root.join("contracts"),
        repo_root.join("docs"),
    ];

    let (invariant_ranges, metric_ranges) = scan_contract_roots(&repo_root, &contract_roots)?;

    let invariants_spine = build_invariants_spine(&invariant_ranges);
    write_pretty_json(
        &repo_root.join("schemas/spine/invariants-spine-v1.json"),
        &invariants_spine,
    )?;

    let metrics_spine = build_metrics_spine(&metric_ranges);
    write_pretty_json(
        &repo_root.join("schemas/spine/entertainment-metrics-spine-v1.json"),
        &metrics_spine,
    )?;

    println!("Spine generation complete.");
    Ok(())
}

// ---------------- Schema spine builder ----------------

fn build_schema_spine(repo_root: &Path, schemas_dir: &Path) -> Result<SchemaSpineIndex> {
    let mut entries: Vec<SchemaEntry> = Vec::new();

    for entry in WalkDir::new(schemas_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        if let Some(schema_entry) = process_schema_file(repo_root, path)? {
            entries.push(schema_entry);
        }
    }

    Ok(SchemaSpineIndex {
        id: "schema-spine-index.v1".to_string(),
        version: "1.0.0".to_string(),
        schemas: entries,
    })
}

fn process_schema_file(repo_root: &Path, path: &Path) -> Result<Option<SchemaEntry>> {
    let data = fs::read_to_string(path)?;
    let v: Value = serde_json::from_str(&data)
        .with_context(|| format!("Failed to parse JSON schema: {}", path.display()))?;

    let id = v
        .get("$id")
        .and_then(|x| x.as_str())
        .unwrap_or_else(|| path_to_schema_id(repo_root, path));

    let kind = infer_kind_from_path(path);
    let spine_category = infer_spine_category(&kind);

    let rel_path = path.strip_prefix(repo_root).unwrap_or(path).to_string_lossy();
    let consumer = SchemaConsumer {
        repo: "HorrorPlace-Constellation-Contracts".to_string(),
        paths: vec![rel_path.to_string()],
    };

    Ok(Some(SchemaEntry {
        schemaId: id.to_string(),
        kind,
        spineCategory: spine_category,
        consumers: vec![consumer],
    }))
}

fn path_to_schema_id(repo_root: &Path, path: &Path) -> &str {
    let rel = path.strip_prefix(repo_root).unwrap_or(path).to_string_lossy();
    Box::leak(
        format!("schema://HorrorPlace-Constellation-Contracts/{}", rel).into_boxed_str(),
    )
}

fn infer_kind_from_path(path: &Path) -> String {
    let components: Vec<String> = path
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();

    if components.iter().any(|c| c == "tooling") {
        return "tooling".to_string();
    }
    if components.iter().any(|c| c == "contracts") {
        return "contract".to_string();
    }
    if components.iter().any(|c| c == "spine") {
        return "spine".to_string();
    }

    "unknown".to_string()
}

fn infer_spine_category(kind: &str) -> String {
    match kind {
        "contract" => "contracts".to_string(),
        "tooling" => "tooling".to_string(),
        "spine" => "spine".to_string(),
        _ => "other".to_string(),
    }
}

// ---------------- Contract scanning for invariants/metrics ----------------

fn scan_contract_roots(
    repo_root: &Path,
    roots: &[PathBuf],
) -> Result<(HashMap<String, RangeAgg>, HashMap<String, RangeAgg>)> {
    let mut invariant_ranges: HashMap<String, RangeAgg> = HashMap::new();
    let mut metric_ranges: HashMap<String, RangeAgg> = HashMap::new();

    let invariant_codes = [
        "CIC", "MDI", "AOS", "RRM", "FCF", "SPR", "RWF", "DET", "HVF", "LSG", "SHCI",
    ];
    let metric_codes = ["UEC", "EMD", "STCI", "CDL", "ARR"];

    for code in invariant_codes.iter() {
        invariant_ranges.insert((*code).to_string(), RangeAgg::new());
    }
    for code in metric_codes.iter() {
        metric_ranges.insert((*code).to_string(), RangeAgg::new());
    }

    for root in roots {
        if !root.exists() {
            continue;
        }

        for entry in WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }

            let data = match fs::read_to_string(path) {
                Ok(d) => d,
                Err(_) => continue,
            };

            let v: Value = match serde_json::from_str(&data) {
                Ok(val) => val,
                Err(_) => continue,
            };

            // Extract invariants
            if let Some(inv_map) = v.get("invariants") {
                extract_invariants(inv_map, &mut invariant_ranges);
            }

            // Extract metrics
            if let Some(metrics_map) = v.get("metrics") {
                extract_metrics(metrics_map, &mut metric_ranges);
            }
        }
    }

    Ok((invariant_ranges, metric_ranges))
}

fn extract_invariants(inv_val: &Value, ranges: &mut HashMap<String, RangeAgg>) {
    let codes = [
        "CIC", "MDI", "AOS", "RRM", "FCF", "SPR", "RWF", "DET", "HVF", "LSG", "SHCI",
    ];

    // Two common patterns:
    // 1) invariants: { "min": { "CIC": 0.5, ... }, "max": { "CIC": 0.9, ... } }
    // 2) invariants: { "CIC": 0.5, "MDI": 0.4, ... }
    if let Some(obj) = inv_val.as_object() {
        if let (Some(min_obj), Some(max_obj)) = (obj.get("min"), obj.get("max")) {
            if let (Some(min_map), Some(max_map)) = (min_obj.as_object(), max_obj.as_object()) {
                for code in codes.iter() {
                    if let (Some(min_v), Some(max_v)) =
                        (min_map.get(*code), max_map.get(*code))
                    {
                        if let (Some(min_f), Some(max_f)) =
                            (min_v.as_f64(), max_v.as_f64())
                        {
                            if let Some(agg) = ranges.get_mut(&code.to_string()) {
                                agg.update(min_f, max_f);
                            }
                        }
                    }
                }
                return;
            }
        }

        // Fallback: single-level numeric fields
        for code in codes.iter() {
            if let Some(v) = obj.get(*code) {
                if let Some(val) = v.as_f64() {
                    if let Some(agg) = ranges.get_mut(&code.to_string()) {
                        agg.update(val, val);
                    }
                }
            }
        }
    }
}

fn extract_metrics(metrics_val: &Value, ranges: &mut HashMap<String, RangeAgg>) {
    let codes = ["UEC", "EMD", "STCI", "CDL", "ARR"];

    if let Some(obj) = metrics_val.as_object() {
        for code in codes.iter() {
            if let Some(entry) = obj.get(*code) {
                if let Some(entry_obj) = entry.as_object() {
                    if let (Some(min_v), Some(max_v)) =
                        (entry_obj.get("min"), entry_obj.get("max"))
                    {
                        if let (Some(min_f), Some(max_f)) =
                            (min_v.as_f64(), max_v.as_f64())
                        {
                            if let Some(agg) = ranges.get_mut(&code.to_string()) {
                                agg.update(min_f, max_f);
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---------------- Spine builders from aggregates ----------------

fn build_invariants_spine(ranges: &HashMap<String, RangeAgg>) -> InvariantsSpine {
    let default_min = 0.0;
    let default_max = 1.0;

    let mut invariants: Vec<InvariantEntry> = Vec::new();

    let invariant_meta: HashMap<&str, (&str, &str)> = [
        ("CIC", ("Catastrophic Imprint Coefficient", "Strength of catastrophic history imprint.")),
        ("MDI", ("Mythic Density Index", "Density of folklore and mythic references.")),
        ("AOS", ("Archival Opacity Score", "Degree of archival gaps and redaction.")),
        ("RRM", ("Ritual Residue Map", "Residual strength of ritual behavior.")),
        ("FCF", ("Folkloric Convergence Factor", "Convergence of independent folklore sources.")),
        ("SPR", ("Spectral Plausibility Rating", "Derived spectral plausibility.")),
        ("RWF", ("Reliability Weighting Factor", "Source reliability weighting.")),
        ("DET", ("Dread Exposure Threshold", "Exposure threshold for dread effects.")),
        ("HVF", ("Haunt Vector Field", "Magnitude of local haunt vectors.")),
        ("LSG", ("Liminal Stress Gradient", "Stress gradient across thresholds.")),
        ("SHCI", ("Spectral-History Coupling Index", "Coupling between spectral entities and history."))
    ]
    .into_iter()
    .collect();

    for (code, agg) in ranges.iter() {
        let mm = agg.to_minmax(default_min, default_max);
        let (name, description) = invariant_meta
            .get(code.as_str())
            .cloned()
            .unwrap_or(("Unknown", "No description provided."));

        invariants.push(InvariantEntry {
            code: code.clone(),
            name: name.to_string(),
            description: description.to_string(),
            range: mm,
            derivedFrom: Vec::new(),
            consumers: vec!["contracts".to_string()],
        });
    }

    InvariantsSpine {
        id: "invariants-spine-v1".to_string(),
        version: "1.0.0".to_string(),
        invariants,
    }
}

fn build_metrics_spine(ranges: &HashMap<String, RangeAgg>) -> MetricsSpine {
    let default_min = 0.0;
    let default_max = 1.0;

    let mut metrics: Vec<MetricEntry> = Vec::new();

    let metric_meta: HashMap<&str, (&str, &str)> = [
        ("UEC", ("Uncertainty Engagement Coefficient", "Degree of engaged uncertainty.")),
        ("EMD", ("Evidential Mystery Density", "Density of unresolved evidence.")),
        ("STCI", ("Safe-Threat Contrast Index", "Contrast between safety and threat states.")),
        ("CDL", ("Cognitive Dissonance Load", "Load of cognitive dissonance imposed.")),
        ("ARR", ("Ambiguous Resolution Ratio", "Ratio of ambiguous to resolved outcomes."))
    ]
    .into_iter()
    .collect();

    for (code, agg) in ranges.iter() {
        let mm = agg.to_minmax(default_min, default_max);
        let (name, description) = metric_meta
            .get(code.as_str())
            .cloned()
            .unwrap_or(("Unknown", "No description provided."));

        metrics.push(MetricEntry {
            code: code.clone(),
            name: name.to_string(),
            description: description.to_string(),
            range: mm,
            recommendedBands: Vec::new(),
        });
    }

    MetricsSpine {
        id: "entertainment-metrics-spine-v1".to_string(),
        version: "1.0.0".to_string(),
        metrics,
    }
}

// ---------------- Utility IO helpers ----------------

fn locate_repo_root() -> Result<PathBuf> {
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join(".git").is_dir() && dir.join("schemas").is_dir() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
        Err(anyhow::anyhow!(
            "Could not locate repo root (expected .git and schemas/)"
        ))?;
    }
    unreachable!()
}

fn write_pretty_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(value)?;
    fs::write(path, json)?;
    Ok(())
}
