# crates/hpc-chat-director/cpp/CMakeLists.txt
cmake_minimum_required(VERSION 3.16)
project(hpc_chat_director_cpp VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Options
option(HPC_CHAT_DIRECTOR_BUILD_CLI "Build the CLI binary" ON)
option(HPC_CHAT_DIRECTOR_BUILD_TESTS "Build test suite" OFF)
option(HPC_CHAT_DIRECTOR_USE_SYSTEM_NLOHMANN "Use system nlohmann/json" OFF)

# Dependencies
if(HPC_CHAT_DIRECTOR_USE_SYSTEM_NLOHMANN)
    find_package(nlohmann_json 3.11.0 REQUIRED)
else()
    # Fetch nlohmann/json via FetchContent
    include(FetchContent)
    FetchContent_Declare(
        nlohmann_json
        URL https://github.com/nlohmann/json/releases/download/v3.11.3/include.zip
        URL_HASH SHA256=3b0e1f2d5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e
    )
    FetchContent_MakeAvailable(nlohmann_json)
endif()

# Library target
add_library(hpc_chat_director STATIC
    src/config.cpp
    src/spine.cpp
    src/chat_director.cpp
    src/validate/schema_validator.cpp
    src/validate/invariant_checker.cpp
    # Add more source files as they are created
)

target_include_directories(hpc_chat_director PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>
)

target_link_libraries(hpc_chat_director PUBLIC
    nlohmann_json::nlohmann_json
)

target_compile_options(hpc_chat_director PRIVATE
    -Wall
    -Wextra
    -Wpedantic
    -Werror
)

# CLI binary
if(HPC_CHAT_DIRECTOR_BUILD_CLI)
    add_executable(hpc-chat-director-cli
        src/cli/main.cpp
    )

    target_link_libraries(hpc-chat-director-cli PRIVATE
        hpc_chat_director
    )

    install(TARGETS hpc-chat-director-cli
        RUNTIME DESTINATION bin
    )
endif()

# Tests
if(HPC_CHAT_DIRECTOR_BUILD_TESTS)
    enable_testing()
    # Add test targets here
endif()

# Install library
install(TARGETS hpc_chat_director
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
    RUNTIME DESTINATION bin
    INCLUDES DESTINATION include
)

install(DIRECTORY include/
    DESTINATION include
    FILES_MATCHING PATTERN "*.hpp"
)
