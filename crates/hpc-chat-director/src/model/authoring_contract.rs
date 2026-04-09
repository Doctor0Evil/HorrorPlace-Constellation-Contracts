use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSafeAuthoringContract {
    pub id: String,
    pub schemaVersion: String,
    pub agent: AgentBlock,
    pub profile: ProfileBlock,
    pub discovery: DiscoveryBlock,
    pub plan: PlanBlock,
    pub safety: SafetyBlock,
    pub telemetry: TelemetryBlock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBlock {
    pub agentId: String,
    pub agentProfileId: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBlock {
    pub profileId: String,
    pub tiers: Vec<String>,
    pub repos: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryBlock {
    pub spineLoaded: bool,
    pub manifestsLoaded: bool,
    pub registriesLoaded: bool,
    #[serde(default)]
    pub sources: Option<DiscoverySources>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoverySources {
    #[serde(default)]
    pub spineIndexRef: Option<String>,
    #[serde(default)]
    pub manifestRefs: Vec<String>,
    #[serde(default)]
    pub registryRefs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanBlock {
    pub artifacts: Vec<PlannedArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedArtifact {
    pub objectKind: String,
    pub targetRepo: String,
    pub targetPath: String,
    pub schemaRef: String,
    pub tier: String,
    #[serde(default)]
    pub invariantsTouched: Vec<String>,
    #[serde(default)]
    pub metricsTouched: Vec<String>,
    pub maxFilesInBundle: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyBlock {
    pub consentTier: String,
    pub maxIntensityBand: u32,
    #[serde(default)]
    pub deadLedgerSurfaces: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryBlock {
    pub sessionId: String,
    #[serde(default)]
    pub uecTarget: Option<f32>,
    #[serde(default)]
    pub arrTarget: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryContract {
    pub id: String,
    pub schemaVersion: String,
    pub agentProfileId: String,
    pub generatedAt: String,
    pub spineRefs: SpineRefs,
    #[serde(default)]
    pub manifestRefs: Vec<String>,
    #[serde(default)]
    pub registryRefs: Vec<String>,
    #[serde(default)]
    pub capabilitySpineRef: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpineRefs {
    pub invariantsSpineId: String,
    pub entertainmentMetricsSpineId: String,
    pub schemaSpineIndexId: String,
}
