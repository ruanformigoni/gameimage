///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : compress
///

#pragma once

#include "../enum.hpp"

#include "../std/filesystem.hpp"

#include "../lib/json.hpp"
#include "../lib/subprocess.hpp"

namespace ns_compress
{

namespace fs = std::filesystem;

// validate() {{{
inline void validate()
{
  auto json_global = ns_json::from_file_default();
  std::string str_project = json_global["project"];
  std::string str_platform = json_global[str_project]["platform"];
  fs::path path_app = json_global[str_project]["path-app"];
  auto enum_platform = ns_enum::from_string<ns_enum::Platform>(str_platform);

  ns_json::Json json_project;
  "Project is not configured"_try([&]{ json_project = ns_json::from_file_project(); });

  // Icon
  "Icon is not installed"_try([&]
  {
    fs::path path_file_icon = path_app / json_project["path-file-icon"];
    ns_fs::ns_path::file_exists<true>(path_file_icon);
    ns_log::write('i', "Found icon '", path_file_icon, "'");
  });

  switch(enum_platform)
  {
    case ns_enum::Platform::WINE:
    {
      // Target
      "Default executable is not selected"_try([&]
      {
        fs::path path_file_target = path_app / json_project["path-file-target"];
        ns_fs::ns_path::file_exists<true>(path_file_target);
        ns_log::write('i', "Found target '", path_file_target, "'");
      });
    }
    break;
    case ns_enum::Platform::RETROARCH:
      // Target default rom
      "Default rom is not selected"_try([&]
      {
        fs::path path_file_target = path_app / json_project["path-file-target"];
        ns_fs::ns_path::file_exists<true>(path_file_target);
        ns_log::write('i', "Found target '", path_file_target, "'");
      });
      // Target default core
      "Default core is not selected"_try([&]
      {
        fs::path path_file_target = path_app / json_project["path-file-core"];
        ns_fs::ns_path::file_exists<true>(path_file_target);
        ns_log::write('i', "Found target '", path_file_target, "'");
      });
      // Rom
      "Failed to validate rom paths"_try([&]
      {
        "Invalid rom path in json for '{}'"_for(json_project["paths-file-target"]
          , [&](fs::path path_file)
          {
            return ns_fs::ns_path::file_exists<false>(path_app / path_file)._bool;
          }
        );
      });
      // Core
      "Failed to validate core paths"_try([&]
      {
        "Invalid core path in json for '{}'"_for(json_project["paths-file-core"]
          , [&](fs::path path_file)
          {
            return ns_fs::ns_path::file_exists<false>(path_app / path_file)._bool;
          }
        );
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
  ns_json::Json json = ns_json::from_file_default();

  // Validate package by platform
  validate();

  // Current project
  std::string str_project = json["project"];

  ns_log::write('i', "project: ", str_project);

  // Path to current application
  std::string str_app = ns_fs::ns_path::dir_exists<true>(json[str_project]["path-app"])._ret;

  // Path to image
  std::string str_image = ns_fs::ns_path::file_exists<true>(json[str_project]["path-image"])._ret;

  // Output file
  std::string str_target = str_app + ".dwarfs";

  // Log
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
