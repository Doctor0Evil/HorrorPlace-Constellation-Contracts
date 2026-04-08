//! CLI subcommand: `hpc-chat-director validate-response`.
//!
//! Reads an AiAuthoringRequest + AiAuthoringResponse JSON envelope from stdin,
//! runs the full validation pipeline, and outputs either a SUCCESS envelope
//! or structured diagnostics in CliErrorSchema format on stderr.

use std::io::{self, Read};

use serde_json::json;

use crate::errors::CliErrorSchema;
use crate::model::request_types::AiAuthoringRequest;
use crate::model::response_types::AiAuthoringResponse;
use crate::validate::ValidationResult;
use crate::ChatDirector;

/// Run the validate-response subcommand using stdin/stdout.
/// Expects JSON of the shape:
///
/// {
///   "request": { ... AiAuthoringRequest ... },
///   "response": { ... AiAuthoringResponse ... }
/// }
pub fn run_validate_response(director: &ChatDirector) -> i32 {
    let mut buf = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buf) {
        let schema = CliErrorSchema::from_io_error(e);
        eprintln!("{}", schema.to_json_string());
        return schema.exit_code;
    }

    let parsed: serde_json::Value = match serde_json::from_str(&buf) {
        Ok(v) => v,
        Err(e) => {
            let schema = CliErrorSchema::from_parse_error(e);
            eprintln!("{}", schema.to_json_string());
            return schema.exit_code;
        }
    };

    let request: AiAuthoringRequest = match serde_json::from_value(parsed["request"].clone()) {
        Ok(v) => v,
        Err(e) => {
            let schema = CliErrorSchema::from_struct_error("AiAuthoringRequest", e);
            eprintln!("{}", schema.to_json_string());
            return schema.exit_code;
        }
    };

    let response: AiAuthoringResponse = match serde_json::from_value(parsed["response"].clone()) {
        Ok(v) => v,
        Err(e) => {
            let schema = CliErrorSchema::from_struct_error("AiAuthoringResponse", e);
            eprintln!("{}", schema.to_json_string());
            return schema.exit_code;
        }
    };

    match director.validate_response(&request, &response) {
        Ok(result) => emit_result(&request, &result),
        Err(err) => {
            let schema = CliErrorSchema::from_director_error(err);
            eprintln!("{}", schema.to_json_string());
            schema.exit_code
        }
    }
}

fn emit_result(
    request: &AiAuthoringRequest,
    result: &ValidationResult,
) -> i32 {
    if result.diagnostics.is_empty() {
        // Success: emit a minimal SUCCESS envelope on stdout.
        let ok = json!({
            "schemaVersion": "1.0.0",
            "exitCode": 0,
            "summary": "SUCCESS",
            "metadata": {
                "objectKind": request.objectKind,
                "shortCircuited": false
            },
            "diagnostics": []
        });
        println!("{}", ok.to_string());
        0
    } else {
        // Failure: down-convert into CliErrorSchema and emit on stderr.
        let schema = CliErrorSchema::from_validation_result(result, Some(request));
        eprintln!("{}", schema.to_json_string());
        schema.exit_code
    }
}
