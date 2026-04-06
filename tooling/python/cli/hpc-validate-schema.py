#!/usr/bin/env python3
"""
hpc-validate-schema.py

Thin CLI wrapper to run JSON Schema Draft 2020-12 validation on schema files
under schemas/**/*.json.

Features:
- Validates one or more schema files.
- Resolves $ref using the schema's $id/id and file-based resolution.
- Emits structured diagnostics with file, jsonPointer, error code, and message.
- Supports JSONL output for AI/CI consumption.
- Non-zero exit code on any validation error.
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional, Tuple

from jsonschema import Draft202012Validator, RefResolver  # type: ignore


REPO_ROOT_SENTINELS = {".git", ".github"}


@dataclass
class Diagnostic:
    severity: str  # "error" or "warning"
    file: str
    code: str
    message: str
    json_pointer: Optional[str] = None
    field: Optional[str] = None
    value: Optional[Any] = None
    context: Optional[Dict[str, Any]] = None


def _find_repo_root(start: Path) -> Path:
    current = start
    while current != current.parent:
        if any((current / s).exists() for s in REPO_ROOT_SENTINELS):
            return current
        current = current.parent
    return start


def _load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def _collect_schema_paths(repo_root: Path, explicit: Optional[List[Path]] = None) -> List[Path]:
    if explicit:
        return [p for p in explicit if p.is_file()]

    base = repo_root / "schemas"
    if not base.is_dir():
        return []

    paths = sorted(base.rglob("*.json"))
    return [p for p in paths if p.is_file()]


def _build_schema_store(schema_paths: Iterable[Path]) -> Dict[str, Dict[str, Any]]:
    """
    Build a simple in-memory store mapping schema IDs and file URIs to schema dicts.

    This lets RefResolver resolve $ref both by $id and by relative path.
    """
    store: Dict[str, Dict[str, Any]] = {}

    for path in schema_paths:
        try:
            schema = _load_json(path)
        except Exception:
            continue

        file_uri = path.resolve().as_uri()
        store[file_uri] = schema

        schema_id = schema.get("$id") or schema.get("id")
        if isinstance(schema_id, str):
            store[schema_id] = schema

    return store


def _make_validator_for_schema(
    schema: Dict[str, Any],
    base_uri: str,
    store: Dict[str, Dict[str, Any]],
) -> Draft202012Validator:
    resolver = RefResolver(base_uri=base_uri, referrer=schema, store=store)
    return Draft202012Validator(schema, resolver=resolver)


def _validate_schema_file(
    path: Path,
    store: Dict[str, Dict[str, Any]],
) -> List[Diagnostic]:
    diagnostics: List[Diagnostic] = []

    try:
        schema = _load_json(path)
    except Exception as exc:
        diagnostics.append(
            Diagnostic(
                severity="error",
                file=str(path),
                code="INVALID_JSON",
                message=f"Schema file is not valid JSON: {exc}",
            )
        )
        return diagnostics

    base_uri = path.resolve().as_uri()
    validator = _make_validator_for_schema(schema, base_uri, store)

    try:
        validator.check_schema(schema)
    except Exception as exc:
        diagnostics.append(
            Diagnostic(
                severity="error",
                file=str(path),
                code="SCHEMA_SELF_INVALID",
                message=f"Schema violates JSON Schema meta-schema: {exc}",
            )
        )
        return diagnostics

    for error in validator.iter_errors({}):
        pointer = "/".join(
            [""] + list(error.absolute_path)
        ) if error.absolute_path else ""
        field = str(error.path[-1]) if error.path else None

        hint = _derive_remediation_hint(error.validator, error.validator_value)
        context = {
            "validator": error.validator,
            "validator_value": error.validator_value,
        }
        if hint is not None:
            context["hint"] = hint

        diagnostics.append(
            Diagnostic(
                severity="error",
                file=str(path),
                code="SCHEMA_REFERENCE_ERROR",
                message=error.message,
                json_pointer=pointer or None,
                field=field,
                value=error.instance,
                context=context,
            )
        )

    return diagnostics


def _derive_remediation_hint(validator: str, validator_value: Any) -> Optional[str]:
    """
    Lightweight remediation hints for common schema issues.
    """
    if validator == "type":
        return "Check that the 'type' keyword is a valid JSON Schema type or array of types."
    if validator == "$ref":
        return "Ensure the $ref target exists and its id/path is correct relative to this schema."
    if validator == "required":
        return "Verify that all required properties are defined under 'properties'."
    if validator == "enum":
        return "Ensure the value is one of the allowed 'enum' entries."
    if validator == "format":
        return "Confirm the value matches the declared 'format' or remove the format constraint."
    return None


def _print_diagnostics(
    diagnostics: List[Diagnostic],
    json_output: bool,
) -> None:
    if json_output:
        for d in diagnostics:
            obj = asdict(d)
            compact = {k: v for k, v in obj.items() if v is not None}
            sys.stdout.write(json.dumps(compact, ensure_ascii=False) + "\n")
    else:
        for d in diagnostics:
            location = f"{d.file}"
            if d.json_pointer:
                location += f"{d.json_pointer}"
            sys.stdout.write(f"[{d.severity.upper()}] {d.code} at {location}: {d.message}\n")
            if d.context and "hint" in d.context:
                sys.stdout.write(f"  hint: {d.context['hint']}\n")


def _parse_args(argv: Optional[List[str]] = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Validate JSON Schema files (Draft 2020-12) under schemas/**/*.json."
    )
    parser.add_argument(
        "paths",
        nargs="*",
        help="Optional schema file paths to validate. If omitted, validates all schemas/**/*.json.",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Emit diagnostics as JSON objects (one per line) for AI/CI consumption.",
    )
    return parser.parse_args(argv)


def main(argv: Optional[List[str]] = None) -> int:
    args = _parse_args(argv)

    repo_root = _find_repo_root(Path.cwd())

    explicit_paths = [Path(p) for p in args.paths] if args.paths else None
    schema_paths = _collect_schema_paths(repo_root, explicit_paths)

    if not schema_paths:
        sys.stderr.write("No schema files found to validate.\n")
        return 0

    store = _build_schema_store(schema_paths)

    all_diagnostics: List[Diagnostic] = []
    for path in schema_paths:
        diags = _validate_schema_file(path, store)
        all_diagnostics.extend(diags)

    _print_diagnostics(all_diagnostics, json_output=args.json)

    has_errors = any(d.severity == "error" for d in all_diagnostics)
    return 1 if has_errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
