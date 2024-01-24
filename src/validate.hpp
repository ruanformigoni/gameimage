///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : validate
///

#pragma once

#include <string>
#include <exception>
#include <filesystem>

#include "enum.hpp"
#include "common.hpp"

#include "std/filesystem.hpp"
#include "lib/log.hpp"


namespace ns_validate
{

namespace fs = std::filesystem;

// platform() {{{
ns_enum::Platform platform(std::string const& str_platform)
{
  try
  {
    return ns_enum::from_string<ns_enum::Platform>(str_platform);
  }
  catch (std::exception const& e)
  {
    ns_log::write('e', e.what());
    "Platform '{}' is invalid"_throw(str_platform);
  } // if

  return {};
} // function: platform }}}

// path_parent_exists() {{{
fs::path path_parent_exists(std::string const& str_path)
{
  // Check if has path
  if ( str_path.empty() )
  {
    "Empty path to dir"_throw();
  } // if

  // Convert string to path
  fs::path path_raw(str_path);

  // Check if parent exists
  fs::path path_parent = ns_fs::ns_path::canonical<true>(path_raw.parent_path())._ret;

  // Return absolute path
  return path_parent /= *std::prev(path_raw.end());
} // function: platform }}}

// path_file_valid() {{{
// Checks if the path is valid
// // Parent path must exist
// // Last component is optional
fs::path path_file_valid(std::string const& str_path)
{
  // Full path, canonical
  fs::path path_full{path_parent_exists(str_path)};

  // Verify file name
  ns_fs::ns_path::file_name<true>(path_full);

  // Return absolute path with file name
  return path_full;
} // function: path_file_valid }}}

// path_file_exists() {{{
// Checks if file exists and is regular file
fs::path path_file_exists(std::string const& str_path)
{
  // Check if path is valid
  fs::path path_full = path_file_valid(str_path);

  // Check if file exists
  ns_fs::ns_path::file_exists<true>(path_full);

  // Return absolute path with file name
  return path_full;
} // function: path_file_exists }}}

} // namespace ns_validate

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
