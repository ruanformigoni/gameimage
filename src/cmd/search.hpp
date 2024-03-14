///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : search
///

#pragma once

#include <filesystem>
#include <regex>

#include <cppcoro/generator.hpp>
#include <matchit.h>

#include "fetch.hpp"

#include "../common.hpp"
#include "../enum.hpp"

#include "../std/filesystem.hpp"

#include "../lib/subprocess.hpp"
#include "../lib/log.hpp"
#include "../lib/db.hpp"

namespace ns_search
{

namespace fs = std::filesystem;
namespace cr = cppcoro;

// enum class Op {{{
enum class Op
{
  ROM,
  CORE,
  BIOS,
  KEYS,
};
// }}}

// anonymous namespace
namespace
{

// search_files() {{{
inline cr::generator<fs::path> search_files(fs::path path_dir
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
    std::string name_file = ns_fs::ns_path::file_name<true>(*entry)._ret;
    std::string path_file_entry = ns_fs::ns_path::canonical<true>(entry->path())._ret;

    if (fs::is_directory(path_file_entry) && std::regex_match(name_file, regex_exclude))
    {
      ns_log::write('i', "Exclude directory '", ns_common::to_string(*entry), "'");
      entry.disable_recursion_pending();
      continue;
    } // if

    if ( fs::is_regular_file(path_file_entry) && std::regex_match(name_file, regex_pattern))
    {
      ns_log::write('i', "Found :: ", fs::relative(path_file_entry, path_dir));
      co_yield fs::relative(path_file_entry, path_dir);
    } // if
  } // for

} // search_files() }}}

// search_dirs() {{{
inline cr::generator<fs::path> search_dirs(fs::path const& path_dir_search)
{
  for(fs::path i : fs::directory_iterator(path_dir_search)) { co_yield i; }
} // search_dirs() }}}

// search_remote() {{{
inline cr::generator<std::string> search_remote(fs::path const& path_dir_fetch)
{
  for( auto i : ns_fetch::cores_list(path_dir_fetch) )
  {
    ns_log::write('i', "Found :: ", i.core);
    co_yield i.core;
  } // for
} // search_remote() }}}

// paths_to_json() {{{
auto paths_to_json(Op op, auto&& opt_path_file_json, auto&& vec_paths)
{
  // Check if should write to json
  if ( ! opt_path_file_json.has_value() ) { return; } // if

  // Open file list
  ns_db::from_file(*opt_path_file_json, [&]<typename T>(T&& db)
  {
    for(auto&& path_file : vec_paths)
    {
      db(ns_enum::to_string_lower(op)) |= path_file;
    }
  }, ns_db::Mode::WRITE);
} // paths_to_json() }}}

} // anonymous namespace

// search_remote() {{{
inline void search_remote(std::optional<std::string> opt_query, std::optional<fs::path> opt_path_file_json)
{
  std::string str_project = ns_db::query(ns_db::file_default(), "project");
  std::string str_platform = ns_db::query(ns_db::file_default(), str_project, "platform");
  fs::path path_dir_project = ns_db::query(ns_db::file_default(), str_project, "path-project");

  // Retrieve operation selected by user
  Op op;

  // Check if has opt_query
  if ( ! opt_query )
  {
    "Empty operation for search"_throw();
  } // if

  // Fetch query
  "Invalid operation for search\n"_try([&]
  { 
    op = ns_enum::from_string<Op>(*opt_query);
  });

  // Get search dir
  fs::path path_dir_search = path_dir_project / ns_db::query(ns_db::file_project(), "path-dir-{}"_fmt(ns_enum::to_string_lower(op)));

  // Handle fetch for each platform
  switch(ns_enum::from_string<ns_enum::Platform>(str_platform))
  {
    case ns_enum::Platform::RETROARCH: paths_to_json(op, opt_path_file_json, search_remote(path_dir_project));
    break;
    case ns_enum::Platform::WINE:
    case ns_enum::Platform::PCSX2:
    case ns_enum::Platform::RPCS3:
    case ns_enum::Platform::YUZU : "Not implemented"_throw();
  } // switch

} // search_remote() }}}

// search_local() {{{
inline void search_local(std::optional<std::string> opt_query, std::optional<fs::path> opt_path_file_json)
{
  std::string str_project = ns_db::query(ns_db::file_default(), "project");
  std::string str_platform = ns_db::query(ns_db::file_default(), str_project, "platform");
  fs::path path_dir_project = ns_db::query(ns_db::file_default(), str_project, "path-project");

  // Retrieve operation selected by user
  Op op;

  // Check if has opt_query
  if ( ! opt_query )
  {
    "Empty operation for search"_throw();
  } // if

  // Fetch query
  "Invalid operation for search\n"_try([&]
  { 
    op = ns_enum::from_string<Op>(*opt_query);
  });

  // Get search dir
  fs::path path_dir_search = path_dir_project / ns_db::query(ns_db::file_project(), "path-dir-{}"_fmt(ns_enum::to_string_lower(op)));

  // Handle fetch for each platform
  switch(ns_enum::from_string<ns_enum::Platform>(str_platform))
  {
    case ns_enum::Platform::WINE:
    {
      // Check if is rom
      "Only rom operation is available for wine"_throw_if([&]{ return op != Op::ROM; });
      // Enter drive_c
      path_dir_search = (path_dir_project / "wine") / "drive_c";
      // Save files to json
      paths_to_json(op, opt_path_file_json, search_files(path_dir_search, R"(.*\.exe$)", R"(windows)"));
    } // case
    break;
    case ns_enum::Platform::RETROARCH: paths_to_json(op, opt_path_file_json, search_files(path_dir_search, R"(.*)", "")); break;
    case ns_enum::Platform::PCSX2    : paths_to_json(op, opt_path_file_json, search_files(path_dir_search, R"(.*)", "")); break;
    case ns_enum::Platform::RPCS3    : paths_to_json(op, opt_path_file_json, search_dirs(path_dir_search)); break;
    case ns_enum::Platform::YUZU     : paths_to_json(op, opt_path_file_json, search_files(path_dir_search, R"(.*)", "")); break;
  } // switch

} // search_local() }}}

} // namespace ns_select

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
