use thiserror::Error;

use crate::envelope::{AuthoringEnvelope, EnvelopeFile};
use crate::file_invariants::{FileTypeInvariants, NamingPattern};
use crate::session::{AuthoringSession, AuthoringTarget, InvariantConstraints};
use regex::Regex;
use serde_json::Value;

pub struct MetricEnvelope {
    pub det: Option<f64>,
    pub cdl_min: Option<f64>,
    pub cdl_max: Option<f64>,
    pub arr_min: Option<f64>,
    pub arr_max: Option<f64>,
    pub intensity_band: Option<String>,
}

#[derive(Debug, Error)]
pub enum LintError {
    #[error("schemaRef mismatch: expected {expected}, found {found}")]
    SchemaRefMismatch { expected: String, found: String },

    #[error("objectKind mismatch: expected {expected}, found {found}")]
    ObjectKindMismatch { expected: String, found: String },

    #[error("envelope has no files")]
    NoFilesInEnvelope,

    #[error("I/O error reading {path}: {source}")]
    Io { path: String, source: std::io::Error },

    #[error("JSON parse error: {source}")]
    Json { source: serde_json::Error },

    #[error("session validation error: {0}")]
    Session(String),

    #[error("fileType invariants violation: {0}")]
    FileType(String),

    #[error("envelope/files violation: {0}")]
    Files(String),
}

pub struct LintContext<'a> {
    pub envelope: &'a AuthoringEnvelope,
    pub session: &'a AuthoringSession,
    pub invariants: &'a FileTypeInvariants,
}

impl<'a> LintContext<'a> {
    pub fn validate_all(&self) -> Result<(), LintError> {
        self.session
            .basic_validate()
            .map_err(LintError::Session)?;

        self.envelope.basic_validate()?;

        self.validate_file_counts()?;
        self.validate_files_against_session()?;
        self.validate_files_against_invariants()?;
        self.validate_entertainment_metrics()?;

        Ok(())
    }

    fn validate_file_counts(&self) -> Result<(), LintError> {
        let max_files = self.session.max_files_per_session;
        let actual_files = self.envelope.files.len() as i64;

        if actual_files > max_files {
            return Err(LintError::Files(format!(
                "envelope contains {actual_files} files, session.maxFilesPerSession = {max_files}"
            )));
        }

        if let Some(max_lines_per_file) = self.session.max_lines_per_file {
            for f in &self.envelope.files {
                let added = f.lines_added.unwrap_or(0);
                if added > max_lines_per_file {
                    return Err(LintError::Files(format!(
                        "file {} exceeds maxLinesPerFile ({} > {})",
                        f.target_path, added, max_lines_per_file
                    )));
                }
            }
        }

        Ok(())
    }

    fn validate_files_against_session(&self) -> Result<(), LintError> {
        let allowed_ops = &self.session.allowed_operations;
        let allowed_kinds = &self.session.allowed_file_kinds;

        for file in &self.envelope.files {
            if !allowed_ops.contains(&file.operation) {
                return Err(LintError::Files(format!(
                    "file {} operation '{}' not in session.allowedOperations",
                    file.target_path, file.operation
                )));
            }

            if !allowed_kinds.contains(&file.file_kind) {
                return Err(LintError::Files(format!(
                    "file {} fileKind '{}' not in session.allowedFileKinds",
                    file.target_path, file.file_kind
                )));
            }

            let target = find_matching_target(&self.session.targets, file).ok_or_else(|| {
                LintError::Files(format!(
                    "file {} not covered by any session.targets entry (kind={}, objectKind={})",
                    file.target_path, file.file_kind, file.object_kind
                ))
            })?;

            if target.operation != file.operation {
                return Err(LintError::Files(format!(
                    "file {} operation '{}' does not match session.target.operation '{}'",
                    file.target_path, file.operation, target.operation
                )));
            }
        }

        Ok(())
    }

    fn validate_files_against_invariants(&self) -> Result<(), LintError> {
        let sha_re = Regex::new(r"^[A-Fa-f0-9]{64}$").unwrap();

        for file in &self.envelope.files {
            if let Some(ref prev) = file.previous_sha256 {
                if !sha_re.is_match(prev) {
                    return Err(LintError::Files(format!(
                        "file {} has invalid previousSha256",
                        file.target_path
                    )));
                }
            }

            if let Some(ref content_hash) = file.content_sha256 {
                if !sha_re.is_match(content_hash) {
                    return Err(LintError::Files(format!(
                        "file {} has invalid contentSha256",
                        file.target_path
                    )));
                }
            }

            let pattern = self
                .invariants
                .find_pattern_for(&file.file_kind, &file.object_kind)
                .ok_or_else(|| {
                    LintError::FileType(format!(
                        "no namingPattern for fileKind='{}', objectKind='{}'",
                        file.file_kind, file.object_kind
                    ))
                })?;

            validate_target_path_against_pattern(&file.target_path, &pattern)?;
        }

        Ok(())
    }

    fn validate_entertainment_metrics(&self) -> Result<(), LintError> {
        let inv = match &self.session.invariant_constraints {
            Some(c) => c,
            None => {
                // No caps declared for this session; skip metric checks.
                return Ok(());
            }
        };

        for file in &self.envelope.files {
            // Heuristic: only inspect JSON contracts.
            if !file.target_path.ends_with(".json") {
                continue;
            }

            let value: Value = match serde_json::from_str(&file.content) {
                Ok(v) => v,
                Err(_) => {
                    // Non‑contract JSON or freeform; ignore.
                    continue;
                }
            };

            let metrics = extract_metric_envelope(&value);

            if let (Some(max_det), Some(mech_det)) = (inv.max_det, metrics.det) {
                if mech_det > max_det {
                    return Err(LintError::Files(format!(
                        "file {}: detCaps.max ({}) exceeds session.invariantConstraints.maxDET ({})",
                        file.target_path, mech_det, max_det
                    )));
                }
            }

            if let (Some(max_cdl), Some(cdl_max)) = (inv.max_cdl, metrics.cdl_max) {
                if cdl_max > max_cdl {
                    return Err(LintError::Files(format!(
                        "file {}: metricTargets.CDL.max ({}) exceeds session.invariantConstraints.maxCDL ({})",
                        file.target_path, cdl_max, max_cdl
                    )));
                }
            }

            if let (Some(min_arr), Some(arr_min)) = (inv.min_arr, metrics.arr_min) {
                if arr_min < min_arr {
                    return Err(LintError::Files(format!(
                        "file {}: metricTargets.ARR.min ({}) is below session.invariantConstraints.minARR ({})",
                        file.target_path, arr_min, min_arr
                    )));
                }
            }

            if let Some(ref band) = metrics.intensity_band {
                self.check_intensity_band(band, &metrics, inv, file)?;
            }
        }

        Ok(())
    }

    fn check_intensity_band(
        &self,
        band: &str,
        metrics: &MetricEnvelope,
        inv: &InvariantConstraints,
        file: &EnvelopeFile,
    ) -> Result<(), LintError> {
        match band {
            "mild" => {
                if let (Some(max_det), Some(det)) = (inv.max_det, metrics.det) {
                    if det > max_det * 0.5 {
                        return Err(LintError::Files(format!(
                            "file {}: intensityBand 'mild' but detCaps.max ({}) > 0.5 * session.maxDET ({})",
                            file.target_path, det, max_det
                        )));
                    }
                }

                if let (Some(session_arr_min), Some(arr_min)) = (inv.min_arr, metrics.arr_min) {
                    if arr_min < session_arr_min {
                        return Err(LintError::Files(format!(
                            "file {}: intensityBand 'mild' but metricTargets.ARR.min ({}) < session.minARR ({})",
                            file.target_path, arr_min, session_arr_min
                        )));
                    }
                }
            }
            "severe" | "adult" => {
                // Hook for future stricter policies; for now, rely on numeric caps.
            }
            _ => {}
        }

        Ok(())
    }
}

fn extract_metric_envelope(doc: &Value) -> MetricEnvelope {
    let det_caps = doc.get("detCaps");
    let det = det_caps
        .and_then(|d| d.get("max"))
        .and_then(|v| v.as_f64());

    let metric_targets = doc.get("metricTargets");

    let (cdl_min, cdl_max) = metric_targets
        .and_then(|mt| mt.get("CDL"))
        .and_then(|cdl| {
            let min = cdl.get("min").and_then(|v| v.as_f64());
            let max = cdl.get("max").and_then(|v| v.as_f64());
            Some((min, max))
        })
        .unwrap_or((None, None));

    let (arr_min, arr_max) = metric_targets
        .and_then(|mt| mt.get("ARR"))
        .and_then(|arr| {
            let min = arr.get("min").and_then(|v| v.as_f64());
            let max = arr.get("max").and_then(|v| v.as_f64());
            Some((min, max))
        })
        .unwrap_or((None, None));

    let intensity_band = doc
        .get("intensityBand")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    MetricEnvelope {
        det,
        cdl_min,
        cdl_max,
        arr_min,
        arr_max,
        intensity_band,
    }
}

fn find_matching_target<'a>(
    targets: &'a [AuthoringTarget],
    file: &EnvelopeFile,
) -> Option<&'a AuthoringTarget> {
    targets.iter().find(|t| {
        t.file_kind == file.file_kind
            && t.object_kind == file.object_kind
            && t.target_path == file.target_path
    })
}

fn validate_target_path_against_pattern(
    target_path: &str,
    pattern: &NamingPattern,
) -> Result<(), LintError> {
    if !target_path.ends_with(&pattern.extension) {
        return Err(LintError::FileType(format!(
            "targetPath '{}' does not end with required extension '{}'",
            target_path, pattern.extension
        )));
    }

    if !pattern.pattern.contains('{') {
        if pattern.pattern != target_path {
            return Err(LintError::FileType(format!(
                "targetPath '{}' does not match fixed pattern '{}'",
                target_path, pattern.pattern
            )));
        }
        return Ok(());
    }

    let prefix = pattern
        .pattern
        .split('{')
        .next()
        .unwrap_or_default();

    if !target_path.starts_with(prefix) {
        return Err(LintError::FileType(format!(
            "targetPath '{}' does not start with required prefix '{}'",
            target_path, prefix
        )));
    }

    Ok(())
}
