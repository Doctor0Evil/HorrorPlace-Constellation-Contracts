use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthoringSession {
    pub id: String,
    #[serde(rename = "schemaRef")]
    pub schema_ref: String,
    #[serde(rename = "objectKind")]
    pub object_kind: String,
    pub tier: String,
    #[serde(rename = "agentProfileId")]
    pub agent_profile_id: String,
    #[serde(rename = "targetRepo")]
    pub target_repo: String,
    #[serde(rename = "targetBranch")]
    pub target_branch: String,
    #[serde(rename = "policyProfileId")]
    pub policy_profile_id: Option<String>,
    #[serde(rename = "fileTypeInvariantsRef")]
    pub file_type_invariants_ref: String,
    #[serde(rename = "allowedOperations")]
    pub allowed_operations: Vec<String>,
    #[serde(rename = "allowedFileKinds")]
    pub allowed_file_kinds: Vec<String>,
    #[serde(rename = "maxFilesPerSession")]
    pub max_files_per_session: i64,
    #[serde(rename = "maxLinesPerFile")]
    pub max_lines_per_file: Option<i64>,
    pub targets: Vec<AuthoringTarget>,
    #[serde(rename = "intensityConstraints")]
    pub intensity_constraints: Option<IntensityConstraints>,
    #[serde(rename = "invariantConstraints")]
    pub invariant_constraints: Option<InvariantConstraints>,
    #[serde(rename = "sessionNotes")]
    pub session_notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AuthoringTarget {
    #[serde(rename = "fileKind")]
    pub file_kind: String,
    #[serde(rename = "objectKind")]
    pub object_kind: String,
    #[serde(rename = "patternKey")]
    pub pattern_key: String,
    #[serde(rename = "targetPath")]
    pub target_path: String,
    pub operation: String,
    #[serde(rename = "requiresExistingFile")]
    pub requires_existing_file: Option<bool>,
    #[serde(rename = "requireStableFileId")]
    pub require_stable_file_id: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct IntensityConstraints {
    #[serde(rename = "maxIntensityBand")]
    pub max_intensity_band: Option<String>,
    #[serde(rename = "forbidAdultBand")]
    pub forbid_adult_band: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct InvariantConstraints {
    #[serde(rename = "maxDET")]
    pub max_det: Option<f64>,
    #[serde(rename = "maxCDL")]
    pub max_cdl: Option<f64>,
    #[serde(rename = "minARR")]
    pub min_arr: Option<f64>,
}

impl AuthoringSession {
    pub fn basic_validate(&self) -> Result<(), String> {
        if self.schema_ref != "ai-authoring-session-contract.v1" {
            return Err(format!(
                "session.schemaRef must be 'ai-authoring-session-contract.v1', found '{}'",
                self.schema_ref
            ));
        }
        if self.object_kind != "aiAuthoringSessionContract" {
            return Err(format!(
                "session.objectKind must be 'aiAuthoringSessionContract', found '{}'",
                self.object_kind
            ));
        }
        if self.allowed_operations.is_empty() {
            return Err("session.allowedOperations must not be empty".to_string());
        }
        if self.allowed_file_kinds.is_empty() {
            return Err("session.allowedFileKinds must not be empty".to_string());
        }
        if self.max_files_per_session < 0 {
            return Err("session.maxFilesPerSession must be >= 0".to_string());
        }
        if self.targets.is_empty() {
            return Err("session.targets must not be empty".to_string());
        }
        Ok(())
    }
}
