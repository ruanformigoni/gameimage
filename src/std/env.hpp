///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : env
///

#include <filesystem>
#include <cstdlib>

#include "../common.hpp"

#pragma once

// Environment variable handling {{{
namespace ns_env
{

namespace fs = std::filesystem;

// enum class Replace {{{
enum class Replace
{
  Y,
  N,
}; // enum class Replace }}}

// dir() {{{
// Fetches a directory path from an environment variable
fs::path dir(const char* name)
{
  // Get environment variable
  const char * value = std::getenv(name) ;
  // Check if variable exists
  if ( ! value )
  {
    "Variable {} not set"_throw(name);
  } // if
  // Create filesystem path
  fs::path path{value};
  // Check if path exists
  if ( ! fs::exists(path) )
  {
    // Try to create destination directory if not exists
    if ( ! fs::create_directories(path) )
    {
      "Could not create directory '{}'"_throw(value);
    }
  }
  // Return validated directory path
  return path;
} // dir() }}}

// set() {{{
// Sets an environment variable
void set(const char* name, const char* value, Replace replace)
{
  setenv(name, value, (replace == Replace::Y));
} // set() }}}

// concat() {{{
// Appends 'extra' to an environment variable 'name'
void concat(const char* name, std::string const& extra)
{
  // Append to var
  if ( const char* var_curr = std::getenv(name); var_curr )
  {
    setenv(name, std::string{var_curr + extra}.c_str(), 1);
  } // if
} // concat() }}}

} // namespace ns_env }}}

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
