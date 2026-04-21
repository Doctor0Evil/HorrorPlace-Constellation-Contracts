use serde::Deserialize;

use crate::session::AuthoringSession;
use crate::validation::LintError;

#[derive(Debug, Deserialize)]
pub struct AuthoringEnvelope {
    pub id: String,
    #[serde(rename = "schemaRef")]
    pub schema_ref: String,
    #[serde(rename = "objectKind")]
    pub object_kind: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub session: AuthoringSession,
    pub files: Vec<EnvelopeFile>,
    #[serde(rename = "referencedIds")]
    pub referenced_ids: Option<Vec<String>>,
    #[serde(rename = "deadLedgerRef")]
    pub dead_ledger_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EnvelopeFile {
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "fileKind")]
    pub file_kind: String,
    #[serde(rename = "objectKind")]
    pub object_kind: String,
    #[serde(rename = "targetPath")]
    pub target_path: String,
    pub operation: String,
    #[serde(rename = "previousSha256")]
    pub previous_sha256: Option<String>,
    #[serde(rename = "contentSha256")]
    pub content_sha256: Option<String>,
    pub content: String,
    #[serde(rename = "linesAdded")]
    pub lines_added: Option<i64>,
    #[serde(rename = "linesRemoved")]
    pub lines_removed: Option<i64>,
}

impl AuthoringEnvelope {
    pub fn basic_validate(&self) -> Result<(), LintError> {
        if self.schema_ref != "ai-authoring-envelope.v1" {
            return Err(LintError::SchemaRefMismatch {
                expected: "ai-authoring-envelope.v1".to_string(),
                found: self.schema_ref.clone(),
            });
        }
        if self.object_kind != "aiAuthoringEnvelope" {
            return Err(LintError::ObjectKindMismatch {
                expected: "aiAuthoringEnvelope".to_string(),
                found: self.object_kind.clone(),
            });
        }
        if self.files.is_empty() {
            return Err(LintError::NoFilesInEnvelope);
        }
        Ok(())
    }
}
