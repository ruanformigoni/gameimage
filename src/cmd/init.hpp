///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : init
// @created     : Friday Jan 19, 2024 19:13:00 -03
///

#pragma once

#include <filesystem>

#include "project.hpp"
#include "../macro.hpp"
#include "../enum.hpp"

#include "../std/filesystem.hpp"
#include "../std/env.hpp"

#include "../lib/log.hpp"
#include "../lib/subprocess.hpp"
#include "../lib/db/build.hpp"
#include "../lib/db/project.hpp"

//
// Initializes a new directory configuration for gameimage
//
namespace ns_init
{

namespace fs = std::filesystem;

// build() {{{
inline void build(fs::path path_dir_build)
{
  // Log
  ns_log::write('i', "path_dir_build: ", path_dir_build);
  // Initialize projects data
  elogerror(ns_db::ns_build::init(path_dir_build));
} // function: build }}}

// project() {{{
inline void project(std::string const& str_name, ns_enum::Platform const& platform)
{
  // Read build database
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Could not read build database");
  // Create project dir and return an absolute path to it
  fs::path path_dir_project_root  = ns_fs::ns_path::dir_create<true>(db_build->path_dir_build / str_name)._ret;
  // The actual project files are nested in /opt/gameimage-games, because that's the final path
  // inside the container
  fs::path path_dir_project = ns_fs::ns_path::dir_create<true>(db_build->path_dir_build / str_name / "opt" / "gameimage-games" / str_name)._ret;
  // Log
  ns_log::write('i', "platform              :", ns_enum::to_string_lower(platform));
  ns_log::write('i', "image                 :", db_build->path_file_image);
  ns_log::write('i', "path_dir_project_root :", path_dir_project_root);
  ns_log::write('i', "path_dir_project      :", path_dir_project);
  // Create novel metadata for project
  db_build->projects.push_back(ns_db::ns_build::Metadata
  {
      .name = str_name
    , .path_dir_project = path_dir_project
    , .path_dir_project_root = path_dir_project_root
    , .platform = platform
  });
  // Write changes to database
  ns_db::ns_build::write(*db_build);
  // Set project as default
  ns_project::set(str_name);
  // Copy boot file for platform
  auto path_file_boot = ns_subprocess::search_path("gameimage-boot");
  ethrow_if(not path_file_boot, "Could not find gameimage-boot in PATH");
  lec(fs::copy_file
    , *path_file_boot
    , path_dir_project / "boot"
    , fs::copy_options::overwrite_existing
  );
  ns_log::write('i', "Copy ", *path_file_boot, " -> ", path_dir_project / "boot");
  // Create project database
  elogerror(ns_db::ns_project::init(path_dir_project, platform));
} // function: project }}}

} // namespace ns_init

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
