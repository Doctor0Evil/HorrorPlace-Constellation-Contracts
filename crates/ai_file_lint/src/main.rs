// invariants_used: [CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI]
// metrics_used: [UEC, EMD, STCI, CDL, ARR]
// tiers: [standard, mature, research]
// deadledger_surface: [zkpproof_schema, verifiers_registry, bundle_attestation, agent_attestation]

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct LintConfig {
    root: PathBuf,
    // Directories where envelopes / logs are allowed.
    allowed_envelope_dirs: Vec<PathBuf>,
    // File extensions to skip (Markdown, images, binaries).
    skip_extensions: Vec<&'static str>,
    // Substrings that must not appear in normal repo files.
    banned_tokens: Vec<&'static str>,
}

#[derive(Debug)]
struct LintFinding {
    path: PathBuf,
    line_number: usize,
    token: String,
    line: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let root = match args.get(1) {
        Some(p) => PathBuf::from(p),
        None => {
            eprintln!("Usage: ai_file_lint <repo-root>");
            std::process::exit(2);
        }
    };

    let config = LintConfig {
        root: root.clone(),
        allowed_envelope_dirs: vec![
            root.join("ai_envelopes"),
            root.join(".ai_envelopes"),
        ],
        skip_extensions: vec![
            "md", "png", "jpg", "jpeg", "gif", "svg", "ico", "pdf", "zip", "tar",
            "gz", "bz2", "xz", "7z", "exe", "dll", "so", "dylib", "wasm",
        ],
        banned_tokens: vec![
            "ai_file_envelope",
            "aiauthoringcontract",
            "targetRepo",
            "target_repo",
            "targetPath",
            "target_path",
            "Files Generated:",
            "Target Repos:",
            "```",
            "[file:",
        ],
    };

    let mut findings = Vec::new();
    if let Err(err) = walk_and_lint(&config.root, &config, &mut findings) {
        eprintln!("Error while walking repository: {err}");
        std::process::exit(3);
    }

    if findings.is_empty() {
        println!("ai_file_lint: OK (no envelope or AI metadata detected in files)");
        std::process::exit(0);
    } else {
        eprintln!("ai_file_lint: FAILED (found {} violation(s))", findings.len());
        for f in &findings {
            eprintln!(
                "{}:{}: banned token {:?} in line:\n    {}",
                f.path.display(),
                f.line_number,
                f.token,
                f.line.trim_end()
            );
        }
        std::process::exit(1);
    }
}

fn walk_and_lint(
    path: &Path,
    config: &LintConfig,
    findings: &mut Vec<LintFinding>,
) -> io::Result<()> {
    if is_allowed_envelope_dir(path, config) {
        // Envelopes/logs are allowed here; skip linting.
        return Ok(());
    }

    let metadata = fs::metadata(path)?;
    if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();
            if should_skip_dir(&p) {
                continue;
            }
            walk_and_lint(&p, config, findings)?;
        }
    } else if metadata.is_file() {
        if should_skip_file(path, config) {
            return Ok(());
        }
        lint_file(path, config, findings)?;
    }
    Ok(())
}

fn is_allowed_envelope_dir(path: &Path, config: &LintConfig) -> bool {
    config
        .allowed_envelope_dirs
        .iter()
        .any(|allowed| path.starts_with(allowed))
}

fn should_skip_dir(path: &Path) -> bool {
    if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
        match name {
            ".git" | "target" | "node_modules" | "dist" | "build" | ".idea" | ".vscode" => true,
            _ => false,
        }
    } else {
        false
    }
}

fn should_skip_file(path: &Path, config: &LintConfig) -> bool {
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        let ext_lower = ext.to_ascii_lowercase();
        if config
            .skip_extensions
            .iter()
            .any(|e| e.eq_ignore_ascii_case(&ext_lower))
        {
            return true;
        }
    }

    // Always lint JSON, YAML, TOML, Rust, Lua, and other text-like files.
    // If you later need binary detection, you can add a heuristic here.
    false
}

fn lint_file(path: &Path, config: &LintConfig, findings: &mut Vec<LintFinding>) -> io::Result<()> {
    let content = fs::read_to_string(path)?;
    for (idx, line) in content.lines().enumerate() {
        for &token in &config.banned_tokens {
            if line.contains(token) {
                findings.push(LintFinding {
                    path: path.to_path_buf(),
                    line_number: idx + 1,
                    token: token.to_string(),
                    line: line.to_string(),
                });
            }
        }
    }
    Ok(())
}
