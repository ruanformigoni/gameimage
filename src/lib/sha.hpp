///
/// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
/// @file        : sha
///

#pragma once

#include <fstream>
#include <filesystem>
#include <sha256.h>

#include "../common.hpp"

namespace fs = std::filesystem;

namespace ns_sha
{

// check_256sum() {{{
inline bool check_256sum(fs::path path_file_src, fs::path path_file_sha)
{
  std::ifstream file_src(path_file_src, std::ifstream::binary);
  std::ifstream file_sha(path_file_sha, std::ifstream::in);

  if (!file_src.good()) { "Cannot open file '{}' "_throw(path_file_src); }
  if (!file_sha.good()) { "Cannot open file '{}' "_throw(path_file_sha); }

  ns_log::write('i', "Calculating SHA for: ", path_file_src);

  SHA256 sha256;
  char buffer[16384];
  while (file_src.read(buffer, sizeof(buffer)) || file_src.gcount())
  {
    sha256.add(buffer, file_src.gcount());
  } // while
  std::string sha256_calculated = sha256.getHash();


  std::string sha256_reference;

  std::getline(file_sha, sha256_reference);

  if ( sha256_reference.find(' ') != std::string::npos )
  {
    sha256_reference.erase(sha256_reference.find(' '));
  } // if

  ns_log::write('i', "Calculated SHA for: ", path_file_src);
  ns_log::write('i', "Generated SHA: ", sha256_calculated);
  ns_log::write('i', "Reference SHA: ", sha256_reference);

  return sha256_calculated == sha256_reference;
} // check_256sum() }}}
  
} // namespace ns_sha

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
