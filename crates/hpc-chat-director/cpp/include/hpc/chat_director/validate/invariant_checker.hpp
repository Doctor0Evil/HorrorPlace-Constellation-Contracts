// crates/hpc-chat-director/cpp/include/hpc/chat_director/validate/invariant_checker.hpp
#pragma once

#include <string>
#include <vector>
#include "../spine.hpp"
#include "../config.hpp"
#include "../errors.hpp"

namespace hpc::chat_director::validate {

/// Validates invariant and metric values against spine definitions.
class InvariantChecker {
public:
    /// Validate a JSON document containing invariant/metric bindings.
    [[nodiscard]] static ValidationResult validate(
        const nlohmann::json& document,
        const SpineIndex& spine,
        ContextMode mode);

    /// Check a single invariant value against its spec.
    [[nodiscard]] static std::optional<ValidationDiagnostic>
    check_invariant_value(const std::string& code,
                         double value,
                         const types::InvariantSpec& spec,
                         const std::string& tier);

    /// Check cross-metric interaction rules.
    [[nodiscard]] static std::vector<ValidationDiagnostic>
    check_interaction_rules(const nlohmann::json& document,
                           const SpineIndex& spine);
};

}  // namespace hpc::chat_director::validate
