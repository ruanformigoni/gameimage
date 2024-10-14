///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : package
///

#pragma once

#include <filesystem>

#include "../lib/subprocess.hpp"
#include "../lib/db.hpp"
#include "project.hpp"

namespace ns_package
{

namespace fs = std::filesystem;

// package() {{{
inline void package(std::string const& str_name_project)
{
  fs::path path_file_image;
  fs::path path_dir_project_root;
  fs::path path_dir_build;

  // Set project to package
  ns_project::set(str_name_project);

  // Get current project
  ns_db::from_file_default([&](auto&& db)
  {
    path_file_image = fs::path(db[str_name_project]["path_file_image"]);
    path_dir_project_root = fs::path(db[str_name_project]["path_dir_project_root"]);
    path_dir_build = fs::path(db["path_dir_build"]);
  }
  , ns_db::Mode::READ);

  // Verify that image exists
  ns_fs::ns_path::file_exists<true>(path_file_image);

  // Verify that directory exists
  ns_fs::ns_path::dir_exists<true>(path_dir_build);

  // Include dwarfs file in image
  fs::path path_file_layer = ns_fs::ns_path::file_exists<true>(path_dir_project_root.string() + ".dwarfs")._ret;
  (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
    .with_piped_outputs()
    .with_args(path_file_image, "fim-layer", "add", path_file_layer)
    .spawn()
    .wait();

  // Copy launcher to outside wizard image
  fs::path path_file_launcher = path_dir_build / "gameimage-launcher";
  fs::copy_file(ns_fs::ns_path::dir_self<true>()._ret / "gameimage-launcher"
    , path_file_launcher
    , fs::copy_options::overwrite_existing
  );

  // Include launcher inside game image
  (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
    .with_piped_outputs()
    .with_args(path_file_image
      , "fim-exec"
      , "cp"
      , path_file_launcher
      , "/fim/static/gameimage-launcher")
    .spawn()
    .wait();

  // Set boot command
  (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
    .with_piped_outputs()
    .with_args(path_file_image
      , "fim-boot"
      , "/bin/bash"
      , "-c"
      , "/fim/static/gameimage-launcher")
    .spawn()
    .wait();

  // Enable notify-send
  (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
    .with_piped_outputs()
    .with_args(path_file_image, "fim-notify", "on")
    .spawn()
    .wait();

  // Commit changes into the image
  (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
    .with_piped_outputs()
    .with_args(path_file_image , "fim-commit")
    .spawn()
    .wait();

} // package() }}}

} // namespace ns_package

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
