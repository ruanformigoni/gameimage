///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : path
// @created     : Friday Jan 19, 2024 18:26:40 -03
///

#pragma once

#include <filesystem>

#include "../common.hpp"



namespace ns_fs
{

namespace fs = std::filesystem;

// namespace ns_path {{{
namespace ns_path
{

// struct Ret {{{
template<typename T>
struct Ret
{
  T _ret;
  bool _bool;
  std::string _msg;

  Ret(T _ret, bool _bool, std::string _msg)
    : _ret(_ret)
    , _bool(_bool)
    , _msg(_msg)
  {}
}; // struct: Ret 
// }}}

// should_throw() {{{
template<bool _throw, typename U, std::convertible_to<std::string> T>
Ret<U> should_throw(T&& t)
{
  if constexpr ( _throw )
  {
    "{}"_throw(t);
  } // if

  return Ret(U{}, false, t);
} // should_throw() }}}

// canonical() {{{
// Try to make path canonical
template<bool _throw> 
Ret<fs::path> canonical(fs::path const& path)
{
  fs::path ret{path};

  try
  {
    // Adjust for relative path
    if (not ret.string().starts_with("/"))
    {
      ret = fs::path(fs::path{"./"} /= ret);
    } // if
    ret = fs::canonical(ret);
  }
  catch(std::exception const& e)
  {
    return should_throw<_throw, fs::path>("Could not make cannonical path for parent of '{}'"_fmt(path));
  }

  return Ret(ret, true, "");
} // function: canonical }}}

// dir_create() {{{
// Creates directories in 'path' if not exists
template<bool _throw> 
Ret<fs::path> dir_create(fs::path const& path_dir)
{
  if( fs::exists(path_dir) && ! fs::is_directory(path_dir) )
  {
    return should_throw<_throw, fs::path>("Path '{}' exists and is not a directory"_fmt(path_dir));
  } // if

  if ( ! fs::exists(path_dir) && ! fs::create_directories(path_dir) )
  {
    return should_throw<_throw, fs::path>("Failed to create directory '{}'"_fmt(path_dir));
  } // if

  return Ret(canonical<_throw>(path_dir)._ret, true, "");
} // dir_create() }}}

// dir_exists() {{{
// File exists and is a directory
template<bool _throw> 
Ret<fs::path> dir_exists(fs::path path_dir)
{
  // Does not exist
  if ( ! fs::exists(path_dir) )
  {
    return should_throw<_throw, fs::path>("Directory '{}' does not exist"_fmt(path_dir));
  } // if

  // Exists, but is not dir
  if ( ! fs::is_directory(path_dir) )
  {
    return should_throw<_throw, fs::path>("Path '{}' exists but is not a directory"_fmt(path_dir));
  } // if

  return Ret(canonical<_throw>(path_dir)._ret, true, "");
} // dir_exists() }}}

// file_exists() {{{
// File exists and is a regular file
template<bool _throw> 
Ret<fs::path> file_exists(fs::path path_file)
{
  // Does not exist
  if ( ! fs::exists(path_file) )
  {
    return should_throw<_throw, fs::path>("File '{}' does not exist"_fmt(path_file));
  } // if

  // Exists, but is not dir
  if ( ! fs::is_regular_file(path_file) )
  {
    return should_throw<_throw, fs::path>("Path '{}' exists but is not a regular file"_fmt(path_file));
  } // if

  return Ret(canonical<_throw>(path_file)._ret, true, "");
} // file_exists() }}}

// file_name() {{{
// Check if last component is a file name
template<bool _throw> 
Ret<std::string> file_name(fs::path const& path)
{
  std::string str_name = path.filename().string();

  if ( str_name.empty() or str_name == "." or str_name == ".." )
  {
    return should_throw<_throw, std::string>("Empty file name for path {}"_fmt(path));
  }

  return Ret(str_name, true, "");
} // function: file_name }}}

} // namespace ns_path }}}

} // namespace ns_fs

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
