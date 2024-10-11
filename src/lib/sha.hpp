///
/// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
/// @file        : sha
///

#pragma once

#include <fstream>
#include <filesystem>
#include <cryptopp/files.h> // Include for SHA256 and SHA512
#include <cryptopp/sha.h> // Include for SHA256 and SHA512
#include <cryptopp/filters.h> // Include for StringSink and HashFilter
#include <cryptopp/hex.h> // Include for HexEncoder

#include "../common.hpp"
#include "../macro.hpp"
#include "../std/exception.hpp"

namespace fs = std::filesystem;

namespace ns_sha
{

enum class SHA_TYPE
{
  SHA256,
  SHA512
};

namespace
{

// check_sha_impl() {{{
inline bool check_sha_impl(fs::path path_file_src, fs::path path_file_sha, SHA_TYPE sha_type = SHA_TYPE::SHA256)
{
  std::ifstream file_src(path_file_src, std::ifstream::binary);
  std::ifstream file_sha(path_file_sha, std::ifstream::in);

  if (!file_src.good()) { "Cannot open file '{}' "_throw(path_file_src); }
  if (!file_sha.good()) { "Cannot open file '{}' "_throw(path_file_sha); }

  ns_log::write('i', "Calculating SHA for: ", path_file_src);

  std::string sha_calculated;

  // Calculated SHA
  if ( sha_type == SHA_TYPE::SHA256 )
  {
    CryptoPP::SHA256 hash;
    CryptoPP::FileSource(file_src, true, new CryptoPP::HashFilter(hash, new CryptoPP::HexEncoder(new CryptoPP::StringSink(sha_calculated))));
  } // if
  else
  {
    CryptoPP::SHA512 hash;
    CryptoPP::FileSource(file_src, true, new CryptoPP::HashFilter(hash, new CryptoPP::HexEncoder(new CryptoPP::StringSink(sha_calculated))));
  } // else

  // Reference SHA
  std::string sha_reference;
  std::getline(file_sha, sha_reference);
  if ( sha_reference.find(' ') != std::string::npos )
  {
    sha_reference.erase(sha_reference.find(' '));
  } // if

  // Normalize to uppercase
  sha_calculated = ns_string::to_upper(sha_calculated);
  sha_reference = ns_string::to_upper(sha_reference);

  ns_log::write('i', "SHA Calculated: ", sha_calculated);
  ns_log::write('i', "SHA Reference : ", sha_reference);

  return sha_calculated == sha_reference;
} // check_sha_impl() }}}

} // namespace

// check_sha() {{{
inline bool check_sha(fs::path path_file_src, fs::path path_file_sha, SHA_TYPE sha_type = SHA_TYPE::SHA256)
{
  auto expected_check = ns_exception::to_expected([&]{ return check_sha_impl(path_file_src, path_file_sha, sha_type); });
  ereturn_if(not expected_check, expected_check.error(), false);
  return *expected_check;
} // check_sha() }}}
  
} // namespace ns_sha

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
