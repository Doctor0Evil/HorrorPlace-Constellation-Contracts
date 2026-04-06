use std::io::Read;
use std::path::PathBuf;

use clap::{Arg, ArgAction, Command};

use crate::{ChatDirector, Config};
use crate::errors::{to_cli_error_schema};
use crate::model::request_types::AiAuthoringRequest;
use crate::model::response_types::AiAuthoringResponse;

/// Entry point used by src/bin/hpc-chat-director.rs.
pub fn run() -> anyhow::Result<i32> {
    let matches = Command::new("hpc-chat-director")
        .about("Schema-first authoring compiler for HorrorPlace constellations")
        .arg(
            Arg::new("config")
                .long("config")
                .value_name("PATH")
                .help("Path to .hpc-config.toml")
                .global(true),
        )
        .arg(
            Arg::new("human")
                .long("human")
                .action(ArgAction::SetTrue)
                .help("Human-readable output instead of JSON"),
        )
        .subcommand(
            Command::new("init")
                .about("Verify configuration, spine, and manifests can be loaded"),
        )
        .subcommand(
            Command::new("plan")
                .about("Normalize an AiAuthoringRequest from stdin JSON"),
        )
        .subcommand(
            Command::new("validate-response")
                .about("Validate an AiAuthoringResponse from stdin JSON"),
        )
        .subcommand(
            Command::new("apply")
                .about("Apply a validated artifact to the filesystem")
                .arg(
                    Arg::new("dry-run")
                        .long("dry-run")
                        .action(ArgAction::SetTrue)
                        .help("Print intended changes as JSON without writing"),
                ),
        )
        .get_matches();

    let config_path = matches
        .get_one::<String>("config")
        .map(PathBuf::from);
    let config = Config::load(config_path.as_ref())?;
    let director = ChatDirector::init(config)?;

    match matches.subcommand() {
        Some(("init", _sub)) => {
            // Initialization already happened; just report success.
            if matches.get_flag("human") {
                println!("CHAT_DIRECTOR initialization succeeded.");
            } else {
                println!("{{\"status\":\"ok\"}}");
            }
            Ok(0)
        }
        Some(("plan", _sub)) => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            let raw: serde_json::Value = serde_json::from_str(&buf)?;
            let req = director.normalize_request(raw)?;

            if matches.get_flag("human") {
                println!("{}", serde_json::to_string_pretty(&req)?);
            } else {
                println!("{}", serde_json::to_string(&req)?);
            }
            Ok(0)
        }
        Some(("validate-response", _sub)) => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            let resp: AiAuthoringResponse = serde_json::from_str(&buf)?;
            let req = resp
                .request_ref
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("requestRef not set in response"))?;
            let req_parsed: AiAuthoringRequest =
                serde_json::from_str(req)?;

            let result = director.validate_response(&req_parsed, &resp);
            if result.passed {
                if matches.get_flag("human") {
                    println!("Validation passed.");
                } else {
                    println!("{\"status\":\"ok\"}");
                }
                Ok(0)
            } else {
                let now = chrono::Utc::now().to_rfc3339();
                let diags = result.ranked_diagnostics();
                let schema = to_cli_error_schema(
                    diags,
                    director.spine().version().cloned(),
                    Some(req_parsed.object_kind.to_string()),
                    Some(req_parsed.tier.to_string()),
                    Some(req_parsed.phase as i32),
                    now,
                );

                let json = serde_json::to_string_pretty(&schema)?;
                eprintln!("{}", json);
                Ok(schema.exit_code)
            }
        }
        Some(("apply", sub)) => {
            let dry_run = sub.get_flag("dry-run");
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            let resp: AiAuthoringResponse = serde_json::from_str(&buf)?;
            let req = resp
                .request_ref
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("requestRef not set in response"))?;
            let req_parsed: AiAuthoringRequest =
                serde_json::from_str(req)?;

            let result = director.validate_response(&req_parsed, &resp);
            if !result.passed {
                let now = chrono::Utc::now().to_rfc3339();
                let diags = result.ranked_diagnostics();
                let schema = to_cli_error_schema(
                    diags,
                    director.spine().version().cloned(),
                    Some(req_parsed.object_kind.to_string()),
                    Some(req_parsed.tier.to_string()),
                    Some(req_parsed.phase as i32),
                    now,
                );
                let json = serde_json::to_string_pretty(&schema)?;
                eprintln!("{}", json);
                return Ok(schema.exit_code);
            }

            if dry_run {
                let actions =
                    crate::generate::plan_file_actions(&director, &req_parsed, &resp)?;
                let json = serde_json::to_string_pretty(&actions)?;
                println!("{}", json);
                Ok(0)
            } else {
                crate::generate::apply_file_actions(&director, &req_parsed, &resp)?;
                if matches.get_flag("human") {
                    println!("Apply completed successfully.");
                } else {
                    println!("{\"status\":\"ok\"}");
                }
                Ok(0)
            }
        }
        _ => {
            // Default: print help.
            let _ = Command::new("hpc-chat-director").print_help();
            Ok(2)
        }
    }
}
