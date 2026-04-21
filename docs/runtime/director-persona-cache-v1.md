---
invariants-used: [CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI]
metrics-used: [UEC, EMD, STCI, CDL, ARR]
modules: [H.Director, H.Consent, H.Budget, H.Safety]
schema-refs:
  - director-persona-contract.v1.json
  - director-comfort-policy.v1.json
  - consent-state-machine.v1.json
  - horror-intensity-budget-policy.v1.json
---

# Director Persona Cache Lifecycle v1

This document defines the cold/warm cache strategy for `H.Director.loadPersona`, including revision-stamp tracking, invalidation rules, and telemetry hooks. The goal is to keep repeated persona lookups cheap while guaranteeing that mid-session consent downgrades or budget soft-caps are reflected immediately.

---

## 1. Cache goals and constraints

The persona cache must satisfy:

- **Correctness**: Never return a stale envelope when consent, budget, or comfort policy has changed.
- **Performance**: Avoid re-parsing contracts or re-computing invariant intersections on every turn.
- **Encapsulation**: Never expose raw vault tables; only derived, runtime envelopes.
- **Auditability**: Log cache hits/misses/invalidations for offline analysis.

The cache is keyed by `sessionId` and holds a single entry per active session.

---

## 2. Cache key and envelope structure

### 2.1 Cache key

```lua
cacheKey = sessionId  -- opaque string, e.g., "sess-abc123"
```

### 2.2 Cache entry shape

Each entry stores:

```lua
{
  -- Immutable persona contract data (cold path only)
  personaContract = { ... },      -- full directorPersonaContract
  comfortPolicy   = { ... },      -- resolved directorComfortPolicy

  -- Derived, runtime envelopes (recomputed on warm path)
  invariantEnvelope = {           -- intersected bands for CIC/AOS/DET/SHCI
    CIC  = { min = 0.5, max = 0.8 },
    AOS  = { min = 0.4, max = 0.9 },
    DET  = { min = 3.0, max = 6.0 },
    SHCI = { min = 0.7, max = 1.0 },
  },
  metricEnvelope = {              -- intersected bands for UEC/EMD/STCI/CDL/ARR
    UEC  = { min = 0.4, max = 0.7 },
    EMD  = { min = 0.1, max = 0.3 },
    STCI = { min = 0.4, max = 0.7 },
    CDL  = { min = 0.5, max = 0.7 },
    ARR  = { min = 0.5, max = 0.7 },
  },

  -- Revision stamps for invalidation
  revision = {
    personaVersion   = "v1.0.0",   -- from personaContract.schemaRef
    consentVersion   = 42,         -- monotonic counter from H.Consent
    budgetVersion    = 17,         -- monotonic counter from H.Budget
    comfortVersion   = "dcp-xyz",  -- from comfortPolicy.policyId
  },

  -- Metadata
  loadedAt = 1713654321,          -- Unix epoch seconds
  lastUsed = 1713654321,
}
```

---

## 3. Cold path: fetch + validate + derive

When `H.Director.loadPersona(sessionContext, directorPersonaId)` is called and no cache entry exists (or the entry is invalid), the cold path executes:

1. **Load persona contract** via `H.Contract.load("directorPersonaContract", directorPersonaId)`.
2. **Validate tier compatibility**: ensure `sessionContext.tier <= personaContract.allowedTier`.
3. **Load comfort policy** via `H.Contract.load("directorComfortPolicy", personaContract.comfortPolicyRef)`.
4. **Check consent compatibility**: intersect persona invariant bands with consent caps from `H.Consent.currentState(sessionId)`.
5. **Compute initial envelopes**:
   - `invariantEnvelope` = intersection of persona bands, consent caps, and ethics guardrails.
   - `metricEnvelope` = intersection of persona metricTargets, consent caps, and comfort policy limits.
6. **Stamp the entry** with current `consentVersion` and `budgetVersion` from modules 1 and 3.
7. **Store in cache** keyed by `sessionId`.
8. **Log telemetry**: `director-persona-cache-cold` event with personaId, envelope hashes, and revision stamps.

If any step fails (e.g., `DIRECTOR_NOT_FOUND`, `CONSENT_INCOMPATIBLE`), the cache entry is cleared and the caller receives an error envelope.

---

## 4. Warm path: re-derive envelopes from cached persona

When a cache entry exists, the warm path avoids re-loading contracts and only re-computes envelopes if revision stamps have changed:

1. **Fetch current revision stamps**:
   ```lua
   local currentConsentVersion = H.Consent.revision(sessionId)
   local currentBudgetVersion  = H.Budget.revision(sessionId)
   ```
2. **Compare with cached stamps**:
   - If `consentVersion` or `budgetVersion` has incremented, re-derive envelopes.
   - If unchanged, return cached `invariantEnvelope` and `metricEnvelope` directly.
3. **Re-derive envelopes** (when needed):
   - Re-intersect persona bands with current consent caps and budget soft-caps.
   - Re-apply comfort policy constraints (max DET per session, cooldown ratios, CDL span).
   - Update `metricEnvelope` with any budget-driven downshifts.
4. **Update cache entry**:
   - Bump `revision.consentVersion` / `revision.budgetVersion` to current values.
   - Update `lastUsed` timestamp.
5. **Log telemetry**: `director-persona-cache-warm` event with `recomputed = true/false`.

This ensures that a mid-session consent downgrade (e.g., user toggles "reduce intensity") is reflected on the next turn without re-parsing contracts.

---

## 5. Invalidation rules

Cache entries must be cleared when:

| Trigger | Action |
|---------|--------|
| `H.Consent.invalidateSession(sessionId)` | Clear cache; next call takes cold path. |
| `H.Budget.terminateSession(sessionId)` | Clear cache; persona no longer valid. |
| `H.Director.unloadPersona(sessionId)` | Explicit API to clear cache. |
| Error during warm-path re-derivation | Clear cache; return error to caller. |
| Session timeout / GC | Evict entry from memory cache. |

Additionally, if `personaContract` or `comfortPolicy` are updated upstream (detected via schema version bump), the next cold path will load the new version; warm-path entries remain valid until their `personaVersion` stamp no longer matches the canonical schema.

---

## 6. Telemetry events

All cache operations emit structured events for audit and tuning:

### 6.1 Cold load
```json
{
  "event": "director-persona-cache-cold",
  "sessionId": "sess-abc123",
  "personaId": "dir-cold-overseer-01",
  "comfortPolicyId": "dcp-ritual-v1",
  "envelopeHashes": {
    "invariantEnvelope": "sha256:...",
    "metricEnvelope": "sha256:..."
  },
  "revision": {
    "personaVersion": "v1.0.0",
    "consentVersion": 42,
    "budgetVersion": 17,
    "comfortVersion": "dcp-ritual-v1"
  },
  "timestamp": 1713654321
}
```

### 6.2 Warm hit (no recompute)
```json
{
  "event": "director-persona-cache-warm",
  "sessionId": "sess-abc123",
  "recomputed": false,
  "revision": {
    "consentVersion": 42,
    "budgetVersion": 17
  },
  "timestamp": 1713654322
}
```

### 6.3 Warm hit (recomputed)
```json
{
  "event": "director-persona-cache-warm",
  "sessionId": "sess-abc123",
  "recomputed": true,
  "reason": "consentVersion-incremented",
  "revision": {
    "consentVersion": 43,
    "budgetVersion": 17
  },
  "envelopeDeltas": {
    "DET.max": { "before": 6.0, "after": 4.0 }
  },
  "timestamp": 1713654323
}
```

### 6.4 Invalidation
```json
{
  "event": "director-persona-cache-invalidated",
  "sessionId": "sess-abc123",
  "reason": "consent-downgrade",
  "timestamp": 1713654324
}
```

---

## 7. Integration with H.Director.loadPersona

The public API remains unchanged; caching is an internal optimization:

```lua
-- Public signature (unchanged)
local result = H.Director.loadPersona(sessionContext, directorPersonaId)

-- Internal flow:
-- 1. Look up cache by sessionContext.sessionId
-- 2. If miss or invalid → cold path
-- 3. If hit → warm path (re-derive if revision stamps changed)
-- 4. Return { ok = true, data = { invariantEnvelope, metricEnvelope, comfortPolicy } }
```

Callers (AI-Chat, engine scripts) treat the returned envelopes as authoritative for the current turn; they do not need to know whether the data came from cold or warm path.

---

## 8. Implementation notes

### 8.1 Lua cache skeleton
```lua
-- engine/library/director_persona_cache.lua
local Cache = {}
local entries = {}  -- keyed by sessionId

function Cache.get(sessionId)
  return entries[sessionId]
end

function Cache.set(sessionId, entry)
  entries[sessionId] = entry
end

function Cache.invalidate(sessionId)
  entries[sessionId] = nil
end

function Cache.cleanup_expired(maxAgeSeconds)
  local now = os.time()
  for sid, entry in pairs(entries) do
    if now - entry.lastUsed > maxAgeSeconds then
      entries[sid] = nil
    end
  end
end

return Cache
```

### 8.2 Revision stamp helpers
Modules 1 (Consent) and 3 (Budget) must expose monotonic revision counters:
```lua
local consentVersion = H.Consent.revision(sessionId)  -- integer, increments on any state change
local budgetVersion  = H.Budget.revision(sessionId)    -- integer, increments on budget update
```

### 8.3 Thread safety
In multi-threaded engines (Unreal, Death-Engine), cache access must be serialized per `sessionId`. A simple mutex per entry or a global read-write lock suffices; contention is low because lookups are read-mostly.

---

## 9. Testing checklist

- [ ] Cold path loads persona + comfort policy and computes envelopes correctly.
- [ ] Warm path returns cached envelopes when revision stamps unchanged.
- [ ] Warm path recomputes envelopes when `consentVersion` or `budgetVersion` increments.
- [ ] Invalidation clears cache and forces cold path on next call.
- [ ] Telemetry events include correct revision stamps and envelope hashes.
- [ ] Error cases (`DIRECTOR_NOT_FOUND`, `CONSENT_INCOMPATIBLE`) clear cache and return structured errors.

---

## 10. Future extensions

- **LRU eviction**: Add max cache size and evict least-recently-used entries when memory pressure is detected.
- **Pre-warming**: Allow `H.Director.preloadPersona(sessionId, directorPersonaId)` to run cold path ahead of first turn.
- **Cross-session persona sharing**: Cache persona contracts (not envelopes) globally to avoid re-parsing identical contracts across sessions.

By treating the persona cache as a revision-stamped, envelope-only layer, `H.Director.loadPersona` stays fast, correct, and auditable across all engines and AI-Chat sessions.
