// crates/hpc-constellation-audit/src/wiring.rs
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct WiringPlan {
    pub schemaVersion: String,
    pub schemaRef: String,
    pub description: String,
    pub repos: Vec<WiringRepo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WiringRepo {
    pub repoName: String,
    pub tier: String,
    pub kind: String,
    pub primaryRole: String,
    pub objectKinds: Vec<WiringObjectKind>,
    pub preferredLanguages: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WiringObjectKind {
    pub name: String,
    pub role: String,
    pub phases: Vec<String>,
    pub languages: Vec<String>,
    pub wiring: serde_json::Value,
}
