// crates/hpc-chat-director/cpp/include/hpc/chat_director/types/invariant_types.hpp
#pragma once

#include <string>
#include <vector>
#include <optional>
#include <nlohmann/json.hpp>

namespace hpc::chat_director::types {

/// Numeric range with optional step for invariant/metric values.
struct InvariantRange {
    double min;
    double max;
    double step;

    [[nodiscard]] bool contains(double value) const noexcept {
        return value >= min && value <= max;
    }

    [[nodiscard]] double clamp(double value) const noexcept {
        if (value < min) return min;
        if (value > max) return max;
        return value;
    }
};

/// Tier-specific override for invariant/metric ranges.
struct TierOverride {
    std::string tier;
    std::optional<double> min;
    std::optional<double> max;
};

/// Drift specification for invariants that may evolve across releases.
struct DriftSpec {
    bool allowed;
    double max_delta_per_release;
};

/// Canonical invariant definition from the schema spine.
struct InvariantSpec {
    std::string id;
    std::string code;      // e.g., "CIC", "AOS", "DET"
    std::string name;
    std::string description;
    std::string class_name;
    InvariantRange range;
    std::vector<TierOverride> tier_overrides;
    DriftSpec drift;

    [[nodiscard]] InvariantRange effective_range_for_tier(const std::string& tier) const {
        InvariantRange result = range;
        for (const auto& override : tier_overrides) {
            if (override.tier == tier) {
                if (override.min.has_value()) result.min = override.min.value();
                if (override.max.has_value()) result.max = override.max.value();
                break;
            }
        }
        return result;
    }
};

/// Derived metric specification (e.g., SPR, SHCI computed from base invariants).
struct DerivedMetricSpec {
    std::string code;
    std::vector<std::string> inputs;  // base invariant/metric codes used in derivation
};

/// Cross-metric interaction rule for validation.
struct InteractionRule {
    std::string rule_id;
    std::string source_metric;
    std::string target_metric;
    std::string relationship;  // e.g., "suppress", "amplify", "gate"
};

/// Safe band for default invariant/metric values.
struct SafeBand {
    double min;
    double max;
};

/// Collection of safe default bands for an objectKind/tier pair.
struct SafeDefaultsBands {
    std::optional<SafeBand> cic;
    std::optional<SafeBand> aos;
    std::optional<SafeBand> det;
    std::optional<SafeBand> uec;
    std::optional<SafeBand> arr;
    std::optional<SafeBand> shci;
};

/// Entry mapping objectKind + tier to safe default bands.
struct SafeDefaultsEntry {
    std::string object_kind;
    std::string tier;
    SafeDefaultsBands bands;
};

/// Entertainment metric specification.
struct MetricSpec {
    std::string name;        // e.g., "UEC", "EMD", "STCI", "CDL", "ARR"
    std::string description;
    std::optional<double> target_min;
    std::optional<double> target_max;
    std::vector<TierOverride> tier_overrides;
};

/// Contract family grouping related schemas.
struct ContractFamily {
    std::string name;
    std::string object_kind;  // e.g., "regionContractCard", "seedContractCard"
    std::vector<std::string> schemas;
    std::vector<std::string> required_invariants;
    std::vector<std::string> required_metrics;
    std::vector<uint8_t> allowed_phases;  // phase IDs 0-4
};

/// Root schema spine index loaded from schema-spine-index-v1.json.
struct SchemaSpine {
    std::string version;
    std::vector<InvariantSpec> invariants;
    std::vector<MetricSpec> metrics;
    std::vector<ContractFamily> contract_families;
    std::vector<DerivedMetricSpec> derived_metrics;
    std::vector<InteractionRule> interaction_rules;
    std::vector<SafeDefaultsEntry> safe_defaults;

    [[nodiscard]] const InvariantSpec* find_invariant_by_code(const std::string& code) const {
        for (const auto& inv : invariants) {
            if (inv.code == code) return &inv;
        }
        return nullptr;
    }

    [[nodiscard]] const MetricSpec* find_metric_by_name(const std::string& name) const {
        for (const auto& metric : metrics) {
            if (metric.name == name) return &metric;
        }
        return nullptr;
    }
};

}  // namespace hpc::chat_director::types

// JSON serialization support via nlohmann/json
namespace nlohmann {
template <>
struct adl_serializer<hpc::chat_director::types::InvariantRange> {
    static void to_json(json& j, const hpc::chat_director::types::InvariantRange& r) {
        j = json{{"min", r.min}, {"max", r.max}, {"step", r.step}};
    }
    static void from_json(const json& j, hpc::chat_director::types::InvariantRange& r) {
        j.at("min").get_to(r.min);
        j.at("max").get_to(r.max);
        j.at("step").get_to(r.step);
    }
};

template <>
struct adl_serializer<hpc::chat_director::types::TierOverride> {
    static void to_json(json& j, const hpc::chat_director::types::TierOverride& o) {
        j = json{{"tier", o.tier}};
        if (o.min) j["min"] = o.min.value();
        if (o.max) j["max"] = o.max.value();
    }
    static void from_json(const json& j, hpc::chat_director::types::TierOverride& o) {
        j.at("tier").get_to(o.tier);
        if (j.contains("min")) o.min = j.at("min").get<double>();
        if (j.contains("max")) o.max = j.at("max").get<double>();
    }
};

template <>
struct adl_serializer<hpc::chat_director::types::DriftSpec> {
    static void to_json(json& j, const hpc::chat_director::types::DriftSpec& d) {
        j = json{{"allowed", d.allowed}, {"maxDeltaPerRelease", d.max_delta_per_release}};
    }
    static void from_json(const json& j, hpc::chat_director::types::DriftSpec& d) {
        j.at("allowed").get_to(d.allowed);
        j.at("maxDeltaPerRelease").get_to(d.max_delta_per_release);
    }
};

template <>
struct adl_serializer<hpc::chat_director::types::InvariantSpec> {
    static void from_json(const json& j, hpc::chat_director::types::InvariantSpec& s) {
        j.at("id").get_to(s.id);
        j.at("code").get_to(s.code);
        j.at("name").get_to(s.name);
        j.at("description").get_to(s.description);
        j.at("class").get_to(s.class_name);
        j.at("range").get_to(s.range);
        j.at("tierOverrides").get_to(s.tier_overrides);
        j.at("drift").get_to(s.drift);
    }
};

template <>
struct adl_serializer<hpc::chat_director::types::MetricSpec> {
    static void from_json(const json& j, hpc::chat_director::types::MetricSpec& m) {
        j.at("name").get_to(m.name);
        j.at("description").get_to(m.description);
        if (j.contains("target_min")) m.target_min = j.at("target_min").get<double>();
        if (j.contains("target_max")) m.target_max = j.at("target_max").get<double>();
        if (j.contains("tierOverrides")) {
            j.at("tierOverrides").get_to(m.tier_overrides);
        }
    }
};

template <>
struct adl_serializer<hpc::chat_director::types::ContractFamily> {
    static void from_json(const json& j, hpc::chat_director::types::ContractFamily& f) {
        j.at("name").get_to(f.name);
        j.at("object_kind").get_to(f.object_kind);
        j.at("schemas").get_to(f.schemas);
        if (j.contains("required_invariants")) {
            j.at("required_invariants").get_to(f.required_invariants);
        }
        if (j.contains("required_metrics")) {
            j.at("required_metrics").get_to(f.required_metrics);
        }
        if (j.contains("allowed_phases")) {
            j.at("allowed_phases").get_to(f.allowed_phases);
        }
    }
};

template <>
struct adl_serializer<hpc::chat_director::types::SchemaSpine> {
    static void from_json(const json& j, hpc::chat_director::types::SchemaSpine& spine) {
        j.at("version").get_to(spine.version);
        if (j.contains("invariants")) j.at("invariants").get_to(spine.invariants);
        if (j.contains("metrics")) j.at("metrics").get_to(spine.metrics);
        if (j.contains("contract_families")) {
            j.at("contract_families").get_to(spine.contract_families);
        }
        if (j.contains("derivedMetrics")) {
            j.at("derivedMetrics").get_to(spine.derived_metrics);
        }
        if (j.contains("interactionRules")) {
            j.at("interactionRules").get_to(spine.interaction_rules);
        }
        if (j.contains("safeDefaults")) {
            j.at("safeDefaults").get_to(spine.safe_defaults);
        }
    }
};
}  // namespace nlohmann
