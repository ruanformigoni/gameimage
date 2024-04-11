///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : path
///

#pragma once

#include <filesystem>
#include <numeric>
#include <boost/dll.hpp>
#include <boost/filesystem.hpp>

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

// ends_with() {{{
// Check if path ends_with components of path_sub
// Returns the initial parts of the src path
template<bool _throw> 
Ret<fs::path> ends_with(fs::path const& std_path_src, fs::path const& std_path_sub)
{
  boost::filesystem::path path_src{std_path_src.string()};
  boost::filesystem::path path_sub{std_path_sub.string()};

  // Check component cound
  if ( path_src.size() < path_sub.size() )
  {
    ns_log::write('d', "SrcPath: ", path_src);
    ns_log::write('d', "Subpath: ", path_sub);
    return should_throw<_throw, fs::path>("Subpath has more components than SrcPath");
  } // if

  // Consume sub_path
  auto pair_it = std::mismatch(path_src.rbegin()
    , path_src.rend()
    , path_sub.rbegin()
    , path_sub.rend()
  );

  // Consumed all path_sub, path_src contains subpath
  if ( pair_it.second == path_sub.rend() )
  {
    // Get number of componets to use in resulting path
    auto components = std::distance(pair_it.first, path_src.rend());
    // Create output path
    auto path_src_slice = std::accumulate(path_src.begin()
      , std::next(path_src.begin(), components)
      , boost::filesystem::path{}, std::divides{}
    );
    // Return as std::filesystem::path
    return Ret(fs::path(path_src_slice.string()), true, "");
  } // if

  return Ret(fs::path{}, false, "");
} // function: ends_with }}}

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

// file_self() {{{
template<bool _throw> 
Ret<fs::path> file_self()
{
  boost::dll::fs::error_code err;
  fs::path path_file_self = boost::dll::program_location(err).c_str();

  if ( err )
  {
    return should_throw<_throw, fs::path>("Failed to fetch location of self");
  } // if

  return Ret(canonical<_throw>(path_file_self)._ret, true, "");
} // file_self() }}}

// dir_self() {{{
template<bool _throw> 
Ret<fs::path> dir_self()
{
  return Ret(file_self<_throw>()._ret.parent_path(), true, "");
} // dir_self() }}}

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

// dir_create() {{{
// Creates directories in 'path' if not exists
template<bool _throw> 
Ret<fs::path> dir_create(fs::path path_dir)
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

// dir_parent_exists() {{{
// Parent file exists and is a directory
template<bool _throw> 
Ret<fs::path> dir_parent_exists(fs::path path)
{
  path = dir_exists<_throw>(canonical<_throw>(path.parent_path())._ret)._ret
      /= file_name<_throw>(path)._ret;

  return Ret(path, true, "");
} // dir_parent_exists() }}}

// dir_executable() {{{
template<bool _throw> 
inline Ret<fs::path> dir_executable() {
  char result[PATH_MAX];

  // Read self path
  ssize_t count = readlink("/proc/self/exe", result, PATH_MAX);

  // Check read if was successful
  if (count <= 0)
  {
    return should_throw<_throw, fs::path>("Could not find path to self");
  }

  // Create parent path
  return Ret(fs::path{result}.parent_path(), true, "");
} // dir_executable() }}}

} // namespace ns_path }}}

} // namespace ns_fs

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
