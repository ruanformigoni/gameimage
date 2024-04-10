///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : fifo
///

#pragma once

#include <cstdio>
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>
#include <fcntl.h>
#include <filesystem>

#include "../std/filesystem.hpp"

#include "../common.hpp"

namespace ns_fifo
{

namespace fs = std::filesystem;

// class Fifo {{{
class Fifo
{
  private:
    int m_fd;
    fs::path m_path_file_fifo;
  
  public:
    template<ns_concept::AsString T>
    Fifo(T&& t);
    ~Fifo();
    template<ns_concept::AsString T>
    void push(T&& t);
}; // class: Fifo }}}

// Fifo::Fifo {{{
template<ns_concept::AsString T>
Fifo::Fifo(T&& t)
{
  m_path_file_fifo = ns_string::to_string(t);
  auto cstr_path_file_fifo = m_path_file_fifo.c_str();

  // Remove the FIFO if it already exists
  if (struct stat buf; stat(cstr_path_file_fifo, &buf) == 0 && S_ISFIFO(buf.st_mode) )
  {
    unlink(cstr_path_file_fifo);
  } // if

  // Create upper dirs
  if ( not ns_fs::ns_path::dir_create<false>(fs::path(cstr_path_file_fifo).parent_path())._bool )
  {
    "Failed to create upper directories for '{}'"_throw(cstr_path_file_fifo);
  } // if

  // Create a new FIFO
  if (mkfifo(cstr_path_file_fifo, 0666) != 0)
  {
    perror("mkfifo");
    "mkfifo failed for '{}'"_throw(cstr_path_file_fifo);
  } // if

  // Open fifo as writer
  if (m_fd = open(cstr_path_file_fifo, O_WRONLY | O_NONBLOCK); m_fd == -1 )
  {
    "Could not open fifo file descriptor for writing"_throw();
    return;
  } // if
} // }}}

// Fifo::~Fifo {{{
inline Fifo::~Fifo()
{
  close(m_fd);
  fs::remove(m_path_file_fifo);
} // }}}

// push() {{{
template<ns_concept::AsString T>
void Fifo::push(T&& t)
{
  auto data = ns_string::to_string(t);
  auto count_bytes_written = write(m_fd, data.c_str(), data.size());
  if ( count_bytes_written == -1 )
  {
    ns_log::write('e', "Failed to write data '", data, "' to fifo");
  } // if
} // push() }}}

} // namespace ns_fifo

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
