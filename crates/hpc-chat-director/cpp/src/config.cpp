// crates/hpc-chat-director/cpp/src/config.cpp
#include "hpc/chat_director/config.hpp"
#include "hpc/chat_director/types/manifest_types.hpp"

#include <fstream>
#include <cstdlib>
#include <algorithm>

namespace fs = std::filesystem;
using json = nlohmann::json;

namespace hpc::chat_director {

static fs::path detect_root(const fs::path& root_hint) {
    // 1. Check explicit hint
    if (fs::exists(root_hint / "schemas")) {
        return root_hint;
    }

    // 2. Check environment variable
    if (const char* env_root = std::getenv("HPC_CONSTELLATION_ROOT")) {
        fs::path path(env_root);
        if (fs::exists(path / "schemas")) {
            return path;
        }
    }

    // 3. Walk up from current directory
    fs::path current = fs::current_path();
    while (!current.empty()) {
        if (fs::exists(current / "schemas") && fs::exists(current / "CMakeLists.txt")) {
            return current;
        }
        current = current.parent_path();
    }

    throw ChatDirectorError("Failed to detect constellation root");
}

static std::vector<types::RepoManifest> load_manifests(const fs::path& root) {
    std::vector<types::RepoManifest> manifests;
    const fs::path manifests_dir = root / "manifests";

    if (!fs::exists(manifests_dir)) {
        return manifests;  // Empty is valid
    }

    for (const auto& entry : fs::directory_iterator(manifests_dir)) {
        if (!entry.is_regular_file()) continue;

        const std::string filename = entry.path().filename().string();
        if (filename.find("repo-manifest.hpc.") != 0 ||
            filename.find(".json") != filename.size() - 5) {
            continue;
        }

        std::ifstream file(entry.path());
        if (!file.is_open()) {
            throw IoError(entry.path().string(), "Could not open file");
        }

        json j;
        file >> j;

        types::RepoManifest manifest;
        j.get_to(manifest);
        manifests.push_back(std::move(manifest));
    }

    return manifests;
}

Config Config::detect(const fs::path& root_hint) {
    const fs::path root = detect_root(root_hint);
    auto manifests = load_manifests(root);

    // Environment variables for configuration
    std::string spine_version = "v1";
    if (const char* env_ver = std::getenv("HPC_SPINE_VERSION")) {
        spine_version = env_ver;
    }

    std::vector<std::string> envelope_versions = {"v1"};
    if (const char* env_envs = std::getenv("HPC_ENVELOPE_VERSIONS")) {
        envelope_versions.clear();
        std::string s(env_envs);
        size_t start = 0;
        while (start < s.size()) {
            size_t end = s.find(',', start);
            if (end == std::string::npos) end = s.size();
            std::string ver = s.substr(start, end - start);
            // Trim whitespace
            ver.erase(0, ver.find_first_not_of(" \t"));
            ver.erase(ver.find_last_not_of(" \t") + 1);
            if (!ver.empty()) envelope_versions.push_back(ver);
            start = end + 1;
        }
    }

    ContextMode mode = ContextMode::Local;
    if (const char* env_mode = std::getenv("HPC_CONTEXT_MODE")) {
        std::string m(env_mode);
        std::transform(m.begin(), m.end(), m.begin(), ::tolower);
        if (m == "ci") mode = ContextMode::CI;
        else if (m == "editor" || m == "editorplugin") mode = ContextMode::EditorPlugin;
    }

    return Config(root, std::move(manifests), mode, spine_version, envelope_versions);
}

}  // namespace hpc::chat_director
