######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : CMakeLists
######################################################################

cmake_minimum_required(VERSION 3.5)

project(GameImage DESCRIPTION "GameImage - FlatImage Game Packer" LANGUAGES CXX)

# Compiler
message(STATUS "Compiler: ${CMAKE_CXX_COMPILER}")
add_definitions("--std=c++23 -Wall -Wextra")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -static")
message(STATUS "CMAKE_C_FLAGS: ${CMAKE_C_FLAGS}")
message(STATUS "CMAKE_CXX_FLAGS: ${CMAKE_CXX_FLAGS}")
message(STATUS "CMAKE_BUILD_TYPE: ${CMAKE_BUILD_TYPE}")

# Tools
## Git
find_program(GIT_COMMAND git)
if(NOT GIT_COMMAND)
  message(FATAL_ERROR "git command not found")
endif()

# External libraries
find_package(fmt REQUIRED)
find_package(cpr REQUIRED)
find_package(argparse REQUIRED)
find_package(nlohmann_json REQUIRED)
find_package(matchit REQUIRED)
find_package(magic_enum REQUIRED)
find_package(Boost REQUIRED)
find_package(easyloggingpp REQUIRED)
find_package(libjpeg-turbo REQUIRED)
find_package(PNG REQUIRED)
find_package(ZLIB REQUIRED)
find_package(cppcoro REQUIRED)
find_package(libzippp REQUIRED)
find_package(cryptopp REQUIRED)
find_package(LibArchive REQUIRED)
find_package(Boost COMPONENTS filesystem REQUIRED)
INCLUDE_DIRECTORIES(${Boost_INCLUDE_DIR})

# Main executable
add_executable(main main.cpp)
target_link_libraries(main PRIVATE
  fmt::fmt 
  argparse::argparse 
  cpr::cpr 
  nlohmann_json::nlohmann_json
  matchit::matchit
  magic_enum::magic_enum
  easyloggingpp::easyloggingpp
  libjpeg-turbo::libjpeg-turbo
  PNG::PNG
  /usr/lib/libboost_filesystem.a
  ZLIB::ZLIB
  cppcoro::cppcoro
  libzippp::libzippp
  cryptopp::cryptopp
  LibArchive::LibArchive
)
target_include_directories(main PRIVATE ${fmt_INCLUDE_DIRS})

# Boot executables for each platform
add_executable(boot boot/boot.cpp)
target_link_libraries(boot PRIVATE
  fmt::fmt 
  argparse::argparse 
  cpr::cpr 
  nlohmann_json::nlohmann_json
  matchit::matchit
  magic_enum::magic_enum
  easyloggingpp::easyloggingpp
  libjpeg-turbo::libjpeg-turbo
  PNG::PNG
  /usr/lib/libboost_filesystem.a
  ZLIB::ZLIB
  cppcoro::cppcoro
)
target_include_directories(boot PRIVATE ${fmt_INCLUDE_DIRS})
