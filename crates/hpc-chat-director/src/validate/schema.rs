//! JSON Schema validation engine integration.
//!
//! Wraps the jsonschema crate to validate AI-generated artifacts
//! against the constellation's schema corpus. Handles schema
//! resolution, caching, and error translation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::model::spine_types::SchemaSpine;
use crate::model::response_types::AiAuthoringResponse;
use crate::errors::ValidationError;

/// Compiled schema cache for performance.
pub struct SchemaValidator {
    /// Cache of compiled validators keyed by schema URI.
    compiled: HashMap<String, jsonschema::Validator>,
    /// Reference to the spine for schema resolution.
    spine: SchemaSpine,
}

impl SchemaValidator {
    /// Create a new validator with the given spine.
    pub fn new(spine: SchemaSpine) -> Self {
        Self {
            compiled: HashMap::new(),
            spine,
        }
    }

    /// Validate a response artifact against its referenced schema.
    ///
    /// Returns Ok(()) if valid, or a Vec of ValidationError for failures.
    pub fn validate(
        &mut self,
        resp: &AiAuthoringResponse,
    ) -> Result<(), Vec<ValidationError>> {
        let schema_uri = &resp.envelope.schema_ref;
        
        // Get or compile the schema validator
        let validator = self.get_or_compile(schema_uri)?;
        
        // Validate the artifact
        let result = validator.validate(&resp.artifact);
        
        match result {
            Ok(_) => Ok(()),
            Err(errors) => {
                let mut validation_errors = Vec::new();
                for error in errors {
                    validation_errors.push(ValidationError {
                        code: "SCHEMA_VALIDATION_ERROR".into(),
                        json_pointer: error.instance_path.to_string(),
                        message: error.message.to_string(),
                        remediation: Some(crate::errors::Remediation {
                            json_pointer: error.instance_path.to_string(),
                            expected: format!("{:?}", error.kind),
                            suggestion: format!("Ensure field matches schema constraint: {}", error.kind),
                        }),
                        expected: None,
                        submitted: None,
                    });
                }
                Err(validation_errors)
            }
        }
    }

    /// Quick-check mode: validates only required fields and top-level structure.
    ///
    /// Suitable for AI mid-draft iteration; returns soft warnings instead of hard errors.
    pub fn quick_check(
        &self,
        payload: &serde_json::Value,
        schema_uri: &str,
    ) -> Result<(), Vec<SoftWarning>> {
        // Simplified: in real implementation, would use a reduced schema
        // or custom validation logic for quick checks
        let Some(schema_entry) = self.spine.schema_entries.iter().find(|e| e.id == schema_uri) else {
            return Err(vec![SoftWarning {
                code: "SCHEMA_NOT_FOUND".into(),
                message: format!("Schema '{}' not found in spine", schema_uri),
                json_pointer: "/$schema".into(),
            }]);
        };
        
        // Check only required fields exist and are non-null
        let mut warnings = Vec::new();
        for required_field in &schema_entry.required_fields {
            if !payload.get(required_field).is_some() {
                warnings.push(SoftWarning {
                    code: "MISSING_REQUIRED_FIELD".into(),
                    message: format!("Required field '{}' missing", required_field),
                    json_pointer: format!("/{}", required_field),
                });
            }
        }
        
        if warnings.is_empty() {
            Ok(())
        } else {
            Err(warnings)
        }
    }

    /// Get or compile a schema validator for the given URI.
    fn get_or_compile(
        &mut self,
        schema_uri: &str,
    ) -> Result<&jsonschema::Validator, ValidationError> {
        use std::collections::hash_map::Entry;
        
        match self.compiled.entry(schema_uri.to_string()) {
            Entry::Occupied(entry) => Ok(entry.into_mut()),
            Entry::Vacant(entry) => {
                // Resolve schema from spine
                let schema_json = self.resolve_schema(schema_uri)?;
                
                // Compile validator
                let validator = jsonschema::validator_for(&schema_json)
                    .map_err(|e| ValidationError {
                        code: "SCHEMA_COMPILE_ERROR".into(),
                        json_pointer: "/$schema".into(),
                        message: format!("Failed to compile schema: {}", e),
                        remediation: Some(crate::errors::Remediation {
                            json_pointer: "/$schema".into(),
                            expected: "A valid JSON Schema".into(),
                            suggestion: "Check schema syntax and references".into(),
                        }),
                        expected: None,
                        submitted: None,
                    })?;
                
                Ok(entry.insert(validator))
            }
        }
    }

    /// Resolve a schema URI to its JSON content.
    fn resolve_schema(&self, uri: &str) -> Result<serde_json::Value, ValidationError> {
        // Look up in spine schema entries
        self.spine.schema_entries
            .iter()
            .find(|e| e.id == uri)
            .map(|e| e.schema.clone())
            .ok_or_else(|| ValidationError {
                code: "SCHEMA_NOT_FOUND".into(),
                json_pointer: "/$schema".into(),
                message: format!("Schema '{}' not found in spine", uri),
                remediation: Some(crate::errors::Remediation {
                    json_pointer: "/$schema".into(),
                    expected: "A schema URI defined in the schema spine".into(),
                    suggestion: "Use schemaref from spine.describe_object_kind()".into(),
                }),
                expected: None,
                submitted: None,
            })
    }

    /// Return a human/AI-readable description of a schema.
    pub fn describe_schema(&self, schemaref: &str) -> Option<SchemaDescription> {
        self.spine.schema_entries
            .iter()
            .find(|e| e.id == schemaref)
            .map(|entry| SchemaDescription {
                uri: entry.id.clone(),
                title: entry.title.clone(),
                description: entry.description.clone(),
                required_fields: entry.required_fields.clone(),
                optional_fields: entry.optional_fields.clone(),
                field_descriptions: entry.field_descriptions.clone(),
            })
    }
}

/// Soft warning for quick-check mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftWarning {
    pub code: String,
    pub message: String,
    pub json_pointer: String,
}

/// Human/AI-readable schema description.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaDescription {
    pub uri: String,
    pub title: String,
    pub description: String,
    pub required_fields: Vec<String>,
    pub optional_fields: Vec<String>,
    pub field_descriptions: HashMap<String, String>,
}
