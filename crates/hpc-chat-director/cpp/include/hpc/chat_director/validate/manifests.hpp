// crates/hpc-chat-director/cpp/include/hpc/chat_director/validate/manifests.hpp
#pragma once

#include <string>
#include <vector>
#include "../errors.hpp"

namespace hpc::chat_director::validate {

/// Validates routing, repo existence, and tier-policy compliance.
ValidationResult validate_manifest_routing(
    const std::string& target_repo,
    const std::string& target_path,
    const std::string& object_kind,
    const std::string& tier
);

/// Checks if a repo accepts the given object kind at the specified tier.
bool is_repo_routing_valid(
    const std::string& repo_id,
    const std::string& object_kind,
    const std::string& tier
);

} // namespace hpc::chat_director::validate
