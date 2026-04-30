# BCI Conversion Contract v1

## Overview

This document specifies the contract for converting external EEG/neuroimaging datasets into the canonical `bci-feature-envelope-v1` format used by the Horror.Place BCI pipeline.

## Input Formats

Supported input formats (implementation-agnostic):
- **EDF/BDF**: European Data Format / Biosemi variant
- **FIF**: MNE-Python native format
- **XDF**: Extensible Data Format for multimodal streams
- **CSV**: Comma-separated values with header row specifying channel names

## Output Format

- **Format**: NDJSON (one JSON object per line)
- **Schema**: `bci-feature-envelope-v1.json` (canonical schema ID: `schema:HorrorPlace-Constellation-Contracts:bci-feature-envelope-v1.json`)
- **Filename convention**: `*.bci-features.ndjson` (non-normative; schema ID is canonical)
- **Encoding**: UTF-8, no BOM

## Mandatory Anonymization

All converters MUST:
1. Strip subject identifiers, player IDs, and any PII
2. Exclude raw waveform arrays or sample-level data
3. Remove demographic fields (age, gender, medical history)
4. Replace session identifiers with anonymized UUIDs

## License Compliance

Converters MUST validate that:
- The source dataset license permits derived feature extraction
- Attribution requirements are documented in output metadata
- Commercial use restrictions are respected in downstream pipelines

## Feature Extraction Requirements

Each feature object in the `features[]` array MUST conform to the `oneOf` schema:
- **ScalarFeature**: `featureType` in ["arousal_score", "valence_score", "asymmetry_index", "coherence_metric"] with `value` in [-100, 100]
- **BandPowersFeature**: `featureType` in ["bandpower_delta", "bandpower_theta", "bandpower_alpha", "bandpower_beta", "bandpower_gamma"] with `bandPowers` object containing delta/theta/alpha/beta/gamma in [0,1]

## Validation

All output MUST pass JSON Schema validation against `bci-feature-envelope-v1.json` before being written to the staging area. Use the `bci_schema_validator` CLI tool in CI.

## Example Flow

```bash
hpc-bci-convert \
  --input data/gameemo_session_042.edf \
  --output staging/session_042.bci-features.ndjson \
  --contract contracts/gameemo-converter-v1.json

# Validate output
bci-schema-validator \
  --schema schemas/telemetry/bci-feature-envelope-v1.json \
  --input staging/session_042.bci-features.ndjson
```

## Prohibited Fields

Enforced via `additionalProperties: false` in schema:
- `subjectId`, `playerId`, `userId`
- `rawWaveform`, `sampleArray`, `timeSeries`
- `demographics`, `medicalHistory`, `consentStatus`
- Any field not explicitly defined in the schema
