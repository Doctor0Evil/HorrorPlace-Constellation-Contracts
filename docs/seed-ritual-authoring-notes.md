# Seed-Ritual Authoring Notes (v1)

This guide gives concrete numeric examples for `stageMetricBands` and `stageIntensityEnvelopes` so authors can avoid common CI failures from `tools/lint-seed-ritual-stage-bands.py`.

---

## 1. STCI/CDL Monotonicity into Confrontation

The linter expects `STCI_min` and `CDL_min` to be **non-decreasing** from Probe → Evidence → Confrontation.

### 1.1 Good pattern (passes)

```json
"stageMetricBands": {
  "probe": {
    "UEC_min": 0.10,
    "EMD_min": 0.05,
    "STCI_min": 0.10,
    "CDL_min": 0.08,
    "ARR_min": 0.10,
    "ARR_max": 0.40
  },
  "evidence": {
    "UEC_min": 0.20,
    "EMD_min": 0.10,
    "STCI_min": 0.15,
    "CDL_min": 0.10,
    "ARR_min": 0.15,
    "ARR_max": 0.50
  },
  "confrontation": {
    "UEC_min": 0.30,
    "EMD_min": 0.20,
    "STCI_min": 0.20,
    "CDL_min": 0.15,
    "ARR_min": 0.20,
    "ARR_max": 0.60
  },
  "aftermath": { "...": "..." },
  "residual": { "...": "..." }
}
```

- `STCI_min`: 0.10 → 0.15 → 0.20 (non-decreasing)
- `CDL_min`: 0.08 → 0.10 → 0.15 (non-decreasing)

CI result: **OK** (no monotonicity violations).

### 1.2 Bad pattern (fails)

```json
"stageMetricBands": {
  "probe": {
    "STCI_min": 0.20,
    "CDL_min": 0.15,
    "ARR_min": 0.10,
    "ARR_max": 0.40
  },
  "evidence": {
    "STCI_min": 0.25,
    "CDL_min": 0.12,
    "ARR_min": 0.15,
    "ARR_max": 0.50
  },
  "confrontation": {
    "STCI_min": 0.22,
    "CDL_min": 0.18,
    "ARR_min": 0.20,
    "ARR_max": 0.60
  },
  "aftermath": { "...": "..." },
  "residual": { "...": "..." }
}
```

- `STCI_min`: 0.20 → 0.25 → 0.22 (**drops** at Confrontation)
- `CDL_min`: 0.15 → 0.12 → 0.18 (**drops** at Evidence)

Typical CI messages:

- `STCI_min monotonicity violated: evidence=0.25 -> confrontation=0.22`
- `CDL_min monotonicity violated: probe=0.15 -> evidence=0.12`

**Fix**: Either keep Confrontation ≥ Evidence, or flatten values when you want a softer plateau:

```json
"evidence":     { "STCI_min": 0.22, "CDL_min": 0.15, ... },
"confrontation":{ "STCI_min": 0.22, "CDL_min": 0.18, ... }
```

---

## 2. Probe Intensity: Non-Zero Floors

If you define a Probe intensity profile in `stageIntensityEnvelopes.probe`, both `audioIntensity.min` and `visualIntensity.min` must be **strictly > 0.0**.

### 2.1 Good pattern (passes)

```json
"stageIntensityEnvelopes": {
  "probe": {
    "audioIntensity":  { "min": 0.10, "max": 0.30 },
    "visualIntensity": { "min": 0.08, "max": 0.25 }
  },
  "evidence": {
    "audioIntensity":  { "min": 0.20, "max": 0.50 },
    "visualIntensity": { "min": 0.15, "max": 0.45 }
  },
  "confrontation": { "...": "..." },
    "aftermath":   { "...": "..." },
  "residual":      { "...": "..." }
}
```

Probe starts with a low but non-zero rumble and visual presence. CI result: **OK**.

### 2.2 Bad pattern (fails)

```json
"stageIntensityEnvelopes": {
  "probe": {
    "audioIntensity":  { "min": 0.0, "max": 0.30 },
    "visualIntensity": { "min": 0.0, "max": 0.25 }
  },
  "evidence": { "...": "..." },
  "confrontation": { "...": "..." },
  "aftermath": { "...": "..." },
  "residual": { "...": "..." }
}
```

Typical CI messages:

- `Probe audioIntensity.min=0.0 must be > 0`
- `Probe visualIntensity.min=0.0 must be > 0`

**Fix options**:

- If you want atmosphere from frame one, set low non-zero floors (e.g., 0.05–0.10).
- If you truly want silence at start, **omit** the Probe envelope entirely:

```json
"stageIntensityEnvelopes": {
  "probe": {},
  "evidence": { ... },
  "confrontation": { ... },
  "aftermath": { ... },
  "residual": { ... }
}
```

(Leaving `probe` absent in practice means the engine falls back to global defaults; do not declare an explicit zero envelope.)

---

## 3. ARR_min and Charter Floors

The linter enforces a global Charter minimum `ARR_min` (configured in `CHARter_ARR_MIN` inside `tools/lint-seed-ritual-stage-bands.py`). Any stage with `ARR_min` below this threshold will fail CI.

For illustration, assume:

```python
CHARter_ARR_MIN = 0.05
```

### 3.1 Good pattern (passes)

```json
"stageMetricBands": {
  "probe": {
    "ARR_min": 0.05,
    "ARR_max": 0.30,
    "UEC_min": 0.10,
    "EMD_min": 0.05,
    "STCI_min": 0.10,
    "CDL_min": 0.08
  },
  "evidence": {
    "ARR_min": 0.07,
    "ARR_max": 0.40,
    "UEC_min": 0.15,
    "EMD_min": 0.08,
    "STCI_min": 0.12,
    "CDL_min": 0.10
  },
  "confrontation": {
    "ARR_min": 0.10,
    "ARR_max": 0.60,
    "UEC_min": 0.20,
    "EMD_min": 0.12,
    "STCI_min": 0.15,
    "CDL_min": 0.12
  },
  "aftermath": { "ARR_min": 0.06, "ARR_max": 0.30, "...": "..." },
  "residual":  { "ARR_min": 0.05, "ARR_max": 0.20, "...": "..." }
}
```

All `ARR_min` values are ≥ 0.05. CI result: **OK**.

### 3.2 Bad pattern (fails)

```json
"stageMetricBands": {
  "probe": {
    "ARR_min": 0.02,
    "ARR_max": 0.30,
    "UEC_min": 0.10,
    "EMD_min": 0.05,
    "STCI_min": 0.10,
    "CDL_min": 0.08
  },
  "evidence": {
    "ARR_min": 0.04,
    "ARR_max": 0.40,
    "UEC_min": 0.15,
    "EMD_min": 0.08,
    "STCI_min": 0.12,
    "CDL_min": 0.10
  },
  "confrontation": { "...": "..." },
  "aftermath": { "...": "..." },
  "residual": { "...": "..." }
}
```

Typical CI messages:

- `probe ARR_min=0.02 is below Charter minimum 0.05`
- `evidence ARR_min=0.04 is below Charter minimum 0.05`

**Fix**: Raise `ARR_min` to at least the Charter value (0.05 in this example), then re-run CI.

---

## 4. Quick Author Checklist Before Committing

When editing a Seed-Ritual:

1. **STCI/CDL into Confrontation**

   - Check `stageMetricBands.probe/evidence/confrontation`:
     - `STCI_min` is non-decreasing.
     - `CDL_min` is non-decreasing.

2. **Probe intensity**

   - If `stageIntensityEnvelopes.probe` exists:
     - Ensure `audioIntensity.min > 0.0`.
     - Ensure `visualIntensity.min > 0.0`.

3. **ARR_min ≥ Charter minimum**

   - For every stage, confirm `ARR_min` is at or above the configured Charter minimum.
   - If unsure of the exact value, inspect `CHARter_ARR_MIN` in `tools/lint-seed-ritual-stage-bands.py`.

4. **Run the linter locally**

   From the repo root:

   ```bash
   python tools/lint-seed-ritual-stage-bands.py schemas/seed-ritual
   ```

   Fix any reported violations before pushing or opening a PR.

Keeping these patterns in mind will prevent most Seed-Ritual CI failures and make it easier for Module-10 and `H.shouldtriggersequence` to guarantee the intended horror envelopes at runtime.
