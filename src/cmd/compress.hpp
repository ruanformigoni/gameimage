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
  fs::path path_app;

  ns_db::from_file_default([&](auto&& db)
  {
    str_project = db["project"];
    str_platform = db[str_project]["platform"];
    path_app = std::string(db[str_project]["path-project"]);
  });

  auto enum_platform = ns_enum::from_string<ns_enum::Platform>(str_platform);

  // Icon
  "Icon is not installed"_try([&]
  {
    fs::path path_file_icon;
    ns_db::from_file_project([&](auto&& db){ path_file_icon = path_app / db["path-file-icon"]; });
    ns_fs::ns_path::file_exists<true>(path_file_icon);
    ns_log::write('i', "Found icon '", path_file_icon, "'");
  });

  switch(enum_platform)
  {
    case ns_enum::Platform::WINE:
    {
      // Rom
      "Default executable is not selected"_try([&]
      {
        fs::path path_file_rom;
        ns_db::from_file_project([&](auto&& db){ path_file_rom = path_app / db["path-file-rom"]; });
        ns_fs::ns_path::file_exists<true>(path_file_rom);
        ns_log::write('i', "Found rom '", path_file_rom, "'");
      });
    }
    break;
    case ns_enum::Platform::RETROARCH:
      // default rom
      "Default rom is not selected"_try([&]
      {
        fs::path path_file_rom;
        ns_db::from_file_project([&](auto&& db){ path_file_rom = path_app / db["path-file-rom"]; });
        ns_fs::ns_path::file_exists<true>(path_file_rom);
        ns_log::write('i', "Found rom '", path_file_rom, "'");
      });
      // default core
      "Default core is not selected"_try([&]
      {
        fs::path path_file_core;
        ns_db::from_file_project([&](auto&& db){ path_file_core = path_app / db["path-file-core"]; });
        ns_log::write('i', "Found core '", path_file_core, "'");
      });
      // all roms
      "Failed to validate rom paths"_try([&]
      {
        ns_db::from_file_project([&](auto&& db)
        {
          for(auto&& path_file : db["paths-file-rom"])
          {
            "Invalid rom path in json for '{}'"_try([&]
            {
              ns_fs::ns_path::file_exists<true>(path_app / path_file);
            }, path_file);
          }
        });
      });
      // all cores
      "Failed to validate core paths"_try([&]
      {
        ns_db::from_file_project([&](auto&& db)
        {
          for(auto&& path_file : db["paths-file-core"])
          {
            "Invalid core path in json for '{}'"_try([&]
            {
              ns_fs::ns_path::file_exists<true>(path_app / path_file);
            }, path_file);
          }
        });
      });
    break;
    case ns_enum::Platform::PCSX2:
      "Not implemented"_throw();
    break;
    case ns_enum::Platform::RPCS3:
      "Not implemented"_throw();
    break;
    case ns_enum::Platform::YUZU:
      "Not implemented"_throw();
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

  // Path to current application
  std::string str_app;

  // Path to image
  std::string str_image;

  ns_db::from_file_default([&](auto&& db)
  {
    // Current project
    str_project = db["project"];

    // Path to current application
    str_app = ns_fs::ns_path::dir_exists<true>(db[str_project]["path-project"])._ret;

    // Path to image
    str_image = ns_fs::ns_path::file_exists<true>(db[str_project]["path-image"])._ret;
  });

  // Output file
  std::string str_target = str_app + ".dwarfs";

  // Log
  ns_log::write('i', "project: ", str_project);
  ns_log::write('i', "image: ", str_image);
  ns_log::write('i', "dir: ", str_app);
  
  // Compress
  ns_subprocess::sync(str_image
    , "fim-exec"
    , "mkdwarfs"
    , "-f"
    , "-i"
    , str_app
    , "-o"
    , str_target
  );

  ns_log::write('i', "Wrote file to '", str_target, "'");
} // compress() }}}

} // namespace ns_compress

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
