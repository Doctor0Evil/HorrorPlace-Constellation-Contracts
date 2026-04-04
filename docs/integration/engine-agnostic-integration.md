# Engine-Agnostic Integration Guide

This document provides patterns for integrating `HorrorPlace-Constellation-Contracts` with game engines, runtime frameworks, and AI tooling—without locking to any specific platform. The goal is to enable Unity, Unreal, Godot, custom engines, and AI-chat systems to consume constellation contracts uniformly.

## Core Principle: Contracts Over Code

All integration points should rely on:

1. **JSON Schema Validation**: Engines load and validate contract cards against canonical schemas before use.
2. **Opaque References**: Engines never embed raw asset paths; they resolve `artifactid` via external loaders.
3. **Tier-Aware Loading**: Engines respect `tier` fields to gate content access at runtime.
4. **Telemetry Envelopes**: Engines emit metrics in `session-metrics-envelope.v1.json` format for feedback loops.

This approach ensures that changing engines or adding new platforms requires only adapter code, not contract redesign.

## Integration Layers

### Layer 1: Schema Loading and Validation

Every engine should include a schema validator (or call the Python CLI):

```python
# Pseudocode: Engine startup
from jsonschema import validate, Draft202012Validator
import requests

def load_contract_card(path: str, schema_ref: str) -> dict:
    # Fetch canonical schema
    schema = requests.get(schema_ref).json()
    # Load and parse contract
    with open(path) as f:
        contract = json.load(f)
    # Validate
    Draft202012Validator(schema).validate(contract)
    return contract
```

Alternatively, embed minimal validation logic in engine-native code (C#, C++, GDScript) using a JSON Schema library.

### Layer 2: Registry Resolution

Engines query NDJSON registries to resolve IDs to implementations:

```lua
-- Lua pseudocode (Godot/LOVE-style)
function resolve_registry_id(registry_type, id)
    local registry = load_ndjson("registry/" .. registry_type .. ".ndjson")
    for _, entry in ipairs(registry) do
        if entry.id == id then
            return entry.artifactid  -- Opaque reference
        end
    end
    return nil
end

-- Usage
local region_artifact = resolve_registry_id("regions", "REG-ARAL-0001")
load_asset(region_artifact)  -- Engine-specific loader
```

### Layer 3: Invariant and Metric Binding

At runtime, engines bind contract-defined invariants/metrics to live systems:

```csharp
// C# pseudocode (Unity-style)
public class RegionController : MonoBehaviour
{
    public RegionContractCard card;  // Loaded and validated at startup

    void Start()
    {
        // Bind invariant values to gameplay systems
        HorrorEngine.SetInvariant("CIC", card.invariantBindings.CIC);
        HorrorEngine.SetInvariant("AOS", card.invariantBindings.AOS);
        
        // Subscribe to metric updates for telemetry
        HorrorEngine.OnMetricChanged += (metric, value) => {
            Telemetry.Emit(new SessionMetricsEnvelope {
                metricName = metric,
                value = value,
                regionId = card.id,
                timestamp = DateTime.UtcNow
            });
        };
    }
}
```

### Layer 4: Telemetry Emission

Engines emit telemetry in the canonical envelope format:

```json
{
  "sessionMetricsEnvelope": {
    "version": "1.0.0",
    "sessionId": "sess-20260115-001",
    "regionId": "REG-ARAL-0001",
    "metrics": {
      "UEC": 72.3,
      "EMD": 0.41,
      "CDL": 3.2
    },
    "timestamp": "2026-01-15T03:15:00Z",
    "prismMetaRef": "prism:reg-aral-0001"
  }
}
```

Telemetry is sent to `Horror.Place-Orchestrator` or a local aggregator for spine index updates.

## Engine-Specific Adapter Patterns

### Unity (C#)

```csharp
// Assets/Plugins/HorrorPlace/ContractLoader.cs
public static class ContractLoader
{
    public static T LoadContract<T>(string path) where T : class
    {
        var schemaPath = GetSchemaPath(typeof(T));
        var schema = Resources.Load<TextAsset>(schemaPath).text;
        var contractJson = File.ReadAllText(path);
        
        // Validate using Newtonsoft.Json + JsonSchema
        var schemaObj = JSchema.Parse(schema);
        var contractObj = JObject.Parse(contractJson);
        if (!contractObj.IsValid(schemaObj, out var errors))
            throw new ValidationException(string.Join("; ", errors));
        
        return JsonConvert.DeserializeObject<T>(contractJson);
    }
}
```

### Unreal Engine (C++)

```cpp
// Source/HorrorPlace/Private/ContractValidator.cpp
bool UContractValidator::ValidateContract(const FString& Path, const FString& SchemaRef)
{
    // Fetch schema via HTTP (or load from packaged content)
    FString SchemaContent;
    FHttpModule::Get().CreateRequest()
        ->OnProcessRequestComplete::CreateLambda([&](FHttpRequestPtr, FHttpResponsePtr Resp, bool) {
            SchemaContent = Resp->GetContentAsString();
        })
        ->ProcessRequest();
    
    // Parse and validate using RapidJSON + custom schema checker
    rapidjson::Document SchemaDoc, ContractDoc;
    SchemaDoc.Parse(TCHAR_TO_UTF8(*SchemaContent));
    ContractDoc.Parse(TCHAR_TO_UTF8(*FFileHelper::LoadFileToString(Path)));
    
    return SchemaValidator::Validate(ContractDoc, SchemaDoc);  // Custom implementation
}
```

### Godot (GDScript)

```gdscript
# res://addons/horror_place/contract_loader.gd
static func load_contract(path: String, schema_ref: String) -> Dictionary:
    var schema = await _fetch_schema(schema_ref)
    var contract = JSON.parse_string(FileAccess.get_file_as_string(path))
    
    # Minimal validation: check required fields
    for field in schema.get("required", []):
        if not contract.has(field):
            push_error(f"Missing required field: {field}")
            return {}
    
    # Type checks (simplified)
    for prop_name, prop_def in schema.get("properties", {}).items():
        if contract.has(prop_name):
            var expected_type = prop_def.get("type")
            var actual_type = typeof(contract[prop_name])
            if not _types_match(expected_type, actual_type):
                push_error(f"Type mismatch for {prop_name}")
                return {}
    
    return contract
```

### Custom Engine / AI Runtime

For non-game contexts (e.g., AI-chat systems, narrative generators):

```python
# Python runtime for AI agents
class ConstellationRuntime:
    def __init__(self, spine_index_path: str):
        self.index = json.load(open(spine_index_path))
    
    def generate_contract(self, intent: str, params: dict) -> dict:
        # Query spine index for valid schema
        schema_ref = self.index["contracts"][intent]["schemaRef"]
        # Generate via AI agent (with prismMeta embedding)
        contract = ai_agent.generate(intent, params, schema_ref)
        # Validate locally
        validate(contract, schema_ref)
        return contract
    
    def emit_telemetry(self, metrics: dict, context: dict):
        envelope = {
            "sessionMetricsEnvelope": {
                "version": "1.0.0",
                "metrics": metrics,
                **context,
                "timestamp": datetime.utcnow().isoformat() + "Z"
            }
        }
        # Send to orchestrator or local aggregator
        requests.post("https://orchestrator.horror.place/telemetry", json=envelope)
```

## Asset Loading and Opaque References

Engines must never hardcode asset paths. Instead:

1. Contract cards reference assets via `artifactid` (e.g., `ipfs:bafy...`, `cid:sha256:...`).
2. Engines use a pluggable loader registry to resolve `artifactid` to actual content:

```typescript
// TypeScript adapter interface
interface AssetLoader {
  canHandle(artifactId: string): boolean;
  load(artifactId: string): Promise<Asset>;
}

// Example: IPFS loader
class IpfsLoader implements AssetLoader {
  canHandle(id: string) { return id.startsWith("ipfs:"); }
  async load(id: string) {
    const cid = id.replace("ipfs:", "");
    const resp = await fetch(`https://ipfs.io/ipfs/${cid}`);
    return await resp.blob();
  }
}

// Engine usage
const loader = [new IpfsLoader(), new HttpLoader()].find(l => l.canHandle(artifactId));
const asset = await loader.load(artifactId);
```

## Tier-Aware Runtime Gating

Engines must enforce `tier` at runtime:

```rust
// Rust pseudocode (Bevy/WASM-style)
fn load_region(card: RegionContractCard) -> Result<Region, Error> {
    match card.tier.as_str() {
        "public" => Ok(load_public_assets(&card)),
        "vault" => {
            // Require deadledgerref validation
            if !validate_deadledgerref(&card.deadledgerref, &card.content_hash) {
                return Err(Error::AccessDenied);
            }
            Ok(load_vault_assets(&card))
        },
        "lab" => {
            // Require experimental flag + contributor auth
            if !config.experimental_mode || !auth.is_contributor() {
                return Err(Error::AccessDenied);
            }
            Ok(load_lab_assets(&card))
        },
        _ => Err(Error::UnknownTier)
    }
}
```

## Testing and Validation

To verify engine integration:

```bash
# Validate contract cards with engine-specific loader
python tooling/python/cli/hpc-validate-schema.py \
  --mode ai-authoring \
  --file examples/minimal-constellation/registry/regions.minimal.ndjson \
  --engine-adapter unity  # or unreal, godot, custom

# Run engine-specific test suite
# Unity: Assets/Tests/HorrorPlace/ContractIntegrationTests.cs
# Unreal: Source/HorrorPlaceTests/Private/ContractValidatorTests.cpp
# Godot: res://tests/horror_place/contract_loader_tests.gd
```

## Related Documents

- `schemas/core/regionContractCard.v1.json`: Canonical contract card schema.
- `schemas/telemetry/session-metrics-envelope.v1.json`: Telemetry format specification.
- `tooling/lua/hpc_contract_cards.lua`: Lua helper for contract loading.
- `examples/minimal-constellation/`: Worked example with minimal engine adapter.
- `research/ai-copilot-behavior-notes.md`: Observations on AI-agent integration patterns.
