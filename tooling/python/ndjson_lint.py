#!/usr/bin/env python3
"""ndjson_lint.py — NDJSON registry validator.

Checks .ndjson files for JSON parse errors, object type enforcement,
ID presence/uniqueness, inline assets, encoding issues, and optional
JSON Schema validation.

Exit codes:
    0 — clean
    1 — non-critical findings
    2 — critical findings
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

__version__ = "1.0.0"

DATA_URI_PATTERN = re.compile(
    r"data:(?:image|audio|video|application)/[a-zA-Z0-9.+-]+;base64,"
)


@dataclass
class Finding:
    file: str
    line: int
    rule: str
    severity: str
    message: str


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


def _iter_ndjson(root: Path) -> Iterator[Path]:
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if not d.startswith(".")]
        for fname in filenames:
            if fname.endswith(".ndjson"):
                yield Path(dirpath) / fname


def _find_schema(ndjson_path: Path, repo_root: Path) -> dict | None:
    """Look for a companion JSON Schema in core/schemas/."""
    schema_dir = repo_root / "core" / "schemas"
    if not schema_dir.is_dir():
        return None
    schema_name = ndjson_path.stem + ".schema.json"
    schema_path = schema_dir / schema_name
    if schema_path.is_file():
        try:
            with open(schema_path, "r", encoding="utf-8") as fh:
                return json.load(fh)
        except (json.JSONDecodeError, UnicodeDecodeError):
            return None
    return None


def _validate_against_schema(record: dict, schema: dict) -> list[str]:
    """Minimal validation: check required fields and type constraints."""
    errors = []
    required = schema.get("required", [])
    properties = schema.get("properties", {})

    for key in required:
        if key not in record:
            errors.append(f"Missing required field: {key}")

    for key, value in record.items():
        if key in properties:
            expected_type = properties[key].get("type")
            if expected_type == "string" and not isinstance(value, str):
                errors.append(f"Field '{key}' expected string, got {type(value).__name__}")
            elif expected_type == "number" and not isinstance(value, (int, float)):
                errors.append(f"Field '{key}' expected number, got {type(value).__name__}")
            elif expected_type == "integer" and not isinstance(value, int):
                errors.append(f"Field '{key}' expected integer, got {type(value).__name__}")
            elif expected_type == "boolean" and not isinstance(value, bool):
                errors.append(f"Field '{key}' expected boolean, got {type(value).__name__}")
            elif expected_type == "array" and not isinstance(value, list):
                errors.append(f"Field '{key}' expected array, got {type(value).__name__}")
            elif expected_type == "object" and not isinstance(value, dict):
                errors.append(f"Field '{key}' expected object, got {type(value).__name__}")

    # Check additionalProperties: false
    if schema.get("additionalProperties") is False:
        extra = set(record.keys()) - set(properties.keys())
        for key in extra:
            errors.append(f"Unexpected additional property: {key}")

    return errors


def scan(repo_path: str) -> ScanResult:
    root = Path(repo_path).resolve()
    result = ScanResult(repo_path=str(root))

    for fp in _iter_ndjson(root):
        rel = str(fp.relative_to(root))
        seen_ids: dict[str, int] = {}
        schema = _find_schema(fp, root)

        # ── Check BOM ────────────────────────────────────────────
        try:
            with open(fp, "rb") as fh:
                bom = fh.read(3)
                if bom == b"\xef\xbb\xbf":
                    result.findings.append(Finding(
                        file=rel, line=1, rule="utf8-bom",
                        severity="warning",
                        message="File begins with UTF-8 BOM.",
                    ))
        except (PermissionError, OSError):
            continue

        # ── Line-by-line validation ──────────────────────────────
        try:
            with open(fp, "r", encoding="utf-8") as fh:
                lines = fh.readlines()
        except UnicodeDecodeError:
            result.findings.append(Finding(
                file=rel, line=1, rule="encoding-error",
                severity="critical",
                message="File is not valid UTF-8.",
            ))
            continue

        if not lines:
            continue

        # ── Trailing newline check ───────────────────────────────
        if lines and not lines[-1].endswith("\n"):
            result.findings.append(Finding(
                file=rel, line=len(lines), rule="missing-trailing-newline",
                severity="info",
                message="File does not end with a trailing newline.",
            ))

        for lineno, raw_line in enumerate(lines, start=1):
            line = raw_line.rstrip("\n").rstrip("\r")

            if not line.strip():
                continue  # skip blank lines

            # ── JSON parse check ─────────────────────────────────
            try:
                record = json.loads(line)
            except json.JSONDecodeError as exc:
                result.findings.append(Finding(
                    file=rel, line=lineno, rule="json-parse-error",
                    severity="critical",
                    message=f"Invalid JSON: {exc.msg}",
                ))
                continue

            # ── Object type enforcement ──────────────────────────
            if not isinstance(record, dict):
                result.findings.append(Finding(
                    file=rel, line=lineno, rule="non-object-line",
                    severity="critical",
                    message=f"Expected JSON object, got {type(record).__name__}.",
                ))
                continue

            # ── ID presence & uniqueness ────────────────────────
            record_id = record.get("id") or record.get("_id") or record.get("ID")
            if record_id is None:
                result.findings.append(Finding(
                    file=rel, line=lineno, rule="missing-id",
                    severity="warning",
                    message="Record has no opaque ID field (id, _id, or ID).",
                ))
            else:
                id_str = str(record_id)
                if id_str in seen_ids:
                    result.findings.append(Finding(
                        file=rel, line=lineno, rule="duplicate-id",
                        severity="critical",
                        message=f'Duplicate ID "{id_str}" (first seen line {seen_ids[id_str]}).',
                    ))
                else:
                    seen_ids[id_str] = lineno

            # ── Inline data-URI detection ────────────────────────
            if DATA_URI_PATTERN.search(line):
                result.findings.append(Finding(
                    file=rel, line=lineno, rule="inline-asset",
                    severity="warning",
                    message="Inline base64 data-URI detected in registry entry.",
                ))

            # ── Optional schema validation ───────────────────────
            if schema is not None:
                errors = _validate_against_schema(record, schema)
                for err in errors:
                    result.findings.append(Finding(
                        file=rel, line=lineno, rule="schema-validation",
                        severity="warning",
                        message=err,
                    ))

    return result


def main() -> None:
    parser = argparse.ArgumentParser(description="NDJSON Registry Linter")
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
            print("No NDJSON lint findings.")

    sys.exit(result.exit_code)


if __name__ == "__main__":
    main()
