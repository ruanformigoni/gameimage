///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : package
///

#pragma once

#include <filesystem>

#include "../lib/subprocess.hpp"
#include "../lib/db/build.hpp"
#include "project.hpp"

namespace ns_package
{

namespace fs = std::filesystem;

// package() {{{
inline void package(std::string const& str_name_project)
{
  // Set project to package
  ns_project::set(str_name_project);

  // Open databases
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Could not open build database");
  auto db_metadata = db_build->find(db_build->project);

  // Verify that image exists
  ns_fs::ns_path::file_exists<true>(db_metadata.path_file_image);

  // Verify that directory exists
  ns_fs::ns_path::dir_exists<true>(db_build->path_dir_build);

  // Execute portal
  auto f_portal = []<typename... Args>(Args&&... args)
  {
    (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
      .with_piped_outputs()
      .with_args(std::forward<Args>(args)...)
      .spawn()
      .wait();
  };

  // Include layer file in image
  fs::path path_file_layer = ns_fs::ns_path::file_exists<true>(db_metadata.path_dir_project_root.string() + ".layer")._ret;
  f_portal(db_metadata.path_file_image, "fim-layer", "add", path_file_layer);

  // Copy launcher to outside wizard image
  fs::path path_file_launcher = db_build->path_dir_build / "gameimage-launcher";
  fs::copy_file(ns_fs::ns_path::dir_self<true>()._ret / "gameimage-launcher"
    , path_file_launcher
    , fs::copy_options::overwrite_existing
  );

  // Include launcher inside game image
  f_portal(db_metadata.path_file_image, "fim-exec", "cp", path_file_launcher, "/fim/static/gameimage-launcher");

  // Set boot command
  f_portal(db_metadata.path_file_image, "fim-boot", "/bin/bash", "-c", "/fim/static/gameimage-launcher");

  // Enable notify-send
  f_portal(db_metadata.path_file_image, "fim-notify", "on");

  // Commit changes into the image
  f_portal(db_metadata.path_file_image , "fim-commit");

} // package() }}}

} // namespace ns_package

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
