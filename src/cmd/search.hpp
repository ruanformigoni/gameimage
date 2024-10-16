///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : search
///

#pragma once

#include <filesystem>
#include <regex>

#include "fetch.hpp"

#include "../common.hpp"
#include "../enum.hpp"

#include "../std/filesystem.hpp"

#include "../lib/log.hpp"
#include "../lib/db/project.hpp"
#include "../lib/db/build.hpp"
#include "../lib/ipc.hpp"

namespace ns_search
{

namespace fs = std::filesystem;

using Op = ns_enum::Op;

// anonymous namespace
namespace
{

// search_files() {{{
inline std::vector<fs::path> search_files(fs::path path_dir_search
  , char const* str_pattern
  , char const* str_exclude)
{
  std::vector<fs::path> ret;

  // Validate file path
  ns_fs::ns_path::dir_exists<true>(path_dir_search);

  ns_log::write('i', "Search directory '", path_dir_search.c_str(), "'");

  // Create regex
  std::regex regex_pattern(str_pattern, std::regex_constants::icase);
  std::regex regex_exclude(str_exclude, std::regex_constants::icase);

  // Find all files that match pattern
  for(auto entry = fs::recursive_directory_iterator(path_dir_search);
    entry != fs::recursive_directory_iterator();
    ++entry)
  {
    std::string name_file = ns_fs::ns_path::file_name<true>(*entry)._ret;

    std::string path_file_entry;
    if ( auto ret = ns_fs::ns_path::canonical<false>(entry->path()); ret._bool )
    {
      path_file_entry = ret._ret;
    } // if
    else
    {
      ns_log::write('i', "Exclude path '", ns_string::to_string(*entry), "'");
      continue;
    } // else

    if (fs::is_directory(path_file_entry) && std::regex_match(name_file, regex_exclude))
    {
      ns_log::write('i', "Exclude directory '", ns_string::to_string(*entry), "'");
      entry.disable_recursion_pending();
      continue;
    } // if

    if ( fs::is_regular_file(path_file_entry) && std::regex_match(name_file, regex_pattern))
    {
      fs::path path_file_found = fs::relative(path_file_entry, path_dir_search.parent_path());
      ret.push_back(path_file_found);
    } // if
  } // for

  return ret;
} // search_files() }}}

// search_dirs() {{{
inline std::vector<fs::path> search_dirs(fs::path const& path_dir_search)
{
  std::vector<fs::path> ret;
  for(fs::path path_file_found : fs::directory_iterator(path_dir_search))
  {
    path_file_found = fs::relative(path_file_found, path_dir_search.parent_path());
    ret.push_back(path_file_found);
  } // for
  return ret;
} // search_dirs() }}}

// search_remote() {{{
inline std::vector<std::string> search_remote()
{
  std::vector<std::string> ret;
  auto expected_cores_list = ns_fetch::fetch_cores();
  ethrow_if(not expected_cores_list, expected_cores_list.error());
  for( auto entry : *expected_cores_list )
  {
    ret.push_back(entry.core);
  } // for
  return ret;
} // search_remote() }}}

// send() {{{
auto send(auto&& vec_paths, std::unique_ptr<ns_ipc::Ipc> const& ipc)
{
  if ( ipc != nullptr )
  {
    std::ranges::for_each(vec_paths, [&](auto&& e){ ipc->send(e); });
    return;
  } // if

  std::ranges::for_each(vec_paths, [&](auto&& e){ ns_log::write('i', "Found: ", e); });
} // send() }}}

} // anonymous namespace

// search_remote() {{{
inline void search_remote(std::optional<std::string> opt_query, bool use_ipc)
{
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Could not open build database");
  auto db_metadata = db_build->find(db_build->project);

  std::unique_ptr<ns_ipc::Ipc> ipc;

  if ( use_ipc )
  {
    // Use self as IPC reference
    fs::path path_file_ipc = ns_fs::ns_path::file_self<true>()._ret;
    // Create ipc
    ipc = std::make_unique<ns_ipc::Ipc>(path_file_ipc, true);
  } // if

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

  // Handle fetch for each platform
  switch(db_metadata.platform)
  {
    case ns_enum::Platform::RETROARCH: send(search_remote(), ipc);
    break;
    case ns_enum::Platform::LINUX:
    case ns_enum::Platform::WINE:
    case ns_enum::Platform::PCSX2:
    case ns_enum::Platform::RPCS3: "Not implemented"_throw();
  } // switch

} // search_remote() }}}

// search_local() {{{
inline void search_local(std::optional<std::string> opt_query, bool use_ipc)
{
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Could not open build database");
  auto db_metadata = db_build->find(db_build->project);
  auto db_project = ns_db::ns_project::read();
  ethrow_if(not db_project, "Could not open project database");

  std::unique_ptr<ns_ipc::Ipc> ipc;

  if ( use_ipc )
  {
    // Use self as IPC reference
    fs::path path_file_ipc = ns_fs::ns_path::file_self<true>()._ret;
    // Create ipc
    ipc = std::make_unique<ns_ipc::Ipc>(path_file_ipc, true);
  } // if

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
  fs::path path_dir_search = db_metadata.path_dir_project / db_project->find_directory(op);

  // Handle fetch for each platform
  switch(db_metadata.platform)
  {
    case ns_enum::Platform::LINUX:
    {
       // Check if is rom
      "Only rom operation is available for linux"_throw_if([&]{ return op != Op::ROM; });
      // Enter application dir
      path_dir_search = (db_metadata.path_dir_project / db_project->find_directory(op));
      // Get files iterator
      auto it_files = search_files(path_dir_search, R"(.*\.sh)", "");
      // Save files to json
      send(it_files, ipc);
    } // case
    break;
    case ns_enum::Platform::WINE:
    {
      // Check if is rom
      "Only rom operation is available for wine"_throw_if([&]{ return op != Op::ROM; });
      // Enter drive_c
      path_dir_search = (db_metadata.path_dir_project / db_project->find_directory(op))  / "drive_c";
      // Get files iterator
      auto it_files = search_files(path_dir_search, R"(.*\.exe$)", R"(windows)");
      // For each file, prepend "wine" to be relative to path_dir_project
      std::vector<fs::path> paths_file_matches;
      std::ranges::for_each(it_files, [&](fs::path const& e){ paths_file_matches.push_back(fs::path("wine") / e); });
      // Save files to json
      send(paths_file_matches, ipc);
    } // case
    break;
    case ns_enum::Platform::RETROARCH: send(search_files(path_dir_search, R"(.*)", ""), ipc); break;
    case ns_enum::Platform::PCSX2    : send(search_files(path_dir_search, R"(.*)", ""), ipc); break;
    case ns_enum::Platform::RPCS3    : send(search_dirs(path_dir_search), ipc);               break;
  } // switch

} // search_local() }}}

} // namespace ns_select

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
