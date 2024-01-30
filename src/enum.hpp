///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : enum
///

#pragma once

#include <string>
#include <magic_enum/magic_enum.hpp>

#include "common.hpp"

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
	BOOT,
	TEST,
	COMPRESS,
	PACKAGE,
};

template<Enum U, std::convertible_to<std::string> T>
inline decltype(auto) from_string(T&& t)
{
	if ( auto opt = magic::enum_cast<U>(t, magic::case_insensitive); opt )
	{
		return U{*opt};
	} // if

	"Could not convert enum"_throw();

	return U{};
}

template<Enum T>
inline decltype(auto) to_string(T&& t)
{
	return magic::enum_name(t);
}

} // namespace ns_enum }}}

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
