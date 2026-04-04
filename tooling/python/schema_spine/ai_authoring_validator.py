"""
AIAuthoringValidator: Enforces contract card rules, prismMeta consistency, and AI-generation constraints.
"""

import json
from pathlib import Path
from typing import Any, Dict, List


class AIAuthoringValidator:
    """Validates AI-generated contract cards against structural and business rules."""

    def __init__(self) -> None:
        self.errors: List[str] = []
        self.warnings: List[str] = []

    def validate(self, contract_path: Path, schema: dict | None = None) -> bool:
        """Run validation checks on a contract card file."""
        try:
            with open(contract_path, "r", encoding="utf-8") as f:
                contract = json.load(f)
        except Exception as e:
            self.errors.append(f"Failed to parse {contract_path}: {e}")
            return False

        self._check_prism_meta(contract, contract_path)
        self._check_tier_gating(contract, contract_path)
        self._check_cross_field_consistency(contract, contract_path)
        
        return len(self.errors) == 0

    def _check_prism_meta(self, contract: dict, path: Path) -> None:
        meta = contract.get("prismMeta")
        if not meta:
            self.errors.append(f"{path}: Missing 'prismMeta' block")
            return

        linkage = meta.get("linkage", {})
        required_linkage = {"targetRepo", "targetPath", "schemaRef", "tier"}
        if missing := required_linkage - linkage.keys():
            self.errors.append(f"{path}: prismMeta.linkage missing: {missing}")

        # Consistency with top-level fields
        if contract.get("targetRepo") != linkage.get("targetRepo"):
            self.errors.append(f"{path}: targetRepo mismatch between top-level and prismMeta.linkage")

    def _check_tier_gating(self, contract: dict, path: Path) -> None:
        tier = contract.get("tier")
        ref = contract.get("deadledgerref", "")
        if tier in ("vault", "lab") and not ref:
            self.errors.append(f"{path}: {tier}-tier contract requires deadledgerref")
        if tier not in ("public", "vault", "lab"):
            self.errors.append(f"{path}: Invalid tier value: '{tier}'")

    def _check_cross_field_consistency(self, contract: dict, path: Path) -> None:
        # Example business rule: path must end with .json or .ndjson
        p = contract.get("path", "")
        if not p.endswith((".json", ".ndjson")):
            self.warnings.append(f"{path}: path does not end with .json or .ndjson: '{p}'")

    def get_report(self) -> dict:
        return {
            "valid": len(self.errors) == 0,
            "errors": self.errors,
            "warnings": self.warnings,
        }
