// crates/hpc-chat-director/cpp/src/cli/main.cpp
#include "hpc/chat_director/chat_director.hpp"
#include "hpc/chat_director/config.hpp"

#include <iostream>
#include <fstream>
#include <filesystem>
#include <CLI/CLI.hpp>  // https://github.com/CLIUtils/CLI11

namespace fs = std::filesystem;
using json = nlohmann::json;

int main(int argc, char** argv) {
    CLI::App app{"CHAT_DIRECTOR — Schema-driven authoring compiler for Horror$Place"};

    std::string root_hint = ".";
    std::string command;
    std::string request_file;
    std::string response_file;
    std::string output_path;
    bool dry_run = false;
    bool verbose = false;

    app.add_option("--root,-r", root_hint, "Constellation root directory")
       ->default_val(".")
       ->check(CLI::ExistingDirectory);

    auto plan_cmd = app.add_subcommand("plan", "Normalize prompt to authoring request");
    plan_cmd->add_option("prompt", "Natural language intent")->required();
    plan_cmd->add_option("--object-kind,-k", "Hint for objectKind");

    auto validate_cmd = app.add_subcommand("validate", "Validate AI-generated response");
    validate_cmd->add_option("--request,-R", request_file, "Request JSON file")->required();
    validate_cmd->add_option("--response,-r", response_file, "Response JSON file")->required();

    auto apply_cmd = app.add_subcommand("apply", "Write validated artifact to disk");
    apply_cmd->add_option("--request,-R", request_file, "Request JSON file")->required();
    apply_cmd->add_option("--response,-r", response_file, "Response JSON file")->required();
    apply_cmd->add_option("--output,-o", output_path, "Output directory");
    apply_cmd->add_flag("--dry-run,-n", dry_run, "Plan only, do not write files");

    app.add_flag("--verbose,-v", verbose, "Enable verbose output");

    CLI11_PARSE(app, argc, argv);

    try {
        // Load configuration
        auto config = hpc::chat_director::Config::detect(root_hint);
        if (verbose) {
            std::cerr << "Loaded config from: " << config.root() << "\n";
            std::cerr << "Spine version: " << config.spine_version() << "\n";
            std::cerr << "Context mode: " << static_cast<int>(config.context_mode()) << "\n";
        }

        // Load director
        auto director = hpc::chat_director::ChatDirector::load_environment(std::move(config));

        if (plan_cmd->parsed()) {
            std::string prompt;
            plan_cmd->get_option("prompt")->results(prompt);
            std::optional<std::string> kind_hint;
            if (plan_cmd->get_option("--object-kind,-k")->count() > 0) {
                plan_cmd->get_option("--object-kind,-k")->results(kind_hint.emplace());
            }

            json request = director->plan_from_prompt(prompt, kind_hint);
            std::cout << request.dump(2) << "\n";

        } else if (validate_cmd->parsed()) {
            // Load request and response files
            std::ifstream req_file(request_file);
            std::ifstream resp_file(response_file);
            if (!req_file || !resp_file) {
                std::cerr << "Error: Could not open input files\n";
                return 1;
            }

            json request, response;
            req_file >> request;
            resp_file >> response;

            auto result = director->validate_response(request, response);

            if (result.passed()) {
                std::cout << "Validation passed\n";
                return 0;
            } else {
                std::cerr << "Validation failed:\n";
                for (const auto& diag : result.ranked_diagnostics()) {
                    std::cerr << "  [" << static_cast<int>(diag.severity) << "] "
                             << diag.code << ": " << diag.message << "\n";
                    if (diag.json_pointer) {
                        std::cerr << "    at: " << diag.json_pointer.value() << "\n";
                    }
                    if (diag.remediation) {
                        std::cerr << "    fix: " << diag.remediation.value() << "\n";
                    }
                }
                return 1;
            }

        } else if (apply_cmd->parsed()) {
            // Load files
            std::ifstream req_file(request_file);
            std::ifstream resp_file(response_file);
            if (!req_file || !resp_file) {
                std::cerr << "Error: Could not open input files\n";
                return 1;
            }

            json request, response;
            req_file >> request;
            resp_file >> response;

            // Validate first
            auto validation = director->validate_response(request, response);
            if (!validation.passed()) {
                std::cerr << "Cannot apply: validation failed\n";
                return 1;
            }

            // Apply
            auto actions = director->apply(request, response, dry_run);

            if (dry_run) {
                std::cout << "Planned actions:\n" << actions.dump(2) << "\n";
            } else {
                std::cout << "Applied: " << actions["path"].get<std::string>() << "\n";
            }

        } else {
            // Default: show catalog
            auto catalog = director->capability_catalog();
            std::cout << catalog.to_json().dump(2) << "\n";
        }

        return 0;

    } catch (const hpc::chat_director::ChatDirectorError& e) {
        std::cerr << "Error: " << e.what() << "\n";
        return 1;
    } catch (const std::exception& e) {
        std::cerr << "Unexpected error: " << e.what() << "\n";
        return 1;
    }
}
