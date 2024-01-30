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

#include "../lib/log.hpp"
#include "../lib/json.hpp"

//
// Initializes a new directory configuration for gameimage
//
namespace ns_init
{

namespace fs    = std::filesystem;

// init() {{{
inline void init(std::string const& str_platform
  , std::string const& str_path_app
  , std::string const& str_path_image)
{
  // Validate
  ns_enum::Platform platform = ns_enum::from_string<ns_enum::Platform>(str_platform);
  fs::path path_image        = ns_fs::ns_path::file_exists<true>(str_path_image)._ret;
  fs::path path_app          = ns_fs::ns_path::dir_create<true>(str_path_app)._ret;

  // Log
  ns_log::write('i', "platform: ", str_platform);
  ns_log::write('i', "image: ", path_image);
  ns_log::write('i', "dir: ", path_app);

  // Check if data file exists
  ns_fs::ns_path::file_exists<true>(path_image);

  // Check if app directory parent exists
  ns_fs::ns_path::dir_exists<true>(path_app.parent_path());

  // Create directory
  ns_fs::ns_path::dir_create<true>(path_app);

  // Create json obj
  ns_json::Json json_config; 

  // Try to open default file if exists
  try
  {
    json_config = ns_json::from_file_default();
  } // try
  catch( std::exception const& e )
  {
    ns_log::write('i', "File ", ns_json::file_default(), " not found");
    ns_log::write('i', "Creating {}"_fmt(ns_json::file_default()));
  } // catch

  // App name is Dir name
  std::string str_name = path_app.filename();

  // Set data
  json_config(str_name)("path-image") = path_image;
  json_config(str_name)("path-app")   = path_app;
  json_config(str_name)("platform")   = ns_enum::to_string(platform);

  // Write to json file
  ns_json::to_file_default(json_config);
} // function: init }}}

} // namespace ns_init

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
