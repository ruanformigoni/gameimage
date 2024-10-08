///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : desktop
///

#pragma once


#include "../lib/db.hpp"
#include "../lib/subprocess.hpp"
#include "../std/vector.hpp"

namespace ns_desktop
{

namespace
{

namespace fs = std::filesystem;

enum class IntegrationItems
{
  MIMETYPE,
  ENTRY,
  ICON
};

}

// desktop() {{{
inline decltype(auto) desktop(std::string str_name, fs::path path_file_icon, std::string str_items)
{
  // Validate icon path
  path_file_icon = ns_fs::ns_path::file_exists<true>(path_file_icon)._ret;

  // Validate items
  auto vec_items = ns_vector::from_string<std::vector<IntegrationItems>>(str_items
    , ','
    , [](auto&& e){ return ns_enum::from_string<IntegrationItems>(e); }
  );
  throw_if(vec_items.empty(), "No integration items available");

  // Path to flatimage
  fs::path path_file_flatimage;

  // Path to project
  fs::path path_dir_project;

  // Path to boot file
  // Get default path
  ns_db::from_file_default([&](auto&& db)
  {
    // Current application
  std::string str_project = db["project"];

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
    db("name") = str_name;
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
    , ns_string::from_container(vec_items , ',', [](auto&& e){ return ns_enum::to_string(e); })
  );
} // desktop() }}}

} // namespace ns_test

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
