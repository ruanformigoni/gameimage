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
#include "../lib/zip.hpp"

#include "fetch.hpp"
#include "select.hpp"

namespace ns_install
{

namespace fs = std::filesystem;
namespace match = matchit;

// enum class Op {{{
enum class Op
{
  ICON,
  ROM,
  CORE,
  BIOS,
  KEYS,
  GUI,
};
// }}}

namespace
{

// remove_files() {{{
void remove_files(Op const& op
  , fs::path const& path_dir_project
  , fs::path const& rpath_file_target)
{
  ns_log::write('i', "Removing '", rpath_file_target, "'");

  fs::path path_file_target = path_dir_project / rpath_file_target;

  // Try to remove file
  if ( fs::is_regular_file(path_file_target) )
  {
    // Verify if source file exists
    path_file_target = ns_fs::ns_path::file_exists<true>(path_file_target)._ret;
    // Remove file
    fs::remove(path_file_target);
  } // if
  // Try to remove dir
  else if( fs::is_directory(path_file_target) )
  {
    // Verify if source directory exists
    path_file_target = ns_fs::ns_path::dir_exists<true>(path_file_target)._ret;
    // Remove directory
    fs::remove_all(path_file_target);
  } // else if
  else
  {
    ns_log::write('i', "File '", path_file_target, "' not found for removal");
  } // else

  // Relative path
  fs::path path_file_dst_relative = fs::relative(path_file_target, path_dir_project);

  // Remove from database
  ns_db::from_file_project([&](auto&& db)
  {
    std::string entry_db = fmt::format("path-file-{}", ns_enum::to_string_lower(op));
    try
    {
      // Remove default key if is default
      if ( db.template contains<false>(entry_db) && db[entry_db] == path_file_dst_relative )
      {
        db.erase(entry_db);
      } // if
    } // try
    catch(std::exception const& e)
    {
      ns_log::write('i', e.what());
    } // catch

    std::string entries_db = fmt::format("paths-file-{}", ns_enum::to_string_lower(op));
    try
    {
      // Remove from list
      if ( db.template contains<false>(entries_db) )
      {
        // Files are kept as relative in db
        db(entries_db).erase(rpath_file_target);
      } // if
    } // try
    catch(std::exception const& e)
    {
      ns_log::write('i', e.what());
    } // catch
  }
  , ns_db::Mode::UPDATE);
} // remove_files() }}}

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
  , ns_db::Mode::READ);

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
inline void emulator(Op op, std::vector<std::string> args)
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

  // Install helpers
  auto f_install_file = [&](std::string const& type, fs::path path_file_src, fs::path path_file_dst)
  {
    ns_log::write('i', "Copy ", path_file_src, " to ", path_file_dst);

    // Check if src and dst are the same
    if ( path_file_src == path_file_dst )
    {
      ns_log::write('i', "Src and dst are the same for '", path_file_src, "'");
    } // if
    else
    {
      // Try to copy file
      try
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
      } // try
      // Try to copy dir
      catch(std::exception const& e)
      {
        ns_log::write('i', e.what());
        // Verify if source directory exists
        path_file_src = ns_fs::ns_path::dir_exists<true>(path_file_src)._ret;
        // Copy recursive
        fs::copy(path_file_src, path_file_dst, fs::copy_options::overwrite_existing | fs::copy_options::recursive);
      } // catch
    } // else

    // Relative path
    fs::path path_file_dst_relative = fs::relative(path_file_dst, path_dir_project);

    // Save in database
    ns_db::from_file_project([&](auto&& db)
    {
      db(fmt::format("paths-file-{}", type)) |= path_file_dst_relative;
    }
    , ns_db::Mode::UPDATE);

    // Set as default
    ns_select::select(std::vector<std::string>{type, path_file_dst_relative});
  }; // f_install_file

  auto f_install_files = [&](std::string const& cmd, fs::path path_dir_dst, std::vector<std::string> const& args)
  {
    // Check for arguments of command
    "No argument provided for command '{}'"_throw_if([&]{ return args.empty(); }, cmd);
    std::ranges::for_each(args, [&](fs::path e)
    {
      e = ns_fs::ns_path::canonical<true>(e)._ret;
      f_install_file(cmd, e, path_dir_dst / e.filename());
    });
  }; // f_install_roms

  // Get command
  switch(op)
  {
    case Op::GUI:
    {
      ns_env::set("FIM_XDG_CONFIG_HOME", path_dir_config.c_str(), ns_env::Replace::Y);
      ns_env::set("FIM_XDG_DATA_HOME", path_dir_data.c_str(), ns_env::Replace::Y);
      ns_subprocess::sync(path_flatimage);
    }
    break;
    case Op::BIOS: { f_install_files("bios", path_dir_bios, args); } break;
    case Op::ROM : { f_install_files("rom", path_dir_rom, args); }; break;
    case Op::CORE: { f_install_files("core", path_dir_core, args); }; break;
    case Op::KEYS: { f_install_files("keys", path_dir_keys, args); }; break;
    case Op::ICON: "Invalid op in emulator install"_throw(); break;
  } // switch
} // emulator() }}}

} // anonymous namespace

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
  , ns_db::Mode::READ);

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
  , ns_db::Mode::UPDATE);
} // icon() }}}

// remove() {{{
template<typename R>
void remove(Op const& op, fs::path const& path_dir_project, R&& files)
{
  // Remove Bios or Rom or Core...
  fs::path path_dir_item = ns_db::query(ns_db::file_project(), "path-dir-{}"_fmt(ns_enum::to_string_lower(op)));
  // Remove file by file
  std::ranges::for_each(files, [&](fs::path e){ remove_files(op, path_dir_project, path_dir_item / e.filename()); });
} // remove() }}}

// install() {{{
inline void install(Op op, std::vector<std::string> args)
{
  if ( args.empty() && op != Op::GUI ) { "Empty arguments for install command"_throw(); }

  // Get platform
  std::string str_project = ns_db::query(ns_db::file_default(), "project");
  ns_enum::Platform enum_platform = ns_enum::from_string<ns_enum::Platform>(
    ns_db::query(ns_db::file_default(), str_project, "platform")
  );

  // Install based on platform
  switch(enum_platform)
  {
    case ns_enum::Platform::WINE: ns_install::wine(args);
    break;
    case ns_enum::Platform::RETROARCH:
    case ns_enum::Platform::PCSX2:
    case ns_enum::Platform::RPCS3:
    case ns_enum::Platform::YUZU: ns_install::emulator(op, args);
    break;
  } // switch
  
} // install() }}}

// remote() {{{
inline void remote(Op const& op, std::vector<std::string> vec_cores)
{
  // Get project
  std::string str_project = ns_db::query(ns_db::file_default(), "project");

  // Get platform
  ns_enum::Platform enum_platform = ns_enum::from_string<ns_enum::Platform>(
    ns_db::query(ns_db::file_default(), str_project, "platform")
  );

  // Core dir
  fs::path rpath_dir_core = ns_db::query(ns_db::file_project(), "path-dir-core");

  if ( enum_platform != ns_enum::Platform::RETROARCH )
  {
    "Core install is only available for retroarch"_throw();
  } // if

  if ( op != Op::CORE )
  {
    "Only download of cores is available"_throw();
  } // if

  // Get project directory
  fs::path path_dir_project = ns_db::query(ns_db::file_default(), str_project, "path-project");

  // Fetch cores / urls
  auto vec_core_url = ns_fetch::cores_list(path_dir_project);

  // Put in a set
  std::set<std::string> set_cores(vec_cores.begin(), vec_cores.end());

  // Install if match
  for( auto i : vec_core_url )
  {
    if ( ! set_cores.contains(i.core) )
    {
      ns_log::write('i', "Skip ", i.core);
      continue;
    } // if

    fs::path path_file_out = path_dir_project / rpath_dir_core / i.core;
    ns_log::write('i', "Install ", i.core, " to ", path_file_out);
    ns_fetch::fetch_file_from_url(path_file_out, i.url);
    // Extract & install
    if ( i.core.ends_with(".zip") )
    {
      // Extract zip
      ns_zip::extract(path_file_out, path_file_out.parent_path());
      // Erase zip
      fs::remove(path_file_out);
      // Install (file ends with .so.zip, remove the .zip to the get extracted file name)
      install(op, std::vector<std::string>{path_file_out.replace_extension("")});
    } // if
  } // for
  
} // remote() }}}

} // namespace ns_install

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
