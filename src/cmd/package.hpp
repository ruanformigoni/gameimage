///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : package
///

#pragma once

#include <filesystem>

#include "../lib/subprocess.hpp"
#include "../lib/db.hpp"

namespace ns_package
{

namespace fs = std::filesystem;

// package() {{{
inline void package(fs::path path_file_dwarfs)
{
  std::string str_project;
  fs::path path_image;

  // Get current project
  ns_db::from_file_default([&](auto&& db)
  {
    str_project = db["project"];
    path_image = fs::path(db[str_project]["path_file_image"]);
  }
  , ns_db::Mode::READ);

  // Verify that image exists
  ns_fs::ns_path::file_exists<true>(path_image);

  // Verify that dwarfs exists
  ns_fs::ns_path::file_exists<true>(path_file_dwarfs);

  // Get stem of dwarfs file
  std::string str_stem_dwarfs = path_file_dwarfs.stem();

  // Get path to the dwarfs mount point
  fs::path path_dir_mount = fs::path{"/fim/mount/"} / str_stem_dwarfs;

  // Include in image
  ns_subprocess::sync(path_image
    , "fim-dwarfs-add"
    , path_file_dwarfs
    , path_dir_mount);

  // Get path to launcher
  fs::path path_file_launcher = ns_fs::ns_path::dir_self<true>()._ret / "gameimage-launcher";

  // Include inside image
  ns_subprocess::sync(path_image
    , "fim-exec"
    , "cp"
    , path_file_launcher
    , "/fim/static/gameimage-launcher");

  // Set boot command
  ns_subprocess::sync(path_image, "fim-cmd", "gameimage-launcher");
  

} // package() }}}

} // namespace ns_package

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
