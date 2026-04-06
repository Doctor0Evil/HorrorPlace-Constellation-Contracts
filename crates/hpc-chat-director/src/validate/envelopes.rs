//! Prism envelope structural validation.
//!
//! Validates the structural integrity of prism-envelope-v1 wrappers:
//! required fields, consistency between request/response refs, and
//! placeholder verification for cryptographic fields. Does not perform
//! actual cryptographic verification (that's downstream).

use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::model::response_types::{PrismEnvelope, DeadLedgerRef, PrismMeta};

/// Result of envelope validation.
#[derive(Debug)]
pub struct EnvelopeValidationResult {
    pub passed: bool,
    pub errors: Vec<EnvelopeError>,
}

/// Structured envelope validation error.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvelopeError {
    pub code: String,
    pub message: String,
    pub json_pointer: String,
    pub submitted_value: Option<serde_json::Value>,
    pub expected: Option<serde_json::Value>,
    pub remediation: String,
}

/// Entry point for envelope validation.
pub fn validate_envelope(
    envelope: &PrismEnvelope,
    config: &Config,
) -> Result<(), Vec<EnvelopeError>> {
    let mut errors = Vec::new();

    // Check envelope version is accepted
    if !config.envelope_versions.contains(&envelope.envelope_version) {
        errors.push(EnvelopeError {
            code: "ENVELOPE_VERSION_UNSUPPORTED".into(),
            message: format!(
                "Envelope version '{}' not in accepted versions: {:?}",
                envelope.envelope_version, config.envelope_versions
            ),
            json_pointer: "/envelopeVersion".into(),
            submitted_value: Some(serde_json::json!(&envelope.envelope_version)),
            expected: Some(serde_json::json!(&config.envelope_versions)),
            remediation: format!(
                "Use an envelope version from the accepted list: {:?}",
                config.envelope_versions
            ),
        });
    }

    // Check required fields
    if envelope.schema_ref.is_empty() {
        errors.push(EnvelopeError {
            code: "MISSING_SCHEMA_REF".into(),
            message: "Required field 'schema_ref' is empty".into(),
            json_pointer: "/schemaRef".into(),
            submitted_value: Some(serde_json::json!(envelope.schema_ref.clone())),
            expected: Some(serde_json::json!("A non-empty canonical schema URI")),
            remediation: "Set schema_ref to the canonical schema URI for the payload".into(),
        });
    }

    if envelope.timestamp.is_empty() {
        errors.push(EnvelopeError {
            code: "MISSING_TIMESTAMP".into(),
            message: "Required field 'timestamp' is empty".into(),
            json_pointer: "/timestamp".into(),
            submitted_value: None,
            expected: Some(serde_json::json!("An ISO 8601 timestamp")),
            remediation: "Set timestamp to the current time in ISO 8601 format".into(),
        });
    }

    // Validate timestamp format if present
    if !envelope.timestamp.is_empty() {
        if let Err(e) = chrono::DateTime::parse_from_rfc3339(&envelope.timestamp) {
            errors.push(EnvelopeError {
                code: "INVALID_TIMESTAMP_FORMAT".into(),
                message: format!("Timestamp is not valid RFC3339: {}", e),
                json_pointer: "/timestamp".into(),
                submitted_value: Some(serde_json::json!(&envelope.timestamp)),
                expected: Some(serde_json::json!("YYYY-MM-DDTHH:MM:SSZ format")),
                remediation: "Use chrono::Utc::now().to_rfc3339() to generate valid timestamps".into(),
            });
        }
    }

    // Validate Dead-Ledger reference structure if present
    if let Some(ref dl_ref) = envelope.deadledger_ref {
        if let Err(e) = validate_deadledger_ref(dl_ref) {
            errors.push(e);
        }
    }

    // Validate prismMeta structure if present
    if let Some(ref meta) = envelope.prisma_meta {
        if let Err(e) = validate_prism_meta(meta) {
            errors.push(e);
        }
    }

    // Validate cryptographic placeholders if present
    if let Some(ref sig) = envelope.signature {
        if !is_valid_hex_signature(sig) {
            errors.push(EnvelopeError {
                code: "INVALID_SIGNATURE_FORMAT".into(),
                message: "Signature must be a valid hex string".into(),
                json_pointer: "/signature".into(),
                submitted_value: Some(serde_json::json!(sig)),
                expected: Some(serde_json::json!("A hex string of appropriate length")),
                remediation: "Use a cryptographic library to generate valid signatures".into(),
            });
        }
    }

    // Validate ZKP commitment placeholder if present
    if let Some(ref zkp) = envelope.zkp_commitment {
        if !is_valid_hex_commitment(zkp) {
            errors.push(EnvelopeError {
                code: "INVALID_ZKP_COMMITMENT_FORMAT".into(),
                message: "ZKP commitment must be a valid hex string".into(),
                json_pointer: "/zkpCommitment".into(),
                submitted_value: Some(serde_json::json!(zkp)),
                expected: Some(serde_json::json!("A hex string of 64 characters")),
                remediation: "Use Dead-Ledger tooling to generate valid ZKP commitments".into(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate Dead-Ledger reference structure.
fn validate_deadledger_ref(dl_ref: &DeadLedgerRef) -> Result<(), EnvelopeError> {
    if dl_ref.proof_envelope_id.is_empty() {
        return Err(EnvelopeError {
            code: "DEADLEDGER_MISSING_PROOF_ID".into(),
            message: "Dead-Ledger reference missing proof_envelope_id".into(),
            json_pointer: "/deadledgerRef/proofEnvelopeId".into(),
            submitted_value: None,
            expected: Some(serde_json::json!("A non-empty proof envelope ID")),
            remediation: "Set proof_envelope_id to a valid Dead-Ledger proof ID".into(),
        });
    }
    
    if dl_ref.verifier_ref.is_empty() {
        return Err(EnvelopeError {
            code: "DEADLEDGER_MISSING_VERIFIER".into(),
            message: "Dead-Ledger reference missing verifier_ref".into(),
            json_pointer: "/deadledgerRef/verifierRef".into(),
            submitted_value: None,
            expected: Some(serde_json::json!("A non-empty verifier reference")),
            remediation: "Set verifier_ref to a valid verifier ID from Dead-Ledger registry".into(),
        });
    }
    
    Ok(())
}

/// Validate PrismMeta structure.
fn validate_prism_meta(meta: &PrismMeta) -> Result<(), EnvelopeError> {
    // RWF must be in valid range if present
    if let Some(rwf) = meta.rwf {
        if rwf < 0.0 || rwf > 1.0 {
            return Err(EnvelopeError {
                code: "RWF_OUT_OF_RANGE".into(),
                message: format!("RWF value {:.3} is outside valid range [0.0, 1.0]", rwf),
                json_pointer: "/prismMeta/rwf".into(),
                submitted_value: Some(serde_json::json!(rwf)),
                expected: Some(serde_json::json!({"min": 0.0, "max": 1.0})),
                remediation: "Ensure RWF is a normalized reliability score between 0.0 and 1.0".into(),
            });
        }
    }
    
    Ok(())
}

/// Check if a string is a valid hex signature.
fn is_valid_hex_signature(s: &str) -> bool {
    s.len() >= 64 && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Check if a string is a valid hex ZKP commitment.
fn is_valid_hex_commitment(s: &str) -> bool {
    s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Helper to wrap a payload in a prism envelope with minimal metadata.
pub fn wrap_in_prism_envelope(
    payload: serde_json::Value,
    generated_by: &str,
    schema_ref: &str,
    config: &Config,
) -> PrismEnvelope {
    PrismEnvelope {
        envelope_version: config.envelope_versions.first().cloned().unwrap_or_else(|| "v1".into()),
        timestamp: chrono::Utc::now().to_rfc3339(),
        schema_ref: schema_ref.to_string(),
        deadledger_ref: None,
        zkp_commitment: None,
        prisma_meta: Some(PrismMeta {
            rwf: None,
            review_status: None,
            generation_phase: None,
            telemetry_hooks: Vec::new(),
        }),
        signature: None,
    }
}
