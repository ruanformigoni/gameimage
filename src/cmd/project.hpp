///
///@author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
///@file        : project
///

#pragma once

#include <string>

#include "../lib/db/build.hpp"

namespace ns_project
{

namespace
{

namespace fs = std::filesystem;

} // namespace

// set() {{{
[[nodiscard]] inline std::expected<void,std::string> set(std::string const& name) noexcept
{
  // Open db
  auto db_build = ns_db::ns_build::read();
  qreturn_if(not db_build, std::unexpected("Could not open build database"));
  // Check if 'name' is present in db
  auto const& opt_metadata = ns_exception::to_expected([&]{ return db_build->find(name); });
  qreturn_if(not opt_metadata, std::unexpected("Could not find project '{}'"_fmt(name)));
  // Set as current
  db_build->project = opt_metadata->name;
  // Update database
  ns_db::ns_build::write(*db_build);
  ns_log::write('i', "Set default project to: ", name);
  return {};
} // set() }}}

// del() {{{
[[nodiscard]] inline std::expected<void,std::string> del(std::string_view str_name) noexcept
{
  // Open build db
  auto db_build = ns_db::ns_build::read();
  qreturn_if(not db_build, std::unexpected("Could not open build database"));
  // Find project
  auto it = std::ranges::find_if(db_build->projects
    , [&](auto&& e){ return e.name == str_name; }
  );
  qreturn_if(it == std::ranges::end(db_build->projects)
    , std::unexpected("Project '{}' not found to delete"_fmt(str_name))
  );
  // Erase project files from disk
  lec(fs::remove_all, it->path_dir_project_root);
  lec(fs::remove, it->path_dir_project_root.string() + ".layer");
  // Erase project from database
  db_build->projects.erase(it);
  // If is current project, blank it out
  if ( db_build->project == str_name ) { db_build->project = ""; } // if
  ns_db::ns_build::write(*db_build);
  return {};
} // del() }}}

} // namespace ns_project

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
