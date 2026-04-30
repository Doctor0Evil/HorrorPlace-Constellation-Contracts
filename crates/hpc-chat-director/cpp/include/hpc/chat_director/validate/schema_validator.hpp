// crates/hpc-chat-director/cpp/include/hpc/chat_director/validate/schema_validator.hpp
#pragma once

#include <string>
#include <vector>
#include <memory>
#include <nlohmann/json.hpp>
#include "../errors.hpp"

namespace hpc::chat_director::validate {

/// JSON Schema validator wrapper with caching.
class SchemaValidator {
public:
    /// Load and compile a schema from file.
    static std::unique_ptr<SchemaValidator> load_from_file(const std::string& schema_path);

    /// Validate a JSON document against this schema.
    [[nodiscard]] ValidationResult validate(const nlohmann::json& document) const;

    /// Quick-check mode: validate only required fields and top-level structure.
    [[nodiscard]] ValidationResult quick_check(const nlohmann::json& document) const;

    /// Return a human-readable description of this schema.
    [[nodiscard]] std::string describe() const;

private:
    explicit SchemaValidator(std::string schema_id, std::string description);

    std::string schema_id_;
    std::string description_;
    // In production, this would hold a compiled schema from a library like
    // nlohmann/json-schema or a custom validator. For v1, we do structural checks.
};

}  // namespace hpc::chat_director::validate
