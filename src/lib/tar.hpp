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
#include <numeric>

#include "../lib/log.hpp"
#include "../std/filesystem.hpp"

namespace ns_tar
{

namespace fs = std::filesystem;

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

// struct Opts {{{
struct Opts
{
  uint32_t strip_components = 0;
  std::optional<std::string> const& opt_path_file_to_extract = std::nullopt;
  std::optional<std::string> const& opt_path_dir_to_extract = std::nullopt;
};
// }}}

// find() {{{
// Finds queried path inside the tarball
// Returns the parent path
// Example:
//   tarball contains: retro/file/hello/world.bin
//   query contains  : hello/world.bin
//   returns         : retro/file
inline std::optional<fs::path> find(fs::path const& path_file_tarball, fs::path const& path_query)
{
  struct archive *a;
  struct archive_entry *entry;
  int r;

  // Create new archive
  a = archive_read_new();

  // Enable all formats
  archive_read_support_filter_all(a);
  archive_read_support_format_all(a);

  // Read file
  r = archive_read_open_filename(a, path_file_tarball.c_str(), 10240);

  // Check for errors
  if (r != ARCHIVE_OK)
  {
    ns_log::write('e', "Failed to open archive ", path_file_tarball);
    return {};
  } // if

  // Read entry from archive
  while (archive_read_next_header(a, &entry) == ARCHIVE_OK)
  {
    fs::path path_entry = archive_entry_pathname(entry);
    if ( auto ret = ns_fs::ns_path::ends_with<false>(path_entry, path_query); ret._bool )
    {
      return ret._ret;
    } // if
    archive_read_data_skip(a);
  } // while

  // Free archive
  r = archive_read_free(a);

  // Check for errors
  if (r != ARCHIVE_OK)
  {
    ns_log::write('e', "Failed to close archive ", path_file_tarball);
    return {};
  } // if

  return std::nullopt;
} // find() }}}

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
inline void extract(std::string const& path_file_archive, Opts const& opts = {})
{
  auto f_check_error = [&](int r, struct archive * a)
  {
    if (r < ARCHIVE_OK)
    {
      std::string string_error = archive_error_string(a);
      ns_log::write('e', string_error);
      throw std::runtime_error(string_error);
    } // if

    if (r < ARCHIVE_WARN)
    {
      ns_log::write('e', archive_error_string(a));
    } // if
  };

  struct archive *a;
  struct archive *ext;
  struct archive_entry *entry;
  int flags;
  int r;

  // Select which attributes we want to restore
  flags = ARCHIVE_EXTRACT_TIME | ARCHIVE_EXTRACT_PERM | ARCHIVE_EXTRACT_ACL | ARCHIVE_EXTRACT_FFLAGS;

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

    // Check for eror in header
    f_check_error(r, a);

    // Only extract specified file
    if ( fs::path path_entry = archive_entry_pathname(entry);
      opts.opt_path_file_to_extract.has_value() && path_entry != *opts.opt_path_file_to_extract )
    {
      ns_log::write('d', "Ignoring file '", path_entry, "'");
      continue;
    } // entry

    // Remove initial components to extract into
    if ( opts.strip_components > 0 )
    {
      fs::path path_entry = archive_entry_pathname(entry);

      auto distance = std::distance(path_entry.begin(), path_entry.end());

      // Check if can strip
      if ( opts.strip_components >= distance ) { continue; }

      // Create stripped path
      path_entry = std::accumulate(std::next(path_entry.begin(), opts.strip_components), path_entry.end(), fs::path{}, std::divides{});

      // Check if path is still valid
      if ( path_entry.empty() ) { continue; }

      // Prepend target path if was specified
      if ( opts.opt_path_dir_to_extract.has_value() )
      {
        path_entry = *opts.opt_path_dir_to_extract / path_entry;
      } // if

      // Set new path to entry
      archive_entry_set_pathname(entry, path_entry.c_str());
    } // entry

    // Write file header
    r = archive_write_header(ext, entry);
    // Check for error in write header
    f_check_error(r, ext);

    // Write file data
    if (archive_entry_size(entry) > 0)
    {
      // copy_data(from, to)
      r = copy_data(a, ext);
      // Check write error
      f_check_error(r, ext);
    } // else if

    // Write out padding required by some formats
    r = archive_write_finish_entry(ext);
    // Check error
    f_check_error(r, ext);

  } // for
  archive_read_close(a);
  archive_read_free(a);
  archive_write_close(ext);
  archive_write_free(ext);
} // extract() }}}

} // namespace ns_tar

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
