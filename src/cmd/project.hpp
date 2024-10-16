///
///@author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
///@file        : project
///

#pragma once

#include <string>

#include "../lib/db/build.hpp"

namespace ns_project
{

// set() {{{
inline void set(std::string const& name)
{
  // Open db
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Could not open build database");
  // Check if 'name' is present in db
  "Could not find project '{}'"_try([&]{ (void) db_build->find(name); }, name);
  // Set as current
  db_build->project = name;
  ns_db::ns_build::write(*db_build);
  ns_log::write('i', "Set default project to: ", name);
} // set() }}}

// get() {{{
inline std::string get()
{
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Could not open build database");
  return db_build->project;
} // get() }}}

} // namespace ns_project

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
