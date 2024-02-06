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

// namespace ns_enum {{{
namespace ns_enum
{

// Concepts
template<typename T>
concept Enum = std::is_enum_v<std::remove_cvref_t<T>>;

// Image Format
enum class ImageFormat
{
	PNG,
	JPG,
	JPEG,
};


// Platform
enum class Platform
{
	WINE,
	RETROARCH,
	PCSX2,
	RPCS3,
	YUZU,
};

// Stage
enum class Stage
{
	NONE,
	FETCH,
	INIT,
	PROJECT,
	INSTALL,
	SEARCH,
	SELECT,
	TEST,
	COMPRESS,
	PACKAGE,
};

template<Enum U, ns_concept::StreamInsertable T>
inline decltype(auto) from_string(T&& t)
{
	if ( auto opt = magic::enum_cast<U>(ns_common::to_string(t), magic::case_insensitive); opt )
	{
		return U{*opt};
	} // if

	"Could not convert enum"_throw();

	return U{};
}

template<Enum T>
inline std::string to_string(T&& t)
{
	return std::string(magic::enum_name(t));
}


template<Enum T>
inline std::string to_string_lower(T&& t)
{
	return ns_string::to_lower(std::string(magic::enum_name(t)));
}

} // namespace ns_enum }}}

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
