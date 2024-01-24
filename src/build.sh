#!/bin/bash

conan install . --build=missing -g CMakeDeps -g CMakeToolchain
cmake --preset conan-release -DCMAKE_BUILD_TYPE=Release -DCMAKE_EXPORT_COMPILE_COMMANDS=1
cmake --build --preset conan-release
