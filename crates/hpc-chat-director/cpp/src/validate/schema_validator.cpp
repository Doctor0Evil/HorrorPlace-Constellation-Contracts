// crates/hpc-chat-director/cpp/src/validate/schema_validator.cpp
#include "hpc/chat_director/validate/schema_validator.hpp"

#include <fstream>
#include <filesystem>

namespace fs = std::filesystem;
using json = nlohmann::json;

namespace hpc::chat_director::validate {

std::unique_ptr<SchemaValidator>
SchemaValidator::load_from_file(const std::string& schema_path) {
    if (!fs::exists(schema_path)) {
        throw IoError(schema_path, "Schema file not found");
    }

    std::ifstream file(schema_path);
    if (!file.is_open()) {
        throw IoError(schema_path, "Could not open schema file");
    }

    json schema_json;
    file >> schema_json;

    std::string schema_id = schema_json.value("$id", "unknown");
    std::string description = schema_json.value("description", "No description");

    return std::make_unique<SchemaValidator>(std::move(schema_id), std::move(description));
}

SchemaValidator::SchemaValidator(std::string schema_id, std::string description)
    : schema_id_(std::move(schema_id)),
      description_(std::move(description)) {}

ValidationResult SchemaValidator::validate(const json& document) const {
    ValidationResult result{.ok = true};

    // Simplified structural validation for v1
    // In production, integrate with nlohmann/json-schema or similar

    // Check required fields if specified in schema
    // This is a placeholder; real implementation would parse $required

    // Check type conformance for known schemas
    if (schema_id_.find("ai-authoring-response") != std::string::npos) {
        if (!document.contains("schemaVersion") || !document.contains("artifact")) {
            result.ok = false;
            result.diagnostics.push_back(ValidationDiagnostic{
                .layer = ValidationLayer::Schema,
                .severity = ValidationSeverity::Error,
                .code = "MISSING_REQUIRED_FIELD",
                .message = "ai-authoring-response must contain 'schemaVersion' and 'artifact'",
                .json_pointer = "/",
                .remediation = "Add the missing required fields to your response"
            });
        }
    }

    // Check for additionalProperties: false violations (simplified)
    // Real implementation would walk the schema tree

    return result;
}

ValidationResult SchemaValidator::quick_check(const json& document) const {
    ValidationResult result{.ok = true};

    // Only check top-level required fields and basic types
    if (!document.is_object()) {
        result.ok = false;
        result.diagnostics.push_back(ValidationDiagnostic{
            .layer = ValidationLayer::Schema,
            .severity = ValidationSeverity::Warning,
            .code = "NOT_OBJECT",
            .message = "Document root should be a JSON object",
            .json_pointer = "/",
            .remediation = "Ensure your artifact is a JSON object, not an array or primitive"
        });
    }

    return result;
}

std::string SchemaValidator::describe() const {
    return description_;
}

}  // namespace hpc::chat_director::validate
