///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : copy
// @created     : Friday Jan 19, 2024 19:36:20 -03
///

#pragma once

#include <filesystem>
#include <fstream>
#include <concepts>
#include <functional>

#include "../common.hpp"


namespace ns_copy
{

namespace fs = std::filesystem;

// file() {{{
template<typename F = std::function<void(double,fs::path,fs::path)> >
inline void file(fs::path const& path_src
  , fs::path const& path_dst
  , F&& f_progress_callback = [](auto&&,auto&&,auto&&){})
{
  std::ifstream file_src(path_src, std::ios::binary);
  std::ofstream file_dst(path_dst, std::ios::binary);

  if ( ! file_src.good() )
  {
    "Could not open file for read: '{}'"_throw(path_src);
  } // if

  if ( ! file_dst.good() )
  {
    "Could not open file for write: '{}'"_throw(path_dst);
  } // if

  // Calculate file size
  file_src.seekg(0, std::ios::end);
  size_t file_size = file_src.tellg();
  file_src.seekg(0, std::ios::beg);

  // Create buffer
  size_t const size_buffer = 1024;
  char buffer[size_buffer];

  // Keep track of total copied data
  size_t size_bytes_copied = 0;

  while (file_src.read(buffer, size_buffer) || file_src.gcount() != 0)
  {
    file_dst.write(buffer, file_src.gcount());
    size_bytes_copied += file_src.gcount();

    double progress = static_cast<double>(size_bytes_copied) / file_size;

    // 100% handler after the loop
    if ( progress != 1.0 )
    {
      f_progress_callback(progress, path_src, path_dst);
    } // if
  } // while

  // Ending callback
  f_progress_callback(1.0, path_src, path_dst);

  // Make file executable
  using std::filesystem::perms;
  fs::permissions(path_dst, perms::owner_all | perms::group_all | perms::others_read);
} // file() }}}

// callback_seconds {{{
template<typename F>
decltype(auto) callback_seconds(std::chrono::seconds seconds, F&& f)
{
  return [=,time_ref = std::chrono::steady_clock::now()] (double percentage, fs::path src, fs::path dst) mutable
  {
    auto time_now = std::chrono::steady_clock::now();
    if (std::chrono::duration_cast<std::chrono::seconds>(time_now - time_ref) >= seconds or percentage == 1.0)
    {
      f(percentage, src, dst);
      time_ref = time_now;
    } // if
  };
} // function: callback_seconds }}}

} // namespace ns_copy

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
