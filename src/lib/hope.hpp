///
/// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
/// @file        : hope
///

#pragma once

#include <expected>

#define hope(expr,ferror,...) \
({ \
  auto _tmp = expr; \
  if (!_tmp.has_value()) \
  { \
    ferror(__VA_ARGS__); \
    ferror(_tmp.error()); \
    return std::unexpected(_tmp.error()); \
  } \
  _tmp.value(); \
})

#define ehope(expr, ...) hope(expr, []<typename... Ts>(Ts&&... ts){ ns_log::write('e', std::forward<Ts>(ts)...); }, __VA_ARGS__)
#define ihope(expr, ...) hope(expr, []<typename... Ts>(Ts&&... ts){ ns_log::write('i', std::forward<Ts>(ts)...); }, __VA_ARGS__)
#define dhope(expr, ...) hope(expr, []<typename... Ts>(Ts&&... ts){ ns_log::write('d', std::forward<Ts>(ts)...); }, __VA_ARGS__)

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
