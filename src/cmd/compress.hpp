///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : compress
///

#pragma once

#include "../enum.hpp"

#include "../std/filesystem.hpp"

#include "../lib/db.hpp"
#include "../lib/subprocess.hpp"

namespace ns_compress
{

namespace fs = std::filesystem;

// validate() {{{
inline void validate()
{
  std::string str_project;
  std::string str_platform;
  fs::path path_project;

  ns_db::from_file_default([&](auto&& db)
  {
    str_project = db["project"];
    str_platform = db[str_project]["platform"];
    path_project = std::string(db[str_project]["path_dir_project"]);
  }
  , ns_db::Mode::READ);

  auto enum_platform = ns_enum::from_string<ns_enum::Platform>(str_platform);

  // Icon
  "Icon is not installed"_try([&]
  {
    fs::path path_file_icon;
    ns_db::from_file_project([&](auto&& db){ path_file_icon = path_project / db["path_file_icon"]; }, ns_db::Mode::READ);
    ns_fs::ns_path::file_exists<true>(path_file_icon);
    ns_log::write('i', "Found icon '", path_file_icon, "'");
  });


  auto f_file_default = [&](auto&& type)
  {
    "Default {} is not selected"_try([&]
    {
      std::string str_tag_db = "path_file_{}"_fmt(type);
      fs::path path_file;
      ns_db::from_file_project([&](auto&& db){ path_file = path_project / db[str_tag_db]; }, ns_db::Mode::READ);
      try
      {
        ns_fs::ns_path::file_exists<true>(path_file);
      } // try
      catch(std::exception const& e)
      {
        ns_fs::ns_path::dir_exists<true>(path_project / path_file);
      } // catch
      ns_log::write('i', "Found ", type, " '", path_file, "'");
    }, type);
  };

  auto f_files_validate = [&](auto&& type)
  {
    std::vector<fs::path> paths_file;

    ns_db::from_file_project([&](auto&& db)
    {
      for(auto&& path_file : db["paths_file_{}"_fmt(type)])
      {
        paths_file.push_back(path_file);
      } // for
    }, ns_db::Mode::READ);
    
    for(auto&& path_file : paths_file)
    {
      "Missing file {} in json for '{}'"_try([&]
      {
        try
        {
          ns_fs::ns_path::file_exists<true>(path_project / path_file);
        } // try
        catch(std::exception const& e)
        {
          ns_fs::ns_path::dir_exists<true>(path_project / path_file);
        } // catch
      }, type, path_file);
    } // for
  };

  switch(enum_platform)
  {
    case ns_enum::Platform::LINUX:
    case ns_enum::Platform::WINE:
    {
      f_file_default("rom");
    }
    break;
    case ns_enum::Platform::RETROARCH:
    {
      // default rom
      f_file_default("rom");
      // default core
      f_file_default("core");
      // all roms
      f_files_validate("rom");
      // all cores
      f_files_validate("core");
    } // case
    break;
    case ns_enum::Platform::PCSX2:
    {
      // default rom
      f_file_default("rom");
      // default bios
      f_file_default("bios");
      // all roms
      f_files_validate("rom");
      // all bios
      f_files_validate("bios");
    } // case
    break;
    case ns_enum::Platform::RPCS3:
    {
      // default rom
      f_file_default("rom");
    } // case
    break;
    case ns_enum::Platform::RYUJINX:
    {
      // default bios is managed by ryujinx
      // default rom
      f_file_default("rom");
      // default key
      f_file_default("keys");
      // all roms
      f_files_validate("rom");
      // all keys
      f_files_validate("keys");
    } // case
    break;
  } // switch

} // validate() }}}
  
// compress() {{{
inline decltype(auto) compress()
{
  // Validate package by platform
  validate();

  // Current project
  std::string str_project;

  // Path to current project
  std::string str_path_project;

  // Path to image
  std::string str_image;

  ns_db::from_file_default([&](auto&& db)
  {
    // Current project
    str_project = db["project"];

    // Path to current application
    str_path_project = ns_fs::ns_path::dir_exists<true>(db[str_project]["path_dir_project"])._ret;

    // Path to image
    str_image = ns_fs::ns_path::file_exists<true>(db[str_project]["path_file_image"])._ret;
  }
  , ns_db::Mode::READ);

  // Output file
  std::string str_target = str_path_project + ".dwarfs";

  // Log
  ns_log::write('i', "project: ", str_project);
  ns_log::write('i', "image: ", str_image);
  ns_log::write('i', "dir: ", str_path_project);
  
  // Compress
  ns_subprocess::sync("/fim/static/fim_portal"
    , str_image
    , "fim-layer"
    , "create"
    , str_path_project
    , str_target
  );

  ns_log::write('i', "Wrote file to '", str_target, "'");
} // compress() }}}

} // namespace ns_compress

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
