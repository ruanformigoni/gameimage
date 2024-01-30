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

#include "lib/log.hpp"

const char * GIMG_PATH_JSON_FETCH = "/tmp/gameimage/json";

// ns_common {{{
namespace ns_common
{
template<typename T>
std::string to_string(T&& t);
} // namespace ns_common }}}

// User defined literals {{{
// Format strings with user-defined literals
inline decltype(auto) operator ""_fmt(const char* str, size_t)
{
  return [str]<typename... Args>(Args&&... args)
  {
    return fmt::format(fmt::runtime(str), ns_common::to_string(std::forward<Args>(args))... ) ;
  };
} // }}}

// class Exception {{{
class Exception : public std::exception
{
  private:
    std::string m_msg;

  public:
    Exception(std::string const& msg)
      : m_msg(msg)
    {
    } // Exception
    const char * what() const noexcept override
    {
      return m_msg.c_str();
    } // what()
}; // class: Exception }}}

// User defined literals {{{

// Format strings with user-defined literals
inline decltype(auto) operator ""_throw(const char* str, size_t)
{
  return [str]<typename... Args>(Args&&... args)
  {
    throw Exception(fmt::format(fmt::runtime(str), ns_common::to_string(std::forward<Args>(args))...));
  };
} 

// Format strings with user-defined literals, throws if condition is false
inline decltype(auto) operator ""_catch(const char* str, size_t)
{
  return [str]<typename F, typename... Args>(F&& f, Args&&... args)
  {
    try
    {
      f();
    } // try
    catch(std::exception const& e)
    {
      ns_log::write('i', fmt::format(fmt::runtime(str), ns_common::to_string(std::forward<Args>(args))...));
    } // catch
  };
}

// Format strings with user-defined literals, throws if condition is false
inline decltype(auto) operator ""_throw_if(const char* str, size_t)
{
  return [str]<typename F, typename... Args>(F&& f, Args&&... args)
  {
    if ( f() )
    {
      throw Exception(fmt::format(fmt::runtime(str), ns_common::to_string(std::forward<Args>(args))...));
    }
  };
}

// Format strings with user-defined literals, throws if condition is false
inline decltype(auto) operator ""_try(const char* str, size_t)
{
  return [str]<typename F, typename... Args>(F&& f, Args&&... args)
  {
    try
    {
      f();
    } // try
    catch(std::exception const& e)
    {
      throw Exception(fmt::format(fmt::runtime(str), ns_common::to_string(std::forward<Args>(args))...));
    } // catch
  };
}

// }}}

// ns_common {{{

namespace ns_common
{

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
  
  // Throw
  "Cannot convert '{}' to a valid string"_throw(boost::typeindex::type_id_with_cvr<T>().pretty_name());

  // Suppress compiler warning of no return, it throws before this
  return {};
} // to_string() }}}

} // namespace ns_common}}}


/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
