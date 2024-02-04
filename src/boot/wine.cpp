///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : wine
///

#include <unistd.h>
#include <limits.h>

#include <filesystem>

#include "../common.hpp"

#include "../std/filesystem.hpp"
#include "../std/env.hpp"

#include "../lib/json.hpp"
#include "../lib/subprocess.hpp"

// Start logging
INITIALIZE_EASYLOGGINGPP

namespace fs = std::filesystem;

int main(int argc, char** argv)
{
  // Start log
  ns_log::init(argc, argv);

  // Adjust environment
  ns_env::set("LC_ALL", "C", ns_env::Replace::N);

  // Flatimage distribution
  ns_log::write('i', "FlatImage distribution: ", ns_env::get("FIM_DIST"));

  // Path to self directory
  fs::path path_dir_self = ns_fs::ns_path::dir_exists<true>(fs::path(argv[0]).parent_path())._ret;

  // Set wine prefix
  ns_env::set("WINEPREFIX", (path_dir_self / "wine").c_str(), ns_env::Replace::Y);

  // Database file
  ns_json::Json json(path_dir_self / "gameimage.json");

  // Binary to execute
  fs::path path_file_target = ns_fs::ns_path::file_exists<true>(path_dir_self / json["path-file-target"])._ret;

  // Enter directory of target file
  fs::current_path(ns_fs::ns_path::dir_exists<true>(path_file_target.parent_path())._ret);

  // Get boot command
  std::string str_cmd = ns_env::get("FIM_BINARY_WINE");

  // Start application
  ns_subprocess::sync(str_cmd.c_str(), path_file_target);

  return EXIT_SUCCESS;
} // main

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
