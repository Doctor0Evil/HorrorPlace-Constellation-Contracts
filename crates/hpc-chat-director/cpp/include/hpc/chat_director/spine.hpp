// crates/hpc-chat-director/cpp/include/hpc/chat_director/spine.hpp
#pragma once

#include <filesystem>
#include <optional>
#include "types/invariant_types.hpp"
#include "errors.hpp"
#include "config.hpp"

namespace hpc::chat_director {

/// Phase identifiers for the constellation lifecycle.
enum class Phase : uint8_t {
    Schema0 = 0,      // Core schemas and invariants/metrics spine
    Registry1 = 1,    // Registry entries and indices
    Bundles2 = 2,     // Higher-order bundles and choreography
    LuaPolicy3 = 3,   // Lua policy modules bound to contracts
    Adapters4 = 4     // Engine adapters and external integration
};

/// Convert Phase to string for serialization.
[[nodiscard]] constexpr const char* phase_to_string(Phase phase) noexcept {
    switch (phase) {
        case Phase::Schema0: return "Schema0";
        case Phase::Registry1: return "Registry1";
        case Phase::Bundles2: return "Bundles2";
        case Phase::LuaPolicy3: return "LuaPolicy3";
        case Phase::Adapters4: return "Adapters4";
        default: return "Unknown";
    }
}

/// Convert Phase to numeric ID.
[[nodiscard]] constexpr uint8_t phase_to_id(Phase phase) noexcept {
    return static_cast<uint8_t>(phase);
}

/// Wrapper around the loaded schema spine with helpers.
class SpineIndex {
public:
    /// Load from explicit root directory.
    static SpineIndex load_from_root(const std::filesystem::path& root);

    /// Load using Config's detected root.
    static SpineIndex load(const Config& config);

    [[nodiscard]] const std::filesystem::path& root() const noexcept { return root_; }
    [[nodiscard]] const std::string& version() const noexcept { return spine_.version; }
    [[nodiscard]] const types::SchemaSpine& inner() const noexcept { return spine_; }

    /// Describe requirements for a given objectKind.
    [[nodiscard]] std::optional<types::ObjectKindProfile>
    describe_object_kind(const std::string& kind) const;

    /// Get safe default bands for an objectKind/tier pair.
    [[nodiscard]] std::optional<types::DefaultBands>
    safe_defaults(const std::string& object_kind, const std::string& tier) const;

    /// Suggest invariant/metric ranges for AI pre-filling.
    [[nodiscard]] std::optional<types::SuggestedRanges>
    suggest_ranges(const std::string& object_kind, const std::string& tier) const;

    /// Get all interaction rules for cross-metric validation.
    [[nodiscard]] const std::vector<types::InteractionRule>& interaction_rules() const noexcept {
        return spine_.interaction_rules;
    }

    /// Get all invariant specifications.
    [[nodiscard]] const std::vector<types::InvariantSpec>& invariant_specs() const noexcept {
        return spine_.invariants;
    }

private:
    explicit SpineIndex(std::filesystem::path root, types::SchemaSpine spine)
        : root_(std::move(root)), spine_(std::move(spine)) {}

    std::filesystem::path root_;
    types::SchemaSpine spine_;
};

}  // namespace hpc::chat_director
