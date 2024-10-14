///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : install
///

#pragma once

#include <cstdlib>
#include <filesystem>

#include "../enum.hpp"
#include "../common.hpp"

#include "../std/env.hpp"
#include "../std/copy.hpp"

#include "../lib/subprocess.hpp"
#include "../lib/db.hpp"
#include "../lib/zip.hpp"
#include "../lib/image.hpp"

#include "fetch.hpp"
#include "select.hpp"

namespace ns_install
{

namespace fs = std::filesystem;

using Op = ns_enum::Op;

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
    std::string entry_db = fmt::format("path_file_{}", ns_enum::to_string_lower(op));
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

    std::string entries_db = fmt::format("paths_file_{}", ns_enum::to_string_lower(op));
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
inline void wine(Op const& op, std::vector<std::string> args)
{
  // Current application
  std::string str_project = ns_db::query(ns_db::file_default(), "project");

  // Default working directory
  fs::path path_dir_project = ns_db::query(ns_db::file_default(), str_project, "path_dir_project");

  // Path to flatimage
  fs::path path_flatimage = ns_db::query(ns_db::file_default(), str_project, "path_file_image");

  // Path to wine prefix
  fs::path path_wineprefix = fs::path{path_dir_project} / "wine";

  // Log
  ns_log::write('i', "application: ", str_project);
  ns_log::write('i', "image: ", path_flatimage);
  ns_log::write('i', "prefix: ", path_wineprefix);

  // Export prefix
  ns_env::set("WINEPREFIX", path_wineprefix.c_str(), ns_env::Replace::Y);

  // Set debug level
  ns_env::set("WINEDEBUG", "fixme-all", ns_env::Replace::N);

  // Set callbacks for wine/winetricks
  auto f_wine = [&]<typename T>(T&& t)
  {
    (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
      .with_piped_outputs()
      .with_args(path_flatimage, "fim-exec", "/opt/wine/bin/wine.sh", std::forward<T>(t))
      .spawn()
      .wait();
  };

  auto f_winetricks = [&]<typename T>(T&& t)
  {
    (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
      .with_piped_outputs()
      .with_args(path_flatimage, "fim-exec", "/opt/wine/bin/wine.sh", "winetricks", std::forward<T>(t))
      .spawn()
      .wait();
  };

  // Execute operation
  switch(op)
  {
    case Op::WINE       : f_wine(args); break;
    case Op::WINETRICKS : f_winetricks(args); break;
    case Op::DXVK       : f_winetricks("dxvk"); break;
    case Op::VKD3D      : f_winetricks("vkd3d"); break;
    default             :  "Unsupported wine operation '{}'"_throw(ns_enum::to_string_lower(op)); break;
  } // switch
} // wine() }}}

// linux() {{{
inline void linux(Op const& op, std::vector<std::string> args)
{
  // Validate op
  if ( op != Op::ROM )
  {
    "Only ROM is valid for the linux platform"_throw();
  } // if

  // Current application
  std::string str_project = ns_db::query(ns_db::file_default(), "project");

  // Path to flatimage
  fs::path path_flatimage = ns_db::query(ns_db::file_default(), str_project, "path_file_image");

  // Project dir
  fs::path path_dir_project = ns_db::query(ns_db::file_default(), str_project, "path_dir_project");
  fs::path path_dir_linux = path_dir_project / "linux";
  lec(fs::create_directories, path_dir_linux);

  // Run selected file
  (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
    .with_piped_outputs()
    .with_args(path_flatimage, "fim-exec", "env", "HOME={}"_fmt(path_dir_linux.c_str()), args)
    .spawn()
    .wait();
} // linux() }}}

// emulator_install_file_ryujinx() {{{
void emulator_install_file_ryujinx(Op const& op, fs::path path_file_src, fs::path path_file_dst)
{
  // Path to project
  std::string str_project = ns_db::query(ns_db::file_default(), "project");
  fs::path path_dir_project = ns_db::query(ns_db::file_default() , str_project, "path_dir_project");

  ns_log::write('i', "Copy ", path_file_src, " to ", path_file_dst);

  // Validate source path
  path_file_src = ns_fs::ns_path::file_exists<true>(path_file_src)._ret;

  // List of installed files
  std::vector<fs::path> paths_file_installed;

  // Check if src and dst are the same
  if ( path_file_src == path_file_dst )
  {
    ns_log::write('i', "Src and dst are the same for '", path_file_src, "'");
    // Include in db afterwards
    paths_file_installed.push_back(path_file_dst);
  } // if
  // Unzip zip file for ryujinx
  else if ( path_file_src.string().ends_with(".zip") )
  {
    ns_log::write('i', "Src file is a regular zip file with path: '", path_file_src, "'");
    // Extract if platform is ryujinx and is a zipfile
    ns_log::write('i', "Extracting '", path_file_src, "'", " to '", path_file_dst.parent_path(), "'");
    ns_zip::extract(path_file_src, path_file_dst.parent_path());
    // Prepend parent dir to each extracted file
    for (auto&& i : ns_zip::list_regular_files(path_file_src) )
    {
      fs::path path_file_target = path_file_dst.parent_path() / i;
      ns_log::write('i', "Unzipped file '", i, "'", " to ", path_file_dst.parent_path());
      // Include in db afterwards
      paths_file_installed.push_back(path_file_target);
    } // for
  } // else if
  else
  {
    // Copy to regular file to destination regular file
    ns_copy::file(path_file_src, path_file_dst, ns_copy::callback_seconds(std::chrono::seconds(1)
    , [&](double percentage, auto&& path_src, auto&& path_dst)
    {
      ns_log::write('i', "Copy ", path_src, " to ", path_dst,  " - ", percentage*100, " %");
    }));
    // Include in db afterwards
    paths_file_installed.push_back(path_file_dst);
  } // else

  // Save in database
  ns_db::from_file_project([&](auto&& db)
  {
    for(auto&& e : paths_file_installed)
    {
      // Path relative to project
      fs::path path_file_dst_relative = fs::relative(e, path_dir_project);
      ns_log::write('i', "Save entry '", path_file_dst_relative, "' in database");
      // Save in DB
      db(fmt::format("paths_file_{}", ns_enum::to_string_lower(op))) |= path_file_dst_relative;
    } // for
  } , ns_db::Mode::UPDATE);

  // Set as default
  if ( not paths_file_installed.empty() )
  {
    ns_select::select(op, fs::relative(paths_file_installed.front(), path_dir_project));
  } // if
} // emulator_install_file_ryujinx() }}}

// emulator_install_file() {{{
void emulator_install_file(Op const& op, fs::path path_file_src, fs::path const& path_file_dst)
{
  if ( op == Op::KEYS )
  {
    emulator_install_file_ryujinx(op, path_file_src, path_file_dst);
    return;
  } // if

  // Path to project
  std::string str_project = ns_db::query(ns_db::file_default(), "project");
  fs::path path_dir_project = ns_db::query(ns_db::file_default() , str_project, "path_dir_project");
  ns_log::write('i', "Copy ", path_file_src, " to ", path_file_dst);

  // Check if src and dst are the same
  if ( path_file_src == path_file_dst )
  {
    ns_log::write('i', "Src and dst are the same for '", path_file_src, "'");
  } // if
  else if ( auto ret = ns_fs::ns_path::file_exists<false>(path_file_src); ret._bool )
  {
    ns_log::write('i', "Src file is a regular file with path: '", path_file_src, "'");
    // Update to canonical
    path_file_src = ret._ret;
    // Copy to target file
    ns_copy::file(path_file_src, path_file_dst, ns_copy::callback_seconds(std::chrono::seconds(1)
    , [&](double percentage, auto&& path_src, auto&& path_dst)
    {
      ns_log::write('i', "Copy ", path_src, " to ", path_dst,  " - ", percentage*100, " %");
    }));
  } // else if
  else if ( auto ret = ns_fs::ns_path::dir_exists<false>(path_file_src); ret._bool )
  {
    ns_log::write('i', "Src file is a directory with path: '", path_file_src, "'");
    // Update to canonical
    path_file_src = ret._ret;
    // Copy recursive
    fs::copy(path_file_src, path_file_dst, fs::copy_options::overwrite_existing | fs::copy_options::recursive);
  } // else
  else
  {
    "Source file is not a directory nor a regular file: '{}'"_throw(path_file_src);
  } // else

  // Relative path
  fs::path path_file_dst_relative = fs::relative(path_file_dst, path_dir_project);

  // Save in database
  ns_db::from_file_project([&](auto&& db)
  {
    db(fmt::format("paths_file_{}", ns_enum::to_string_lower(op))) |= path_file_dst_relative;
  }
  , ns_db::Mode::UPDATE);

  // Set as default
  ns_select::select(op, path_file_dst_relative);
} // emulator_install_file() }}}

// emulator() {{{
inline void emulator(Op op, std::vector<std::string> args)
{
  // Current project name
  std::string str_project = ns_db::query(ns_db::file_default(), "project");

  // Path to project
  fs::path path_dir_project = ns_db::query(ns_db::file_default(), str_project, "path_dir_project");

  // Path to flatimage
  fs::path path_flatimage = ns_db::query(ns_db::file_default(), str_project, "path_file_image");

  // Install paths
  fs::path path_dir_config = path_dir_project / ns_db::query(ns_db::file_project(), "path_dir_config");
  fs::path path_dir_data   = path_dir_project / ns_db::query(ns_db::file_project(), "path_dir_data");
  fs::path path_dir_rom    = path_dir_project / ns_db::query(ns_db::file_project(), "path_dir_rom");
  fs::path path_dir_core   = path_dir_project / ns_db::query(ns_db::file_project(), "path_dir_core");
  fs::path path_dir_bios   = path_dir_project / ns_db::query(ns_db::file_project(), "path_dir_bios");
  fs::path path_dir_keys   = path_dir_project / ns_db::query(ns_db::file_project(), "path_dir_keys");

  // Log
  ns_log::write('i', "application   : ", str_project);
  ns_log::write('i', "path config   : ", path_dir_config);
  ns_log::write('i', "path core     : ", path_dir_core);
  ns_log::write('i', "path rom      : ", path_dir_rom);
  ns_log::write('i', "path bios     : ", path_dir_bios);
  ns_log::write('i', "path keys     : ", path_dir_keys);

  // Install helpers
  auto f_install_files = [&](Op const& op, fs::path path_dir_dst, std::vector<std::string> const& args)
  {
    // For each entry run the install command
    std::ranges::for_each(args, [&](fs::path e)
    {
      e = ns_fs::ns_path::canonical<true>(e)._ret;
      emulator_install_file(op, e, path_dir_dst / e.filename());
    });
  }; // f_install_roms

  // Get command
  switch(op)
  {
    case Op::GUI:
    {
      (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
        .with_piped_outputs()
        .with_var("FIM_XDG_CONFIG_HOME", path_dir_config.c_str())
        .with_var("FIM_XDG_DATA_HOME", path_dir_data.c_str())
        .with_args(path_flatimage)
        .spawn()
        .wait();
    }
    break;
    case Op::BIOS: { f_install_files(op, path_dir_bios, args); } break;
    case Op::ROM : { f_install_files(op, path_dir_rom , args); } break;
    case Op::CORE: { f_install_files(op, path_dir_core, args); } break;
    case Op::KEYS: { f_install_files(op, path_dir_keys, args); } break;
    default: "Invalid op in emulator install"_throw(); break;
  } // switch
} // emulator() }}}

} // anonymous namespace

// icon() {{{
inline void icon(std::string str_file_icon)
{
  // Current application
  std::string str_project = ns_db::query(ns_db::file_default(), "project");

  // Default working directory
  fs::path path_dir_project = ns_db::query(ns_db::file_default(), str_project, "path_dir_project");

  // Validate that file exists
  fs::path path_file_icon_src = ns_fs::ns_path::file_exists<true>(str_file_icon)._ret;

  // Create icon directory and set file name
  fs::path path_dir_icon = path_dir_project / "icon";
  ns_fs::ns_path::dir_create<true>(path_dir_icon);
  fs::path path_file_icon_dst = path_dir_icon / "icon.png";
  fs::path path_file_icon_gray_dst = path_dir_icon / "icon.grayscale.png";

  // Resize icon
  ns_image::resize(str_file_icon, path_file_icon_dst, 300, 450);

  // Create grayscale icon
  ns_image::grayscale(path_file_icon_dst, path_file_icon_gray_dst);

  // Save icon path in project database
  ns_db::from_file_project([&](auto&& db)
  {
    db("path_file_icon") = fs::relative(path_file_icon_dst, path_dir_project);
  }
  , ns_db::Mode::UPDATE);
} // icon() }}}

// remove() {{{
template<typename R>
void remove(Op const& op, fs::path const& path_dir_project, R&& files)
{
  // Remove Bios or Rom or Core...
  fs::path path_dir_item = ns_db::query(ns_db::file_project(), "path_dir_{}"_fmt(ns_enum::to_string_lower(op)));
  // Remove file by file
  std::ranges::for_each(files, [&](fs::path e){ remove_files(op, path_dir_project, path_dir_item / e.filename()); });
} // remove() }}}

// install() {{{
inline void install(Op op, std::vector<std::string> args)
{
  // Get platform
  std::string str_project = ns_db::query(ns_db::file_default(), "project");
  ns_enum::Platform enum_platform = ns_enum::from_string<ns_enum::Platform>(
    ns_db::query(ns_db::file_default(), str_project, "platform")
  );

  // Install based on platform
  switch(enum_platform)
  {
    case ns_enum::Platform::LINUX: ns_install::linux(op, args);
    break;
    case ns_enum::Platform::WINE: ns_install::wine(op, args);
    break;
    case ns_enum::Platform::RETROARCH:
    case ns_enum::Platform::PCSX2:
    case ns_enum::Platform::RPCS3:
    case ns_enum::Platform::RYUJINX: ns_install::emulator(op, args);
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
  fs::path rpath_dir_core = ns_db::query(ns_db::file_project(), "path_dir_core");

  if ( enum_platform != ns_enum::Platform::RETROARCH )
  {
    "Core install is only available for retroarch"_throw();
  } // if

  if ( op != Op::CORE )
  {
    "Only download of cores is available"_throw();
  } // if

  // Get project directory
  fs::path path_dir_project = ns_db::query(ns_db::file_default(), str_project, "path_dir_project");

  // Fetch cores / urls
  auto expected_vec_core_url = ns_fetch::cores_list(path_dir_project);
  ethrow_if(not expected_vec_core_url, expected_vec_core_url.error());

  // Put in a set
  std::set<std::string> set_cores(vec_cores.begin(), vec_cores.end());

  // Install if match
  for( auto i : *expected_vec_core_url )
  {
    if ( ! set_cores.contains(i.core) )
    {
      ns_log::write('i', "Skip ", i.core);
      continue;
    } // if

    fs::path path_file_out = path_dir_project / rpath_dir_core / i.core;
    ns_log::write('i', "Install ", i.core, " to ", path_file_out);
    auto expected_fetch_core = ns_fetch::fetch_file_from_url(path_file_out, i.url);
    econtinue_if(not expected_fetch_core, expected_fetch_core.error());
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
