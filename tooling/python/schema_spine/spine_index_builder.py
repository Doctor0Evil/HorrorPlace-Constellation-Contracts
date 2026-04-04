"""
SpineIndexBuilder: Scans canonical JSON Schemas and generates a machine-readable spine index.
Extracts invariants, metrics, contract definitions, registry patterns, and consumer mappings.
"""

import json
import os
from pathlib import Path
from typing import Any, Dict, List, Optional
from datetime import datetime, timezone


class SpineIndexBuilder:
    """Builds and validates the schema-spine-index.json from a directory of JSON Schema files."""

    def __init__(self, schema_root: str | Path) -> None:
        self.schema_root = Path(schema_root)
        self.index: Dict[str, Any] = {
            "indexVersion": "1.0.0",
            "generatedAt": datetime.now(timezone.utc).isoformat(),
            "schemas": {},
            "invariants": {},
            "metrics": {},
            "contracts": {},
            "registries": {},
            "consumers": {},
        }

    def scan(self) -> Dict[str, Any]:
        """Walk the schema directory, parse files, and populate the index."""
        for schema_path in self.schema_root.rglob("*.json"):
            try:
                with open(schema_path, "r", encoding="utf-8") as f:
                    schema = json.load(f)
            except json.JSONDecodeError as e:
                print(f"[WARN] Skipping invalid JSON: {schema_path} ({e})")
                continue

            self._process_schema(schema, schema_path)

        self._deduplicate_consumers()
        return self.index

    def _process_schema(self, schema: dict, path: Path) -> None:
        schema_id = schema.get("$id", str(path.relative_to(self.schema_root)))
        title = schema.get("title", "Untitled")
        version = title.split(" v")[-1] if " v" in title else "0.0.0"

        # Track schema metadata
        self.index["schemas"][schema_id] = {
            "title": title,
            "version": version,
            "tier": schema.get("x-tier", "public"),
            "fields": self._extract_fields(schema),
            "consumedBy": [],
        }

        # Extract invariants
        if "invariant" in schema.get("x-tags", []):
            for prop_name, prop_def in schema.get("properties", {}).items():
                self.index["invariants"][prop_name] = {
                    "definedIn": schema_id,
                    "type": prop_def.get("type", "unknown"),
                    "range": [prop_def.get("minimum", 0), prop_def.get("maximum", 1)],
                    "semantics": prop_def.get("description", ""),
                    "usedInContracts": [],
                    "usedInRegistries": [],
                }

        # Extract metrics
        if "metric" in schema.get("x-tags", []):
            for prop_name, prop_def in schema.get("properties", {}).items():
                self.index["metrics"][prop_name] = {
                    "definedIn": schema_id,
                    "type": prop_def.get("type", "unknown"),
                    "range": [prop_def.get("minimum", 0), prop_def.get("maximum", 1)],
                    "semantics": prop_def.get("description", ""),
                    "telemetryEnvelope": "session-metrics-envelope.v1.json",
                    "usedInContracts": [],
                }

        # Identify contract & registry schemas
        if "contract" in str(schema.get("title", "")).lower():
            contract_type = schema_id.split("/")[-1].replace(".v1.json", "")
            self.index["contracts"][contract_type] = {
                "schemaRef": schema_id,
                "version": version,
                "requiredFields": schema.get("required", []),
                "consumedBy": [],
            }
        elif "registry" in schema_id:
            registry_type = schema_id.split("/")[-1].replace("registry-", "").replace(".v1.json", "")
            self.index["registries"][registry_type] = {
                "schemaRef": schema_id,
                "idPattern": r"^[A-Z]{2,4}-[A-Z0-9]{3,6}-\d{4}$",
                "requiredRefs": ["deadledgerref", "artifactid"],
                "maintainedBy": [],
            }

    def _extract_fields(self, schema: dict) -> dict:
        """Extract simplified field definitions from a schema for indexing."""
        fields = {}
        for prop_name, prop_def in schema.get("properties", {}).items():
            fields[prop_name] = {
                "type": prop_def.get("type", "object"),
                "required": prop_name in schema.get("required", []),
                "range": [prop_def.get("minimum", 0), prop_def.get("maximum", 1)]
                if "minimum" in prop_def or "maximum" in prop_def
                else None,
            }
        return fields

    def _deduplicate_consumers(self) -> None:
        """Placeholder for consumer aggregation logic. In production, parses .horrorplace-contracts.json files."""
        pass

    def save(self, output_path: str | Path) -> None:
        """Write the generated index to a JSON file."""
        out = Path(output_path)
        out.parent.mkdir(parents=True, exist_ok=True)
        with open(out, "w", encoding="utf-8") as f:
            json.dump(self.index, f, indent=2)
        print(f"[OK] Spine index written to {out}")
