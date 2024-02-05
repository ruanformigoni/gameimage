///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : select
// @created     : Monday Feb 05, 2024 15:02:17 -03
///

#pragma once

#include <filesystem>
#include <regex>

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

enum class Op
{
  ROM,
  CORE,
};


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

// rom() {{{
inline void rom(ns_enum::Platform enum_platform
  , Op op
  , fs::path path_dir_project
  , fs::path path_file_rom)
{
  ns_json::Json json_project;
  // Try to open existing file
  "Creating {}"_catch([&]{ json_project = ns_json::from_file_project(); }, ns_json::file_project());

  switch(enum_platform)
  {
    case ns_enum::Platform::WINE:
    {
      "Only rom selection is available for wine"_throw_if([&]{ return op != Op::ROM; });
      path_file_rom = (fs::path("wine") / "drive_c") / path_file_rom;
    } // case
    break;
    case ns_enum::Platform::RETROARCH:
    {
      "Only rom and core selection are available for retroarch"_throw_if([&]
      {
        return op != Op::ROM && op != Op::CORE;
      });
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
  ns_fs::ns_path::file_exists<true>(path_dir_project / path_file_rom);
  // Set as default rom file
  json_project("path-file-{}"_fmt(ns_enum::to_string_lower(op))) = path_file_rom;
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
  "Valid options are rom, core"_throw_if([&]{ return args.empty(); });

  // Retrieve operation selected by user
  Op op;
  "Invalid operation '{}'"_try([&]
  { 
    op = ns_enum::from_string<Op>(args.front());
  }, args.front());
  args.erase(args.begin());

  // Check if args were passed to the selected operation
  "No argument provided for the '{}' operation"_throw_if([&]{ return args.empty(); }, ns_enum::to_string(op));
  rom(enum_platform, op, json[str_project]["path-project"], args.front());
} // }}}

} // namespace ns_select

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
