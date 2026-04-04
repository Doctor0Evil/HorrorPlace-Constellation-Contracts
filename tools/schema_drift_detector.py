#!/usr/bin/env python3
"""schema_drift_detector.py — Scan repos for schema drift.

Walks a repository tree looking at .json, .ndjson, .lua, and .rs files
for non-canonical identifier usage, missing schema structure, schema
openness violations, and non-canonical H.* API calls.

Exit codes:
    0 — clean, no findings
    1 — non-critical findings present
    2 — critical findings present
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Iterator

from invariants_spec import CANONICAL_IDENTIFIERS

__version__ = "1.0.0"

REQUIRED_SCHEMA_KEYS = {"$id", "type", "properties", "required", "additionalProperties"}
IDENTIFIER_PATTERN = re.compile(r"\b([A-Z]{2,5})\b")
LUA_H_API_PATTERN = re.compile(r"H\.(\w+)\s*\(")
RUST_TOKEN_PATTERN = re.compile(
    r"(?:Invariant|Metric)::([\w]+)"
)


@dataclass
class Finding:
    file: str
    line: int
    rule: str
    severity: str          # "critical" | "warning" | "info"
    message: str
    suggestion: str = ""


@dataclass
class ScanResult:
    repo_path: str
    findings: list[Finding] = field(default_factory=list)

    @property
    def has_critical(self) -> bool:
        return any(f.severity == "critical" for f in self.findings)

    @property
    def exit_code(self) -> int:
        if not self.findings:
            return 0
        return 2 if self.has_critical else 1


def _iter_files(root: Path, extensions: set[str]) -> Iterator[Path]:
    """Yield files matching *extensions* under *root*, skipping hidden dirs."""
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if not d.startswith(".")]
        for fname in filenames:
            if Path(fname).suffix in extensions:
                yield Path(dirpath) / fname


def _check_json_schema(filepath: Path, result: ScanResult) -> None:
    """Validate JSON Schema files under core/schemas/."""
    if "core/schemas" not in str(filepath):
        return
    try:
        with open(filepath, "r", encoding="utf-8") as fh:
            data = json.load(fh)
    except (json.JSONDecodeError, UnicodeDecodeError):
        return

    if not isinstance(data, dict):
        return

    rel = str(filepath.relative_to(result.repo_path))

    # Check for missing required keys
    missing = REQUIRED_SCHEMA_KEYS - set(data.keys())
    if missing:
        severity = "critical" if "$id" in missing else "warning"
        result.findings.append(Finding(
            file=rel, line=1, rule="schema-structure",
            severity=severity,
            message=f"Schema missing required keys: {', '.join(sorted(missing))}",
            suggestion=f"Add missing keys to {rel}.",
        ))

    # Check additionalProperties
    if data.get("additionalProperties") is not False:
        result.findings.append(Finding(
            file=rel, line=1, rule="schema-openness",
            severity="warning",
            message="Schema does not set additionalProperties: false.",
            suggestion='Set "additionalProperties": false for contract safety.',
        ))

    # Check properties for non-canonical identifiers
    props = data.get("properties", {})
    for key in props:
        if IDENTIFIER_PATTERN.fullmatch(key) and key not in CANONICAL_IDENTIFIERS:
            result.findings.append(Finding(
                file=rel, line=1, rule="non-canonical-id",
                severity="critical",
                message=f'Non-canonical identifier "{key}" in schema properties.',
                suggestion=f"Remove or replace \"{key}\" with a canonical identifier.",
            ))


def _check_json_data(filepath: Path, result: ScanResult) -> None:
    """Scan non-schema JSON / NDJSON files for non-canonical identifiers."""
    rel = str(filepath.relative_to(result.repo_path))
    try:
        with open(filepath, "r", encoding="utf-8") as fh:
            for lineno, line in enumerate(fh, start=1):
                for match in IDENTIFIER_PATTERN.finditer(line):
                    token = match.group(1)
                    if len(token) >= 2 and token not in CANONICAL_IDENTIFIERS:
                        # Heuristic: skip common English words / abbreviations
                        if token in {"ID", "OK", "URL", "URI", "API", "JSON",
                                     "HTTP", "HTML", "CSS", "UTF", "NULL",
                                     "TRUE", "FALSE", "NDJSON", "YAML", "TOML",
                                     "GIT", "SSH", "GPG", "EOF", "BOM", "ISO",
                                     "RFC", "UUID", "MIME", "BASE", "RAW"}:
                            continue
                        result.findings.append(Finding(
                            file=rel, line=lineno, rule="non-canonical-id",
                            severity="warning",
                            message=f'Possible non-canonical identifier "{token}".',
                            suggestion="Verify against invariants_spec.py.",
                        ))
    except (UnicodeDecodeError, PermissionError):
        pass


def _check_lua(filepath: Path, result: ScanResult) -> None:
    """Scan Lua files for H.* API calls referencing non-canonical IDs."""
    rel = str(filepath.relative_to(result.repo_path))
    try:
        with open(filepath, "r", encoding="utf-8") as fh:
            for lineno, line in enumerate(fh, start=1):
                for match in LUA_H_API_PATTERN.finditer(line):
                    token = match.group(1)
                    if token.upper() not in CANONICAL_IDENTIFIERS:
                        result.findings.append(Finding(
                            file=rel, line=lineno, rule="lua-noncanonical-api",
                            severity="critical",
                            message=f'H.{token}() references non-canonical identifier.',
                            suggestion=f"Replace H.{token}() with a canonical H.* call.",
                        ))
    except (UnicodeDecodeError, PermissionError):
        pass


def _check_rust(filepath: Path, result: ScanResult) -> None:
    """Scan Rust files for Invariant::/Metric:: tokens."""
    rel = str(filepath.relative_to(result.repo_path))
    try:
        with open(filepath, "r", encoding="utf-8") as fh:
            for lineno, line in enumerate(fh, start=1):
                for match in RUST_TOKEN_PATTERN.finditer(line):
                    token = match.group(1).upper()
                    if token not in CANONICAL_IDENTIFIERS:
                        result.findings.append(Finding(
                            file=rel, line=lineno, rule="rust-noncanonical-token",
                            severity="critical",
                            message=f'Rust source references non-canonical token "{token}".',
                            suggestion=f"Update to a canonical Invariant/Metric variant.",
                        ))
    except (UnicodeDecodeError, PermissionError):
        pass


def scan(repo_path: str) -> ScanResult:
    """Run all schema-drift checks on *repo_path* and return findings."""
    root = Path(repo_path).resolve()
    result = ScanResult(repo_path=str(root))

    for fp in _iter_files(root, {".json"}):
        _check_json_schema(fp, result)
        _check_json_data(fp, result)

    for fp in _iter_files(root, {".ndjson"}):
        _check_json_data(fp, result)

    for fp in _iter_files(root, {".lua"}):
        _check_lua(fp, result)

    for fp in _iter_files(root, {".rs"}):
        _check_rust(fp, result)

    return result


def main() -> None:
    parser = argparse.ArgumentParser(description="Schema Drift Detector")
    parser.add_argument("repo", help="Path to repository root")
    parser.add_argument("--json", action="store_true", help="Output JSON")
    args = parser.parse_args()

    result = scan(args.repo)

    if args.json:
        print(json.dumps([asdict(f) for f in result.findings], indent=2))
    else:
        for f in result.findings:
            print(f"[{f.severity.upper()}] {f.file}:{f.line} — {f.message}")
        if not result.findings:
            print("No schema drift findings.")

    sys.exit(result.exit_code)


if __name__ == "__main__":
    main()
