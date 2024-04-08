///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : string
///

#pragma once

#include <algorithm>
#include <sstream>
#include <boost/type_index.hpp>
#include <fmt/core.h>
#include <boost/algorithm/string.hpp>

#include "concepts.hpp"

namespace ns_string
{

// replace_substrings() {{{
inline std::string replace_substrings(std::string string
  , std::string const& substring
  , std::string const& replacement)
{
  boost::algorithm::replace_all(string, substring, replacement);
  return string;
} // replace_substrings()  }}}

// to_string() {{{
template<typename T>
inline std::string to_string(T&& t)
{
  if constexpr ( ns_concept::StringConvertible<T> )
  {
    return t;
  } // if
  else if constexpr ( ns_concept::StringConstructible<T> )
  {
    return std::string{t};
  } // else if
  else if constexpr ( ns_concept::Numeric<T> )
  {
    return std::to_string(t);
  } // else if 
  else if constexpr ( ns_concept::StreamInsertable<T> )
  {
    std::stringstream ss;
    ss << t;
    return ss.str();
  } // else if 
  
  throw std::runtime_error(fmt::format("Cannot convert '{}' to a valid string"
    , boost::typeindex::type_id_with_cvr<T>().pretty_name())
  );
} // to_string() }}}

// to_lower() {{{
template<ns_concept::AsString T>
std::string to_lower(T&& t)
{
  std::string ret = to_string(t);
  std::ranges::for_each(ret, [](auto& c){ c = std::tolower(c); });
  return ret;
} // to_lower() }}}

// to_upper() {{{
template<ns_concept::AsString T>
std::string to_upper(T&& t)
{
  std::string ret = to_string(t);
  std::ranges::for_each(ret, [](auto& c){ c = std::toupper(c); });
  return ret;
} // to_upper() }}}

// from_container() {{{
template<typename T>
std::string from_container(T&& t, char sep = ' ')
{
  std::stringstream ret;
  for( auto it = t.begin(); it != t.end(); ++it )
  {
    ret << *it;
    if ( std::next(it) != t.end() ) { ret << sep; }
  } // if
  return ret.str();
} // from_container() }}}

} // namespace ns_string

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
