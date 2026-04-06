//! CLI subcommand wiring and argument parsing.
//!
//! Provides the user-facing interface for CHAT_DIRECTOR, supporting
//! machine-readable JSON output by default and structured error reporting.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::errors::Error;

#[derive(Parser)]
#[command(name = "hpc-chat-director")]
#[command(about = "HorrorPlace Schema-Driven Authoring Compiler", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Initialize the constellation environment (detect spine, manifests).
    Init {
        /// Path to the constellation root directory.
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
    },
    /// Normalize a prompt into an AiAuthoringRequest JSON.
    Plan {
        /// The prompt or intent string.
        prompt: String,
        /// Optional target repository override.
        #[arg(long)]
        target_repo: Option<String>,
    },
    /// Validate an AiAuthoringResponse against the full constraint stack.
    ValidateResponse {
        /// Path to the request JSON file.
        request_file: PathBuf,
        /// Path to the response JSON file.
        response_file: PathBuf,
    },
    /// Apply a validated file to disk.
    Apply {
        /// Path to the validated file JSON.
        validated_file: PathBuf,
        /// Preview changes without writing to disk.
        #[arg(long, default_value = "false")]
        dry_run: bool,
    },
    /// Describe capabilities of an objectKind, phase, or tier.
    Describe {
        /// Filter by objectKind (e.g., "moodContract").
        #[arg(long)]
        object_kind: Option<String>,
        /// Filter by phase (e.g., 2).
        #[arg(long)]
        phase: Option<u8>,
    },
}

/// Run the CLI based on parsed arguments.
pub fn run() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { root } => crate::cli::init::run(&root),
        Command::Plan { prompt, target_repo } => crate::cli::plan::run(&prompt, target_repo.as_deref()),
        Command::ValidateResponse { request_file, response_file } => {
            crate::cli::validate_response::run(&request_file, &response_file)
        }
        Command::Apply { validated_file, dry_run } => {
            crate::cli::apply::run(&validated_file, dry_run)
        }
        Command::Describe { object_kind, phase } => crate::cli::describe::run(object_kind.as_deref(), phase),
    }
}
