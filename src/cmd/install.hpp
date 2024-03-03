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
#include "../lib/db.hpp"

#include "select.hpp"

namespace ns_install
{

namespace fs = std::filesystem;
namespace match = matchit;


// icon() {{{
inline void icon(std::string str_file_icon)
{
  namespace gil = boost::gil;

  // Current application
  std::string str_app;

  // Current application directory
  fs::path path_app;

  ns_db::from_file_default([&](auto&& db)
  {
    // Current application
    str_app = db["project"];

    // Current application directory
    path_app = std::string(db[str_app]["path-project"]);
  }
  , std::ios_base::in);

  // Validate that file exists
  fs::path path_file_icon_src = ns_fs::ns_path::file_exists<true>(str_file_icon)._ret;

  // File extension
  std::string ext = path_file_icon_src.extension();

  // // Check result
  "Empty file extension"_throw_if([&]{ return ext.empty(); });

  // // Remove the leading dot
  ext.erase(ext.begin());

  // Create icon directory and set file name
  fs::path path_dir_icon = path_app / "icon";
  ns_fs::ns_path::dir_create<true>(path_dir_icon);
  fs::path path_file_icon_dst = path_dir_icon / "icon.png";

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

  // Save icon path in project database
  ns_db::from_file_project([&](auto&& db)
  {
    db("path-file-icon") = fs::relative(path_file_icon_dst, path_app);
  }
  , std::ios_base::out);
} // icon() }}}

// wine() {{{
inline void wine(std::vector<std::string> args)
{
  // Current application
  std::string str_app;

  // Path to flatimage
  fs::path path_flatimage;

  // Get default path
  ns_db::from_file_default([&](auto&& db)
  {
    // Current application
    str_app = db["project"];

    // Path to flatimage
    path_flatimage = ns_fs::ns_path::file_exists<true>(db[str_app]["path-image"])._ret;
  }
  , std::ios_base::in);

  // Default working directory
  fs::path path_dir_project = ns_fs::ns_path::canonical<true>(str_app)._ret;

  // Path to wine prefix
  fs::path path_wineprefix = fs::path{path_dir_project} / "wine";

  // Log
  ns_log::write('i', "application: ", str_app);
  ns_log::write('i', "image: ", path_flatimage);
  ns_log::write('i', "prefix: ", path_wineprefix);

  // Export prefix
  ns_env::set("WINEPREFIX", path_wineprefix.c_str(), ns_env::Replace::Y);

  // Set debug level
  ns_env::set("WINEDEBUG", "fixme-all", ns_env::Replace::N);

  // Set callbacks for wine/winetricks
  auto f_wine = [&]<typename... _Args>(_Args&&... _args)
  {
    ns_subprocess::sync(path_flatimage, "fim-exec", "wine", std::forward<_Args>(_args)...);
  };

  auto f_winetricks = [&]<typename... _Args>(_Args&&... _args)
  {
    ns_subprocess::sync(path_flatimage, "fim-exec", "winetricks", std::forward<_Args>(_args)...);
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

// emulator() {{{
inline void emulator(std::vector<std::string> args)
{
  // Current project name
  std::string str_project = ns_db::query(ns_db::file_default(), "project");

  // Path to project
  fs::path path_dir_project = ns_db::query(ns_db::file_default(), str_project, "path-project");

  // Path to flatimage
  fs::path path_flatimage = ns_db::query(ns_db::file_default(), str_project, "path-image");

  // Install paths
  fs::path path_dir_config = path_dir_project / ns_db::query(ns_db::file_project(), "path-dir-config");
  fs::path path_dir_data   = path_dir_project / ns_db::query(ns_db::file_project(), "path-dir-data");
  fs::path path_dir_rom    = path_dir_project / ns_db::query(ns_db::file_project(), "path-dir-rom");
  fs::path path_dir_core   = path_dir_project / ns_db::query(ns_db::file_project(), "path-dir-core");
  fs::path path_dir_bios   = path_dir_project / ns_db::query(ns_db::file_project(), "path-dir-bios");
  fs::path path_dir_keys   = path_dir_project / ns_db::query(ns_db::file_project(), "path-dir-keys");

  // Log
  ns_log::write('i', "application   : ", str_project);
  ns_log::write('i', "path config   : ", path_dir_config);
  ns_log::write('i', "path core     : ", path_dir_core);
  ns_log::write('i', "path rom      : ", path_dir_rom);
  ns_log::write('i', "path bios     : ", path_dir_bios);
  ns_log::write('i', "path keys     : ", path_dir_keys);

  // No command
  if ( args.empty() )
  {
    ns_log::write('i', "No command passed to install phase");
    return;
  }

  // Install helpers
  auto f_install = [&](std::string const& type, fs::path path_file_src, fs::path path_file_dst)
  {
    // Verify if source file exists
    path_file_src = ns_fs::ns_path::file_exists<true>(path_file_src)._ret;
    // Copy to target file
    ns_copy::file(path_file_src, path_file_dst, ns_copy::callback_seconds(std::chrono::seconds(1)
      , [&](double percentage, auto&& path_src, auto&& path_dst)
      {
        ns_log::write('i', "Copy ", path_src, " to ", path_dst,  " - ", percentage*100, " %");
      })
    );

    // Relative path
    fs::path path_file_dst_relative = fs::relative(path_file_dst, path_dir_project);

    // Save in database
    ns_db::from_file_project([&](auto&& db)
    {
      db(fmt::format("paths-file-{}", type)) |= path_file_dst_relative;
    }
    , std::ios_base::out);

    // Set as default
    ns_select::select(std::vector<std::string>{type, path_file_dst_relative});
  }; // f_install

  auto f_install_files = [&](std::string const& cmd, fs::path path_dir_dst, std::vector<std::string> const& args)
  {
    // Check for arguments of command
    "No argument provided for command '{}'"_throw_if([&]{ return args.empty(); }, cmd);
    std::ranges::for_each(args, [&](fs::path e){ f_install(cmd, e, path_dir_dst / e.filename()); });
  }; // f_install_roms

  // Get command
  std::string str_cmd = args.front();
  args.erase(args.begin());

  match::match(str_cmd)
  (
    match::pattern | "gui"      = [&]
    {
      ns_env::set("FIM_XDG_CONFIG_HOME", path_dir_config.c_str(), ns_env::Replace::Y);
      ns_env::set("FIM_XDG_DATA_HOME", path_dir_data.c_str(), ns_env::Replace::Y);
      ns_subprocess::sync(path_flatimage);
    },
    match::pattern | "bios"     = [&]{ f_install_files("bios", path_dir_bios, args); },
    match::pattern | "rom"      = [&]{ f_install_files("rom", path_dir_rom, args); },
    match::pattern | "core"     = [&]{ f_install_files("core", path_dir_core, args); },
    match::pattern | "keys"     = [&]{ f_install_files("keys", path_dir_keys, args); },
    match::pattern | match::_   = [&]{ "Unknown command '{}'"_throw(str_cmd.c_str()); }
  );
} // emulator() }}}

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

  // Get platform
  ns_enum::Platform enum_platform;
  ns_db::from_file_default([&](auto&& db)
  { 
    std::string str_app = db[db["project"]]["platform"];
    enum_platform = ns_enum::from_string<ns_enum::Platform>(str_app);
  }
  , std::ios_base::in);

  // Install based on platform
  switch(enum_platform)
  {
    case ns_enum::Platform::WINE: ns_install::wine(args);
    break;
    case ns_enum::Platform::RETROARCH:
    case ns_enum::Platform::PCSX2:
    case ns_enum::Platform::YUZU: ns_install::emulator(args);
    break;
    case ns_enum::Platform::RPCS3:
      "Not implemented"_throw();
  } // switch
  
} // install() }}}

} // namespace ns_install

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
