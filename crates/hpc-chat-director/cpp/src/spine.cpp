// crates/hpc-chat-director/cpp/src/spine.cpp
#include "hpc/chat_director/spine.hpp"
#include "hpc/chat_director/types/invariant_types.hpp"

#include <fstream>
#include <filesystem>

namespace fs = std::filesystem;
using json = nlohmann::json;

namespace hpc::chat_director {

SpineIndex SpineIndex::load_from_root(const fs::path& root) {
    const fs::path spine_path = root / "schemas" / "core" / "schema-spine-index-v1.json";

    if (!fs::exists(spine_path)) {
        throw IoError(spine_path.string(), "Schema spine index not found");
    }

    std::ifstream file(spine_path);
    if (!file.is_open()) {
        throw IoError(spine_path.string(), "Could not open spine index");
    }

    json j;
    file >> j;

    types::SchemaSpine spine;
    j.get_to(spine);

    return SpineIndex(root, std::move(spine));
}

SpineIndex SpineIndex::load(const Config& config) {
    return load_from_root(config.root());
}

std::optional<types::ObjectKindProfile>
SpineIndex::describe_object_kind(const std::string& kind) const {
    std::vector<std::string> required_invariants;
    std::vector<std::string> required_metrics;
    std::vector<std::string> tiers;

    for (const auto& family : spine_.contract_families) {
        if (family.object_kind == kind) {
            for (const auto& inv : family.required_invariants) {
                if (std::find(required_invariants.begin(), required_invariants.end(), inv)
                    == required_invariants.end()) {
                    required_invariants.push_back(inv);
                }
            }
            for (const auto& met : family.required_metrics) {
                if (std::find(required_metrics.begin(), required_metrics.end(), met)
                    == required_metrics.end()) {
                    required_metrics.push_back(met);
                }
            }
            for (uint8_t phase : family.allowed_phases) {
                // Map phase to tier hints (simplified for v1)
                if (phase <= 1 && std::find(tiers.begin(), tiers.end(), "Tier1") == tiers.end()) {
                    tiers.push_back("Tier1");
                }
                if (phase >= 1 && phase <= 3 && std::find(tiers.begin(), tiers.end(), "Tier2") == tiers.end()) {
                    tiers.push_back("Tier2");
                }
                if (phase >= 2 && std::find(tiers.begin(), tiers.end(), "Tier3") == tiers.end()) {
                    tiers.push_back("Tier3");
                }
            }
        }
    }

    if (required_invariants.empty() && required_metrics.empty()) {
        return std::nullopt;  // Unknown objectKind
    }

    return types::ObjectKindProfile{
        .object_kind = kind,
        .required_invariants = std::move(required_invariants),
        .allowed_metrics = std::move(required_metrics),
        .tiers = std::move(tiers)
    };
}

std::optional<types::DefaultBands>
SpineIndex::safe_defaults(const std::string& object_kind, const std::string& tier) const {
    for (const auto& entry : spine_.safe_defaults) {
        if (entry.object_kind == object_kind && entry.tier == tier) {
            return types::DefaultBands{
                .cic = entry.bands.cic,
                .aos = entry.bands.aos,
                .det = entry.bands.det,
                .uec = entry.bands.uec,
                .arr = entry.bands.arr,
                .shci = entry.bands.shci
            };
        }
    }
    return std::nullopt;
}

std::optional<types::SuggestedRanges>
SpineIndex::suggest_ranges(const std::string& object_kind, const std::string& tier) const {
    if (auto defaults = safe_defaults(object_kind, tier)) {
        return types::SuggestedRanges{
            .object_kind = object_kind,
            .tier = tier,
            .cic = defaults->cic,
            .aos = defaults->aos,
            .det = defaults->det,
            .uec = defaults->uec,
            .arr = defaults->arr,
            .shci = defaults->shci
        };
    }
    return std::nullopt;
}

}  // namespace hpc::chat_director
