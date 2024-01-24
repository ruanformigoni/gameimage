///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : fifo
///

#pragma once

#include <cstdio>
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>
#include <cstdlib>
#include <filesystem>

#include "../common.hpp"

namespace fifo
{

namespace fs = std::filesystem;

// create() {{{
// Create a FIFO for writing progress data
inline void create(const char* str_name)
{
  // Check if already exists
  if (struct stat buf; stat(str_name, &buf) == 0 && S_ISFIFO(buf.st_mode) )
  {
    return;
  } // if

  // Create upper dirs
  auto path = fs::path(str_name);
  auto path_parent = path.parent_path();
  if ( ! fs::exists(path_parent) )
  {
    if ( ! fs::create_directories(path_parent) )
    {
      "Failed to create upper directories for '{}'"_throw(str_name);
    }
  }

  // Remove the FIFO if it already exists
  unlink(str_name);

  // Create a new FIFO
  if (mkfifo(str_name, 0666) != 0) {
    perror("mkfifo");
    "mkfifo failed for '{}'"_throw(str_name);
  }
} // create() }}}

// push() {{{
inline void push(const char* str_name, std::string data)
{
  if (int fd = open(str_name, O_WRONLY | O_NONBLOCK); fd != -1)
  {
    write(fd, data.c_str(), data.size());
    close(fd);
  } // if
} // push() }}}

} // namespace fifo

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
