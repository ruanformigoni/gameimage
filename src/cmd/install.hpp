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
#include <boost/gil/extension/numeric/sampler.hpp>
#include <boost/gil/extension/numeric/resample.hpp>

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
  std::string str_app = json["project"];

  // Current application directory
  fs::path path_app = json[str_app]["path-app"];

  // Validate that file exists
  fs::path path_file_icon_src = ns_fs::ns_path::file_exists<true>(str_file_icon)._ret;

  // File extension
  std::string ext = path_file_icon_src.extension();

  // // Check result
  "Empty file extension"_throw_if([&]{ return ! ext.empty(); });

  // // Remove the leading dot
  ext.erase(ext.begin());

  // Create icon directory and set file name
  fs::path path_dir_icon = path_app /= "icon";
  ns_fs::ns_path::dir_create<true>(path_dir_icon);
  fs::path path_file_icon_dst = path_dir_icon /= "icon.png";

  // Get enum option
  ns_enum::ImageFormat image_format;

  // Check image type
  "Image type '{}' is not supported, supported types are '.jpg, .jpeg, .png'"_try(
    [&]{ image_format = ns_enum::from_string<ns_enum::ImageFormat>(ext); }
    , ext
  );

  ns_log::write('i', "Reading image from ", path_file_icon_src);
  gil::rgb8_image_t img; 
  switch ( image_format )
  {
    // Convert jpg to png
    case ns_enum::ImageFormat::JPG:
    case ns_enum::ImageFormat::JPEG:
      gil::read_image(path_file_icon_src, img, gil::jpeg_tag());
      break;
    // Copy
    case ns_enum::ImageFormat::PNG:
      gil::read_image(path_file_icon_src, img, gil::png_tag());
      break;
  } // switch

  ns_log::write('i', "Image size is ", std::to_string(img.width()), "x", std::to_string(img.height()));

  // Target dimms
  int const width = 600;
  int const height = 900;

  // Calculate desired and current aspected ratios
  double src_aspect = static_cast<double>(img.width()) / img.height();
  double dst_aspect = static_cast<double>(width) / height;

  // Calculate novel dimensions that preserve the aspect ratio
  int width_new  = (src_aspect >  dst_aspect)? static_cast<int>(src_aspect * height) : width;
  int height_new = (src_aspect <= dst_aspect)? static_cast<int>(width / src_aspect ) : height;

  // Resize
  gil::rgb8_image_t img_resized(width_new, height_new);
  ns_log::write('i', "Image  aspect ratio is ", std::to_string(src_aspect));
  ns_log::write('i', "Target aspect ratio is ", std::to_string(dst_aspect));
  ns_log::write('i', "Resizing image to ", std::to_string(width_new), "x", std::to_string(height_new));
  gil::resize_view(gil::const_view(img), gil::view(img_resized), gil::bilinear_sampler());

  // Calculate crop
  int crop_x = (width_new - width) / 2;
  int crop_y = (height_new - height) / 2;

  // Crop the image
  auto view_img_cropped = gil::subimage_view(gil::view(img_resized), crop_x, crop_y, width, height);
  
  ns_log::write('i', "Writing image to ", path_file_icon_dst);
  gil::write_view(path_file_icon_dst, view_img_cropped, gil::png_tag());
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
inline void install(std::vector<std::string> args)
{
  if ( args.empty() ) { "Empty arguments for install command"_throw(); }

  // Install icon
  if ( args.front() == "icon" )
  {
    // Pop front
    args.erase(args.begin());
    // Check if has icon path
    "No file name specified for icon"_throw_if([&]{ return args.empty(); });
    // Create icon
    icon(args.front());
    return;
  } // if

  // Forward arguments by platform
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
