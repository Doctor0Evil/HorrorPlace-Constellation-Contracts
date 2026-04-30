// crates/hpc-chat-director/cpp/src/validate/manifests.cpp
#include "hpc/chat_director/validate/manifests.hpp"
#include <regex>
#include <unordered_map>
#include <algorithm>

namespace hpc::chat_director::validate {

static const std::unordered_map<std::string, std::vector<std::string>> REPO_ALLOWED_KINDS = {
    {"HorrorPlace-Constellation-Contracts", {"schema", "ai_envelope", "manifest", "registry_format"}},
    {"HorrorPlace-Neural-Resonance-Lab",    {"binding_pack", "telemetry_schema", "curve_catalog", "tooling"}},
    {"Death-Engine",                        {"style_pack", "lua_module", "cpp_adapter", "runtime_config"}}
};

bool is_repo_routing_valid(const std::string& repo_id, const std::string& object_kind, const std::string& tier) {
    auto it = REPO_ALLOWED_KINDS.find(repo_id);
    if (it == REPO_ALLOWED_KINDS.end()) return false;

    // Tier gating: Tier1 cannot accept runtime/execution artifacts
    if (tier == "Tier1" && (object_kind == "lua_module" || object_kind == "cpp_adapter")) return false;

    return std::find(it->second.begin(), it->second.end(), object_kind) != it->second.end();
}

ValidationResult validate_manifest_routing(const std::string& target_repo, const std::string& target_path, const std::string& object_kind, const std::string& tier) {
    ValidationResult result{.ok = true};

    if (target_repo.empty()) {
        result.ok = false;
        result.diagnostics.push_back({ValidationLayer::Manifest, ValidationSeverity::Error, "EMPTY_TARGET_REPO", "targetRepo must be specified."});
        return result;
    }

    if (!is_repo_routing_valid(target_repo, object_kind, tier)) {
        result.ok = false;
        result.diagnostics.push_back({ValidationLayer::Manifest, ValidationSeverity::Error, "ROUTING_VIOLATION",
            "Object kind '" + object_kind + "' is not permitted in repo '" + target_repo + "' at tier '" + tier + "'."});
    }

    // Path syntax check
    static const std::regex VALID_PATH(R"(^(?:[a-zA-Z0-9_\-]+/)*[a-zA-Z0-9_\-]+\.(json|md|cpp|hpp|lua|toml|yml)$)");
    if (!std::regex_match(target_path, VALID_PATH)) {
        result.diagnostics.push_back({ValidationLayer::Manifest, ValidationSeverity::Warning, "PATH_FORMAT_WARN",
            "targetPath does not match canonical naming conventions."});
    }

    return result;
}

} // namespace hpc::chat_director::validate
