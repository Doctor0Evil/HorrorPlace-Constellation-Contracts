// crates/hpc-chat-director/cpp/src/chat_director.cpp
#include "hpc/chat_director/chat_director.hpp"
#include "hpc/chat_director/types/invariant_types.hpp"
#include "hpc/chat_director/validate/schema_validator.hpp"
#include "hpc/chat_director/validate/invariant_checker.hpp"

#include <algorithm>
#include <fstream>

namespace fs = std::filesystem;
using json = nlohmann::json;

namespace hpc::chat_director {

std::unique_ptr<ChatDirector> ChatDirector::load_environment(Config config) {
    SpineIndex spine = SpineIndex::load(config);
    auto manifests = config.manifests();  // Already loaded by Config::detect
    return std::make_unique<ChatDirector>(
        std::move(config), std::move(spine), std::move(manifests));
}

ChatDirector::ChatDirector(Config config,
                           SpineIndex spine,
                           std::vector<types::RepoManifest> manifests)
    : config_(std::move(config)),
      spine_(std::move(spine)),
      manifests_(std::move(manifests)) {}

CapabilityCatalog ChatDirector::capability_catalog() const {
    CapabilityCatalog catalog;
    catalog.spine_version = spine_.version();

    // Collect object kinds from contract families
    for (const auto& family : spine_schema().contract_families) {
        if (std::find(catalog.available_object_kinds.begin(),
                      catalog.available_object_kinds.end(),
                      family.object_kind) == catalog.available_object_kinds.end()) {
            catalog.available_object_kinds.push_back(family.object_kind);
        }
    }

    // Collect repos from manifests
    for (const auto& manifest : manifests_) {
        catalog.available_repos.push_back(types::RepoSummary{
            .name = manifest.repo_name,
            .tier = types::tier_to_string(manifest.tier),
            .path = ""  // Could be populated with actual path resolution
        });
    }

    // Collect invariants
    for (const auto& inv : spine_schema().invariants) {
        catalog.invariants.push_back(types::InvariantSummary{
            .name = inv.name,
            .description = inv.description,
            .min = inv.range.min,
            .max = inv.range.max
        });
    }

    // Collect metrics
    for (const auto& metric : spine_schema().metrics) {
        catalog.metrics.push_back(types::MetricSummary{
            .name = metric.name,
            .description = metric.description,
            .target_min = metric.target_min,
            .target_max = metric.target_max
        });
    }

    // Collect phases
    catalog.phases = {
        {0, "Schema0", "Core schemas and invariants/metrics spine definition."},
        {1, "Registry1", "Registry entries and region/seed/mood/event indices."},
        {2, "Bundles2", "Higher-order bundles and choreography contracts."},
        {3, "LuaPolicy3", "Lua policy modules bound to contracts and metrics."},
        {4, "Adapters4", "Engine adapters and external pipeline integration."}
    };

    return catalog;
}

std::optional<types::PolicyChecklist>
ChatDirector::policy_checklist_for_repo(const std::string& repo_name) const {
    for (const auto& manifest : manifests_) {
        if (manifest.repo_name == repo_name) {
            return types::PolicyChecklist::build(manifest);
        }
    }
    return std::nullopt;
}

json ChatDirector::plan_from_prompt(
    const std::string& prompt,
    std::optional<std::string> object_kind_hint) const {
    // Simplified intent parsing for v1
    // In production, this would use NLP or pattern matching

    std::string object_kind = object_kind_hint.value_or("regionContractCard");

    // Determine target repo from manifests
    std::string target_repo = "HorrorPlace-Atrocity-Seeds";  // Default for regions
    std::string target_path = "contracts/regions/";

    // Get safe defaults for pre-filling
    auto ranges = spine_.suggest_ranges(object_kind, "Tier2");

    json request;
    request["schemaVersion"] = "ai-authoring-request-v1";
    request["requestId"] = "auto-generated-" + std::to_string(std::time(nullptr));
    request["objectKind"] = object_kind;
    request["targetRepo"] = target_repo;
    request["targetPath"] = target_path;
    request["tier"] = "Tier2";

    // Pre-fill invariant bands if available
    if (ranges) {
        json bands;
        if (ranges->cic) bands["CIC"] = json{{"min", ranges->cic->min}, {"max", ranges->cic->max}};
        if (ranges->aos) bands["AOS"] = json{{"min", ranges->aos->min}, {"max", ranges->aos->max}};
        if (ranges->det) bands["DET"] = json{{"min", ranges->det->min}, {"max", ranges->det->max}};
        if (ranges->uec) bands["UEC"] = json{{"min", ranges->uec->min}, {"max", ranges->uec->max}};
        if (ranges->arr) bands["ARR"] = json{{"min", ranges->arr->min}, {"max", ranges->arr->max}};
        if (ranges->shci) bands["SHCI"] = json{{"min", ranges->shci->min}, {"max", ranges->shci->max}};
        if (!bands.empty()) request["suggestedBands"] = bands;
    }

    request["prompt"] = prompt;
    return request;
}

json ChatDirector::generate_skeleton(const json& request) const {
    // Delegate to generation module
    // Simplified stub for v1
    const std::string object_kind = request.at("objectKind").get<std::string>();

    json skeleton;
    skeleton["id"] = "auto-generated-" + std::to_string(std::time(nullptr));
    skeleton["schemaVersion"] = "HorrorMappingConfig.BCI.v1";
    skeleton["objectKind"] = object_kind;

    // Add minimal required fields based on objectKind
    if (object_kind == "regionContractCard") {
        skeleton["invariantBindings"] = json::object();
        skeleton["metricTargets"] = json::object();
        skeleton["registryReady"] = false;
    }
    // ... other objectKind handlers

    return skeleton;
}

ValidationResult ChatDirector::validate_response(
    const json& request,
    const json& response) const {
    // Run full validation pipeline
    ValidationResult result{.ok = true};

    // 1. Schema validation
    auto schema_result = validate::SchemaValidator::validate(response);
    if (!schema_result.ok) {
        result.ok = false;
        result.diagnostics.insert(result.diagnostics.end(),
                                  schema_result.diagnostics.begin(),
                                  schema_result.diagnostics.end());
    }

    // 2. Invariant/metric checks
    auto inv_result = validate::InvariantChecker::validate(
        response, spine_, config_.context_mode());
    if (!inv_result.ok) {
        result.ok = false;
        result.diagnostics.insert(result.diagnostics.end(),
                                  inv_result.diagnostics.begin(),
                                  inv_result.diagnostics.end());
    }

    // 3. Manifest routing and policy checks
    // ... (implementation in validate/manifests.cpp)

    // 4. Envelope structure checks
    // ... (implementation in validate/envelopes.cpp)

    return result;
}

json ChatDirector::apply(
    const json& request,
    const json& response,
    bool dry_run) const {
    const std::string target_repo = request.at("targetRepo").get<std::string>();
    const std::string target_path = request.at("targetPath").get<std::string>();
    const std::string filename = response.at("filename").get<std::string>();

    fs::path output_path;
    if (config_.output_dir()) {
        output_path = config_.output_dir().value() / target_path / filename;
    } else {
        output_path = config_.root() / target_repo / target_path / filename;
    }

    json actions;
    actions["action"] = dry_run ? "plan" : "write";
    actions["path"] = output_path.string();
    actions["repo"] = target_repo;
    actions["filename"] = filename;

    if (!dry_run) {
        // Ensure directory exists
        fs::create_directories(output_path.parent_path());

        // Write file
        std::ofstream file(output_path);
        if (!file.is_open()) {
            throw IoError(output_path.string(), "Could not open output file");
        }
        file << response.dump(2);  // Pretty-print with 2-space indent
        file.close();
    }

    return actions;
}

EnvironmentSummary ChatDirector::environment_summary() const {
    EnvironmentSummary summary;
    summary.root = config_.root();
    summary.spine_version = config_.spine_version();
    summary.context_mode = config_.context_mode();

    for (const auto& family : spine_schema().contract_families) {
        if (std::find(summary.available_object_kinds.begin(),
                      summary.available_object_kinds.end(),
                      family.object_kind) == summary.available_object_kinds.end()) {
            summary.available_object_kinds.push_back(family.object_kind);
        }
    }

    for (const auto& manifest : manifests_) {
        summary.available_repos.push_back(manifest.repo_name);
    }

    return summary;
}

std::vector<CanonicalTarget> ChatDirector::plan_paths(
    const std::string& intent,
    const std::string& object_kind) const {
    std::vector<CanonicalTarget> targets;

    // Simple routing logic for v1
    for (const auto& manifest : manifests_) {
        if (const auto* rule = manifest.find_rule_for(object_kind)) {
            for (const auto& path_pattern : rule->allowed_paths) {
                targets.push_back(CanonicalTarget{
                    .target_repo = manifest.repo_name,
                    .target_path = path_pattern,
                    .tier = manifest.tier,
                    .confidence = 0.9  // High confidence for explicit rules
                });
            }
        }
    }

    // Sort by confidence descending
    std::sort(targets.begin(), targets.end(),
        [](const CanonicalTarget& a, const CanonicalTarget& b) {
            return a.confidence > b.confidence;
        });

    return targets;
}

nlohmann::json CapabilityCatalog::to_json() const {
    json j;
    if (spine_version) j["spineVersion"] = spine_version.value();
    j["availableObjectKinds"] = available_object_kinds;

    json repos;
    for (const auto& r : available_repos) {
        repos.push_back(json{{"name", r.name}, {"tier", r.tier}, {"path", r.path}});
    }
    j["availableRepos"] = repos;

    json invs;
    for (const auto& i : invariants) {
        json inv;
        inv["name"] = i.name;
        inv["description"] = i.description;
        if (i.min) inv["min"] = i.min.value();
        if (i.max) inv["max"] = i.max.value();
        invs.push_back(inv);
    }
    j["invariants"] = invs;

    json mets;
    for (const auto& m : metrics) {
        json met;
        met["name"] = m.name;
        met["description"] = m.description;
        if (m.target_min) met["targetMin"] = m.target_min.value();
        if (m.target_max) met["targetMax"] = m.target_max.value();
        mets.push_back(met);
    }
    j["metrics"] = mets;

    json phases;
    for (const auto& p : phases) {
        phases.push_back(json{
            {"id", p.id},
            {"name", p.name},
            {"description", p.description}
        });
    }
    j["phases"] = phases;

    return j;
}

}  // namespace hpc::chat_director
