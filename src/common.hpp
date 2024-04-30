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

#include "std/concepts.hpp"
#include "std/string.hpp"

#include "lib/log.hpp"

inline const char * GIMG_PATH_JSON_FETCH = "/tmp/gameimage/json";

// macros {{{
#define return_if(cond, ...) \
  if (cond) { return __VA_ARGS__; }

#define return_if_else(cond, val1, val2) \
  if (cond) { return val1; } else { return val2; }

#define break_if(cond) \
  if ( (cond) ) { break; }

#define continue_if(cond) \
  if ( (cond) ) { continue; }

#define assign_if(cond, var, val) \
  if ( cond ) { var = val; }

#define assign_or_return(val, cond, ret) \
  val; if ( not cond ) { return ret; }
// }}}

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
inline decltype(auto) operator ""_fmt(const char* str, size_t)
{
  return [str]<typename... Args>(Args&&... args)
  {
    return fmt::format(fmt::runtime(str), ns_string::to_string(std::forward<Args>(args))... ) ;
  };
} //

// Format strings with user-defined literals
inline decltype(auto) operator ""_throw(const char* str, size_t)
{
  return [str]<typename... Args>(Args&&... args)
  {
    throw Exception(fmt::format(fmt::runtime(str), ns_string::to_string(std::forward<Args>(args))...));
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
      ns_log::write('e', e.what());
      ns_log::write('e', fmt::format(fmt::runtime(str), ns_string::to_string(std::forward<Args>(args))...));
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
      throw Exception(fmt::format(fmt::runtime(str), ns_string::to_string(std::forward<Args>(args))...));
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
      ns_log::write('e', e.what());
      throw Exception(fmt::format(fmt::runtime(str), ns_string::to_string(std::forward<Args>(args))...));
    } // catch
  };
}

// }}}

// ns_common {{{

namespace ns_common
{

// check_and() {{{
template<ns_concept::Enum T, ns_concept::Enum... Args>
constexpr bool check_and(T&& t, Args&&... flags)
{
  return static_cast<int>(t) & ( static_cast<int>(flags) & ... );
} // check_and() }}}

// catch_to_optional() {{{
template<typename F, typename... Args>
auto catch_to_optional(F&& f, Args&&... args) -> std::optional<decltype(f(args...))>
{
  try
  {
    auto val = make_optional(f(std::forward<Args>(args)...));
    ns_log::write('i', "Optional: ", *val);
    return val;
  } // try
  catch(std::exception const& e)
  {
    ns_log::write('i', "Optional (caught): ", e.what());
    return std::nullopt;
  } // catch
} // catch_to_optional() }}}

} // namespace ns_common}}}

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
