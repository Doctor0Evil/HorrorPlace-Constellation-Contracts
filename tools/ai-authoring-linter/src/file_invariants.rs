use serde::Deserialize;
use std::fs;

use crate::validation::LintError;

#[derive(Debug, Deserialize)]
pub struct FileTypeInvariants {
    #[serde(rename = "fileKinds")]
    pub file_kinds: Vec<String>,
    #[serde(rename = "namingPatterns")]
    pub naming_patterns: Vec<NamingPattern>,
    #[serde(rename = "authoringConstraints")]
    pub authoring_constraints: AuthoringConstraints,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NamingPattern {
    #[serde(rename = "fileKind")]
    pub file_kind: String,
    #[serde(rename = "objectKind")]
    pub object_kind: String,
    pub pattern: String,
    pub extension: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthoringConstraints {
    #[serde(rename = "requireExplicitTarget")]
    pub require_explicit_target: bool,
    #[serde(rename = "forbidAdHocRename")]
    pub forbid_ad_hoc_rename: bool,
    #[serde(rename = "requireStableFileId")]
    pub require_stable_file_id: bool,
}

impl FileTypeInvariants {
    pub fn load_from_path(path: &str) -> Result<Self, LintError> {
        let data = fs::read_to_string(path).map_err(|e| LintError::Io {
            path: path.to_string(),
            source: e,
        })?;
        let parsed: FileTypeInvariants =
            serde_json::from_str(&data).map_err(|e| LintError::Json { source: e })?;
        Ok(parsed)
    }

    pub fn find_pattern_for(
        &self,
        file_kind: &str,
        object_kind: &str,
    ) -> Option<NamingPattern> {
        self.naming_patterns
            .iter()
            .find(|p| p.file_kind == file_kind && p.object_kind == object_kind)
            .cloned()
    }
}
