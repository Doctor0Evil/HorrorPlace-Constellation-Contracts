"""
schema_spine: Core validation and index-building utilities for HorrorPlace-Constellation-Contracts.

Provides:
- Spine index generation from canonical JSON Schemas
- NDJSON registry linting and reference validation
- AI authoring contract validation and prismMeta enforcement
"""

__version__ = "1.0.0"
__author__ = "HorrorPlace Constellation Contributors"

from .spine_index_builder import SpineIndexBuilder
from .registry_linter import RegistryLinter
from .ai_authoring_validator import AIAuthoringValidator

__all__ = [
    "SpineIndexBuilder",
    "RegistryLinter",
    "AIAuthoringValidator",
]
