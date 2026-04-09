-- test_player_envelope_fps_standard.lua
-- Minimal test wiring the standard example through the validator.

local json = require("dkjson")          -- or your preferred JSON lib
local validator = require("validate_player_envelope_fps")

local function read_file(path)
  local f, err = io.open(path, "r")
  if not f then
    error("ERR_TEST_OPEN_FILE: " .. tostring(err))
  end
  local data = f:read("*a")
  f:close()
  return data
end

local function load_json(path)
  local text = read_file(path)
  local obj, pos, jerr = json.decode(text, 1, nil)
  if not obj then
    error("ERR_TEST_JSON_DECODE: " .. tostring(jerr) .. " at " .. tostring(pos))
  end
  return obj
end

local function main()
  -- Path is relative to repo root; adjust if your CI runs from a subdirectory.
  local env = load_json("examples/player-envelope-fps-standard.json")

  -- For now, treat caps as coming directly from env.safeCaps.
  -- Later you can replace this with spine-derived tier caps.
  local caps = env.safeCaps

  local ok, err = pcall(function()
    return validator.validate_player_envelope(env, caps)
  end)

  if not ok then
    io.stderr:write("player-envelope-fps-standard: validation FAILED: " .. tostring(err) .. "\n")
    os.exit(1)
  end

  print("player-envelope-fps-standard: validation OK")
  os.exit(0)
end

main()
