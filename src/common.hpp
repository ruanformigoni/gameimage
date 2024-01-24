///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : common
///

#pragma once

#include <exception>
#include <cstdlib>
#include <fmt/ranges.h>
#include <sstream>
#include <typeinfo>
#include <boost/type_index.hpp>

#include "std/concepts.hpp"

const char * GIMG_PATH_JSON_FETCH = "/tmp/gameimage/json";

// ns_common {{{
namespace ns_common
{
template<typename T>
std::string to_string(T&& t);
} // namespace ns_common }}}

// User defined literals {{{
// Format strings with user-defined literals
decltype(auto) operator ""_fmt(const char* str, size_t)
{
  return [str]<typename... Args>(Args&&... args)
  {
    return fmt::format(fmt::runtime(str), ns_common::to_string(std::forward<Args>(args))...);
  };
} // }}}

// User defined literals {{{
// Format strings with user-defined literals
decltype(auto) operator ""_throw(const char* str, size_t)
{
  return [str]<typename... Args>(Args&&... args)
  {
    throw std::runtime_error(fmt::format(fmt::runtime(str), ns_common::to_string(std::forward<Args>(args))...));
  };
} // }}}

// ns_common {{{

namespace ns_common
{

// to_string() {{{
template<typename T>
std::string to_string(T&& t)
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
  
  // Throw
  "Cannot convert '{}' to a valid string"_throw(boost::typeindex::type_id_with_cvr<T>().pretty_name());

  // Suppress compiler warning of no return, it throws before this
  return {};
} // to_string() }}}

} // namespace ns_common}}}


/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
