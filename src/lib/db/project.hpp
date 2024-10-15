///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : project
///

#pragma once

#include "../db.hpp"

namespace ns_db::ns_project
{

namespace
{

namespace fs = std::filesystem;

// init_impl() {{{
decltype(auto) init_impl(fs::path const& path_dir_project, ns_enum::Platform const& platform)
{
  // Configure data directory names
  fs::path path_dir_config   = "config";
  fs::path path_dir_data     = "data";
  fs::path path_dir_rom      = "rom";
  fs::path path_dir_core     = "core";
  fs::path path_dir_bios     = "bios";
  fs::path path_dir_keys     = "keys";
  fs::path path_dir_linux    = "linux";

  // Create directories
  ns_fs::ns_path::dir_create<true>(path_dir_project);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_config);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_data);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_rom);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_core);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_bios);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_keys);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_linux);

  // Log created directories
  ns_log::write('i', "path_dir_config       :", path_dir_config);
  ns_log::write('i', "path_dir_data         :", path_dir_data);
  ns_log::write('i', "path_dir_rom          :", path_dir_rom);
  ns_log::write('i', "path_dir_core         :", path_dir_core);
  ns_log::write('i', "path_dir_bios         :", path_dir_bios);
  ns_log::write('i', "path_dir_keys         :", path_dir_keys);
  ns_log::write('i', "path_dir_linux        :", path_dir_linux);


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
    db_project("path_dir_linux")   = path_dir_linux;
  }
  , ns_db::Mode::CREATE);
} // init_impl() }}}

} // namespace

// init() {{{
inline decltype(auto) init(fs::path const& path_dir_project, ns_enum::Platform const& platform)
{
  return ns_exception::to_expected([&]
  { 
    init_impl(path_dir_project, platform);
 });
} // init() }}}

} // namespace ns_db::ns_project

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
