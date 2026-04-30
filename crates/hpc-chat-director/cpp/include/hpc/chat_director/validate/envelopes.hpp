// crates/hpc-chat-director/cpp/include/hpc/chat_director/validate/envelopes.hpp
#pragma once

#include <string>
#include <nlohmann/json.hpp>
#include "../errors.hpp"
#include "../types/response_types.hpp"

namespace hpc::chat_director::validate {

/// Validates structural integrity and required fields of an AI envelope.
ValidationResult validate_envelope_structure(const nlohmann::json& data);

/// Verifies schema hash and version alignment for AI-generated responses.
ValidationResult validate_schema_verification(
    const types::AiAuthoringResponse& response,
    const std::string& expected_schema_hash
);

} // namespace hpc::chat_director::validate
