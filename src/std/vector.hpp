///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : vector
///

#pragma once

#include "concepts.hpp"

namespace ns_vector
{

template<ns_concept::IterableForward V, typename T = typename V::value_type>
T pop_back(V& vec)
{
  T value = vec.front();
  vec.erase(vec.begin());
  return value;
}

} // namespace ns_vector

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
