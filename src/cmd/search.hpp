///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : search
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
inline void search(std::optional<fs::path> to_json, std::optional<std::string> query)
{
  std::string str_project;
  std::string str_platform;
  fs::path path_project;

  ns_db::from_file_default([&](auto&& db)
  {
    str_project = db["project"];
    str_platform = db[str_project]["platform"];
    path_project = std::string(db[str_project]["path-project"]); 
  }, std::ios_base::in);

  ns_enum::Platform enum_platform = ns_enum::from_string<ns_enum::Platform>(str_platform);

  // Retrieve operation selected by user
  Op op;

  // Check if has query
  if ( ! query )
  {
    "Empty operation for search"_throw();
  } // if

  // Fetch query
  "Invalid operation for search\n"_try([&]
  { 
    op = ns_enum::from_string<Op>(*query);
  });

  // Writes paths to json database
  auto f_paths_to_json = [&](Op op, std::vector<fs::path> const& vec_paths)
  {
    // Write to json
    if ( to_json.has_value() )
    {
      // Erase file if exists
      fs::remove(*to_json);

      // Open file list
      ns_db::from_file(*to_json, [&]<typename T>(T&& db)
      {
        for(fs::path const& path_file : vec_paths)
        {
          db(ns_enum::to_string_lower(op)) |= path_file;
        }
      }, std::ios::out);
    } // if
  };

  // Searches for existing files matching search inside the path_search path
  auto f_search_files = [](fs::path const& path_search
    , std::string str_search
    , std::string str_exclude)
  {
    // Save to vec for json
    std::vector<fs::path> vec_paths;
    // Search executables
    for(auto i : ns_impl::search(path_search, str_search.c_str(), str_exclude.c_str()))
    {
      i = fs::relative(i, path_search);
      ns_log::write('i', "Found :: ", i);
      vec_paths.push_back(i);
    } // for
    // Return written paths
    return vec_paths;
  };

  // Get op as str
  fs::path path_search = path_project / ns_db::query(ns_db::file_project(), "path-dir-{}"_fmt(ns_enum::to_string_lower(op)));
  switch(enum_platform)
  {
    case ns_enum::Platform::WINE:
    {
      // Check if is rom
      "Only rom operation is available for wine"_throw_if([&]{ return op != Op::ROM; });
      // Enter drive_c
      path_search = (path_project / "wine") / "drive_c";
      // Save files to json
      f_paths_to_json(op, f_search_files(path_search, R"(.*\.exe$)", R"(windows)"));
    } // case
    break;
    case ns_enum::Platform::RETROARCH : f_paths_to_json(op, f_search_files(path_search, R"(.*)", "")); break;
    case ns_enum::Platform::PCSX2     : f_paths_to_json(op, f_search_files(path_search, R"(.*)", "")); break;
    case ns_enum::Platform::RPCS3     : f_paths_to_json(op, f_search_files(path_search, R"(.*)", "")); break;
    case ns_enum::Platform::YUZU      : f_paths_to_json(op, f_search_files(path_search, R"(.*)", "")); break;
  } // switch

} // search() }}}

} // namespace ns_select

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
