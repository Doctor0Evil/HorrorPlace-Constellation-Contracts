use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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

fn main() -> Result<()> {
    let repo_root = locate_repo_root()?;
    let schemas_dir = repo_root.join("schemas");

    let mut entries: Vec<SchemaEntry> = Vec::new();

    for entry in WalkDir::new(&schemas_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "json" {
                if let Some(schema_entry) = process_schema_file(&repo_root, path)? {
                    entries.push(schema_entry);
                }
            }
        }
    }

    let spine = SchemaSpineIndex {
        id: "schema-spine-index.v1".to_string(),
        version: "1.0.0".to_string(),
        schemas: entries,
    };

    let out_path = repo_root.join("schemas/spine/schema-spine-index-v1.json");
    let out_dir = out_path
        .parent()
        .context("schema spine parent directory not found")?;
    fs::create_dir_all(out_dir)?;
    let json = serde_json::to_string_pretty(&spine)?;
    fs::write(&out_path, json)?;

    println!("Wrote schema spine index to {}", out_path.display());
    Ok(())
}

fn locate_repo_root() -> Result<PathBuf> {
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join(".git").is_dir() && dir.join("schemas").is_dir() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    Err(anyhow::anyhow!(
        "Could not locate repo root (expected .git and schemas/)"
    ))
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
    // Fallback: use a synthetic id derived from the path.
    // In practice you may want to tighten this or require $id.
    let rel = path.strip_prefix(repo_root).unwrap_or(path).to_string_lossy();
    Box::leak(format!("schema://HorrorPlace-Constellation-Contracts/{}", rel).into_boxed_str())
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
