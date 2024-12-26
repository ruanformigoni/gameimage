///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : wine
///

#include <unistd.h>

#include <filesystem>

#include "../common.hpp"
#include "../enum.hpp"

#include "../std/filesystem.hpp"
#include "../std/env.hpp"
#include "../std/copy.hpp"

#include "../lib/db/project.hpp"
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

// get_xdg_data_home() {{{
fs::path get_xdg_data_home()
{
  fs::path xdg_data_home;

  // Override if XDG_DATA_HOME
  if ( const char* str_data_home = ns_env::get("XDG_DATA_HOME"); str_data_home )
  {
    xdg_data_home = fs::path{str_data_home};
  } // if
  // Default to $HOME/.local/share
  else if ( const char* str_dir_home = ns_env::get("HOME"); str_dir_home )
  {
    xdg_data_home = fs::path{str_dir_home} / ".local/share";
  } // if
  else
  {
    "Could not determine XDG_DATA_HOME, is HOME set?"_throw();
  } // else
  // Log XDG_DATA_HOME

  ns_log::write('i', "XDG_DATA_HOME: '", xdg_data_home, "'");

  return xdg_data_home;
} // get_xdg_data_home() }}}

// db_files_copy() {{{
void db_files_copy(ns_db::ns_project::Project& db_project
  , ns_enum::Op const& op
  , fs::path const& path_dir_self
  , fs::path const& path_dir_dst)
{
  std::vector<fs::path> paths_file_entry = db_project.find_files(op);

  for(fs::path path_file_entry_src : paths_file_entry)
  {
    ns_log::write('i', "Found entry '", path_file_entry_src, "'");
    // Prepend working dir path
    path_file_entry_src = path_dir_self / path_file_entry_src;
    // Create path for copy destination
    fs::path path_file_entry_target = path_dir_dst / path_file_entry_src.filename();
    // Try to create directories to copy entry into
    ns_fs::ns_path::dir_create<true>(path_file_entry_target.parent_path());
    // Try to copy entry
    ns_copy::file(path_file_entry_src, path_file_entry_target);
  } // for
} // db_files_copy() }}}

// args() {{{
std::vector<std::string> args(fs::path const& path_dir_self, fs::path const& path_file_executable)
{
  // Standard path for wine args list
  fs::path path_file_args = path_dir_self / "gameimage.args.json";

  // Query if the executable has any arguments
  auto expected_arguments = ns_db::from_file<std::string>(path_file_args, [&](auto&& db)
  {
    return db.template value<std::string>(path_file_executable);
  }, ns_db::Mode::READ);

  // Check if has any argument
  return_if(not expected_arguments.has_value() or expected_arguments->empty()
    , (ns_log::write('i', "No arguments for ", path_file_executable), std::vector<std::string>{})
  )

  // Split arguments by space
  return ns_string::split(*expected_arguments, ' ');
} // args() }}}

// env() {{{
void env(fs::path const& path_dir_self)
{
  // Default path for env var list
  fs::path path_file_env = path_dir_self / "gameimage.env.json";

  // Set variables
  std::ignore = ns_db::from_file(path_file_env, [&](auto&& db)
  {
    for(auto&& e : db.keys())
    {
      if ( auto value = db.template value<std::string>(e) )
      {
        ns_env::set(e, *value, ns_env::Replace::Y);
        ns_log::write('i', "Set environment variable '", e, "' to '", *value, "'");
      } // if
      else
      {
        ns_log::write('e', "Failed to get value for key '", e, "'");
      } // else
    }
  }
  , ns_db::Mode::READ);
} // env() }}}

// boot_linux() {{{
void boot_linux(ns_db::ns_project::Project& db_project, fs::path const& path_dir_self)
{
  // Enter application directory
  fs::current_path(path_dir_self);

  // Create full path to rom
  fs::path path_file_rom_relative = db_project.path_file_rom;
  fs::path path_file_rom = path_dir_self / db_project.path_file_rom;

  // If GIMG_LAUNCHER_EXECUTABLE is defined, use it instead
  if ( const char* var = ns_env::get("GIMG_LAUNCHER_EXECUTABLE"); var != nullptr )
  {
    path_file_rom_relative = var;
    path_file_rom = path_dir_self / path_file_rom_relative;
  } // if

  // Include exec and read permissions (allow to fail)
  lec(fs::permissions, path_file_rom
    , fs::perms::owner_exec | fs::perms::group_exec | fs::perms::others_exec
    | fs::perms::owner_read | fs::perms::group_read | fs::perms::others_read
    , fs::perm_options::add
  );

  // Start application
  ns_log::write('i', "Execute: ", path_file_rom);

  auto optional_path_file_bash = ns_subprocess::search_path("bash");
  ereturn_if (not optional_path_file_bash, "Could not find bash");
  std::ignore = ns_subprocess::Subprocess(*optional_path_file_bash)
    .with_piped_outputs()
    .with_args("-c", R"("{}" "$@")"_fmt(path_file_rom))
    .with_args("--", args(path_dir_self, path_file_rom_relative))
    .spawn()
    .wait();
} // boot_linux() }}}

// boot_wine() {{{
void boot_wine(ns_db::ns_project::Project& db_project, fs::path const& path_dir_self)
{
  // Set wine prefix
  ns_env::set("WINEPREFIX", (path_dir_self / "wine").c_str(), ns_env::Replace::Y);

  // Binary to execute
  fs::path path_file_rom_relative = db_project.path_file_rom;
  fs::path path_file_rom = ns_fs::ns_path::file_exists<true>(path_dir_self / path_file_rom_relative)._ret;

  // If GIMG_LAUNCHER_EXECUTABLE is defined, use it instead
  if ( const char* var = ns_env::get("GIMG_LAUNCHER_EXECUTABLE"); var != nullptr )
  {
    path_file_rom_relative = var;
    path_file_rom = path_dir_self / path_file_rom_relative;
  } // if

  // Enter directory of rom file
  fs::current_path(ns_fs::ns_path::dir_exists<true>(path_file_rom.parent_path())._ret);

  // Start application
  std::ignore = ns_subprocess::Subprocess(ns_env::get_or_throw("FIM_BINARY_WINE"))
    .with_piped_outputs()
    .with_args(path_file_rom, args(path_dir_self, path_file_rom_relative))
    .spawn()
    .wait();
} // boot_wine() }}}

// boot_retroarch() {{{
void boot_retroarch(ns_db::ns_project::Project& db_project, fs::path const& path_dir_self)
{
  // Check if has bios
  db_files_copy(db_project, ns_enum::Op::BIOS, path_dir_self, (get_xdg_config_home() / "retroarch/system"));

  // Start application
  std::ignore = ns_subprocess::Subprocess(ns_env::get_or_throw("FIM_BINARY_RETROARCH"))
    .with_piped_outputs()
    .with_args("-L", path_dir_self / db_project.path_file_core, path_dir_self / db_project.path_file_rom)
    .spawn()
    .wait();
} // boot_retroarch() }}}

// boot_pcsx2() {{{
void boot_pcsx2(ns_db::ns_project::Project& db_project, fs::path const& path_dir_self)
{
  // Check if has bios
  db_files_copy(db_project, ns_enum::Op::BIOS, path_dir_self, ( get_xdg_config_home() / "PCSX2/bios"));

  // Start application
  std::ignore = ns_subprocess::Subprocess(ns_env::get_or_throw("FIM_BINARY_PCSX2"))
    .with_piped_outputs()
    .with_args("--", path_dir_self / db_project.path_file_rom)
    .spawn()
    .wait();
} // boot_pcsx2() }}}

// boot_rpcs3() {{{
void boot_rpcs3(ns_db::ns_project::Project& db_project, fs::path const& path_dir_self)
{
  std::ignore = ns_subprocess::Subprocess(ns_env::get_or_throw("FIM_BINARY_RPCS3"))
    .with_piped_outputs()
    .with_args("--allow-any-location", "--no-gui", "--", path_dir_self / db_project.path_file_rom)
    .spawn()
    .wait();
} // boot_rpcs3() }}}

// boot() {{{
void boot(int argc, char** argv)
{
  // Path to self directory
  fs::path path_dir_self = ns_fs::ns_path::dir_self<true>()._ret;

  // Set HOME
  ns_env::set("FIM_HOME", path_dir_self.c_str(), ns_env::Replace::Y);

  // Start log
  ns_log::init(argc, argv, path_dir_self / "gameimage.log");

  // Try to set env
  try
  {
    env(path_dir_self);
  }
  catch(std::exception const& e)
  {
    ns_log::write('e', "Could not set environment: ", e.what());
  } // catch

  // // Adjust environment
  // ns_env::set("LC_ALL", "C", ns_env::Replace::N);

  // Flatimage distribution
  ns_log::write('i', "FlatImage distribution: ", ns_env::get_or_throw("FIM_DIST"));

  // Open db
  auto db_project = ns_db::ns_project::read(path_dir_self / "gameimage.json");
  ethrow_if(not db_project, "Could not open build database");

  // Database file
  ns_enum::Platform platform = db_project->platform;

  switch(platform)
  {
    case ns_enum::Platform::LINUX    : boot_linux(*db_project, path_dir_self)     ; break;
    case ns_enum::Platform::WINE     : boot_wine(*db_project, path_dir_self)      ; break;
    case ns_enum::Platform::RETROARCH: boot_retroarch(*db_project, path_dir_self) ; break;
    case ns_enum::Platform::PCSX2    : boot_pcsx2(*db_project, path_dir_self)     ; break;
    case ns_enum::Platform::RPCS3    : boot_rpcs3(*db_project, path_dir_self)     ; break;
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
