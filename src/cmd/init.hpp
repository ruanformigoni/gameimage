///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : init
// @created     : Friday Jan 19, 2024 19:13:00 -03
///

#pragma once

#include <filesystem>

#include "../common.hpp"
#include "../enum.hpp"

#include "../std/filesystem.hpp"
#include "../std/copy.hpp"
#include "../std/env.hpp"

#include "../lib/log.hpp"
#include "../lib/db.hpp"

//
// Initializes a new directory configuration for gameimage
//
namespace ns_init
{

namespace fs = std::filesystem;

// init() {{{
inline void init(std::string const& str_platform
  , std::string const& str_path_app
  , std::string const& str_path_image)
{
  // Validate
  ns_enum::Platform platform = ns_enum::from_string<ns_enum::Platform>(str_platform);
  fs::path path_image        = ns_fs::ns_path::file_exists<true>(str_path_image)._ret;
  fs::path path_app          = ns_fs::ns_path::dir_create<true>(str_path_app)._ret;

  // Log
  ns_log::write('i', "platform: ", str_platform);
  ns_log::write('i', "image: ", path_image);
  ns_log::write('i', "dir: ", path_app);

  // Check if data file exists
  ns_fs::ns_path::file_exists<true>(path_image);

  // Check if app directory parent exists
  ns_fs::ns_path::dir_exists<true>(path_app.parent_path());

  // Create directory
  ns_fs::ns_path::dir_create<true>(path_app);

  // Update global
  ns_db::from_file_default([&](auto&& db_global)
  {
    // App name is Dir name
    std::string str_name = path_app.filename();

    // Set as default project
    db_global("project") = path_app.filename();

    // Set data
    db_global(str_name)("path-image")   = path_image;
    db_global(str_name)("path-project") = path_app;
    db_global(str_name)("platform")     = ns_enum::to_string(platform);
  });

  // Copy boot file for platform
  fs::path path_file_boot = ns_fs::ns_path::file_exists<true>(
    ns_env::dir("GIMG_SCRIPT_DIR") / "boot"
  )._ret;
  fs::copy_file(path_file_boot, path_app / "boot", fs::copy_options::overwrite_existing);
  ns_log::write('i', "Copy ", path_file_boot, " -> ", path_app / "boot");

  // Update project
  ns_db::from_file_project([&](auto&& db_project)
  {
    db_project("platform") = ns_enum::to_string(platform);
  });
} // function: init }}}

} // namespace ns_init

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
