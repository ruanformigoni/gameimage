///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : select
// @created     : Monday Feb 05, 2024 15:02:17 -03
///

#pragma once

#include <filesystem>
#include <regex>

#include <cppcoro/generator.hpp>
#include <matchit.h>

#include "../common.hpp"
#include "../enum.hpp"

#include "../std/filesystem.hpp"

#include "../lib/subprocess.hpp"
#include "../lib/log.hpp"
#include "../lib/json.hpp"

namespace ns_select
{

namespace fs = std::filesystem;
namespace cr = cppcoro;

enum class Op
{
  TARGET,
  CORE,
};


// namespace ns_impl {{{
namespace ns_impl
{

// search() {{{
inline cr::generator<fs::path> search(fs::path path_dir
  , char const* str_pattern
  , char const* str_exclude)
{
  // Validate file path
  ns_fs::ns_path::dir_exists<true>(path_dir);

  ns_log::write('i', "Search directory '", path_dir.c_str(), "'");

  // Create regex
  std::regex regex_pattern(str_pattern);
  std::regex regex_exclude(str_exclude);

  // Find all files that match pattern
  for(auto entry = fs::recursive_directory_iterator(path_dir);
    entry != fs::recursive_directory_iterator();
    ++entry)
  {
    std::string target = ns_fs::ns_path::file_name<true>(*entry)._ret;

    if (fs::is_directory(entry->path()) && std::regex_match(target, regex_exclude))
    {
      ns_log::write('i', "Exclude directory '", ns_common::to_string(*entry), "'");
      entry.disable_recursion_pending();
      continue;
    } // if

    if ( fs::is_regular_file(entry->path()) && std::regex_match(target, regex_pattern))
    {
      fs::path curr = entry->path();
      co_yield curr;
    } // if
  } // for

} // search() }}}

} // namespace ns_impl }}}

// search() {{{
inline void search(ns_enum::Platform enum_platform, fs::path path)
{
  switch(enum_platform)
  {
    case ns_enum::Platform::WINE:
      // Enter drive_c
      path = (path / "wine") / "drive_c";
      // Search executables
      for(auto i : ns_impl::search(path, R"(.*\.exe$)", R"(windows)"))
      {
        i = fs::relative(i, path);
        ns_log::write('i', "Found :: ", i);
      }
      break;
    case ns_enum::Platform::RETROARCH:
      "Not implemented"_throw();
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

} // search() }}}

// core() {{{
inline void core(ns_enum::Platform enum_platform
  , fs::path path_dir_project
  , fs::path path_file_core)
{
  ns_json::Json json_project;
  // Try to open existing file
  "Creating {}"_catch([&]{ json_project = ns_json::from_file_project(); }, ns_json::file_project());

  switch(enum_platform)
  {
    case ns_enum::Platform::WINE:
    {
      "Core selection is not available for the wine platform"_throw();
    } // case
    break;
    case ns_enum::Platform::RETROARCH:
    {
      path_file_core = fs::path("core") / path_file_core;
    } // case
    break;
    case ns_enum::Platform::PCSX2:
    {
      "Not implemented"_throw();
    } // case
    break;
    case ns_enum::Platform::RPCS3:
    {
      "Not implemented"_throw();
    } // case
    break;
    case ns_enum::Platform::YUZU:
    {
      "Not implemented"_throw();
    } // case
    break;
  } // switch

  // Check if is regular file
  ns_fs::ns_path::file_exists<true>(path_dir_project / path_file_core);
  // Set as default core file
  json_project("path-file-core") = path_file_core;
  // Save to file
  ns_json::to_file_project(json_project);

} // select() }}}

// target() {{{
inline void target(ns_enum::Platform enum_platform
  , fs::path path_dir_project
  , fs::path path_file_target)
{
  ns_json::Json json_project;
  // Try to open existing file
  "Creating {}"_catch([&]{ json_project = ns_json::from_file_project(); }, ns_json::file_project());

  switch(enum_platform)
  {
    case ns_enum::Platform::WINE:
    {
      path_file_target = (fs::path("wine") / "drive_c") / path_file_target;
    } // case
    break;
    case ns_enum::Platform::RETROARCH:
    {
      path_file_target = fs::path("rom") / path_file_target;
    } // case
    break;
    case ns_enum::Platform::PCSX2:
    {
      "Not implemented"_throw();
    } // case
    break;
    case ns_enum::Platform::RPCS3:
    {
      "Not implemented"_throw();
    } // case
    break;
    case ns_enum::Platform::YUZU:
    {
      "Not implemented"_throw();
    } // case
    break;
  } // switch

  // Check if is regular file
  ns_fs::ns_path::file_exists<true>(path_dir_project / path_file_target);
  // Set as default target file
  json_project("path-file-target") = path_file_target;
  // Save to file
  ns_json::to_file_project(json_project);

} // select() }}}

// select() {{{
inline void select(std::vector<std::string> args)
{
  ns_json::Json json = ns_json::from_file_default();
  std::string str_project = json["project"];
  std::string str_app = json[str_project]["platform"];
  ns_enum::Platform enum_platform = ns_enum::from_string<ns_enum::Platform>(str_app);

  // Check if args were passed
  "Empty arguments for select command\n"
  "Valid options are target, core"_throw_if([&]{ return args.empty(); });

  // Retrieve operation selected by user
  Op op;
  "Invalid operation '{}'"_try([&]
  { 
    op = ns_enum::from_string<Op>(args.front());
  }, args.front());
  args.erase(args.begin());

  // Check if args were passed to the selected operation
  "No argument provided for the '{}' operation"_throw_if([&]{ return args.empty(); }, ns_enum::to_string(op));
  target(enum_platform, json[str_project]["path-app"], args.front());
} // }}}

} // namespace ns_select

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
