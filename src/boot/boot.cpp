///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : wine
///

#include <unistd.h>
#include <limits.h>

#include <filesystem>

#include "../common.hpp"
#include "../enum.hpp"

#include "../std/filesystem.hpp"
#include "../std/env.hpp"
#include "../std/copy.hpp"

#include "../lib/db.hpp"
#include "../lib/subprocess.hpp"

// Start logging
INITIALIZE_EASYLOGGINGPP

namespace fs = std::filesystem;

// get_xdg_config_home() {{{
fs::path get_xdg_config_home()
{
  fs::path xdg_config_home;

  // Override if XDG_CONFIG_HOME
  if ( const char* str_config_home = ns_env::get("XDG_CONFIG_HOME"); str_config_home )
  {
    xdg_config_home = fs::path{str_config_home};
  } // if
  // Default to $HOME/.config
  else if ( const char* str_dir_home = ns_env::get("HOME"); str_dir_home )
  {
    xdg_config_home = fs::path{str_dir_home} / ".config";
  } // if
  else
  {
    "Could not determine XDG_CONFIG_HOME, is HOME set?"_throw();
  } // else
  // Log XDG_CONFIG_HOME

  ns_log::write('i', "XDG_CONFIG_HOME: '", xdg_config_home, "'");

  return xdg_config_home;
} // get_xdg_config_home() }}}

// db_files_copy() {{{
void db_files_copy(std::string db_entry
  , fs::path const& path_file_database
  , fs::path const& path_dir_src
  , fs::path const& path_dir_dst)
{
  // Check if has entry
  ns_db::from_file(path_file_database.c_str()
  , [&](auto&& db)
  {
    if ( ! db.template contains<false>(db_entry))
    {
      return;
    } // if

    std::vector<fs::path> paths_file_entry;

    for (auto&& path_file_entry : db[db_entry])
    {
      paths_file_entry.push_back(path_file_entry);
      ns_log::write('i', "Found entry '", path_file_entry, "'");
    } // for

    for(fs::path path_file_entry_src : paths_file_entry)
    {
      // Prepend working dir path
      path_file_entry_src = path_dir_src / path_file_entry_src;
      // Create path for copy destination
      fs::path path_file_entry_target = path_dir_dst / path_file_entry_src.filename();
      // Try to create directories to copy entry into
      ns_fs::ns_path::dir_create<true>(path_file_entry_target.parent_path());
      // Try to copy entry
      ns_copy::file(path_file_entry_src, path_file_entry_target);
    } // for
  } , std::ios_base::in); // ns_db::from_file
} // db_files_copy() }}}

// boot() {{{
void boot(int argc, char** argv)
{
  // Path to self directory
  fs::path path_dir_self = ns_fs::ns_path::dir_self<true>()._ret;

  // Start log
  ns_log::init(argc, argv, path_dir_self / "gameimage.log");

  // Adjust environment
  ns_env::set("LC_ALL", "C", ns_env::Replace::N);

  // Flatimage distribution
  ns_log::write('i', "FlatImage distribution: ", ns_env::get("FIM_DIST"));

  // Get platform
  ns_enum::Platform platform;

  fs::path path_file_database = path_dir_self / "gameimage.json";

  // Database file
  ns_db::from_file(path_file_database
  , [&](auto&& db)
  {
    platform = ns_enum::from_string<ns_enum::Platform>(std::string(db["platform"]));
  }
  , std::ios_base::in);

  switch(platform)
  {
    case ns_enum::Platform::WINE:
    {
      // Set wine prefix
      ns_env::set("WINEPREFIX", (path_dir_self / "wine").c_str(), ns_env::Replace::Y);

      // Binary to execute
      fs::path path_file_rom;

      // Database
      ns_db::from_file(path_file_database
      , [&](auto&& db)
      {
        path_file_rom = ns_fs::ns_path::file_exists<true>(path_dir_self / db["path-file-rom"])._ret;
      }
      , std::ios_base::in);

      // Enter directory of rom file
      fs::current_path(ns_fs::ns_path::dir_exists<true>(path_file_rom.parent_path())._ret);

      // Get boot command
      std::string str_cmd = ns_env::get("FIM_BINARY_WINE");

      // Start application
      ns_subprocess::sync(str_cmd.c_str(), path_file_rom);
    } // case
    break;
    case ns_enum::Platform::RETROARCH:
    {
      // Rom
      fs::path path_file_rom;

      // Core
      fs::path path_file_core;

      // Database
      ns_db::from_file(path_file_database
      , [&](auto&& db)
      {
        // Rom
        path_file_rom = ns_fs::ns_path::file_exists<true>(path_dir_self / db["path-file-rom"])._ret;

        // Core
        path_file_core = ns_fs::ns_path::file_exists<true>(path_dir_self / db["path-file-core"])._ret;
      }
      , std::ios_base::in);

      // Check if has bios
      db_files_copy("paths-file-bios"
        , path_file_database
        , path_dir_self
        , ( get_xdg_config_home() / "retroarch/system")
      );

      // Get boot command
      std::string str_cmd = ns_env::get("FIM_BINARY_RETROARCH");

      // Start application
      ns_subprocess::sync(str_cmd.c_str(), "-L", path_file_core, path_file_rom);
    } // case
    break;
    case ns_enum::Platform::PCSX2:
    {
      // Rom
      fs::path path_file_rom;

      // Bios
      fs::path path_file_bios;

      // Database
      ns_db::from_file(path_file_database
      , [&](auto&& db)
      {
        // Rom
        path_file_rom = ns_fs::ns_path::file_exists<true>(path_dir_self / db["path-file-rom"])._ret;

        // Bios
        path_file_bios = ns_fs::ns_path::file_exists<true>(path_dir_self / db["path-file-bios"])._ret;
      }
      , std::ios_base::in);

      // Check if has bios
      db_files_copy("paths-file-bios"
        , path_file_database
        , path_dir_self
        , ( get_xdg_config_home() / "PCSX2/bios")
      );

      // Get boot command
      std::string str_cmd = ns_env::get("FIM_BINARY_PCSX2");

      // Start application
      ns_subprocess::sync(str_cmd.c_str(), "--", path_file_rom);
    } // case
  } // switch
} // function: boot }}}

// main() {{{
int main(int argc, char** argv)
{
  try
  {
    boot(argc, argv);
  } // try
  catch(std::exception const& e)
  {
    fmt::println("Could not boot program with error: {}", e.what());
  } // catch
} // main() }}}

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
