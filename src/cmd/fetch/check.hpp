///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : check
///

#pragma once

#include <cpr/cpr.h>

#include "../../lib/log.hpp"
#include "../../lib/sha.hpp"

namespace ns_fetch::ns_check
{

namespace
{

namespace fs = std::filesystem;

// check_file_from_sha() {{{
inline bool check_file_from_sha(fs::path const& path_file_src
    , fs::path const& path_file_sha
    , ns_sha::SHA_TYPE const& sha_type)
{
  return ns_sha::check_sha(path_file_src, path_file_sha, sha_type);
} // }}}

// check_file_from_url_impl() {{{
inline bool check_file_from_url_impl(fs::path const& path_file_src, cpr::Url const& url)
{
  uintmax_t size_reference = fs::file_size(path_file_src);
  uintmax_t size_calculated = 0;

  ns_log::write('i', "SIZE: Reference is ", size_reference);

  // Get size of file to download
  cpr::Response response_head = cpr::Head(url);
  if ( response_head.status_code != 200 )
  {
    ns_log::write('e', "Could not fetch remote size to compare local size with");
    return false;
  } // if

  auto it = response_head.header.find("Content-Length");
  if (it == response_head.header.end())
  {
    ns_log::write('e', "Could not find field 'Content-Length' in response");
    return false;
  } // if

  size_calculated = std::stoi(it->second);
  ns_log::write('i', "SIZE: Calculated is ", size_calculated);

  if ( size_reference != size_calculated )
  {
    ns_log::write('e', "Size reference differs from size_calculated");
    return false;
  } // if

  return true;
} // check_file_from_url_impl() }}}

// check_file_from_url() {{{
inline bool check_file_from_url(fs::path const& path_file_src, cpr::Url const& url)
{
  auto expected_check = ns_exception::to_expected([&]{ return check_file_from_url_impl(path_file_src, url); });
  ereturn_if(not expected_check, expected_check.error(), false);
  return *expected_check;
} // check_file_from_url() }}}

} // namespace

// check_file() {{{
inline bool check_file(fs::path const& path_file_src
  , fs::path const& path_file_sha
  , ns_sha::SHA_TYPE const& sha_type
  , cpr::Url const& url)
{
  return check_file_from_sha(path_file_src, path_file_sha, sha_type) or check_file_from_url(path_file_src, url);
} // check_file() }}}

} // namespace ns_fetch::ns_check

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
