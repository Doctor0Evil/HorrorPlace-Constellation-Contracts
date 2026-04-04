"""
RegistryLinter: Validates NDJSON registry files against canonical schemas and governance rules.
"""

import json
import re
from pathlib import Path
from typing import Any, Dict, List, Tuple


class RegistryLinter:
    """Lint NDJSON registries for schema compliance, ID patterns, and reference validity."""

    ID_PATTERN = re.compile(r"^[A-Z]{2,4}-[A-Z0-9]{3,6}-\d{4}$")
    DEAD_LEDGER_PATTERN = re.compile(r"^zkp:sha256:[a-f0-9]{64}(:sig:ed25519:[a-f0-9]{128})?(:tier:(public|vault|lab))?$")
    REQUIRED_FIELDS = {"id", "schemaref", "deadledgerref", "artifactid", "createdAt", "status"}

    def __init__(self) -> None:
        self.errors: List[Dict[str, Any]] = []
        self.warnings: List[Dict[str, Any]] = []

    def lint_file(self, filepath: Path, schema: dict | None = None) -> bool:
        """Lint a single NDJSON file. Returns True if valid, False otherwise."""
        file_errors = []
        seen_ids = set()

        with open(filepath, "r", encoding="utf-8") as f:
            for line_num, line in enumerate(f, 1):
                line = line.strip()
                if not line:
                    continue

                try:
                    entry = json.loads(line)
                except json.JSONDecodeError as e:
                    self.errors.append({"file": str(filepath), "line": line_num, "msg": f"Invalid JSON: {e}"})
                    continue

                self._validate_entry(entry, filepath, line_num, seen_ids)

        return len([e for e in self.errors if e.get("file") == str(filepath)]) == 0

    def _validate_entry(self, entry: dict, filepath: Path, line: int, seen_ids: set) -> None:
        # Required fields
        missing = self.REQUIRED_FIELDS - entry.keys()
        if missing:
            self.errors.append({"file": str(filepath), "line": line, "msg": f"Missing required fields: {', '.join(missing)"})

        # ID format
        entry_id = entry.get("id", "")
        if not self.ID_PATTERN.match(entry_id):
            self.errors.append({"file": str(filepath), "line": line, "msg": f"Invalid ID format: '{entry_id}'"})
        if entry_id in seen_ids:
            self.errors.append({"file": str(filepath), "line": line, "msg": f"Duplicate ID: '{entry_id}'"})
        seen_ids.add(entry_id)

        # deadledgerref format
        ref = entry.get("deadledgerref", "")
        if not self.DEAD_LEDGER_PATTERN.match(ref):
            self.warnings.append({"file": str(filepath), "line": line, "msg": f"Non-compliant deadledgerref: '{ref[:20]}...'"})

        # Tier consistency
        tier = entry.get("tier", "public")
        if "vault" in ref and tier != "vault":
            self.errors.append({"file": str(filepath), "line": line, "msg": "Tier mismatch: vault-tier ref but non-vault tier"})

    def get_report(self) -> dict:
        return {
            "valid": len(self.errors) == 0,
            "errors": self.errors,
            "warnings": self.warnings,
            "counts": {"errors": len(self.errors), "warnings": len(self.warnings)},
        }
