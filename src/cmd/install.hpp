///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : install
///

#pragma once

#include <cstdlib>
#include <filesystem>
#include <matchit.h>

#include <boost/gil.hpp>
#include <boost/gil/extension/io/jpeg.hpp>
#include <boost/gil/extension/io/png.hpp>

#include "../enum.hpp"
#include "../common.hpp"

#include "../std/env.hpp"
#include "../std/copy.hpp"

#include "../lib/subprocess.hpp"
#include "../lib/json.hpp"

namespace ns_install
{

namespace fs = std::filesystem;
namespace match = matchit;


// icon() {{{
inline void icon(std::string str_file_icon)
{
  namespace gil = boost::gil;

  // Current application
  ns_json::Json json = ns_json::from_default_file();
  std::string str_app = json["default"];

  // Current application directory
  fs::path path_app = json[str_app]["path-app"];

  // Validate that file exists
  fs::path path_file_icon_src = ns_fs::ns_path::file_exists<true>(str_file_icon)._ret;

  // File extension
  std::string ext = path_file_icon_src.extension();

  // Create icon directory and set file name
  fs::path path_dir_icon = path_app /= "icon";
  ns_fs::ns_path::dir_create<true>(path_dir_icon);
  fs::path path_file_icon_dst = path_dir_icon /= "icon.png";

  gil::rgb8_image_t img;
  switch ( ns_enum::from_string<ns_enum::ImageFormat>(ext) )
  {
    // Convert jpg to png
    case ns_enum::ImageFormat::JPG:
    case ns_enum::ImageFormat::JPEG:
      gil::read_image(path_file_icon_src, img, gil::jpeg_tag());
      gil::write_view(path_file_icon_dst, gil::view(img), gil::png_tag());
      break;
    // Copy
    case ns_enum::ImageFormat::PNG:
      ns_copy::file(path_file_icon_src, path_file_icon_dst);
      break;
  } // switch
  
} // icon() }}}

// wine() {{{
inline void wine(std::vector<std::string> args)
{
  // Get default path
  ns_json::Json json = ns_json::from_default_file();

  // Current application
  std::string str_app = json["default"];

  // Default working directory
  fs::path path_cwd = ns_fs::ns_path::canonical<true>(str_app)._ret;

  // Path to flatimage
  fs::path path_flatimage = ns_fs::ns_path::file_exists<true>(json[str_app]["path-image"])._ret;

  // Path to wine prefix
  fs::path path_wineprefix = fs::path{path_cwd} /= "wine";

  // Log
  ns_log::write('i', "application: ", str_app);
  ns_log::write('i', "image: ", path_flatimage);
  ns_log::write('i', "prefix: ", path_wineprefix);

  // Export prefix
  ns_env::set("WINEPREFIX", path_wineprefix.c_str(), ns_env::Replace::N);

  // Set debug level
  ns_env::set("WINEDEBUG", "fixme-all", ns_env::Replace::N);

  // Update PATH
  ns_env::concat("PATH",":/opt/wine/bin");

  // Set callbacks for wine/winetricks
  auto f_wine = [&]<typename... _Args>(_Args&&... args)
  {
    ns_subprocess::subprocess(path_flatimage, "fim-exec", "wine", std::forward<_Args>(args)...);
  };

  auto f_winetricks = [&]<typename... _Args>(_Args&&... args)
  {
    ns_subprocess::subprocess(path_flatimage, "fim-exec", "winetricks", std::forward<_Args>(args)...);
  };

  // No command
  if ( args.empty() )
  {
    ns_log::write('i', "No command for wine");
    return;
  }

  // Get command
  std::string str_cmd = args.front();
  args.erase(args.begin());

  match::match(str_cmd)
  (
    match::pattern | "winetricks" = [&]{ f_winetricks(args); },
    match::pattern | "wine"       = [&]{ f_wine(args); },
    match::pattern | "dxvk"       = [&]{ f_winetricks("dxvk"); },
    match::pattern | "vkd3d"      = [&]{ f_winetricks("vkd3d"); },
    match::pattern | match::_     = [&]{ "Unknown command '{}'"_throw(str_cmd.c_str()); }
  );
} // wine() }}}

// install() {{{
void install(std::string const& str_platform, std::vector<std::string> const& args)
{
  ns_json::Json json = ns_json::from_default_file();
  std::string str_app = json[json["default"]]["platform"];
  ns_enum::Platform enum_platform = ns_enum::from_string<ns_enum::Platform>(str_app);

  switch(enum_platform)
  {
    case ns_enum::Platform::WINE:
      ns_install::wine(args);
      break;
    case ns_enum::Platform::RETROARCH:
      break;
    case ns_enum::Platform::PCSX2:
      break;
    case ns_enum::Platform::RPCS3:
      break;
    case ns_enum::Platform::YUZU:
      break;
  } // switch
  
} // install() }}}

} // namespace ns_install

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
