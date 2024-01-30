///
///@author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
///@file        : project
///

#pragma once

#include <string>

#include "../lib/json.hpp"

namespace ns_project
{

// set() {{{
inline void set(std::string const& s)
{
  // Tries to open default config file
  ns_json::Json json = ns_json::from_file_default();

  // Check if exists or throw
  json.contains<true>(s);

  // Updates default
  json("project") = s;

  // Tries to write back to default config file
  ns_json::to_file_default(json);
} // set() }}}

// get() {{{
inline std::string get()
{
  // Tries to open default config file
  ns_json::Json json = ns_json::from_file_default();

  // Returns default
  return json["project"];
} // get() }}}

} // namespace ns_project

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
