---
title: Dead‑Ledger Timing Algebra v1
version: 1.0.0
invariants_used: [CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI]
metrics_used: [UEC, EMD, STCI, CDL, ARR]
tiers: [standard, mature, research]
deadledger_surface: [bundle_attestation, agent_attestation, spectral_seed_attestation, bcistateproof, policy_profile]
description: "Canonical timing algebra, canonical recurrence, CSI/QDI formal definitions, timingPolicy schema snippets, and CI test templates for Dead‑Ledger enforcement."
---

### Overview

This draft formalizes a compact, engine‑agnostic timing algebra and the canonical definitions for **Cooldown Saturation Index (CSI)** and **Quiet Debt Index (QDI)** so they can be published as Dead‑Ledger‑governed policy artifacts and consumed by CHAT_DIRECTOR, pacing contracts, and engine bindings. Two sentences from the repository draft that anchor this file: “Cooldown Saturation Index measures how compressed or congested the scare / high-tension cooldown windows are within a session or session segment, relative to their ideal recovery durations.” “All timing signals defined here are normalized to the closed interval [0,1], with explicit semantics that can be expressed as JSON scalar ranges in the invariants/metrics spine.”

This file contains:
- **Canonical recurrence form** for derived timing signals.  
- **Formal CSI and QDI definitions** with bounded ranges, monotonicity constraints, and narrowing rules.  
- **`timingPolicy` JSON schema snippets** for Dead‑Ledger profiles and decision tokens.  
- **CI test templates** (kernel tests, golden‑file parity, telemetry replay assertions).  
- **Implementation checklist** and example bindings for Rust/Lua/C++/C#.

---

### Canonical timing algebra (discrete update form)

**Purpose.** Provide a small, auditable family of update rules that guarantee boundedness and simple monotonicity properties while remaining easy to implement across engines.

**Canonical discrete update (reference form).**



\[
Z_{t+1} = (1-\lambda) Z_t + \lambda \,\sigma(w\cdot x_t + b)
\]



- **Variables**
  - \(Z_t\): timing signal scalar at discrete step \(t\), \(Z_t \in [0,1]\).
  - \(\lambda \in (0,1]\): smoothing / responsiveness parameter; tier‑bounded.
  - \(x_t\): feature vector derived from telemetry (e.g., cooldownElapsedRatio, truncatedFraction, recentDET).
  - \(w,b\): kernel weights and bias; kernel‑family parameters.
  - \(\sigma(\cdot)\): squashing function mapping \(\mathbb{R}\to[0,1]\) (e.g., logistic or clipped linear).

**Guaranteed properties**
- **Boundedness.** If \(Z_0\in[0,1]\) and \(\sigma\) maps to \([0,1]\), then \(Z_t\in[0,1]\) for all \(t\).
- **Monotonicity (feature monotone).** For features that represent *more compression* or *more debt*, choose \(w\ge 0\) so that increasing those features cannot decrease \(Z\).
- **Tier constraints.** Each tier constrains \(\lambda\) and weight magnitudes to bound per‑tick drift.

**Implementation note.** Engines may implement this recurrence directly or call a shared timing kernel family that returns \(Z_{t+1}\) given \(Z_t\) and \(x_t\).

---

### Cooldown Saturation Index (CSI) — formal definition

**Symbol:** `CSI`  
**Storage path:** `timingSignals.CSI.value` (session‑experience envelope)  
**Domain:** \([0,1]\)  
**Type:** Derived metric (not authored in content contracts)

**Canonical feature set (suggested)**
- **scheduledCooldown_i**: planned cooldown duration for window \(i\).
- **actualElapsed_i**: elapsed time before next high‑intensity event for window \(i\).
- **truncatedFlag_i**: 1 if \(actualElapsed_i < \alpha \cdot scheduledCooldown_i\) (e.g., \(\alpha=0.5\)), else 0.
- **compressionRatio_i**: \(\min(1, actualElapsed_i / scheduledCooldown_i)\).

**Reference aggregation (windowed)**
1. Compute per‑window pressure \(p_i = 1 - compressionRatio_i\).
2. Compute truncated fraction \(T = \frac{\sum_i truncatedFlag_i}{N}\).
3. Compute mean pressure \(\bar p = \frac{1}{N}\sum_i p_i\).

**Canonical mapping (engine‑agnostic)**


\[
CSI = \sigma\big( \beta_T \cdot T + \beta_p \cdot \bar p + b \big)
\]


- \(\sigma\) is a monotone squashing function to \([0,1]\).
- \(\beta_T,\beta_p \ge 0\) ensure monotonicity: more truncation or higher compression increases CSI.

**Narrowing rules (spine‑encoded)**
- **Tier base ranges** (example):
  - `standard`: target band \([0.0,0.5]\)
  - `mature`: target band \([0.0,0.7]\)
  - `research`: target band \([0.0,1.0]\)
- **Cross‑metric interactions** (spine rules):
  - **TX001 DET–CSI Amplify:** If sustained \(DET > DET_{th}\) for \(t_{dur}\), then reduce CSI.max by \(\Delta\) (relative or absolute) for the affected phase.
  - **Policy gating:** If bcistate indicates overload, apply stricter CSI.max from `timingPolicy`.

**Monotonicity constraint**
- For any valid telemetry change that increases truncated fraction \(T\) or mean pressure \(\bar p\), CSI must not decrease.

**Example JSON schema fragment (spine entry)**
```json
"CSI": {
  "baseRange": [0.0, 1.0],
  "tierOverrides": {
    "standard": {"band": [0.0, 0.5]},
    "mature": {"band": [0.0, 0.7]},
    "research": {"band": [0.0, 1.0]}
  }
}
