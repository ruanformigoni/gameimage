///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : exception
///

#pragma once

#include <optional>
#include <sstream>
#include <expected>

namespace ns_exception
{

template<std::regular_invocable F>
void ignore(F&& f)
{
  try { f(); } catch (...) {}
} // function: ignore

template<std::regular_invocable F>
requires std::is_default_constructible_v<std::invoke_result_t<F>>
auto or_default(F&& f) -> std::invoke_result_t<F>
{
  try { return f(); } catch (...) { return std::invoke_result_t<F>{}; }
} // function: or_default

template<std::regular_invocable F, typename T>
requires std::is_default_constructible_v<T>
and std::is_convertible_v<T, std::invoke_result_t<F>>
T or_else(F&& f, T&& t)
{
  try { return f(); } catch (...) { return t; }
} // function: or_else

template<std::regular_invocable F, std::regular_invocable G>
requires std::same_as<std::invoke_result_t<F>,std::invoke_result_t<G>>
decltype(auto) or_else(F&& f, G&& g)
{
  if constexpr ( std::is_void_v<std::invoke_result_t<F>> )
  {
    try { f(); } catch (...) { g(); }
  } // if
  else
  {
    try { return f(); } catch (...) { return g(); }
  } // else
} // function: or_else

template<std::regular_invocable F>
auto to_optional(F&& f) -> std::optional<std::invoke_result_t<F>>
{
  try { return std::make_optional(f()); } catch (...) { return std::nullopt; }
} // function: to_optional

template<std::regular_invocable F>
requires (not std::is_void_v<std::invoke_result_t<F>>)
auto to_expected(F&& f) -> std::expected<std::invoke_result_t<F>, std::string>
{
  try { return f(); } catch (std::exception const& e) { return std::unexpected(e.what()); }
} // function: to_optional

template<typename F, typename... Args>
requires std::is_invocable_v<F, Args...>
and std::is_void_v<std::invoke_result_t<F, Args...>>
auto to_expected(F&& f, Args&&... args) -> std::expected<std::true_type, std::string>
{
  try
  {
    f(std::forward<Args>(args)...);
    return std::true_type{};
  }
  catch (std::exception const& e)
  {
    return std::unexpected(std::string{e.what()});
  }
  catch (...)
  {
    return std::unexpected(std::string{"Exception does not inherit from std::exception"});
  }
} // function: to_optional

template<typename F, typename... Args>
requires std::is_invocable_v<F, Args...>
and (not std::is_void_v<std::invoke_result_t<F, Args...>>)
auto to_expected(F&& f, Args&&... args) -> std::expected<std::invoke_result_t<F,Args...>, std::string>
{
  try
  {
    return f(std::forward<Args>(args)...);
  }
  catch (std::exception const& e)
  {
    return std::unexpected(std::string{e.what()});
  }
  catch (...)
  {
    return std::unexpected(std::string{"Exception does not inherit from std::exception"});
  }
} // function: to_optional

} // namespace ns_exception

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
