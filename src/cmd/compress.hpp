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
  
// compress() {{{
inline decltype(auto) compress()
{
  ns_json::Json json = ns_json::from_default_file();

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
  ns_subprocess::subprocess(str_image
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
