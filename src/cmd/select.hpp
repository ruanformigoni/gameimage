///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : select
// @created     : Monday Feb 05, 2024 15:02:17 -03
///

#pragma once

#include <filesystem>

#include <matchit.h>

#include "../common.hpp"
#include "../enum.hpp"

#include "../std/filesystem.hpp"

#include "../lib/db/build.hpp"
#include "../lib/db/project.hpp"

namespace ns_select
{

namespace fs = std::filesystem;

using Op = ns_enum::Op;

// by_op() {{{
inline void by_op(ns_enum::Platform enum_platform
  , Op op
  , fs::path path_dir_project
  , fs::path path_file_target)
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
    ns_fs::ns_path::file_exists<true>(path_dir_project / path_file_target);
  } // try
  catch(std::exception const& e)
  {
    ns_fs::ns_path::dir_exists<true>(path_dir_project / path_file_target);
  } // catch

  auto db_project = ns_db::ns_project::read();
  ethrow_if(not db_project, "Could not open project database '{}'"_fmt(db_project.error()));
  switch (op)
  {
    case ns_enum::Op::ROM: db_project->path_file_rom = path_file_target; break;
    case ns_enum::Op::BIOS: db_project->path_file_bios = path_file_target; break;
    case ns_enum::Op::CORE: db_project->path_file_core = path_file_target; break;
    default: throw std::runtime_error("Cannot set default for '{}'"_fmt(ns_enum::to_string(op)));
  } // switch
  ns_db::ns_project::write(*db_project);
} // select() }}}

// select() {{{
inline void select(Op const& op, fs::path const& path_file_target)
{
  // Open db
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Could not open build database");
  auto db_metadata = db_build->find(db_build->project);
  // Select the default file by platform
  by_op(db_metadata.platform, op, db_metadata.path_dir_project, path_file_target);
} // }}}

} // namespace ns_select

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
