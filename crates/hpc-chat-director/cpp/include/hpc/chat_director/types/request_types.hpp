// crates/hpc-chat-director/cpp/include/hpc/chat_director/types/request_types.hpp
#pragma once

#include <string>
#include <vector>
#include <optional>
#include <nlohmann/json.hpp>

namespace hpc::chat_director::types {

/// Region hints for binding resolution.
struct RegionHints {
    std::string regionClass;
    std::string style;
    double cic{0.0};
    double aos{0.0};
    double det{0.0};
    double lsg{0.0};
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(RegionHints, regionClass, style, cic, aos, det, lsg)

/// Constraints imposed on the authoring request.
struct Constraints {
    std::vector<std::string> allowedTiers;
    std::vector<std::string> allowedSafetyProfiles;
    std::string maxCurveComplexity{"medium"};
    std::optional<int> maxBindings;
    std::optional<std::string> notes;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Constraints, allowedTiers, allowedSafetyProfiles, maxCurveComplexity, maxBindings, notes)

/// ai-bci-geometry-request-v1 mirror.
struct AiAuthoringRequest {
    std::string schemaVersion{"1.0.0"};
    std::string targetRepo;
    std::string targetPath;
    std::string bindingSchemaId;
    RegionHints regionHints;
    std::string experienceType;
    Constraints constraints;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(AiAuthoringRequest, schemaVersion, targetRepo, targetPath, bindingSchemaId, regionHints, experienceType, constraints)

} // namespace hpc::chat_director::types
