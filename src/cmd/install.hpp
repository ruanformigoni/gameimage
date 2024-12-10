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
#include "../lib/db/build.hpp"
#include "../lib/db/project.hpp"
#include "../lib/zip.hpp"
#include "../lib/image.hpp"

#include "fetch.hpp"

namespace ns_install
{

namespace fs = std::filesystem;

using Op = ns_enum::Op;

namespace
{

// remove_files() {{{
void remove_files(ns_db::ns_project::Project& project
  , Op const& op
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

  // Remove from database
  project.erase(op, rpath_file_target);
  ns_db::ns_project::write(project);
} // remove_files() }}}

// wine() {{{
inline void wine(fs::path const& path_file_image,
  ns_db::ns_build::Metadata& db_metadata
  , Op const& op, std::vector<std::string> args)
{
  // Path to wine prefix
  fs::path path_wineprefix = db_metadata.path_dir_project / "wine";

  // Log
  ns_log::write('i', "application: ", db_metadata.name);
  ns_log::write('i', "image: ", path_file_image);
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
      .with_args(path_file_image, "fim-exec", "/opt/wine/bin/wine.sh", std::forward<T>(t))
      .spawn()
      .wait();
  };

  auto f_winetricks = [&]<typename T>(T&& t)
  {
    (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
      .with_piped_outputs()
      .with_args(path_file_image, "fim-exec", "/opt/wine/bin/wine.sh", "winetricks", std::forward<T>(t))
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
inline void linux(fs::path const& path_file_image
  , ns_db::ns_build::Metadata& db_metadata
  , Op const& op, std::vector<std::string> args)
{
  // Validate op
  ethrow_if ( op != Op::ROM,  "Only ROM is valid for the linux platform");

  // Project dir
  fs::path path_dir_linux = db_metadata.path_dir_project / "linux";
  lec(fs::create_directories, path_dir_linux);

  // Run selected file
  (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
    .with_piped_outputs()
    .with_args(path_file_image, "fim-exec", "env", "HOME={}"_fmt(path_dir_linux.c_str()), args)
    .spawn()
    .wait();
} // linux() }}}

// emulator_install_file() {{{
void emulator_install_file(ns_db::ns_build::Metadata& db_metadata, Op const& op, fs::path path_file_src, fs::path const& path_file_dst)
{
  ereturn_if( op == Op::KEYS, "No platform uses 'keys'");

  // Path to project
  ns_log::write('i', "Copy ", path_file_src, " to ", path_file_dst);

  // // Check if src and dst are the same
  if ( path_file_src == path_file_dst )
  {
    ns_log::write('i', "Src and dst are the same for '{}'"_fmt(path_file_src));
  }
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
  fs::path path_file_dst_relative = fs::relative(path_file_dst, db_metadata.path_dir_project);

  // Save in database
  auto db_project = ns_db::ns_project::read();
  ereturn_if(not db_project, "Could not open project database");
  // Check if is installed in database
  auto op_files = db_project->find_files(op);
  ireturn_if(std::ranges::contains(op_files, path_file_dst_relative), "File '{}' is already installed"_fmt(path_file_dst_relative));
  // Include in database
  db_project->append(op, path_file_dst_relative);
  db_project->set_default(op, path_file_dst_relative);
  ns_db::ns_project::write(*db_project);
} // emulator_install_file() }}}

// emulator() {{{
inline void emulator(fs::path const& path_file_image
  , ns_db::ns_build::Metadata& db_metadata
  , Op op
  , std::vector<std::string> args)
{
  auto db_project = ns_db::ns_project::read();
  ethrow_if(not db_project, "Could not read project database");

  // Install paths
  fs::path path_dir_config = db_metadata.path_dir_project / db_project->path_dir_config;
  fs::path path_dir_data   = db_metadata.path_dir_project / db_project->path_dir_data;
  fs::path path_dir_rom    = db_metadata.path_dir_project / db_project->path_dir_rom;
  fs::path path_dir_core   = db_metadata.path_dir_project / db_project->path_dir_core;
  fs::path path_dir_bios   = db_metadata.path_dir_project / db_project->path_dir_bios;

  // Log
  ns_log::write('i', "application   : ", db_metadata.name);
  ns_log::write('i', "path config   : ", path_dir_config);
  ns_log::write('i', "path core     : ", path_dir_core);
  ns_log::write('i', "path rom      : ", path_dir_rom);
  ns_log::write('i', "path bios     : ", path_dir_bios);

  // Install helpers
  auto f_install_files = [&](Op const& op, fs::path path_dir_dst, std::vector<std::string> const& args)
  {
    // For each entry run the install command
    std::ranges::for_each(args, [&](fs::path e)
    {
      e = ns_fs::ns_path::canonical<true>(e)._ret;
      emulator_install_file(db_metadata, op, e, path_dir_dst / e.filename());
    });
  }; // f_install_roms

  // Get command
  switch(op)
  {
    case Op::GUI:
    {
      (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
        .with_piped_outputs()
        .with_args(path_file_image, "fim-exec", "/opt/rpcs3/boot")
        .spawn()
        .wait();
    }
    break;
    case Op::BIOS: { f_install_files(op, path_dir_bios, args); } break;
    case Op::ROM : { f_install_files(op, path_dir_rom , args); } break;
    case Op::CORE: { f_install_files(op, path_dir_core, args); } break;
    default: "Invalid op in emulator install"_throw(); break;
  } // switch
} // emulator() }}}

} // anonymous namespace

// icon() {{{
inline void icon(ns_db::ns_build::Metadata& db_metadata, std::string str_file_icon)
{
  // Validate that file exists
  fs::path path_file_icon_src = ns_fs::ns_path::file_exists<true>(str_file_icon)._ret;

  // Create icon directory and set file name
  fs::path path_dir_icon = db_metadata.path_dir_project / "icon";
  ns_fs::ns_path::dir_create<true>(path_dir_icon);
  fs::path path_file_icon_dst = path_dir_icon / "icon.png";
  fs::path path_file_icon_gray_dst = path_dir_icon / "icon.grayscale.png";

  // Resize icon
  ns_image::resize(str_file_icon, path_file_icon_dst, 300, 450);

  // Create grayscale icon
  ns_image::grayscale(path_file_icon_dst, path_file_icon_gray_dst);

  // Save icon path in project database
  auto db_project = ns_db::ns_project::read();
  ethrow_if(not db_project, "Could not read project database");
  db_project->path_file_icon = fs::relative(path_file_icon_dst, db_metadata.path_dir_project);
  ns_db::ns_project::write(*db_project);
} // icon() }}}

// remove() {{{
template<typename R>
void remove(Op const& op, fs::path const& path_dir_project, R&& files)
{
  auto db_project = ns_db::ns_project::read();
  ethrow_if(not db_project, "Could not read project database");

  fs::path path_dir_item = db_project->find_directory(op);

  // Remove file by file
  std::ranges::for_each(files, [&](fs::path e)
  {
    remove_files(*db_project, op, path_dir_project, path_dir_item / e.filename());
  });
} // remove() }}}

// install() {{{
inline void install(Op op, std::vector<std::string> args)
{
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Could not open build database '{}'"_fmt(db_build.error()));
  auto db_metadata = db_build->find(db_build->project);

  // Install based on platform
  switch(db_metadata.platform)
  {
    case ns_enum::Platform::LINUX: ns_install::linux(db_build->path_file_image, db_metadata, op, args);
    break;
    case ns_enum::Platform::WINE: ns_install::wine(db_build->path_file_image, db_metadata, op, args);
    break;
    case ns_enum::Platform::RETROARCH:
    case ns_enum::Platform::PCSX2:
    case ns_enum::Platform::RPCS3: ns_install::emulator(db_build->path_file_image, db_metadata, op, args);
    break;
  } // switch

} // install() }}}

// remote() {{{
inline void remote(Op const& op, std::vector<std::string> vec_cores)
{
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Could not open build database '{}'"_fmt(db_build.error()));
  auto db_metadata = db_build->find(db_build->project);
  auto db_project = ns_db::ns_project::read();
  ethrow_if(not db_project, "Could not open project database '{}'"_fmt(db_project.error()));
  // Core dir
  ethrow_if(db_project->platform != ns_enum::Platform::RETROARCH, "Core install is only available for retroarch" );
  ethrow_if(op != Op::CORE, "Only download of cores is available" );
  // Fetch cores / urls
  auto expected_vec_core_url = ns_fetch::fetch_cores();
  ethrow_if(not expected_vec_core_url, expected_vec_core_url.error());
  // Install if match
  for( auto [core,url] : *expected_vec_core_url )
  {
    // Skip un-selected core
    qcontinue_if(std::ranges::find(vec_cores, core) == std::ranges::end(vec_cores));
    // Check file type
    econtinue_if (not core.ends_with(".zip"), "Invalid file format, it should be 'zip'");
    // Create output file path
    fs::path path_file_zip = db_metadata.path_dir_project / db_project->path_dir_core / core;
    fs::path path_file_core = db_metadata.path_dir_project / db_project->path_dir_core / fs::path{core}.replace_extension("");
    ns_log::write('i', "Install ", path_file_zip, " to ", path_file_core);
    // Download file
    auto expected_fetch_core = ns_fetch::fetch_file_from_url(path_file_zip, url);
    econtinue_if(not expected_fetch_core, expected_fetch_core.error());
    // Extract zip
    ns_zip::extract(path_file_zip, path_file_core.parent_path());
    // Erase zip
    fs::remove(path_file_zip);
    // Install (file ends with .so.zip, remove the .zip to the get extracted file name)
    install(op, std::vector<std::string>{path_file_core});
  } // for

} // remote() }}}

} // namespace ns_install

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
