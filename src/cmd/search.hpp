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
#include "../lib/json.hpp"

namespace ns_search
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
inline void search(std::vector<std::string> args)
{
  ns_json::Json json = ns_json::from_file_default();
  std::string str_project = json["project"];
  std::string str_app = json[str_project]["platform"];
  ns_enum::Platform enum_platform = ns_enum::from_string<ns_enum::Platform>(str_app);
  fs::path path_project = json[str_project]["path-app"];

  // Retrieve operation selected by user
  Op op;
  "Invalid operation '{}'"_try([&]
  { 
    op = ns_enum::from_string<Op>(args.front());
  }, args.front());
  args.erase(args.begin());

  switch(enum_platform)
  {
    case ns_enum::Platform::WINE:
    {
      // Enter drive_c
      path_project = (path_project / "wine") / "drive_c";
      // Search executables
      for(auto i : ns_impl::search(path_project, R"(.*\.exe$)", R"(windows)"))
      {
        i = fs::relative(i, path_project);
        ns_log::write('i', "Found :: ", i);
      } // for
    } // case
    break;
    case ns_enum::Platform::RETROARCH:
    {
      // Enter drive_c
      path_project = path_project / "rom";
      // Search executables
      for(auto i : ns_impl::search(path_project, R"(.*\.exe$)", R"(windows)"))
      {
        i = fs::relative(i, path_project);
        ns_log::write('i', "Found :: ", i);
      } // for
    } // case
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

} // namespace ns_select

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
