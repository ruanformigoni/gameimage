///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : vector
///

#pragma once

#include "concepts.hpp"
#include "string.hpp"

namespace ns_vector
{

template<ns_concept::IterableForward V, typename T = typename V::value_type>
T pop_back(V& vec)
{
  T value = vec.front();
  vec.erase(vec.begin());
  return value;
}

template<std::ranges::input_range R = std::vector<std::string>
  , typename F = std::function<std::string(typename std::remove_cvref_t<R>::value_type)>>
inline R from_string(ns_concept::AsString auto&& t
  , char delimiter
  , auto&& f = [](auto&& e){ return e; })
{
  R tokens;
  std::string token;
  std::istringstream stream_token(ns_string::to_string(t));

  while (std::getline(stream_token, token, delimiter))
  {
    tokens.push_back(f(token));
  } // while

  return tokens;
} // from_string


} // namespace ns_vector

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
