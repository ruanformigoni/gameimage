///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : zip
///

#pragma once

#include "../std/filesystem.hpp"

#include <libzippp/libzippp.h>

namespace fs = std::filesystem;

namespace ns_zip
{

// list_regular_files() {{{
inline std::vector<fs::path> list_regular_files(fs::path const& path_file_zip)
{
  std::vector<fs::path> ret;

  libzippp::ZipArchive zf(path_file_zip);

  // Open zip file
  if ( not zf.open(libzippp::ZipArchive::ReadOnly) )
  {
    "Failed to open zip file"_throw(path_file_zip);
  } // if

  std::vector<libzippp::ZipEntry> entries = zf.getEntries();

  for(auto it{entries.begin()}; it != entries.end(); ++it)
  {
    libzippp::ZipEntry entry = *it;

    if (entry.isDirectory()) { continue; } // if
    else { ret.push_back(entry.getName()); } // else
  } // for
  
  // Close file
  zf.close();

  return ret;
} // list_regular_files() }}}

// extract() {{{
inline void extract(fs::path const& path_file_zip, fs::path const& path_dir_out)
{
  libzippp::ZipArchive zf(path_file_zip);

  // Open zip file
  if ( not zf.open(libzippp::ZipArchive::ReadOnly) )
  {
    "Failed to open zip file"_throw(path_file_zip);
  } // if

  ns_log::write('i', "Extracting ", path_file_zip);

  std::vector<libzippp::ZipEntry> entries = zf.getEntries();

  for(auto it{entries.begin()}; it != entries.end(); ++it)
  {
    libzippp::ZipEntry entry = *it;

    fs::path path_file_out = path_dir_out / entry.getName();

    if (entry.isDirectory())
    {
      // Create parent dirs
      ns_fs::ns_path::dir_create<true>(path_file_out);
    } // if
    else
    {
      // Create parent dirs
      ns_fs::ns_path::dir_create<true>(path_file_out.parent_path());

      // Open file to write
      std::ofstream out_file(path_file_out, std::ios::binary);

      // Write to file from zip
      entry.readContent(out_file);

      // Close file
      out_file.close();
    } // else

  } // for
  
  // Close file
  zf.close();
} // extract() }}}
 
} // namespace ns_zip

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
