///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : desktop
///

#pragma once


#include "../lib/db.hpp"
#include "../lib/subprocess.hpp"

namespace ns_desktop
{

namespace fs = std::filesystem;

// desktop() {{{
inline decltype(auto) desktop(fs::path path_file_icon)
{
  // Validate icon path
  path_file_icon = ns_fs::ns_path::file_exists<true>(path_file_icon)._ret;

  // Current application
  std::string str_project;

  // Path to flatimage
  fs::path path_file_flatimage;

  // Path to boot file
  // Get default path
  ns_db::from_file_default([&](auto&& db)
  {
    // Current application
    str_project = db["project"];

    // Path to flatimage
    path_file_flatimage = ns_fs::ns_path::file_exists<true>(db[str_project]["path_file_image"])._ret;
  }
  , ns_db::Mode::READ);

  // Configure application name
  ns_subprocess::sync(path_file_flatimage
    , "fim-config-set"
    , "name"
    , str_project);

  // Copy icon to inside the image
  ns_subprocess::sync(path_file_flatimage
    , "fim-exec"
    , "cp"
    , path_file_icon.string()
    , "/fim/desktop/icon.png");

  // Configure icon
  ns_subprocess::sync(path_file_flatimage
    , "fim-config-set"
    , "icon"
    , "'\"\\$FIM_DIR_MOUNT\"/fim/desktop/icon.png'");

  // Set categories
  ns_subprocess::sync(path_file_flatimage
    , "fim-config-set"
    , "categories"
    , "Game");

  // Enable desktop integration
  ns_subprocess::sync(path_file_flatimage
    , "fim-config-set"
    , "desktop"
    , "1");

} // desktop() }}}
 
} // namespace ns_test

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
