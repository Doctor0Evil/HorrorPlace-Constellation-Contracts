# Integration with Dead-Ledger Network

This document describes how `HorrorPlace-Constellation-Contracts` integrates with the `HorrorPlace-Dead-Ledger-Network` repository for cryptographic gating, zero-knowledge proof validation, and access control across constellation tiers.

## Purpose of Dead-Ledger Integration

The Dead-Ledger Network provides:

1. **Cryptographic Attestation**: `deadledgerref` values that prove an artifact's provenance, tier eligibility, and compliance with governance rules.
2. **Zero-Knowledge Proof (ZKP) Validation**: Privacy-preserving verification of age gating, entitlements, and access rights without exposing raw data.
3. **Multi-Sig Governance**: Distributed approval workflows for vault/lab-tier content modifications.
4. **DID/KYC Alignment**: Decentralized identity integration for contributor authentication and audit trails.

This repo defines the contract-level interface to these capabilities; the Dead-Ledger Network implements the cryptographic infrastructure.

## deadledgerref Format and Validation

Every registry entry and contract card in the constellation includes a `deadledgerref` field:

```json
{
  "deadledgerref": "zkp:sha256:abc123...:sig:ed25519:def456...:tier:vault"
}
```

### Structure

| Component | Format | Description |
|-----------|--------|-------------|
| `zkp` | literal | Indicates a zero-knowledge proof is embedded. |
| `sha256:<hash>` | hex string | Content hash of the artifact being attested. |
| `sig:ed25519:<sig>` | hex string | Ed25519 signature from an authorized governance key. |
| `tier:<value>` | enum | Access tier this attestation grants: `public`, `vault`, `lab`. |

### Validation Rules

The registry linter (`hpc-lint-registry.py`) enforces:

1. `deadledgerref` must be present for all `vault` and `lab` tier entries.
2. The `sha256` hash must match the artifact's content (computed at validation time).
3. The `sig` must verify against a public key listed in `HorrorPlace-Dead-Ledger-Network/keys/governance.pub`.
4. The `tier` value must match the entry's top-level `tier` field.

Example validation snippet (Python):

```python
import hashlib, nacl.signing

def validate_deadledgerref(ref: str, content: bytes, public_key: bytes) -> bool:
    parts = ref.split(":")
    if parts[0] != "zkp" or len(parts) < 6:
        return False
    expected_hash = parts[2]
    signature = bytes.fromhex(parts[4])
    verifier = nacl.signing.VerifyKey(public_key)
    # Verify hash
    if hashlib.sha256(content).hexdigest() != expected_hash:
        return False
    # Verify signature over hash + tier
    message = f"{expected_hash}:{parts[6]}".encode()
    try:
        verifier.verify(message, signature)
        return True
    except nacl.exceptions.BadSignature:
        return False
```

## ZKP Schema Integration

The Dead-Ledger Network defines ZKP schemas for common proofs:

- `zkpproof-age-gate.v1.json`: Proves user is over 18 without revealing DOB.
- `zkpproof-entitlement.v1.json`: Proves user has access to vault-tier content.
- `zkpproof-contributor.v1.json`: Proves contributor identity and signing authority.

This repo references these schemas via `$ref` in contract definitions:

```json
{
  "entitlementProof": {
    "$ref": "https://raw.githubusercontent.com/Doctor0Evil/HorrorPlace-Dead-Ledger-Network/main/schemas/zkpproof-entitlement.v1.json"
  }
}
```

### Validation Workflow

1. AI agent generates a contract card with `tier: "vault"`.
2. Agent requests a ZKP from Dead-Ledger Network (via API or local tool).
3. Dead-Ledger returns a `zkpproof` JSON object.
4. Agent embeds the proof's hash in `deadledgerref`.
5. CI validates the proof structure and signature before merging.

## Multi-Sig Governance Hooks

For vault/lab-tier changes, the Dead-Ledger Network requires multi-signature approval. This repo's CI workflows integrate with this via:

1. **Pre-Merge Gate**: `constellation-precommit-pack.yml` checks if a change affects vault/lab content.
2. **Signature Collection**: If yes, the workflow pauses and requests signatures from configured governance keys.
3. **Proof Embedding**: Once signatures are collected, the workflow updates `deadledgerref` and proceeds.

Example GitHub Actions step:

```yaml
- name: Request multi-sig approval for vault-tier change
  if: contains(github.event.pull_request.changed_files, 'vault')
  uses: Doctor0Evil/HorrorPlace-Dead-Ledger-Network/.github/actions/request-multisig@main
  with:
    governance_keys: ${{ secrets.GOVERNANCE_KEYS }}
    threshold: 3
    timeout_minutes: 60
```

## DID/KYC Integration

Contributors to vault/lab repos must authenticate via decentralized identity (DID) and optional KYC. This repo's `agentProfile` schema includes fields for identity verification:

```json
{
  "agentProfile": {
    "constraints": {
      "requireDIDAuth": true,
      "allowedDIDMethods": ["did:key", "did:ethr"],
      "kycLevel": "optional"  // or "required" for lab-tier agents
    }
  }
}
```

### Authentication Flow

1. Agent presents DID proof at CI startup.
2. Dead-Ledger Network verifies DID against registry of authorized contributors.
3. If KYC is required, agent presents verifiable credential (VC) issued by trusted issuer.
4. CI proceeds only if authentication succeeds.

## Privacy and Data Minimization

Integration with Dead-Ledger Network enforces strict privacy rules:

- ❌ No raw personal data (names, emails, DOB) stored in contracts or registries.
- ❌ No ZKP verification keys or secrets committed to Git; use GitHub secrets or external vaults.
- ✅ All proofs are zero-knowledge: verification reveals only validity, not underlying data.
- ✅ Audit logs (who signed what, when) are stored in Dead-Ledger Network, not in this repo.

## Testing and Validation

To test Dead-Ledger integration locally:

```bash
# Clone Dead-Ledger Network repo
git clone https://github.com/Doctor0Evil/HorrorPlace-Dead-Ledger-Network

# Generate test ZKP for a sample contract card
python HorrorPlace-Dead-Ledger-Network/tooling/generate-test-proof.py \
  --content examples/minimal-constellation/registry/regions.minimal.ndjson \
  --tier vault \
  --output test-proof.json

# Validate the proof against the contract
python tooling/python/cli/hpc-validate-schema.py \
  --mode ai-authoring \
  --file examples/minimal-constellation/registry/regions.minimal.ndjson \
  --deadledger-ref "$(cat test-proof.json | jq -r .deadledgerref)" \
  --strict
```

Both commands should succeed if integration is correct.

## Related Documents

- `HorrorPlace-Dead-Ledger-Network/README.md`: Cryptographic infrastructure overview.
- `schemas/registry/registry-entry-base.v1.json`: Base schema requiring `deadledgerref`.
- `docs/tooling/prismMeta-and-agentProfiles.md`: How `prismMeta` references ZKP proofs.
- `tooling/python/schema_spine/registry_linter.py`: Reference implementation of deadledgerref validation.
- `.github/ISSUE_TEMPLATE/contract-change-proposal.md`: Template for proposing governance rule changes.
