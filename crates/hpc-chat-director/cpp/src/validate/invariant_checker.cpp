// crates/hpc-chat_director/cpp/src/validate/invariant_checker.cpp
#include "hpc/chat_director/validate/invariant_checker.hpp"
#include "hpc/chat_director/types/invariant_types.hpp"

#include <cmath>

using json = nlohmann::json;

namespace hpc::chat_director::validate {

ValidationResult InvariantChecker::validate(
    const json& document,
    const SpineIndex& spine,
    ContextMode mode) {

    ValidationResult result{.ok = true};

    // Extract invariant bindings if present
    if (document.contains("invariantBindings") && document["invariantBindings"].is_object()) {
        const json& bindings = document["invariantBindings"];

        for (auto it = bindings.begin(); it != bindings.end(); ++it) {
            const std::string code = it.key();
            const auto* spec = spine.inner().find_invariant_by_code(code);

            if (!spec) {
                // Unknown invariant - error in strict mode, warning otherwise
                ValidationSeverity severity = (mode == ContextMode::CI)
                    ? ValidationSeverity::Error
                    : ValidationSeverity::Warning;

                result.diagnostics.push_back(ValidationDiagnostic{
                    .layer = ValidationLayer::Invariants,
                    .severity = severity,
                    .code = "UNKNOWN_INVARIANT",
                    .message = "Unknown invariant code: " + code,
                    .json_pointer = "/invariantBindings/" + code,
                    .remediation = "Use only invariants defined in the schema spine: " +
                                   list_known_invariants(spine)
                });
                if (severity == ValidationSeverity::Error) result.ok = false;
                continue;
            }

            // Parse value (support both scalar and object forms)
            double value;
            if (it->is_number()) {
                value = it->get<double>();
            } else if (it->is_object() && it->contains("value")) {
                value = it->at("value").get<double>();
            } else {
                result.diagnostics.push_back(ValidationDiagnostic{
                    .layer = ValidationLayer::Invariants,
                    .severity = ValidationSeverity::Error,
                    .code = "INVALID_VALUE_FORMAT",
                    .message = "Invariant '" + code + "' must be a number or {\"value\": number}",
                    .json_pointer = "/invariantBindings/" + code,
                    .remediation = "Provide a numeric value or {\"value\": <number>} object"
                });
                result.ok = false;
                continue;
            }

            // Check range
            if (auto diag = check_invariant_value(code, value, *spec, "Tier2")) {
                result.diagnostics.push_back(diag.value());
                if (diag->severity == ValidationSeverity::Error) result.ok = false;
            }
        }
    }

    // Check interaction rules
    auto interaction_diags = check_interaction_rules(document, spine);
    for (auto& diag : interaction_diags) {
        if (diag.severity == ValidationSeverity::Error) result.ok = false;
        result.diagnostics.push_back(std::move(diag));
    }

    return result;
}

std::optional<ValidationDiagnostic>
InvariantChecker::check_invariant_value(const std::string& code,
                                       double value,
                                       const types::InvariantSpec& spec,
                                       const std::string& tier) {
    const auto effective_range = spec.effective_range_for_tier(tier);

    if (!effective_range.contains(value)) {
        std::string remediation = "For " + tier + ", " + spec.name +
                                 " must be in [" + std::to_string(effective_range.min) +
                                 ", " + std::to_string(effective_range.max) + "]";

        // Add drift-aware suggestion if applicable
        if (spec.drift.allowed && spec.drift.max_delta_per_release > 0) {
            remediation += ". Small deviations may be acceptable if justified by drift policy.";
        }

        return ValidationDiagnostic{
            .layer = ValidationLayer::Invariants,
            .severity = ValidationSeverity::Error,
            .code = "INV_RANGE_EXCEEDED",
            .message = spec.name + " value " + std::to_string(value) +
                      " outside allowed range [" +
                      std::to_string(effective_range.min) + ", " +
                      std::to_string(effective_range.max) + "]",
            .json_pointer = "/invariantBindings/" + code,
            .remediation = std::move(remediation)
        };
    }

    return std::nullopt;
}

std::vector<ValidationDiagnostic>
InvariantChecker::check_interaction_rules(const json& document,
                                         const SpineIndex& spine) {
    std::vector<ValidationDiagnostic> diagnostics;

    // Example rule: high DET suppresses ARR floor
    // Real implementation would iterate spine.interaction_rules()

    if (document.contains("invariantBindings")) {
        const auto& bindings = document["invariantBindings"];
        if (bindings.contains("DET") && bindings.contains("ARR")) {
            double det = bindings["DET"].is_number()
                ? bindings["DET"].get<double>()
                : bindings["DET"].value("value", 0.0);

            double arr = bindings["ARR"].is_number()
                ? bindings["ARR"].get<double>()
                : bindings["ARR"].value("value", 0.0);

            // Rule: if DET > 8.0, ARR must be >= 0.4
            if (det > 8.0 && arr < 0.4) {
                diagnostics.push_back(ValidationDiagnostic{
                    .layer = ValidationLayer::Invariants,
                    .severity = ValidationSeverity::Warning,
                    .code = "INV_INTERACTION_VIOLATION",
                    .message = "High DET (" + std::to_string(det) +
                              ") typically requires ARR >= 0.4, but got " +
                              std::to_string(arr),
                    .json_pointer = "/invariantBindings/ARR",
                    .remediation = "Consider raising ARR to >= 0.4 when DET is high, "
                                  "or document why this exception is justified"
                });
            }
        }
    }

    return diagnostics;
}

// Helper to list known invariants (simplified)
static std::string list_known_invariants(const SpineIndex& spine) {
    std::string list;
    for (size_t i = 0; i < spine.invariant_specs().size(); ++i) {
        if (i > 0) list += ", ";
        list += spine.invariant_specs()[i].code;
    }
    return list;
}

}  // namespace hpc::chat_director::validate
