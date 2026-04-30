// crates/hpc-chat-director/cpp/include/hpc/chat_director/types/response_types.hpp
#pragma once

#include <string>
#include <vector>
#include <nlohmann/json.hpp>
#include "request_types.hpp"

namespace hpc::chat_director::types {

/// Metadata for AI-generated responses.
struct ResponseMeta {
    std::string authorAgent{"ai-chat"};
    std::string notes;
    std::string createdAt; // ISO-8601
    std::string schemaHash; // SHA-256 of the schema used during generation
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(ResponseMeta, authorAgent, notes, createdAt, schemaHash)

/// ai-bci-geometry-response-v1 mirror.
struct AiAuthoringResponse {
    std::string schemaVersion{"1.0.0"};
    std::vector<nlohmann::json> bindings; // Preserves schema flexibility for binding arrays
    ResponseMeta meta;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(AiAuthoringResponse, schemaVersion, bindings, meta)

} // namespace hpc::chat_director::types
