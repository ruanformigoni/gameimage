///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : concepts
// @created     : Tuesday Jan 23, 2024 19:59:31 -03
///

#pragma once

#include <string>
#include <type_traits>
#include <concepts>
#include <vector>
#include <variant>

namespace ns_concept
{
template <typename T>
concept Variant = requires(T){ std::variant_size_v<T>; };

template<typename T>
concept Enum = std::is_enum_v<T>;

template<typename T>
concept Iterable = requires(T t)
{
  { t.begin() } -> std::input_iterator;
  { t.end() } -> std::input_iterator;
};

template<typename T>
concept IterableConst = requires(T t)
{
  { t.cbegin() } -> std::input_iterator;
  { t.cend() } -> std::input_iterator;
};


// Helper to check if a type is a specialization of std::vector
template <typename>
struct is_vector : std::false_type {};

template <typename T, typename Allocator>
struct is_vector<std::vector<T, Allocator>> : std::true_type {};

// Define a concept based on the helper trait
template <typename T>
concept IsVector = is_vector<T>::value;

template<typename T>
concept IterableForward = std::forward_iterator<T>;

template<typename T>
concept StringConvertible = std::is_convertible_v<std::decay_t<T>, std::string>;

template<typename T>
concept StringConstructible = std::constructible_from<std::string, std::decay_t<T>>;

template<typename T>
concept Numeric = std::integral<std::decay_t<T>>
  or std::floating_point<std::decay_t<T>>;

template<typename T>
concept StreamInsertable = requires(T t, std::ostream& os)
{
  { os << t } -> std::same_as<std::ostream&>;
};

template<typename T>
concept AsString = StringConvertible<T> or StringConstructible<T> or Numeric<T> or StreamInsertable<T>;

} // namespace ns_concept

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
