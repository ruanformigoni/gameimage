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

template<ns_concept::Iterable R = std::vector<std::string>>
inline R from_string(ns_concept::AsString auto&& s
  , char delimiter
  , std::function<typename R::value_type(std::string)> f = [](auto&& e){ return e; })
{
  R tokens;
  std::string token;
  std::istringstream stream_token(ns_string::to_string(s));

  while (std::getline(stream_token, token, delimiter))
  {
    tokens.push_back(f(token));
  } // while

  return tokens;
} // from_string


} // namespace ns_vector

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
