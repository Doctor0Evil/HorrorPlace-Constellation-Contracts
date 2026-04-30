// crates/hpc-chat-director/cpp/include/hpc/chat_director/chat_director.hpp
#pragma once

#include <memory>
#include <optional>
#include <filesystem>
#include <nlohmann/json.hpp>
#include "config.hpp"
#include "spine.hpp"
#include "types/manifest_types.hpp"
#include "errors.hpp"

namespace hpc::chat_director {

/// Capability catalog for AI pre-flight discovery.
struct CapabilityCatalog {
    std::optional<std::string> spine_version;
    std::vector<std::string> available_object_kinds;
    std::vector<types::RepoSummary> available_repos;
    std::vector<types::InvariantSummary> invariants;
    std::vector<types::MetricSummary> metrics;
    std::vector<types::PhaseSummary> phases;

    [[nodiscard]] nlohmann::json to_json() const;
};

/// Canonical target for file placement suggestions.
struct CanonicalTarget {
    std::string target_repo;
    std::string target_path;
    types::Tier tier;
    double confidence;  // 0.0 to 1.0
};

/// Core façade for constellation authoring operations.
class ChatDirector {
public:
    /// Load the constellation environment from a resolved Config.
    static std::unique_ptr<ChatDirector> load_environment(Config config);

    /// Build from pre-loaded components (for testing).
    ChatDirector(Config config, SpineIndex spine, std::vector<types::RepoManifest> manifests);

    /// Access the loaded schema spine.
    [[nodiscard]] const types::SchemaSpine& spine_schema() const noexcept {
        return spine_.inner();
    }

    /// Access the loaded repo manifests.
    [[nodiscard]] const std::vector<types::RepoManifest>& repo_manifests() const noexcept {
        return manifests_;
    }

    /// Return a capability catalog for AI pre-flight discovery.
    [[nodiscard]] CapabilityCatalog capability_catalog() const;

    /// Build a policy checklist for a given repo.
    [[nodiscard]] std::optional<types::PolicyChecklist>
    policy_checklist_for_repo(const std::string& repo_name) const;

    /// Normalize a natural-language prompt into a structured authoring request.
    /// Returns JSON matching ai-authoring-request-v1.json schema.
    [[nodiscard]] nlohmann::json plan_from_prompt(
        const std::string& prompt,
        std::optional<std::string> object_kind_hint) const;

    /// Generate a minimal, schema-compliant skeleton for the requested object.
    [[nodiscard]] nlohmann::json generate_skeleton(const nlohmann::json& request) const;

    /// Validate an AI-generated response against the full constraint stack.
    [[nodiscard]] ValidationResult validate_response(
        const nlohmann::json& request,
        const nlohmann::json& response) const;

    /// Apply a validated contract to disk at its manifest-approved path.
    /// If dry_run is true, no files are written; returns planned actions.
    [[nodiscard]] nlohmann::json apply(
        const nlohmann::json& request,
        const nlohmann::json& response,
        bool dry_run) const;

    /// Return environment summary for path planning.
    [[nodiscard]] EnvironmentSummary environment_summary() const;

    /// Suggest canonical target paths for a given intent and objectKind.
    [[nodiscard]] std::vector<CanonicalTarget>
    plan_paths(const std::string& intent, const std::string& object_kind) const;

private:
    Config config_;
    SpineIndex spine_;
    std::vector<types::RepoManifest> manifests_;
};

}  // namespace hpc::chat_director
