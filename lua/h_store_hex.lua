-- lua/h_store_hex.lua
-- H.Store.Hex: helpers for {tiles -> bits -> hex} and {hex -> bits -> tile allow/deny}.

local M = {}

-- Optional: allow injection of a bit library for environments where 'bit' is not available.
local bitlib = nil

local function resolve_bitlib()
    if bitlib then
        return bitlib
    end
    local ok, b = pcall(require, "bit")
    if ok and b then
        bitlib = b
        return bitlib
    end
    local ok32, b32 = pcall(require, "bit32")
    if ok32 and b32 then
        bitlib = b32
        return bitlib
    end
    bitlib = nil
    return nil
end

function M.set_bitlib(b)
    bitlib = b
end

local function byte_to_hex(b)
    if b < 0 then
        b = b + 256
    end
    return string.format("%02X", b % 256)
end

local function hex_to_byte_pair(hex, i)
    local hi = tonumber(hex:sub(i, i), 16)
    local lo = tonumber(hex:sub(i + 1, i + 1), 16)
    if not hi or not lo then
        return nil
    end
    return hi * 16 + lo
end

-- Encode a boolean array into a hex mask string.
-- items: array of booleans (true = allowed, false = denied).
-- Returns hex string (e.g., "0F03").
function M.encode_bool_array_to_hex(items)
    local n = #items
    if n == 0 then
        return ""
    end

    local bytes = {}
    local byte_val = 0
    local bit_index = 0

    for idx = 1, n do
        local v = items[idx] and 1 or 0
        if v ~= 0 then
            byte_val = byte_val + (1 << bit_index)
        end
        bit_index = bit_index + 1

        if bit_index == 8 then
            bytes[#bytes + 1] = byte_val
            byte_val = 0
            bit_index = 0
        end
    end

    if bit_index > 0 then
        bytes[#bytes + 1] = byte_val
    end

    local parts = {}
    for i = 1, #bytes do
        parts[#parts + 1] = byte_to_hex(bytes[i])
    end
    return table.concat(parts)
end

-- Decode a hex mask into a boolean array of length 'count'.
-- hex: hex string.
-- count: number of items (tiles) that should be represented.
-- Returns array of booleans.
function M.decode_hex_to_bool_array(hex, count)
    local result = {}
    if count <= 0 or hex == "" then
        return result
    end

    local len = #hex
    local byte_count = math.floor(len / 2)
    local idx = 0

    for i = 0, byte_count - 1 do
        local pos = i * 2 + 1
        local b = hex_to_byte_pair(hex, pos)
        if not b then
            break
        end
        for bit_index = 0, 7 do
            if idx >= count then
                break
            end
            local bit_set = (b & (1 << bit_index)) ~= 0
            result[idx + 1] = bit_set
            idx = idx + 1
        end
        if idx >= count then
            break
        end
    end

    -- If hex string was shorter than needed, pad with false.
    while idx < count do
        result[idx + 1] = false
        idx = idx + 1
    end

    return result
end

-- Build a hex mask for a set of allowed tiles.
-- tile_ids: array of tile ids (strings).
-- allowed_set: table where allowed_set[tile_id] == true for allowed tiles.
-- Returns hex string and the ordering used (copy of tile_ids).
function M.tiles_to_hex(tile_ids, allowed_set)
    local order = {}
    for i = 1, #tile_ids do
        order[i] = tile_ids[i]
    end

    local bits = {}
    for i = 1, #order do
        local tid = order[i]
        bits[i] = allowed_set[tid] and true or false
    end

    local hex = M.encode_bool_array_to_hex(bits)
    return hex, order
end

-- Given a hex mask and a tile ordering, return a table of allowed tiles.
-- hex: hex string.
-- tile_ids: array of tile ids in the same order used for encoding.
-- Returns table where result[tile_id] == true if allowed by the mask.
function M.hex_to_tiles(hex, tile_ids)
    local count = #tile_ids
    local bits = M.decode_hex_to_bool_array(hex, count)
    local allowed = {}

    for i = 1, count do
        if bits[i] then
            local tid = tile_ids[i]
            allowed[tid] = true
        end
    end

    return allowed
end

-- Convenience check: is a given tile allowed by this hex mask and ordering?
function M.is_tile_allowed(hex, tile_ids, tile_id)
    local count = #tile_ids
    local bits = M.decode_hex_to_bool_array(hex, count)
    local index = nil
    for i = 1, count do
        if tile_ids[i] == tile_id then
            index = i
            break
        end
    end
    if not index then
        return false
    end
    return bits[index] == true
end

return M
