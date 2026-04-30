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
- **Schema**: `bci-feature-envelope-v1.json`
- **Filename convention**: `*.bci-features.ndjson`
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

Each feature object in the `features[]` array MUST include:
- `featureId`: Unique identifier within the window
- `featureType`: One of the canonical enum values
- `channel`: Electrode/channel label (10-20 system preferred)
- `value`: Numeric feature value (range depends on type)

Optional but recommended:
- `bandPowers`: Object with delta/theta/alpha/beta/gamma sub-values
- `arousalScore`, `valenceScore`: Affective estimates in [0,1]

## Validation

All output MUST pass JSON Schema validation against `bci-feature-envelope-v1.json` before being written to the staging area.

## Example Flow

```bash
hpc-bci-convert \
  --input data/gameemo_session_042.edf \
  --output staging/session_042.bci-features.ndjson \
  --contract contracts/gameemo-converter-v1.json
```

## Prohibited Fields

The following fields are explicitly forbidden in output:
- `subjectId`, `playerId`, `userId`
- `rawWaveform`, `sampleArray`, `timeSeries`
- `demographics`, `medicalHistory`, `consentStatus`
- Any field not defined in the schema (`additionalProperties: false`)
