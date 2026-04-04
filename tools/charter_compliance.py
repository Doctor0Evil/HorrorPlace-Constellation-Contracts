#!/usr/bin/env python3
"""charter_compliance.py — Rivers of Blood Charter enforcement scanner.

Detects forbidden file types, base64 blobs, sensitive URLs, data-URIs,
content descriptors, and binary masquerade files.

Exit codes:
    0 — clean
    1 — non-critical findings
    2 — critical findings
    3 — charter violations (worst case)
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

# ── Forbidden binary / media extensions ──────────────────────────────
FORBIDDEN_EXTENSIONS: frozenset[str] = frozenset({
    ".mp3", ".mp4", ".avi", ".mov", ".mkv", ".wav", ".flac", ".ogg",
    ".exe", ".bin", ".dll", ".so", ".dylib",
    ".zip", ".tar", ".gz", ".rar", ".7z",
    ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".tiff", ".webp",
    ".psd", ".ai", ".sketch",
})

# ── Base64 blob detection (100+ chars) ───────────────────────────────
BASE64_PATTERN = re.compile(
    r"(?:data:[a-zA-Z0-9/+.-]+;base64,)?[A-Za-z0-9+/]{100,}={0,2}"
)

# ── Sensitive URL patterns ───────────────────────────────────────────
SENSITIVE_URL_PATTERNS: list[re.Pattern] = [
    re.compile(r"https?://(?:.*\.)?tier[23]\.horror\.place", re.IGNORECASE),
    re.compile(r"https?://(?:.*\.)?internal\.horror\.place", re.IGNORECASE),
    re.compile(r"https?://(?:.*\.)?staging\.horror\.place", re.IGNORECASE),
    re.compile(r"https?://10\.\d{1,3}\.\d{1,3}\.\d{1,3}"),
    re.compile(r"https?://192\.168\.\d{1,3}\.\d{1,3}"),
    re.compile(r"https?://172\.(?:1[6-9]|2\d|3[01])\.\d{1,3}\.\d{1,3}"),
]

# ── Data-URI pattern ─────────────────────────────────────────────────
DATA_URI_PATTERN = re.compile(
    r"data:(?:image|audio|video|application)/[a-zA-Z0-9.+-]+;base64,"
)

# ── Content descriptor patterns ──────────────────────────────────────
CONTENT_DESCRIPTOR_PATTERNS: list[re.Pattern] = [
    re.compile(r"\bgraphic\s+depiction\b", re.IGNORECASE),
    re.compile(r"\bNSFW\b"),
    re.compile(r"\braw\s+content\b", re.IGNORECASE),
    re.compile(r"\bexplicit\s+(?:content|material|imagery)\b", re.IGNORECASE),
    re.compile(r"\bgore\s+(?:content|imagery|depiction)\b", re.IGNORECASE),
    re.compile(r"\bunredacted\b", re.IGNORECASE),
]

# ── Text-like extensions for masquerade check ────────────────────────
TEXT_EXTENSIONS: frozenset[str] = frozenset({
    ".json", ".ndjson", ".txt", ".md", ".lua", ".rs", ".toml", ".yaml",
    ".yml", ".cfg", ".ini", ".csv", ".xml", ".html",
})


@dataclass
class Finding:
    file: str
    line: int
    rule: str
    severity: str          # "critical" | "charter" | "warning" | "info"
    message: str
    requires_review: bool = False


@dataclass
class ScanResult:
    repo_path: str
    findings: list[Finding] = field(default_factory=list)

    @property
    def has_charter_violations(self) -> bool:
        return any(f.severity == "charter" for f in self.findings)

    @property
    def has_critical(self) -> bool:
        return any(f.severity in ("critical", "charter") for f in self.findings)

    @property
    def exit_code(self) -> int:
        if not self.findings:
            return 0
        if self.has_charter_violations:
            return 3
        if self.has_critical:
            return 2
        return 1


def _iter_all_files(root: Path) -> Iterator[Path]:
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if not d.startswith(".")]
        for fname in filenames:
            yield Path(dirpath) / fname


def _is_binary(filepath: Path, sample_size: int = 8192) -> bool:
    """Heuristic: file is binary if it contains null bytes in first *sample_size* bytes."""
    try:
        with open(filepath, "rb") as fh:
            chunk = fh.read(sample_size)
            return b"\x00" in chunk
    except (PermissionError, OSError):
        return False


def scan(repo_path: str) -> ScanResult:
    root = Path(repo_path).resolve()
    result = ScanResult(repo_path=str(root))

    for fp in _iter_all_files(root):
        rel = str(fp.relative_to(root))
        ext = fp.suffix.lower()

        # ── Forbidden extensions ─────────────────────────────────
        if ext in FORBIDDEN_EXTENSIONS:
            result.findings.append(Finding(
                file=rel, line=0, rule="forbidden-filetype",
                severity="charter",
                message=f"Forbidden file type: {ext}",
                requires_review=True,
            ))
            continue

        # ── Binary masquerade ────────────────────────────────────
        if ext in TEXT_EXTENSIONS and _is_binary(fp):
            result.findings.append(Finding(
                file=rel, line=0, rule="binary-masquerade",
                severity="charter",
                message=f"Binary content masquerading as {ext} file.",
                requires_review=True,
            ))
            continue

        # ── Text-content scanning ────────────────────────────────
        if ext not in TEXT_EXTENSIONS:
            continue

        try:
            with open(fp, "r", encoding="utf-8") as fh:
                for lineno, line in enumerate(fh, start=1):
                    # Base64 blobs
                    if BASE64_PATTERN.search(line):
                        result.findings.append(Finding(
                            file=rel, line=lineno, rule="base64-blob",
                            severity="critical",
                            message="Base64 blob exceeding 100 characters detected.",
                            requires_review=True,
                        ))

                    # Sensitive URLs
                    for pattern in SENSITIVE_URL_PATTERNS:
                        if pattern.search(line):
                            result.findings.append(Finding(
                                file=rel, line=lineno, rule="sensitive-url",
                                severity="critical",
                                message="URL pointing to sensitive/internal domain.",
                                requires_review=True,
                            ))
                            break

                    # Data-URIs
                    if DATA_URI_PATTERN.search(line):
                        result.findings.append(Finding(
                            file=rel, line=lineno, rule="data-uri",
                            severity="charter",
                            message="Data-URI with embedded media content.",
                            requires_review=True,
                        ))

                    # Content descriptors
                    for pattern in CONTENT_DESCRIPTOR_PATTERNS:
                        if pattern.search(line):
                            result.findings.append(Finding(
                                file=rel, line=lineno, rule="content-descriptor",
                                severity="charter",
                                message=f"Content descriptor matched: {pattern.pattern}",
                                requires_review=True,
                            ))
                            break
        except (UnicodeDecodeError, PermissionError):
            pass

    return result


def main() -> None:
    parser = argparse.ArgumentParser(description="Charter Compliance Scanner")
    parser.add_argument("repo", help="Path to repository root")
    parser.add_argument("--json", action="store_true", help="Output JSON")
    args = parser.parse_args()

    result = scan(args.repo)

    if args.json:
        print(json.dumps([asdict(f) for f in result.findings], indent=2))
    else:
        for f in result.findings:
            flag = " [REQUIRES REVIEW]" if f.requires_review else ""
            print(f"[{f.severity.upper()}] {f.file}:{f.line} — {f.message}{flag}")
        if not result.findings:
            print("No charter compliance findings.")

    sys.exit(result.exit_code)


if __name__ == "__main__":
    main()
