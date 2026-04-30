// crates/hpc-chat-director/cpp/include/hpc/chat_director/errors.hpp
#pragma once

#include <string>
#include <vector>
#include <memory>
#include <exception>
#include <nlohmann/json.hpp>

namespace hpc::chat_director {

/// Severity levels for validation diagnostics.
enum class ValidationSeverity {
    Error,    // Blocking issue
    Warning,  // Non-blocking but should be addressed
    Info      // Informational note
};

/// Layers of the validation pipeline.
enum class ValidationLayer {
    Schema,      // JSON Schema conformance
    Invariants,  // Invariant/metric range checks
    Manifest,    // Repo routing and tier policies
    Envelope,    // Prism envelope structure
    Phase,       // Phase lifecycle rules
    Other        // Unclassified
};

/// Machine-readable diagnostic with remediation hints for AI tools.
struct ValidationDiagnostic {
    ValidationLayer layer;
    ValidationSeverity severity;
    std::string code;              // e.g., "INV_RANGE_EXCEEDED"
    std::string message;           // Human-readable description
    std::optional<std::string> json_pointer;  // JSON Pointer to failing field
    std::optional<std::string> remediation;   // Suggested fix for AI

    [[nodiscard]] bool is_error() const noexcept {
        return severity == ValidationSeverity::Error;
    }
};

/// Aggregated result of a validation operation.
struct ValidationResult {
    bool ok;
    std::vector<ValidationDiagnostic> diagnostics;

    [[nodiscard]] bool passed() const noexcept { return ok; }

    [[nodiscard]] const ValidationDiagnostic* first_error() const {
        for (const auto& d : diagnostics) {
            if (d.is_error()) return &d;
        }
        return nullptr;
    }

    /// Return diagnostics sorted by severity (Error > Warning > Info).
    [[nodiscard]] std::vector<ValidationDiagnostic> ranked_diagnostics() const {
        std::vector<ValidationDiagnostic> sorted = diagnostics;
        auto severity_rank = [](ValidationSeverity s) -> int {
            switch (s) {
                case ValidationSeverity::Error: return 0;
                case ValidationSeverity::Warning: return 1;
                case ValidationSeverity::Info: return 2;
                default: return 3;
            }
        };
        std::sort(sorted.begin(), sorted.end(),
            [severity_rank](const ValidationDiagnostic& a, const ValidationDiagnostic& b) {
                return severity_rank(a.severity) < severity_rank(b.severity);
            });
        return sorted;
    }
};

/// Base exception for CHAT_DIRECTOR errors.
class ChatDirectorError : public std::exception {
public:
    explicit ChatDirectorError(std::string msg) : message_(std::move(msg)) {}
    [[nodiscard]] const char* what() const noexcept override { return message_.c_str(); }
    [[nodiscard]] const std::string& message() const noexcept { return message_; }

protected:
    std::string message_;
};

/// I/O or filesystem error.
class IoError : public ChatDirectorError {
public:
    IoError(std::string path, std::string details)
        : ChatDirectorError("I/O error at '" + path + "': " + details),
          path_(std::move(path)) {}
    [[nodiscard]] const std::string& path() const noexcept { return path_; }
private:
    std::string path_;
};

/// JSON parsing or schema validation error.
class ParseError : public ChatDirectorError {
public:
    explicit ParseError(std::string details)
        : ChatDirectorError("Parse error: " + details) {}
};

/// Invariant/metric range violation.
class InvariantError : public ChatDirectorError {
public:
    InvariantError(std::string invariant_name, double value, double min, double max)
        : ChatDirectorError("Invariant '" + invariant_name + "' value " +
                           std::to_string(value) + " outside range [" +
                           std::to_string(min) + ", " + std::to_string(max) + "]"),
          invariant_name_(std::move(invariant_name)),
          value_(value), min_(min), max_(max) {}

    [[nodiscard]] const std::string& invariant_name() const noexcept { return invariant_name_; }
    [[nodiscard]] double value() const noexcept { return value_; }
    [[nodiscard]] double min() const noexcept { return min_; }
    [[nodiscard]] double max() const noexcept { return max_; }

private:
    std::string invariant_name_;
    double value_, min_, max_;
};

/// Manifest routing or policy violation.
class ManifestError : public ChatDirectorError {
public:
    ManifestError(std::string repo, std::string object_kind, std::string reason)
        : ChatDirectorError("Manifest error for repo '" + repo + "', objectKind '" +
                           object_kind + "': " + reason),
          repo_(std::move(repo)), object_kind_(std::move(object_kind)) {}

    [[nodiscard]] const std::string& repo() const noexcept { return repo_; }
    [[nodiscard]] const std::string& object_kind() const noexcept { return object_kind_; }

private:
    std::string repo_, object_kind_;
};

/// Phase lifecycle rule violation.
class PhaseError : public ChatDirectorError {
public:
    PhaseError(uint8_t phase, std::string object_kind, std::string reason)
        : ChatDirectorError("Phase " + std::to_string(phase) + " violation for '" +
                           object_kind + "': " + reason),
          phase_(phase), object_kind_(std::move(object_kind)) {}

    [[nodiscard]] uint8_t phase() const noexcept { return phase_; }
    [[nodiscard]] const std::string& object_kind() const noexcept { return object_kind_; }

private:
    uint8_t phase_;
    std::string object_kind_;
};

/// Validation failure with structured diagnostics.
class ValidationError : public ChatDirectorError {
public:
    explicit ValidationError(ValidationResult result)
        : ChatDirectorError("Validation failed with " +
                           std::to_string(result.diagnostics.size()) + " issue(s)"),
          result_(std::move(result)) {}

    [[nodiscard]] const ValidationResult& result() const noexcept { return result_; }

private:
    ValidationResult result_;
};

}  // namespace hpc::chat_director
