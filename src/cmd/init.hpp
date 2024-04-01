///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : init
// @created     : Friday Jan 19, 2024 19:13:00 -03
///

#pragma once

#include <filesystem>

#include "../common.hpp"
#include "../enum.hpp"

#include "../std/filesystem.hpp"
#include "../std/copy.hpp"
#include "../std/env.hpp"

#include "../lib/log.hpp"
#include "../lib/db.hpp"

//
// Initializes a new directory configuration for gameimage
//
namespace ns_init
{

namespace fs = std::filesystem;

// init() {{{
inline void init(std::string const& str_platform
  , std::string const& str_path_project
  , std::string const& str_path_image)
{
  // Set paths
  ns_enum::Platform platform = ns_enum::from_string<ns_enum::Platform>(str_platform);
  fs::path path_image        = ns_fs::ns_path::file_exists<true>(str_path_image)._ret;
  fs::path path_dir_project  = ns_fs::ns_path::dir_create<true>(str_path_project)._ret;
  fs::path path_dir_build    = path_dir_project.parent_path();
  fs::path path_dir_config   = "config";
  fs::path path_dir_data     = "data";
  fs::path path_dir_rom      = "rom";
  fs::path path_dir_core     = "core";
  fs::path path_dir_bios     = "bios";
  fs::path path_dir_keys     = "keys";

  // Adjust by platform
  if ( platform == ns_enum::Platform::RYUJINX )
  {
    path_dir_bios = path_dir_config / "Ryujinx/nand/system/Contents/registered";
    path_dir_keys = path_dir_config / "Ryujinx/system";
  } // if

  // Log
  ns_log::write('i', "platform         : ", str_platform);
  ns_log::write('i', "image            : ", path_image);
  ns_log::write('i', "path_dir_project : ", path_dir_project);
  ns_log::write('i', "path_dir_build   : ", path_dir_build);
  ns_log::write('i', "path_dir_config  : ", path_dir_config);
  ns_log::write('i', "path_dir_data    : ", path_dir_data);
  ns_log::write('i', "path_dir_rom     : ", path_dir_rom);
  ns_log::write('i', "path_dir_core    : ", path_dir_core);
  ns_log::write('i', "path_dir_bios    : ", path_dir_bios);
  ns_log::write('i', "path_dir_keys    : ", path_dir_keys);

  // Check if data file exists
  ns_fs::ns_path::file_exists<true>(path_image);

  // Check if project directory parent exists
  ns_fs::ns_path::dir_exists<true>(path_dir_project.parent_path());

  // Create directories
  ns_fs::ns_path::dir_create<true>(path_dir_project);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_config);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_data);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_rom);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_core);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_bios);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_keys);

  // Set global data
  ns_db::from_file_default([&](auto&& db_global)
  {
    // project name is Dir name
    std::string str_name = path_dir_project.filename();

    // build dir
    db_global("path_dir_build") = path_dir_build;

    // Set as default project
    db_global("project") = str_name;

    // Append to project list
    db_global("projects") |= str_name;

    // Set data
    db_global(str_name)("path_file_image")   = path_image;
    db_global(str_name)("path_dir_project") = path_dir_project;
    db_global(str_name)("platform")     = ns_enum::to_string(platform);
  }
  , fs::exists(ns_db::file_default())? ns_db::Mode::UPDATE : ns_db::Mode::CREATE);

  // Copy boot file for platform
  fs::path path_file_boot = ns_fs::ns_path::file_exists<true>(
    ns_env::dir("GIMG_SCRIPT_DIR") / "gameimage-boot"
  )._ret;
  fs::copy_file(path_file_boot, path_dir_project / "boot", fs::copy_options::overwrite_existing);
  ns_log::write('i', "Copy ", path_file_boot, " -> ", path_dir_project / "boot");

  // Set project data
  ns_db::from_file_project([&](auto&& db_project)
  {
    db_project("project")          = path_dir_project.filename();
    db_project("platform")         = ns_enum::to_string(platform);
    db_project("path_dir_config")  = path_dir_config;
    db_project("path_dir_data")    = path_dir_data;
    db_project("path_dir_bios")    = path_dir_bios;
    db_project("path_dir_rom")     = path_dir_rom;
    db_project("path_dir_core")    = path_dir_core;
    db_project("path_dir_keys")    = path_dir_keys;
  }
  , ns_db::Mode::CREATE);
} // function: init }}}

} // namespace ns_init

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
