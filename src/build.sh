#!/bin/bash

conan install . --build=missing -g CMakeDeps -g CMakeToolchain -s build_type=Debug
cmake --preset conan-debug -DCMAKE_BUILD_TYPE=Debug -DCMAKE_EXPORT_COMPILE_COMMANDS=1
cmake --build --preset conan-debug
