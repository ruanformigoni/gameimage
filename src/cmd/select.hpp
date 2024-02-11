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
#include "../lib/db.hpp"

namespace ns_select
{

namespace fs = std::filesystem;

// enum class Op {{{
enum class Op
{
  ROM,
  CORE,
  BIOS,
};
// }}}

// by_op() {{{
inline void by_op(ns_enum::Platform enum_platform
  , Op op
  , fs::path path_dir_project
  , fs::path path_file_op)
{
  switch(enum_platform)
  {
    case ns_enum::Platform::WINE:
    {
      "Only rom selection is available for wine"_throw_if([&]{ return op != Op::ROM; });
      path_file_op = (fs::path("wine") / "drive_c") / path_file_op;
    } // case
    break;
    case ns_enum::Platform::RETROARCH:
    {
      "Only rom, core and bios selection are available for retroarch"_throw_if([&]
      {
        return op != Op::ROM && op != Op::CORE && op != Op::BIOS;
      });
    } // case
    break;
    case ns_enum::Platform::PCSX2:
    {
      "Only rom and bios selection are available for pcsx2"_throw_if([&]
      {
        return op != Op::ROM && op != Op::BIOS;
      });
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
  ns_fs::ns_path::file_exists<true>(path_dir_project / path_file_op);

  ns_db::from_file_project([&](auto&& db)
  {
    // Get op
    std::string str_op = "path-file-{}"_fmt(ns_enum::to_string_lower(op));
    // Set as default rom file
    db(str_op) = path_file_op;
    ns_log::write('i', "Selected ", str_op, ": ", path_file_op);
  }
  , std::ios_base::out);

} // select() }}}

// select() {{{
inline void select(std::vector<std::string> args)
{
  std::string str_project;
  std::string str_platform;
  fs::path path_project;

  ns_db::from_file_default([&](auto&& db)
  {
    str_project  = db["project"];
    str_platform = db[str_project]["platform"];
    path_project = fs::path(db[str_project]["path-project"]);
  }
  , std::ios_base::in);

  ns_enum::Platform enum_platform = ns_enum::from_string<ns_enum::Platform>(str_platform);

  // Check if args were passed
  "Empty arguments for select command\n"
  "Valid options are rom, core and bios"_throw_if([&]{ return args.empty(); });

  // Retrieve operation selected by user
  Op op;
  "Invalid operation '{}'"_try([&]
  { 
    op = ns_enum::from_string<Op>(args.front());
  }, args.front());
  args.erase(args.begin());

  // Check if args were passed to the selected operation
  "No argument provided for the '{}' operation"_throw_if([&]{ return args.empty(); }, ns_enum::to_string(op));
  by_op(enum_platform, op, path_project, args.front());
} // }}}

} // namespace ns_select

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
