// Source adapted from:
// https://github.com/libarchive/libarchive/wiki/Examples#user-content-A_Universal_Decompressor

#pragma once

#include <sys/types.h>

#include <sys/stat.h>

#include <archive.h>
#include <archive_entry.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "../lib/log.hpp"

namespace ns_tar
{

namespace 
{

// copy_data() {{{
inline int copy_data(struct archive *ar, struct archive *aw)
{
  int r;
  const void *buff;
  size_t size;
  la_int64_t offset;

  for (;;)
  {
    r = archive_read_data_block(ar, &buff, &size, &offset);

    if (r == ARCHIVE_EOF)
    {
      return (ARCHIVE_OK);
    } // if
    if (r < ARCHIVE_OK)
    {
      return (r);
    } // if
    r = archive_write_data_block(aw, buff, size, offset);
    if (r < ARCHIVE_OK)
    {
      ns_log::write('e', archive_error_string(aw));
      return (r);
    }
  } // for
} // copy_data() }}}

} // namespace 

// list() {{{
inline std::vector<std::string> list(const char* filename)
{
  std::vector<std::string> ret;

  struct archive *a;
  struct archive_entry *entry;
  int r;

  // Create new archive
  a = archive_read_new();

  // Enable all formats
  archive_read_support_filter_all(a);
  archive_read_support_format_all(a);

  // Read file
  r = archive_read_open_filename(a, filename, 10240);

  // Check for errors
  if (r != ARCHIVE_OK)
  {
    ns_log::write('e', "Failed to open archive ", filename);
    return {};
  } // if

  // Read entry from archive
  while (archive_read_next_header(a, &entry) == ARCHIVE_OK)
  {
    ret.push_back(archive_entry_pathname(entry));
    archive_read_data_skip(a);
  }

  // Free archive
  r = archive_read_free(a);

  // Check for errors
  if (r != ARCHIVE_OK)
  {
    ns_log::write('e', "Failed to close archive ", filename);
    return {};
  } // if

  return ret;
} // list() }}}

// extract() {{{
inline void extract(std::string const& path_file_archive, std::string const& path_file_to_extract)
{
  struct archive *a;
  struct archive *ext;
  struct archive_entry *entry;
  int flags;
  int r;

  /* Select which attributes we want to restore. */
  flags = ARCHIVE_EXTRACT_TIME;
  flags |= ARCHIVE_EXTRACT_PERM;
  flags |= ARCHIVE_EXTRACT_ACL;
  flags |= ARCHIVE_EXTRACT_FFLAGS;

  // Source archive
  a = archive_read_new();
  archive_read_support_format_all(a);
  archive_read_support_filter_all(a);

  // Write to disk
  ext = archive_write_disk_new();
  archive_write_disk_set_options(ext, flags);
  archive_write_disk_set_standard_lookup(ext);

  // Open archive
  if ((r = archive_read_open_filename(a, path_file_archive.c_str(), 10240)))
  {
    ns_log::write('i', "Failed to open archive", path_file_archive, "'");
    return;
  } // if

  // Read files & extract match
  for (;;)
  {
    r = archive_read_next_header(a, &entry);

    if (r == ARCHIVE_EOF)
    {
      break;
    } // if

    if (r < ARCHIVE_OK)
    {
      ns_log::write('e', archive_error_string(a));
    } // if

    if (r < ARCHIVE_WARN)
    {
      return;
    } // if

    if ( std::string str_entry = archive_entry_pathname(entry); str_entry != path_file_to_extract )
    {
      ns_log::write('i', "Ignoring file '", str_entry, "'");
      continue;
    } // entry

    r = archive_write_header(ext, entry);

    if (r < ARCHIVE_OK)
    {
      ns_log::write('e', archive_error_string(ext));
    } // if
    else if (archive_entry_size(entry) > 0)
    {
      r = copy_data(a, ext);
      if (r < ARCHIVE_OK)
      {
        ns_log::write('e', archive_error_string(ext));
        return;
      }
      if (r < ARCHIVE_WARN)
      {
        return;
      } // if
    } // else if

    r = archive_write_finish_entry(ext);

    if (r < ARCHIVE_OK)
    {
      ns_log::write('e', archive_error_string(ext));
    } // if

    if (r < ARCHIVE_WARN)
    {
      return;
    } // if
  } // for
  archive_read_close(a);
  archive_read_free(a);
  archive_write_close(ext);
  archive_write_free(ext);
} // extract() }}}

} // namespace ns_tar

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
