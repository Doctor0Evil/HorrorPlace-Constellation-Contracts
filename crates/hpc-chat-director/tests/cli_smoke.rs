//! Smoke tests for CLI subcommands: init, plan, validate-response, apply.
//!
//! These tests verify that the CLI produces expected JSON output,
//! respects exit codes, and handles errors in a machine-readable way.

use std::process::Command;
use tempfile::TempDir;
use serde_json::Value;

/// Test that `hpc-chat-director init` succeeds with valid environment.
#[test]
fn test_cli_init_success() {
    let temp_dir = setup_test_environment();
    let binary = build_binary_path();

    let output = Command::new(&binary)
        .arg("init")
        .arg("--root")
        .arg(temp_dir.path())
        .arg("--json")
        .output()
        .expect("Failed to execute init command");

    assert!(
        output.status.success(),
        "init should succeed with valid environment: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Parse JSON output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output should be valid JSON");
    
    assert_eq!(json["status"], "success");
    assert!(json["spine_version"].is_string());
    assert!(json["manifest_count"].is_number());
}

/// Test that `hpc-chat-director plan` produces structured request.
#[test]
fn test_cli_plan_produces_request() {
    let temp_dir = setup_test_environment();
    let binary = build_binary_path();

    let output = Command::new(&binary)
        .arg("plan")
        .arg("Create a mood contract for a liminal space")
        .arg("--root")
        .arg(temp_dir.path())
        .arg("--json")
        .output()
        .expect("Failed to execute plan command");

    assert!(
        output.status.success(),
        "plan should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output should be valid JSON");
    
    // Should have request object
    assert!(json["request"].is_object());
    let request = &json["request"];
    
    assert_eq!(request["object_kind"], "moodContract");
    assert!(request["target_repo"].is_string());
    assert!(request["target_path"].is_string());
    
    // Should have generation guide
    assert!(json["generation_guide"].is_object());
}

/// Test that `hpc-chat-director validate-response` handles invalid input.
#[test]
fn test_cli_validate_response_invalid_input() {
    let temp_dir = setup_test_environment();
    let binary = build_binary_path();

    // Create invalid request file
    let req_path = temp_dir.path().join("invalid_request.json");
    std::fs::write(&req_path, r#"{"invalid": "request"}"#).unwrap();
    
    // Create invalid response file
    let resp_path = temp_dir.path().join("invalid_response.json");
    std::fs::write(&resp_path, r#"{"invalid": "response"}"#).unwrap();

    let output = Command::new(&binary)
        .arg("validate-response")
        .arg(&req_path)
        .arg(&resp_path)
        .arg("--root")
        .arg(temp_dir.path())
        .arg("--json")
        .output()
        .expect("Failed to execute validate-response command");

    // Should fail with non-zero exit code
    assert!(
        !output.status.success(),
        "validate-response should fail with invalid input"
    );
    
    // Exit code should be 1 (validation failure) or 2 (config error)
    let exit_code = output.status.code().unwrap_or(-1);
    assert!(
        exit_code == 1 || exit_code == 2,
        "Exit code should be 1 or 2, got {}",
        exit_code
    );

    // stderr should contain JSON error schema
    let stderr = String::from_utf8_lossy(&output.stderr);
    let json: Value = serde_json::from_str(&stderr).expect("Error output should be valid JSON");
    
    assert!(json["schema_version"].is_string());
    assert!(json["exit_code"].is_number());
    assert!(json["diagnostics"].is_array());
}

/// Test that `hpc-chat-director apply --dry-run` previews changes.
#[test]
fn test_cli_apply_dry_run() {
    let temp_dir = setup_test_environment();
    let binary = build_binary_path();

    // Create a minimal validated file
    let validated_path = temp_dir.path().join("validated.json");
    let validated_content = r#"{
        "targetRepo": "Horror.Place",
        "targetPath": "moods/test_v1.json",
        "content": {"id": "mood.test.v1"},
        "contentHash": "abc123",
        "softDiagnostics": [],
        "provenance": {
            "validatedAt": "2026-01-01T00:00:00Z",
            "validatorVersion": "0.1.0",
            "validationTrace": []
        }
    }"#;
    std::fs::write(&validated_path, validated_content).unwrap();

    let output = Command::new(&binary)
        .arg("apply")
        .arg(&validated_path)
        .arg("--dry-run")
        .arg("--root")
        .arg(temp_dir.path())
        .arg("--json")
        .output()
        .expect("Failed to execute apply command");

    assert!(
        output.status.success(),
        "apply --dry-run should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output should be valid JSON");
    
    // Should have actions array
    assert!(json["actions"].is_array());
    let actions = &json["actions"];
    assert!(!actions.as_array().unwrap().is_empty());
    
    // Should indicate dry run
    assert_eq!(json["dry_run"], true);
    
    // File should NOT have been written
    let target_path = temp_dir.path().join("repos").join("Horror.Place").join("moods").join("test_v1.json");
    assert!(!target_path.exists(), "dry-run should not write files");
}

/// Test that CLI respects exit codes for different error types.
#[test]
fn test_cli_exit_codes() {
    let temp_dir = setup_test_environment();
    let binary = build_binary_path();

    // Test config error (missing root)
    let output = Command::new(&binary)
        .arg("init")
        .arg("--root")
        .arg("/nonexistent/path")
        .arg("--json")
        .output()
        .expect("Failed to execute init command");
    
    let exit_code = output.status.code().unwrap_or(-1);
    assert_eq!(exit_code, 2, "Config error should exit with code 2");

    // Test validation error (handled in test_cli_validate_response_invalid_input)
    // Test phase error would require more complex setup
}

/// Test that `hpc-chat-director --version --json` returns capability info.
#[test]
fn test_cli_version_json() {
    let binary = build_binary_path();

    let output = Command::new(&binary)
        .arg("--version")
        .arg("--json")
        .output()
        .expect("Failed to execute version command");

    assert!(
        output.status.success(),
        "version should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output should be valid JSON");
    
    assert!(json["binaryVersion"].is_string());
    assert!(json["spineVersion"].is_string());
    assert!(json["supportedSchemas"].is_array());
    assert!(json["knownObjectKinds"].is_array());
}

/// Build path to the compiled binary.
fn build_binary_path() -> String {
    // In CI/test environment, binary is in target/debug/
    // Adjust based on your build setup
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // Remove test binary name
    path.push("hpc-chat-director");
    
    // Fallback to cargo build path
    if !path.exists() {
        path = std::path::PathBuf::from("target/debug/hpc-chat-director");
    }
    
    path.to_string_lossy().to_string()
}

/// Set up minimal test environment with spine and manifests.
fn setup_test_environment() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create minimal spine
    let spine_dir = root.join("schemas").join("core");
    std::fs::create_dir_all(&spine_dir).unwrap();
    
    let spine_content = r#"{
        "version": "v1",
        "$id": "schema://HorrorPlace-Constellation-Contracts/schema-spine-index-v1.json",
        "title": "Test Spine",
        "description": "Minimal spine for CLI tests",
        "invariants": {
            "CIC": {
                "name": "CIC",
                "canonicalRange": {"min": 0.0, "max": 1.0},
                "tierOverrides": {},
                "driftMode": "static",
                "compatibleWith": [],
                "description": "Contextual Integrity Coefficient",
                "requiredBy": []
            }
        },
        "metrics": {},
        "contractFamilies": [
            {
                "name": "mood",
                "kinds": ["moodContract"],
                "requiredInvariants": ["CIC"],
                "optionalInvariants": [],
                "requiredMetrics": [],
                "optionalMetrics": [],
                "allowedPhases": [2],
                "tierRestrictions": {}
            }
        ],
        "interactionRules": [],
        "safeDefaults": {}
    }"#;
    
    std::fs::write(
        spine_dir.join("schema-spine-index-v1.json"),
        spine_content
    ).unwrap();

    // Create minimal manifests
    let manifests_dir = root.join("manifests");
    std::fs::create_dir_all(&manifests_dir).unwrap();
    
    let horror_place = r#"{
        "repo": "Horror.Place",
        "tier": "T1",
        "allowedObjectKinds": ["moodContract"],
        "allowedSchemas": ["schema://Horror.Place/*"],
        "defaultTargetPaths": {"moodContract": "moods/{id}.json"},
        "rules": {"oneFilePerRequest": true},
        "authoringHints": {"tierRationale": "Tier 1 is public contract-only."}
    }"#;
    
    std::fs::write(
        manifests_dir.join("repo-manifest.hpc.horror-place.json"),
        horror_place
    ).unwrap();

    temp_dir
}
