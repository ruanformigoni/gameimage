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

  // Path to project
  fs::path path_dir_project;

  // Path to boot file
  // Get default path
  ns_db::from_file_default([&](auto&& db)
  {
    // Current application
    str_project = db["project"];

    // Path to flatimage
    path_file_flatimage = ns_fs::ns_path::file_exists<true>(db[str_project]["path_file_image"])._ret;

    // Path to current project
    path_dir_project = static_cast<fs::path>(db[str_project]["path_dir_project"]);
  }
  , ns_db::Mode::READ);

  fs::path path_file_desktop = path_dir_project / "desktop.json";

  // Configure application data
  ns_db::from_file(path_file_desktop
  , [&](auto&& db)
  {
    db("name") = str_project;
    db("icon") = path_file_icon;
    db("categories") = std::vector<std::string>{"Game"};
  }, ns_db::Mode::CREATE);

  // Apply application data
  ns_subprocess::sync("/fim/static/fim_portal"
    , path_file_flatimage
    , "fim-desktop"
    , "setup"
    , path_file_desktop);

  // Enable desktop integration
  ns_subprocess::sync("/fim/static/fim_portal"
    , path_file_flatimage
    , "fim-desktop"
    , "enable"
    , "entry,mimetype,icon");
} // desktop() }}}
 
} // namespace ns_test

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
