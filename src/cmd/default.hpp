///
///@author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
///@file        : default
///

#pragma once

#include <string>

#include "../lib/json.hpp"

namespace ns_default
{

// set() {{{
void set(std::string const& s)
{
  // Tries to open default config file
  ns_json::Json json = ns_json::from_default_file();

  // Updates default
  json["default"] = s;

  // Tries to write back to default config file
  ns_json::to_default_file(json);
} // set() }}}

// get() {{{
std::string get()
{
  // Tries to open default config file
  ns_json::Json json = ns_json::from_default_file();

  // Returns default
  return json["default"];
} // get() }}}

} // namespace ns_default

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
