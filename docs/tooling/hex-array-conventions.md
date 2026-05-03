# Hex Array Conventions

This document defines the canonical conventions for hex-encoded bitmasks used in `hex_arrays.hex_payload` and related tooling. It covers byte/bit ordering, array ID patterns, and the expectations that engine and AI-chat code must follow.

These conventions apply to:

- `hex_arrays` in `horrorplace_runtime_events.sqlite`.
- Any future hex-encoded masks used for tile, region, or routing maps.

## 1. Byte ordering

Hex payloads are treated as big-endian sequences of bytes:

- The first two hex characters (`payload[1..2]`) represent the first byte.
- The second pair (`payload[3..4]`) represent the second byte, and so on.
- The first byte corresponds to the first 8 items in the logical ordering (e.g., tiles), the second byte to the next 8 items, etc.

Formally, let:

- `bytes[0]` be the first byte represented in the hex string.
- `bytes[1]` be the second byte, etc.

Then:

- `bytes[0]` encodes indices 0–7.
- `bytes[1]` encodes indices 8–15.
- In general, `bytes[i]` encodes indices `i * 8` through `i * 8 + 7`.

## 2. Bit ordering inside a byte

Within each byte, bits are numbered from least significant bit (LSB) to most significant bit (MSB):

- Bit 0 (LSB) is the first item in that byte’s block.
- Bit 7 (MSB) is the last item in that byte’s block.

For index `idx` in the logical item list:

- `byte_index = floor(idx / 8)`
- `bit_index  = idx % 8`

If we interpret `bytes[byte_index]` as an unsigned 8-bit integer:

- If `(bytes[byte_index] & (1 << bit_index)) != 0`, the item at `idx` is enabled (`1`).
- Otherwise, it is disabled (`0`).

This convention is independent of machine endianness; it is purely a logical mapping.

## 3. Logical ordering of items

The mask does not encode ordering by itself. Every usage must define a deterministic ordering of items (e.g., tiles) and use that ordering consistently in both encoding and decoding.

For tile masks:

- The recommended ordering is `ORDER BY tile_id ASC` within a fixed region.
- The same ordering must be used whenever encoding or decoding a mask for that `(region_id, chain_id, snapshot_id)` combination.

## 4. Per-chain array IDs

Hex arrays may be produced per snapshot, per region, and per rule chain.

Recommended `array_id` pattern for per-chain tile masks:

- `array_id = "snapshot:" + snapshot_id + ":chain:" + chain_id + ":region:" + region_id`

Examples:

- `snapshot:2026-05-03T13:05:12Z:chain:spawn_safety:region:atrocity-basin-v3`

The `hex_arrays` table should also store the structured components in columns where possible:

- `kind        = "tile_mask_per_chain"`
- `region_id   = region id`
- `tile_id     = NULL` (for region-wide masks)
- Additional chain metadata (e.g., chain name) can be resolved via joins on `invariant_chains`.

## 5. Zero, padding, and length

- Zero mask: a mask where all bits are zero is represented as a sequence of `00` bytes (e.g., `00`, `0000`, etc.), depending on how many bytes are needed for the item list length.
- The number of items is implied by the context (e.g., tile count for a region); unused trailing bits (when item count is not a multiple of 8) must be set to zero and ignored during decoding.
- It is valid for different rows to have different payload lengths as long as the encoding/decoding logic knows the expected item count for that mask.

## 6. Usage patterns

- Engine and tool code should treat `hex_payload` as authoritative for allow/deny decisions for a given chain and region.
- AI-chat agents should avoid constructing masks manually; instead, they should either:
  - Request masks via runtime APIs, or
  - Use the shared helper functions (e.g., `H.Store.Hex`) when generating hex payloads during offline tools or migrations.

These conventions are stable and should not be changed without a coordinated schema and code migration.
