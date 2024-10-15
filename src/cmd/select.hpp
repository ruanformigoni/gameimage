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

using Op = ns_enum::Op;

// by_op() {{{
inline void by_op(ns_enum::Platform enum_platform
  , Op op
  , fs::path path_dir_project
  , fs::path path_file_op)
{
  switch(enum_platform)
  {
    case ns_enum::Platform::LINUX:
    {
      "Only the rom option is available for linux"_throw_if([&]{ return op != Op::ROM; });
    } // case
    break;
    case ns_enum::Platform::WINE:
    {
      "Only the rom option is available for wine"_throw_if([&]{ return op != Op::ROM; });
    } // case
    break;
    case ns_enum::Platform::RETROARCH:
    {
      "Only rom, core and bios options are available for retroarch"_throw_if([&]
      {
        return op != Op::ROM && op != Op::CORE && op != Op::BIOS;
      });
    } // case
    break;
    case ns_enum::Platform::PCSX2:
    {
      "Only rom and bios options are available for pcsx2"_throw_if([&]
      {
        return op != Op::ROM && op != Op::BIOS;
      });
    } // case
    break;
    case ns_enum::Platform::RPCS3:
    {
      "Only rom and bios options are available for rpcs3"_throw_if([&]
      {
        return op != Op::ROM && op != Op::BIOS;
      });
    } // case
    break;
  } // switch

  // Check if is regular file or directory
  try
  {
    ns_fs::ns_path::file_exists<true>(path_dir_project / path_file_op);
  } // try
  catch(std::exception const& e)
  {
    ns_fs::ns_path::dir_exists<true>(path_dir_project / path_file_op);
  } // catch

  ns_db::from_file_project([&](auto&& db)
  {
    // Get op
    std::string str_op = "path_file_{}"_fmt(ns_enum::to_string_lower(op));
    // Set as default rom file
    db(str_op) = path_file_op;
    ns_log::write('i', "Selected ", str_op, ": ", path_file_op);
  }
  , ns_db::Mode::UPDATE);

} // select() }}}

// select() {{{
inline void select(Op const& op, std::string const& entry)
{
  std::string str_project;
  std::string str_platform;
  fs::path path_project;

  ns_db::from_file_default([&](auto&& db)
  {
    str_project  = db["project"];
    str_platform = db[str_project]["platform"];
    path_project = fs::path(db[str_project]["path_dir_project"]);
  }
  , ns_db::Mode::READ);

  ns_enum::Platform enum_platform = ns_enum::from_string<ns_enum::Platform>(str_platform);

  by_op(enum_platform, op, path_project, entry);
} // }}}

} // namespace ns_select

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
