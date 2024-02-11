///
///@author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
///@file        : project
///

#pragma once

#include <string>

#include "../lib/db.hpp"

namespace ns_project
{

// set() {{{
inline void set(std::string const& s)
{
  // Tries to open default config file
  ns_db::from_file_default([&](auto&& db)
  {
    // Check if exists or throw
    db.template contains<true>(s);

    // Updates default
    db("project") = s;

    ns_log::write('i', "Set default project to: ", s);
  }
  , std::ios_base::out);
} // set() }}}

// get() {{{
inline std::string get()
{
  std::string ret;

  // Get project from database
  ns_db::from_file_default([&](auto&& db)
  {
    ret = db["project"];
  }
  , std::ios_base::in);

  ns_log::write('i', "Default project: ", ret);

  // Returns default
  return ret;
} // get() }}}

} // namespace ns_project

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
