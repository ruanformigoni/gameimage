#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# export HOME="$SCRIPT_DIR"

conan install . --build=missing -g CMakeDeps -g CMakeToolchain -s build_type=Debug
cmake --preset conan-debug -DCMAKE_BUILD_TYPE=Debug -DCMAKE_EXPORT_COMPILE_COMMANDS=1
cmake --build --preset conan-debug
