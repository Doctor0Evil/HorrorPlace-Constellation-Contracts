# New Entertainment Metric Specification

## 1. Metric Identity

- **Metric Code**: `FAS`
- **Full Name**: Fright Anticipation Score
- **Domain**: entertainment-metrics
- **Description (1–2 sentences)**:
  <!-- Describe what this metric measures and when it changes. -->

- **Units / Range**:
  - Numerical range: [0.0, 1.0] (or other; specify)
  - Monotonicity: (e.g., higher = more anticipation)
  - Sampling: per-frame / per-session / per-event

- **Primary Use Cases**:
  - (e.g., anticipatory build-up before jumpscares)
  - (e.g., gating high-detachment patterns)

---

## 2. JSON Schema Updates

### 2.1 Canonical schema files

- [ ] `schemas/entertainment-metrics-v1.json`  
  - Add `FAS` property under `properties` with:
    - type: `number`
    - minimum: `0.0`
    - maximum: `1.0`
    - description: (copy from above)
  - Add to `required`? (yes/no + rationale)

- [ ] Any envelope schemas that must carry this metric:
  - [ ] `schemas/bci-metrics-envelope-v1.json`
  - [ ] `schemas/bci-feature-envelope-v1.json`
  - [ ] Others: ____________________

For each schema, specify the exact JSON snippet to insert:

```jsonc
"FAS": {
  "type": "number",
  "minimum": 0.0,
  "maximum": 1.0,
  "description": "Fright anticipation score; higher = sustained anticipation without release."
}
```

---

## 3. Rust Struct Updates

### 3.1 Core metric structs

- [ ] Crate: `crates/bci_metrics`  
  File(s):
  - [ ] `src/entertainment_metrics.rs`
  - [ ] `src/bci_metrics_envelope.rs` (if applicable)

Add fields:

```rust
pub struct EntertainmentMetrics {
    // existing fields ...
    pub fas: f32, // Fright Anticipation Score [0.0, 1.0]
}
```

Update:

- [ ] Serde derives (`#[derive(Serialize, Deserialize)]`) field attributes if needed.
- [ ] Default implementations (`impl Default`) including reasonable `fas` default.
- [ ] Any conversion traits (e.g. `From<Dto>` / `Into<Dto>`).

### 3.2 Pattern / scheduler structs

List all structs that must see the new metric:

- [ ] `BciSummary`
- [ ] Scheduler inputs (`PodSchedulingContext`, etc.)
- [ ] Any formula runtime contexts

For each, specify:

```rust
pub struct BciSummary {
    // ...
    pub fright_anticipation_score: f32, // mirrors FAS
}
```

---

## 4. C++ Header Updates

List all C++ interfaces that expose metrics to engines:

- [ ] Repo: `HorrorPlace-Codebase-of-Death`
  - [ ] `include/bci/entertainment_metrics.hpp`
  - [ ] `include/bci/bci_envelope.hpp`
  - [ ] Other headers: ____________________

For each header:

- Add `float fas;` (or `double`, per convention).
- Update:
  - [ ] Constructors
  - [ ] Serialization / deserialization
  - [ ] ABI‑stable structs used by game/engine boundaries

Example:

```cpp
struct EntertainmentMetrics {
    // ...
    float fas; // Fright Anticipation Score [0.0, 1.0]
};
```

---

## 5. SQL Schema & Migrations

### 5.1 Core metrics tables

List tables where `FAS` must be stored:

- [ ] `entertainment_metrics`
- [ ] `bcirequestframe` (if denormalized)
- [ ] Any analytics summary tables / views

For each table, define migrations:

```sql
ALTER TABLE entertainment_metrics
  ADD COLUMN fas REAL DEFAULT 0.0 NOT NULL;

-- Optional: add to views
CREATE OR REPLACE VIEW entertainment_metrics_summary AS
SELECT
    session_id,
    AVG(fas) AS avg_fas,
    -- existing aggregates...
FROM entertainment_metrics
GROUP BY session_id;
```

- [ ] Update any queries that select all metrics (`SELECT *` vs explicit fields).

---

## 6. Mapping Formulas & Integration

### 6.1 Formulacatalog entries

For patterns that should depend on `FAS`, define candidate formulas:

- Patterns: (e.g., `corpsebloom`, `hanging-skin`)
- Parameters to adjust:
  - `motionSmear`
  - `decayGrain`
  - Audio (`heartbeatGain`, `breathGain`)

Example formula for catalog:

- Parameter: `motionSmear` for `corpsebloom`
- Symbolic:  
  \[
    mS = 0.20 + 0.40 V + 0.25 FAS
  \]

Record for `formulacatalog`:

```sql
INSERT INTO formulacatalog
    (patternid, paramcode, symbolicexpr, formulatype, rsquared, dependson)
VALUES
    (:patternid, 'mS', '0.20 + 0.40 V + 0.25 FAS', 'linear', 0.98, '["V", "FAS"]');
```

### 6.2 Scheduler / gates

Specify how `FAS` influences gates / objectives:

- [ ] Modify entertainment gate formulas (if needed).
- [ ] Add `FAS` to scheduler objective functions (e.g. prefer patterns that build anticipation when `FAS` < target).

Example pseudo‑objective:

\[
C_{p,b} = E_{p,b} - Q_{p,b} + \lambda (FAS_{target} - FAS_{b})^2
\]

---

## 7. Telemetry & Validation

- [ ] Log `FAS` to telemetry with:
  - timestamps,
  - pattern choices,
  - UEC, EMD, STCI, CDL, ARR.
- [ ] Define sanity checks:
  - `0.0 <= FAS <= 1.0`
  - Correlation expectations (e.g., FAS rises before spikes in ARR).

Checklist:

- [ ] All JSON schemas updated and validated.
- [ ] Rust / C++ build passes.
- [ ] SQL migrations applied and queries updated.
- [ ] Formulacatalog entries added.
- [ ] Telemetry dashboards updated to show `FAS`.
