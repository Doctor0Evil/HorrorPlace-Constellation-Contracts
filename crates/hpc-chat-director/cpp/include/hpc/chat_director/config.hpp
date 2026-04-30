// crates/hpc-chat-director/cpp/include/hpc/chat_director/config.hpp
#pragma once

#include <string>
#include <vector>
#include <optional>
#include <filesystem>
#include <nlohmann/json.hpp>
#include "types/manifest_types.hpp"
#include "errors.hpp"

namespace hpc::chat_director {

/// Execution context that adjusts validation strictness.
enum class ContextMode {
    Local,        // Development: warnings, auto-fix hints
    CI,           // Continuous integration: hard failures, strict checks
    EditorPlugin  // IDE integration: real-time feedback, non-blocking
};

/// Runtime configuration for CHAT_DIRECTOR.
class Config {
public:
    /// Detect constellation root and load manifests.
    static Config detect(const std::filesystem::path& root_hint);

    [[nodiscard]] const std::filesystem::path& root() const noexcept { return root_; }
    [[nodiscard]] const std::vector<types::RepoManifest>& manifests() const noexcept { return manifests_; }
    [[nodiscard]] ContextMode context_mode() const noexcept { return context_mode_; }
    [[nodiscard]] const std::string& spine_version() const noexcept { return spine_version_; }
    [[nodiscard]] const std::vector<std::string>& envelope_versions() const noexcept { return envelope_versions_; }
    [[nodiscard]] std::optional<std::filesystem::path> output_dir() const { return output_dir_; }

    /// Set output directory for generated artifacts.
    Config& with_output_dir(std::filesystem::path dir) {
        output_dir_ = std::move(dir);
        return *this;
    }

    /// Find manifest by repo name.
    [[nodiscard]] const types::RepoManifest* find_manifest(const std::string& repo_name) const {
        for (const auto& m : manifests_) {
            if (m.repo_name == repo_name) return &m;
        }
        return nullptr;
    }

private:
    Config(std::filesystem::path root,
           std::vector<types::RepoManifest> manifests,
           ContextMode mode,
           std::string spine_version,
           std::vector<std::string> envelope_versions)
        : root_(std::move(root)),
          manifests_(std::move(manifests)),
          context_mode_(mode),
          spine_version_(std::move(spine_version)),
          envelope_versions_(std::move(envelope_versions)) {}

    std::filesystem::path root_;
    std::vector<types::RepoManifest> manifests_;
    ContextMode context_mode_;
    std::string spine_version_;
    std::vector<std::string> envelope_versions_;
    std::optional<std::filesystem::path> output_dir_;
};

/// Summary of the detected constellation environment.
struct EnvironmentSummary {
    std::filesystem::path root;
    std::string spine_version;
    ContextMode context_mode;
    std::vector<std::string> available_object_kinds;
    std::vector<std::string> available_repos;

    [[nodiscard]] nlohmann::json to_json() const {
        nlohmann::json j;
        j["root"] = root.string();
        j["spineVersion"] = spine_version;
        j["contextMode"] = static_cast<int>(context_mode);
        j["availableObjectKinds"] = available_object_kinds;
        j["availableRepos"] = available_repos;
        return j;
    }
};

}  // namespace hpc::chat_director
