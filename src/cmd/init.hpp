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
void init(ns_enum::Platform platform
  , fs::path const& path_app
  , fs::path const& path_image)
{
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
    json_config = ns_json::from_default_file();
  } // try
  catch( std::exception const& e )
  {
    ns_log::write('i', e.what());
    ns_log::write('i', "Creating {}"_fmt(ns_json::default_file()));
  } // catch

  // App name is Dir name
  std::string str_name = path_app.filename();

  // Set data
  json_config[str_name]["path-image"] = path_image;
  json_config[str_name]["path-app"]   = path_app;
  json_config[str_name]["platform"]   = ns_enum::to_string(platform);

  // Write to json file
  ns_json::to_default_file(json_config);
} // function: init }}}

} // namespace ns_init

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
