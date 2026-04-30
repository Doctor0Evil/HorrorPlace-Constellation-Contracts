// crates/hpc-chat-director/cpp/src/validate/envelopes.cpp
#include "hpc/chat_director/validate/envelopes.hpp"
#include <algorithm>

namespace hpc::chat_director::validate {

ValidationResult validate_envelope_structure(const nlohmann::json& data) {
    ValidationResult result{.ok = true};
    const std::vector<std::string> REQUIRED_FIELDS = {"schemaVersion", "targetRepo", "targetPath", "bindingSchemaId"};

    for (const auto& field : REQUIRED_FIELDS) {
        if (!data.contains(field)) {
            result.ok = false;
            result.diagnostics.push_back({ValidationLayer::Envelope, ValidationSeverity::Error, "MISSING_REQUIRED_FIELD",
                "Envelope missing required field: " + field});
        } else if (!data[field].is_string()) {
            result.ok = false;
            result.diagnostics.push_back({ValidationLayer::Envelope, ValidationSeverity::Error, "INVALID_FIELD_TYPE",
                "Field '" + field + "' must be a string."});
        }
    }

    if (data.contains("experienceType")) {
        static const std::vector<std::string> ALLOWED_TYPES = {"monster-mode", "slow-dread", "startle-light", "ambient-creep"};
        std::string exp_type = data["experienceType"].get<std::string>();
        if (std::find(ALLOWED_TYPES.begin(), ALLOWED_TYPES.end(), exp_type) == ALLOWED_TYPES.end()) {
            result.diagnostics.push_back({ValidationLayer::Envelope, ValidationSeverity::Warning, "UNKNOWN_EXPERIENCE_TYPE",
                "experienceType '" + exp_type + "' is not in the standard taxonomy."});
        }
    }

    return result;
}

ValidationResult validate_schema_verification(const types::AiAuthoringResponse& response, const std::string& expected_schema_hash) {
    ValidationResult result{.ok = true};

    if (response.meta.schemaHash.empty()) {
        result.ok = false;
        result.diagnostics.push_back({ValidationLayer::Envelope, ValidationSeverity::Error, "MISSING_SCHEMA_HASH",
            "Response must include schemaVerification.schemaHash."});
        return result;
    }

    if (response.meta.schemaHash != expected_schema_hash) {
        result.ok = false;
        result.diagnostics.push_back({ValidationLayer::Envelope, ValidationSeverity::Error, "SCHEMA_HASH_MISMATCH",
            "Generated schema hash does not match the canonical spine hash. Regenerate using current schema."});
    }

    if (response.bindings.empty()) {
        result.diagnostics.push_back({ValidationLayer::Envelope, ValidationSeverity::Warning, "EMPTY_BINDINGS_ARRAY",
            "Response contains zero bindings."});
    }

    return result;
}

} // namespace hpc::chat_director::validate
