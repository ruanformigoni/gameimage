///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : env
///

#include <filesystem>
#include <cstdlib>

#include "../common.hpp"

#include "../std/filesystem.hpp"

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
// Tries to create if not exists
inline fs::path dir(const char* name)
{
  // Get environment variable
  const char * value = std::getenv(name) ;
  // Check if variable exists
  if ( ! value )
  {
    "Variable {} not set"_throw(name);
  } // if

  // Create if not exists
  fs::path path_env = ns_fs::ns_path::dir_exists<true>(value)._ret;

  // Log
  // ns_log::write('d', "ns_env::dir ", name, " -> ", path_env);

  // Return validated directory path
  return path_env;
} // dir() }}}

// file() {{{
// Fetches a directory path from an environment variable
// Tries to create if not exists
inline fs::path file(const char* name)
{
  // Get environment variable
  const char * value = std::getenv(name) ;

  // Check if variable exists
  if ( ! value )
  {
    "Variable {} not set"_throw(name);
  } // if

  // Create if not exists
  fs::path path_env = ns_fs::ns_path::file_exists<true>(value)._ret;

  // Log
  // ns_log::write('d', "ns_env::file ", name, " -> ", path_env);

  // Return validated directory path
  return path_env;
} // file() }}}

// set() {{{
// Sets an environment variable
template<ns_concept::AsString T, ns_concept::AsString U>
void set(T&& name, U&& value, Replace replace)
{
  // ns_log::write('d', "ns_env::set ", name, " -> ", value);
  setenv(ns_string::to_string(name).c_str(), ns_string::to_string(value).c_str(), (replace == Replace::Y));
} // set() }}}

// concat() {{{
// Appends 'extra' to an environment variable 'name'
inline void concat(const char* name, std::string const& extra)
{
  // Append to var
  if ( const char* var_curr = std::getenv(name); var_curr )
  {
    // ns_log::write('d', "ns_env::concat ", name, " -> ", extra);
    setenv(name, std::string{var_curr + extra}.c_str(), 1);
  } // if
} // concat() }}}

// get() {{{
// Get an env variable if exists
inline const char* get(const char* name)
{
  const char* ret = std::getenv(name);

  // ns_log::write('d', "ns_env::get ", name, " -> ", ret);

  return ret;
} // get() }}}

// get_or_throw() {{{
// Get an env variable
template<typename T = const char*>
inline T get_or_throw(const char* name)
{
  const char* value = std::getenv(name);

  if (not value)
  {
    throw std::runtime_error("Variable '{}' is undefined"_fmt(name));
  } // if

  return value;
} // get_or_throw() }}}

} // namespace ns_env }}}

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
