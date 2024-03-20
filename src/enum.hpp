///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : enum
///

#pragma once

#include <string>
#include <magic_enum/magic_enum.hpp>

#include "common.hpp"

#include "std/string.hpp"
#include "std/concepts.hpp"

namespace magic = magic_enum;

// namespace ns_enum
namespace ns_enum
{

// Concepts
template<typename T>
concept Enum = std::is_enum_v<std::remove_cvref_t<T>>;

// enum class ImageFormat {{{
enum class ImageFormat
{
	PNG,
	JPG,
	JPEG,
}; // }}}

// enum class Platform {{{
enum class Platform
{
  LINUX,
	WINE,
	RETROARCH,
	PCSX2,
	RPCS3,
	RYUJINX,
}; // enum class Platform }}}

// enum class Op {{{
enum class Op
{
  ICON,
  ROM,
  CORE,
  BIOS,
  KEYS,
  CONFIG,
  DATA,
  GUI,
	WINE,
	WINETRICKS,
	DXVK,
	VKD3D
}; // enum class Op }}}

// enum class Stage {{{
enum class Stage
{
	FETCH,
	INIT,
	PROJECT,
	INSTALL,
	SEARCH,
	SELECT,
	TEST,
	DESKTOP,
	COMPRESS,
	PACKAGE,
}; // enum class Stage }}}

// from_string() {{{
template<Enum U, ns_concept::StreamInsertable T>
inline decltype(auto) from_string(T&& t)
{
  auto opt = magic::enum_cast<U>(ns_string::to_string(t), magic::case_insensitive);

	if ( ! opt ) { "Could not convert enum from '{}'"_throw(t); } // if

	return U{*opt};
} // }}}

// to_string() {{{
template<Enum T>
inline std::string to_string(T&& t)
{
	return std::string(magic::enum_name(std::forward<T>(t)));
} // to_string() }}}

// to_string_lower() {{{
template<Enum T>
inline std::string to_string_lower(T&& t)
{
	return ns_string::to_lower(ns_string::to_string(magic::enum_name(std::forward<T>(t))));
} // to_string_lower() }}}

// check_and() {{{
template<ns_concept::Enum T, ns_concept::Enum... Args>
constexpr bool check_and(T&& t, Args&&... flags)
{
  return static_cast<int>(t) & ( static_cast<int>(flags) & ... );
} // check_and() }}}

} // namespace ns_enum

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
