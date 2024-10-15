///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : init
// @created     : Friday Jan 19, 2024 19:13:00 -03
///

#pragma once

#include <filesystem>

#include "../common.hpp"
#include "../macro.hpp"
#include "../enum.hpp"

#include "../std/filesystem.hpp"
#include "../std/copy.hpp"
#include "../std/env.hpp"

#include "../lib/log.hpp"
#include "../lib/db/global.hpp"
#include "../lib/db/project.hpp"

//
// Initializes a new directory configuration for gameimage
//
namespace ns_init
{

namespace fs = std::filesystem;

// init() {{{
inline void init(std::string const& str_platform
  , fs::path path_dir_project
  , fs::path path_file_image)
{
  // Set platform
  ns_enum::Platform platform = ns_enum::from_string<ns_enum::Platform>(str_platform);
  // Set path to runner flatimage
  path_file_image = ns_fs::ns_path::file_exists<true>(path_file_image)._ret;
  // Create project dir and return an absolute path to it
  fs::path path_dir_project_root  = ns_fs::ns_path::dir_create<true>(path_dir_project)._ret;
  // Verify if build dir exists and return an absolute path for it
  fs::path path_dir_build = ns_fs::ns_path::dir_exists<true>(path_dir_project_root.parent_path())._ret;
  // The actual project files are nested in /opt/gameimage-games, because that's where they'll be in
  // the final flatimage
  path_dir_project = ns_fs::ns_path::dir_create<true>(path_dir_project / "opt" / "gameimage-games" / path_dir_project.filename())._ret;
  // Log
  ns_log::write('i', "platform              :", str_platform);
  ns_log::write('i', "image                 :", path_file_image);
  ns_log::write('i', "path_dir_project_root :", path_dir_project_root);
  ns_log::write('i', "path_dir_project      :", path_dir_project);
  ns_log::write('i', "path_dir_build        :", path_dir_build);
  // Check if data file exists
  ns_fs::ns_path::file_exists<true>(path_file_image);
  // Check if project directory parent exists
  ns_fs::ns_path::dir_exists<true>(path_dir_project.parent_path());
  // Initialize projects data
  elogerror(ns_db::ns_global::init(path_dir_build
    , path_dir_project
    , path_dir_project_root
    , path_file_image
    , platform
  ));
  // Copy boot file for platform
  fs::path path_file_boot = ns_fs::ns_path::file_exists<true>(
    ns_env::dir("GIMG_SCRIPT_DIR") / "gameimage-boot"
  )._ret;
  fs::copy_file(path_file_boot, path_dir_project / "boot", fs::copy_options::overwrite_existing);
  ns_log::write('i', "Copy ", path_file_boot, " -> ", path_dir_project / "boot");
  // Create project database
  elogerror(ns_db::ns_project::init(path_dir_project, platform));
} // function: init }}}

} // namespace ns_init

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
