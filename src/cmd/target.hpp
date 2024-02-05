///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : target
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

namespace ns_target
{

namespace fs = std::filesystem;
namespace cr = cppcoro;
namespace match = matchit;

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

enum class TargetOperation
{
  TARGET,
  CORE,
};

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

// select() {{{
inline void select(ns_enum::Platform enum_platform
  , TargetOperation operation
  , fs::path dir_base
  , fs::path path_file)
{
  ns_json::Json json_project;
  // Try to open existing file
  "Creating {}"_catch([&]{ json_project = ns_json::from_file_project(); }, ns_json::file_project());

  switch(enum_platform)
  {
    case ns_enum::Platform::WINE:
    {
      // Wine only selects 'rom', a .exe or .msi
      "Operation select for '{}' not available in wine"_throw_if(
        [&]{ return operation != TargetOperation::TARGET; }
      );
      // Enter drive_c
      path_file = (fs::path("wine") / "drive_c") / path_file;
      // Check if is regular file
      ns_fs::ns_path::file_exists<true>(dir_base / path_file);
      // Set as default target file
      json_project("path-file-target") = path_file;
    } // case
    break;
    case ns_enum::Platform::RETROARCH:
      // Check if is regular file
      ns_fs::ns_path::file_exists<true>(dir_base / path_file);
      // Save selected target
      switch(operation)
      {
        case TargetOperation::TARGET: json_project("path-file-target") = path_file;
        break;
        case TargetOperation::CORE: json_project("path-file-core") = path_file;
        break;
      }
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

  // Save to file
  ns_json::to_file_project(json_project);

} // select() }}}

// target() {{{
inline void target(std::vector<std::string> args)
{
  ns_json::Json json = ns_json::from_file_default();
  std::string str_project = json["project"];
  std::string str_app = json[str_project]["platform"];
  ns_enum::Platform enum_platform = ns_enum::from_string<ns_enum::Platform>(str_app);

  // Check if args were passed
  "Empty arguments for target command"_throw_if([&]{ return args.empty(); });

  // Retrieve user-input operation
  std::string str_operation{args.front()};
  args.erase(args.begin());

  // Select operation
  match::match(str_operation)
  (
    match::pattern | "search" = [&]{ search(enum_platform, json[str_project]["path-app"]); },
    match::pattern | "select" = [&]
    {
      // Check if args were passed
      "Empty sub-command for select\n"
      "Valid commands are target and core"_throw_if([&]{ return args.empty(); });
      std::string str_select_cmd = args.front(); args.erase(args.begin());

      // Select target
      "No file provided to set as the default target file"_throw_if([&]{ return args.empty(); });
      select(enum_platform
        , ns_enum::from_string<TargetOperation>(str_select_cmd)
        , json[str_project]["path-app"]
        , args.front()
      );
    },
    match::pattern | match::_ = [&]
    {
      "Invalid target command '{}'\n"
      "Valid commands are search and select"_throw(str_operation);
    }
  );

} // }}}

} // namespace ns_target

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
