///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : string
///

#pragma once

#include <algorithm>
#include <sstream>

#include "concepts.hpp"

namespace ns_string
{


template<ns_concept::StreamInsertable T>
std::string to_lower(T&& t)
{
  std::stringstream ss;
  ss << t;
  std::string ret = ss.str();
  std::ranges::for_each(ret, [](auto& c){ c = std::tolower(c); });
  return ret;
} // to_lower

} // namespace ns_string

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
