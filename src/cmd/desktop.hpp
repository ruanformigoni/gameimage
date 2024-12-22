///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : desktop
///

#pragma once

#include "../lib/image.hpp"
#include "../lib/db/build.hpp"
#include "../lib/subprocess.hpp"
#include "../std/vector.hpp"

namespace ns_desktop
{

enum class Op
{
  ICON,
  SETUP,
};

namespace
{

namespace fs = std::filesystem;

}

enum class IntegrationItems
{
  MIMETYPE,
  ENTRY,
  ICON
};


// icon() {{{
inline void icon(fs::path const& path_file_icon)
{
  // Open databases
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Could not open build database");
  auto db_metadata = db_build->find(db_build->project);

  // Create icon path
  fs::path path_file_icon_dst = db_build->path_dir_build / "desktop.png";

  // Resize icon to specified icon path
  ns_image::resize(path_file_icon, path_file_icon_dst, 300, 450);
} // icon() }}}

// desktop() {{{
inline void desktop(std::string str_name, std::vector<IntegrationItems> vec_items)
{
  // Open databases
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Could not open build database");
  auto db_metadata = db_build->find(db_build->project);

  // Check if output file exists
  db_build->path_file_output = ns_fs::ns_path::file_exists<true>(db_build->path_file_output)._ret;

  // Path to project
  fs::path path_dir_build = db_build->path_dir_build;
  fs::path path_file_desktop = path_dir_build / "desktop.json";
  fs::path path_file_icon = ns_fs::ns_path::file_exists<true>(path_dir_build / "desktop.png")._ret;

  // Create application data
  std::ignore = ns_db::from_file(path_file_desktop
  , [&](auto&& db)
  {
    db("name") = str_name;
    db("icon") = path_file_icon;
    db("categories") = std::vector<std::string>{"Game"};
  }, ns_db::Mode::CREATE);

  // Apply application data
  std::ignore = ns_subprocess::Subprocess("/fim/static/fim_portal")
    .with_piped_outputs()
    .with_args(db_build->path_file_output, "fim-desktop", "setup", path_file_desktop)
    .spawn()
    .wait();

  // Enable desktop integration
  std::ignore = ns_subprocess::Subprocess("/fim/static/fim_portal")
    .with_piped_outputs()
    .with_args(db_build->path_file_output
      , "fim-desktop"
      , "enable"
      , ns_string::from_container(vec_items , ',', [](auto&& e){ return ns_enum::to_string(e); }))
    .spawn()
    .wait();
} // desktop() }}}

} // namespace ns_test

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
