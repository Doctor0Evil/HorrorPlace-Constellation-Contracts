mod envelope;
mod file_invariants;
mod session;
mod validation;

use clap::Parser;
use serde_json::from_reader;
use std::fs::File;
use std::io::{self, Read};

use crate::envelope::AuthoringEnvelope;
use crate::file_invariants::FileTypeInvariants;
use crate::validation::{LintContext, LintError};

#[derive(Parser, Debug)]
#[command(name = "ai-authoring-linter")]
#[command(about = "Lints AI authoring envelopes against session contracts and file-type invariants.")]
struct Cli {
    /// Path to the envelope JSON file. If omitted, reads from stdin.
    #[arg(long)]
    envelope: Option<String>,

    /// Path to the file-type invariants JSON file.
    #[arg(long, default_value = "contracts/authoring/file-type-invariants.core.v1.json")]
    file_type_invariants: String,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("ai-authoring-linter: {e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), LintError> {
    let envelope: AuthoringEnvelope = {
        if let Some(path) = cli.envelope {
            let file = File::open(&path).map_err(|e| LintError::Io {
                path: path.clone(),
                source: e,
            })?;
            from_reader(file).map_err(|e| LintError::Json { source: e })?
        } else {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .map_err(|e| LintError::Io {
                    path: "<stdin>".to_string(),
                    source: e,
                })?;
            serde_json::from_str(&buf).map_err(|e| LintError::Json { source: e })?
        }
    };

    envelope.basic_validate()?;

    let session = &envelope.session;
    let invariants = FileTypeInvariants::load_from_path(&cli.file_type_invariants)?;

    let ctx = LintContext {
        envelope: &envelope,
        session,
        invariants: &invariants,
    };

    ctx.validate_all()?;
    Ok(())
}
