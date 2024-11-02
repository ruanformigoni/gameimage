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
inline void desktop(std::string str_name, fs::path path_file_icon, std::string str_items)
{
  // Open databases
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Could not open build database");
  auto db_metadata = db_build->find(db_build->project);

  // Check if output file exists
  db_build->path_file_output = ns_fs::ns_path::file_exists<true>(db_build->path_file_output)._ret;

  // Validate icon path
  path_file_icon = ns_fs::ns_path::file_exists<true>(path_file_icon)._ret;

  // Validate items
  auto vec_items = ns_vector::from_string<std::vector<IntegrationItems>>(str_items
    , ','
    , [](auto&& e){ return ns_enum::from_string<IntegrationItems>(e); }
  );
  throw_if(vec_items.empty(), "No integration items available");

  // Path to project
  fs::path path_dir_build = db_build->path_dir_build;
  fs::path path_file_desktop = path_dir_build / "desktop.json";
  fs::path path_file_icon_resized = path_dir_build / "desktop.png";

  // Resize icon
  ns_image::resize(path_file_icon, path_file_icon_resized, 300, 450);

  // Create application data
  ns_db::from_file(path_file_desktop
  , [&](auto&& db)
  {
    db("name") = str_name;
    db("icon") = path_file_icon_resized;
    db("categories") = std::vector<std::string>{"Game"};
  }, ns_db::Mode::CREATE);

  // Apply application data
  (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
    .with_piped_outputs()
    .with_args(db_build->path_file_output, "fim-desktop", "setup", path_file_desktop)
    .spawn()
    .wait();

  // Enable desktop integration
  (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
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
