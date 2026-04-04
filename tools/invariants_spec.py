#!/usr/bin/env python3
"""invariants_spec.py — Canonical source of truth for Horror.Place identifiers.

This module defines the complete set of recognized invariant and metric
identifiers across the Horror.Place constellation.  Every other scanner
imports CANONICAL_IDENTIFIERS from here to decide whether a token is
valid.  Adding or removing an identifier here is the *only* sanctioned
way to change the canonical set.
"""

from __future__ import annotations

__version__ = "1.0.0"

# ── Historical Invariants (11) ───────────────────────────────────────
HISTORICAL_INVARIANTS: frozenset[str] = frozenset({
    "CIC",   # Constellation Integrity Coefficient
    "MDI",   # Manifest Drift Index
    "AOS",   # Archival Oscillation Score
    "RRM",   # Registry Reconciliation Metric
    "FCF",   # File Consistency Factor
    "SPR",   # Schema Propagation Ratio
    "RWF",   # Rewrite Frequency
    "DET",   # Data Entropy Threshold
    "HVF",   # Historical Variance Factor
    "LSG",   # Lineage Stability Gauge
    "SHCI",  # Structural Hash Consistency Index
})

# ── Entertainment Metrics (5) ────────────────────────────────────────
ENTERTAINMENT_METRICS: frozenset[str] = frozenset({
    "UEC",   # User Engagement Coefficient
    "EMD",   # Emotional Dispersion
    "STCI",  # Scenario Tension Curve Index
    "CDL",   # Content Density Level
    "ARR",   # Audience Retention Rate
})

# ── Combined canonical set (16) ──────────────────────────────────────
CANONICAL_IDENTIFIERS: frozenset[str] = HISTORICAL_INVARIANTS | ENTERTAINMENT_METRICS

# ── Helpers ──────────────────────────────────────────────────────────

def is_canonical(token: str) -> bool:
    """Return True if *token* is a recognized canonical identifier."""
    return token in CANONICAL_IDENTIFIERS


def classify(token: str) -> str | None:
    """Return 'historical' | 'entertainment' | None for a given token."""
    if token in HISTORICAL_INVARIANTS:
        return "historical"
    if token in ENTERTAINMENT_METRICS:
        return "entertainment"
    return None


if __name__ == "__main__":
    print(f"Canonical identifiers ({len(CANONICAL_IDENTIFIERS)}):")
    for ident in sorted(CANONICAL_IDENTIFIERS):
        print(f"  {ident:6s}  ({classify(ident)})")
