//! Binary entry point for `hpc-chat-director`.
//!
//! A thin wrapper that calls `cli::run()`, handling process-level
//! concerns: exit codes, signal handling, and global error formatting.

use std::process::ExitCode;
use clap::Parser;
use hpc_chat_director::cli::Cli;
use hpc_chat_director::errors::Error;

fn main() -> ExitCode {
    // Parse CLI arguments
    let cli = Cli::parse();
    
    // Run the CLI and map result to exit code
    match run_cli(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            // Format error based on output mode
            if cli.json_output() {
                // JSON mode: output structured error to stderr
                let error_schema = build_cli_error_schema(&err);
                eprintln!("{}", serde_json::to_string(&error_schema).unwrap_or_else(|_| {
                    format!("{{\"error\": \"Failed to serialize error: {}\"}}", err)
                }));
            } else {
                // Human mode: output readable message to stderr
                eprintln!("Error: {}", err);
            }
            ExitCode::from(err.exit_code())
        }
    }
}

/// Run the CLI based on parsed arguments.
fn run_cli(cli: Cli) -> Result<(), Error> {
    use hpc_chat_director::cli::Command;
    
    match cli.command {
        Command::Init { root } => {
            hpc_chat_director::cli::init::run(&root)
        }
        Command::Plan { prompt, target_repo } => {
            let config = hpc_chat_director::config::Config::detect(&cli.root_or_default())?;
            let spine = hpc_chat_director::spine::load(&config)?;
            let manifests = hpc_chat_director::manifests::load_all(&config)?;
            
            let result = hpc_chat_director::cli::plan::run(
                &prompt,
                target_repo.as_deref(),
                &config,
                &spine,
                &manifests,
            )?;
            
            // Output result as JSON
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        Command::ValidateResponse { request_file, response_file } => {
            let config = hpc_chat_director::config::Config::detect(&cli.root_or_default())?;
            let director = hpc_chat_director::ChatDirector::load_environment(&config.root)?;
            
            hpc_chat_director::cli::validate_response::run(
                &request_file,
                &response_file,
                &director,
            )
        }
        Command::Apply { validated_file, dry_run } => {
            let config = hpc_chat_director::config::Config::detect(&cli.root_or_default())?;
            let manifests = hpc_chat_director::manifests::load_all(&config)?;
            
            let result = hpc_chat_director::cli::apply::run(
                &validated_file,
                dry_run,
                &config,
                &manifests,
            )?;
            
            // Output result as JSON
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        Command::Describe { object_kind, phase } => {
            let config = hpc_chat_director::config::Config::detect(&cli.root_or_default())?;
            let director = hpc_chat_director::ChatDirector::load_environment(&config.root)?;
            
            let result = hpc_chat_director::cli::describe::run(
                object_kind.as_deref(),
                phase,
                cli.tier.as_deref(),
                &director,
            )?;
            
            // Output result as JSON
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
    }
}

/// Build a CLI error schema for JSON output.
fn build_cli_error_schema(err: &Error) -> serde_json::Value {
    serde_json::json!({
        "schemaVersion": "v1",
        "exitCode": err.exit_code(),
        "summary": err.to_string(),
        "metadata": {
            "errorType": std::any::type_name_of_val(err),
        },
        "diagnostics": [{
            "code": error_code_for(err),
            "layer": error_layer_for(err),
            "severity": "error",
            "message": err.to_string(),
            "jsonPointer": "/",
            "fixOrder": 1,
        }]
    })
}

/// Map error to machine-readable code.
fn error_code_for(err: &Error) -> String {
    match err {
        Error::Config { .. } => "CONFIG_ERROR".into(),
        Error::Io { .. } => "IO_ERROR".into(),
        Error::Parse { .. } => "PARSE_ERROR".into(),
        Error::SpineLoad { .. } => "SPINE_LOAD_ERROR".into(),
        Error::SpineParse { .. } => "SPINE_PARSE_ERROR".into(),
        Error::ManifestLoad { .. } => "MANIFEST_LOAD_ERROR".into(),
        Error::ManifestParse { .. } => "MANIFEST_PARSE_ERROR".into(),
        Error::Phase(pe) => match pe {
            hpc_chat_director::errors::PhaseError::PhaseForbidden { .. } => "PHASE_FORBIDDEN".into(),
            hpc_chat_director::errors::PhaseError::PromotionBlocked { .. } => "PROMOTION_BLOCKED".into(),
        },
        Error::Validation(ve) => ve.code.clone(),
        Error::Cli { .. } => "CLI_ERROR".into(),
        Error::Internal { .. } => "INTERNAL_ERROR".into(),
        Error::Json(_) => "JSON_ERROR".into(),
    }
}

/// Map error to validation layer.
fn error_layer_for(err: &Error) -> String {
    match err {
        Error::Phase(_) => "phase".into(),
        Error::Validation(_) => "validation".into(),
        _ => "config".into(),
    }
}

impl Cli {
    /// Check if JSON output is requested.
    fn json_output(&self) -> bool {
        // Could check for --json flag; default to true for AI compatibility
        true
    }
    
    /// Get root path from CLI or default to current directory.
    fn root_or_default(&self) -> std::path::PathBuf {
        // Could extract from --root flag; default to "."
        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
    }
}
