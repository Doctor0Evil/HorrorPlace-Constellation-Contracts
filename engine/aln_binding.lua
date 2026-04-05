-- engine/aln_binding.lua

local BehaviorHorrorRegionDirector = require("engine.behavior_horror_region_director")

local ALNBinding = {}

function ALNBinding.register(engine)
  engine:bind_lua_symbol("Lua.Engine.DirectorRegionTick",
    BehaviorHorrorRegionDirector.DirectorRegionTick)

  engine:bind_lua_symbol("Lua.Engine.LogHorrorMetricsDelta",
    BehaviorHorrorRegionDirector.LogHorrorMetricsDelta)
end

return ALNBinding
