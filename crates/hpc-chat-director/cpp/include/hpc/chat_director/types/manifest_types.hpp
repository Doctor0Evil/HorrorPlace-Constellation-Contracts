// crates/hpc-chat-director/cpp/include/hpc/chat_director/types/manifest_types.hpp
#pragma once

#include <string>
#include <vector>
#include <optional>
#include <nlohmann/json.hpp>
#include "invariant_types.hpp"

namespace hpc::chat_director::types {

/// Repository tier in the constellation.
enum class Tier {
    Tier1,  // Public contract-only surfaces
    Tier2,  // Vault-style repos for styles, seeds, personas
    Tier3   // Research tier for BCI, neural resonance
};

/// Convert Tier enum to string for serialization.
[[nodiscard]] inline std::string tier_to_string(Tier tier) {
    switch (tier) {
        case Tier::Tier1: return "Tier1";
        case Tier::Tier2: return "Tier2";
        case Tier::Tier3: return "Tier3";
        default: return "Unknown";
    }
}

/// Parse Tier from string.
[[nodiscard]] inline std::optional<Tier> tier_from_string(const std::string& s) {
    if (s == "Tier1") return Tier::Tier1;
    if (s == "Tier2") return Tier::Tier2;
    if (s == "Tier3") return Tier::Tier3;
    return std::nullopt;
}

/// AI authoring role for policy gating.
enum class AiRole {
    Architect,   // Can define new schemas/contracts
    Implementer, // Can generate concrete artifacts
    Auditor      // Read-only validation and review
};

/// Rule controlling which AI roles may target specific paths.
struct AuthoringRule {
    std::string rule;          // e.g., "TIER1_CONTRACT_ONLY"
    std::string description;   // Human-readable explanation
};

/// Routing rule mapping objectKind to allowed filesystem paths.
struct TargetRule {
    std::string object_kind;
    std::vector<std::string> allowed_paths;    // Path patterns (e.g., "contracts/regions/*.json")
    std::vector<std::string> allowed_schemas;  // Schema IDs allowed for this objectKind
    std::vector<AiRole> allowed_roles;         // Empty = all roles allowed
    std::vector<std::string> notes;            // Policy notes for diagnostics
};

/// Repository manifest defining routing and policy rules.
struct RepoManifest {
    std::string repo_name;      // Logical repo identifier
    Tier tier;
    std::vector<std::string> schemas_whitelist;  // Allowed schema IDs
    std::vector<TargetRule> target_rules;
    bool implicit_deny{false};  // If true, unlisted objectKinds are forbidden
    std::vector<AuthoringRule> ai_authoring_rules;
    std::optional<double> min_rwf_for_tier;  // Minimum Reliability Weighting Factor

    [[nodiscard]] bool allows_object_kind(const std::string& object_kind) const {
        if (implicit_deny) {
            for (const auto& rule : target_rules) {
                if (rule.object_kind == object_kind) return true;
            }
            return false;
        }
        return true;  // Default allow if not implicit_deny
    }

    [[nodiscard]] const TargetRule* find_rule_for(const std::string& object_kind) const {
        for (const auto& rule : target_rules) {
            if (rule.object_kind == object_kind) return &rule;
        }
        return nullptr;
    }

    [[nodiscard]] std::optional<std::string> default_path_for(const std::string& object_kind) const {
        if (const auto* rule = find_rule_for(object_kind)) {
            if (!rule->allowed_paths.empty()) {
                return rule->allowed_paths.front();  // Return first as default hint
            }
        }
        return std::nullopt;
    }

    [[nodiscard]] std::pair<std::optional<std::string>, std::optional<std::string>>
    tier_violation_hints_for() const {
        // Return (charter_rationale, suggested_alternative_repo)
        switch (tier) {
            case Tier::Tier1:
                return {"Tier1 repos are public contract-only surfaces; narrative content belongs in vault repos.",
                        "HorrorPlace-Atrocity-Seeds"};
            case Tier::Tier2:
                return {"Tier2 repos require explicit deadledgerref for high-intensity content.",
                        std::nullopt};
            case Tier::Tier3:
                return {"Tier3 research content requires governance approval before promotion.",
                        std::nullopt};
            default:
                return {std::nullopt, std::nullopt};
        }
    }
};

/// Policy checklist item for AI self-validation.
struct PolicyChecklistItem {
    std::string code;          // e.g., "ONE_FILE_PER_REQUEST"
    std::string description;   // Human-readable rule explanation
};

/// Aggregated policy checklist for a repo.
struct PolicyChecklist {
    std::string repo_name;
    std::string tier;
    std::vector<PolicyChecklistItem> items;

    static PolicyChecklist build(const RepoManifest& manifest) {
        PolicyChecklist checklist;
        checklist.repo_name = manifest.repo_name;
        checklist.tier = tier_to_string(manifest.tier);

        // Add standard policy items based on tier
        checklist.items.push_back({"ONE_FILE_PER_REQUEST", "Each response must contain exactly one artifact"});
        checklist.items.push_back({"SCHEMA_REF_REQUIRED", "All artifacts must reference a valid schema ID"});

        if (manifest.tier == Tier::Tier1) {
            checklist.items.push_back({"NO_RAW_NARRATIVE", "Tier1 repos accept only contract schemas, no raw horror content"});
        }

        // Add repo-specific rules
        for (const auto& rule : manifest.ai_authoring_rules) {
            checklist.items.push_back({rule.rule, rule.description});
        }

        return checklist;
    }
};

}  // namespace hpc::chat_director::types

// JSON serialization for manifest types
namespace nlohmann {
template <>
struct adl_serializer<hpc::chat_director::types::Tier> {
    static void to_json(json& j, hpc::chat_director::types::Tier tier) {
        j = hpc::chat_director::types::tier_to_string(tier);
    }
    static void from_json(const json& j, hpc::chat_director::types::Tier& tier) {
        const std::string s = j.get<std::string>();
        if (auto t = hpc::chat_director::types::tier_from_string(s)) {
            tier = t.value();
        } else {
            tier = hpc::chat_director::types::Tier::Tier1;  // Default fallback
        }
    }
};

template <>
struct adl_serializer<hpc::chat_director::types::AiRole> {
    static void to_json(json& j, hpc::chat_director::types::AiRole role) {
        switch (role) {
            case hpc::chat_director::types::AiRole::Architect: j = "Architect"; break;
            case hpc::chat_director::types::AiRole::Implementer: j = "Implementer"; break;
            case hpc::chat_director::types::AiRole::Auditor: j = "Auditor"; break;
        }
    }
    static void from_json(const json& j, hpc::chat_director::types::AiRole& role) {
        const std::string s = j.get<std::string>();
        if (s == "Architect") role = hpc::chat_director::types::AiRole::Architect;
        else if (s == "Implementer") role = hpc::chat_director::types::AiRole::Implementer;
        else if (s == "Auditor") role = hpc::chat_director::types::AiRole::Auditor;
        else role = hpc::chat_director::types::AiRole::Implementer;  // Default
    }
};

template <>
struct adl_serializer<hpc::chat_director::types::AuthoringRule> {
    static void from_json(const json& j, hpc::chat_director::types::AuthoringRule& r) {
        j.at("rule").get_to(r.rule);
        j.at("description").get_to(r.description);
    }
};

template <>
struct adl_serializer<hpc::chat_director::types::TargetRule> {
    static void from_json(const json& j, hpc::chat_director::types::TargetRule& r) {
        j.at("object_kind").get_to(r.object_kind);
        j.at("allowed_paths").get_to(r.allowed_paths);
        if (j.contains("allowed_schemas")) {
            j.at("allowed_schemas").get_to(r.allowed_schemas);
        }
        if (j.contains("allowed_roles")) {
            j.at("allowed_roles").get_to(r.allowed_roles);
        }
        if (j.contains("notes")) {
            j.at("notes").get_to(r.notes);
        }
    }
};

template <>
struct adl_serializer<hpc::chat_director::types::RepoManifest> {
    static void from_json(const json& j, hpc::chat_director::types::RepoManifest& m) {
        j.at("repo_name").get_to(m.repo_name);
        j.at("tier").get_to(m.tier);
        if (j.contains("schemas_whitelist")) {
            j.at("schemas_whitelist").get_to(m.schemas_whitelist);
        }
        if (j.contains("target_rules")) {
            j.at("target_rules").get_to(m.target_rules);
        }
        if (j.contains("implicit_deny")) {
            j.at("implicit_deny").get_to(m.implicit_deny);
        }
        if (j.contains("ai_authoring_rules")) {
            j.at("ai_authoring_rules").get_to(m.ai_authoring_rules);
        }
        if (j.contains("min_rwf_for_tier")) {
            m.min_rwf_for_tier = j.at("min_rwf_for_tier").get<double>();
        }
    }
};
}  // namespace nlohmann
