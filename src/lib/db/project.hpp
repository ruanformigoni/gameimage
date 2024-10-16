///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : project
///

#pragma once

#include "../db.hpp"

namespace ns_db::ns_project
{

namespace
{

namespace fs = std::filesystem;

// struct Project {{{
class Project
{
  private:
    fs::path m_path_file_db;
    Project(fs::path const& path_file_db)
      : m_path_file_db(path_file_db)
    {};
  public:
    std::string project;
    ns_enum::Platform platform;
    fs::path path_dir_config;
    fs::path path_dir_data;
    fs::path path_dir_bios;
    fs::path path_dir_rom;
    fs::path path_dir_core;
    fs::path path_dir_keys;
    fs::path path_dir_linux;
    fs::path path_file_bios;
    fs::path path_file_core;
    fs::path path_file_icon;
    fs::path path_file_rom;
    std::vector<fs::path> paths_file_bios;
    std::vector<fs::path> paths_file_core;
    std::vector<fs::path> paths_file_rom;
    void set_default(ns_enum::Op const op, fs::path const& rpath_file_target);
    void append(ns_enum::Op const op, fs::path const& rpath_file_target);
    void erase(ns_enum::Op const op, fs::path const& rpath_file_target);
    fs::path find_directory(ns_enum::Op const op);
    fs::path find_file(ns_enum::Op const op);
    std::vector<fs::path> const& find_files(ns_enum::Op const op);
  friend Project read_impl(fs::path path_file_db);
  friend void write_impl(Project const& project);
}; // struct Project }}}

// set_default() {{{
void Project::set_default(ns_enum::Op const op, fs::path const& rpath_file_target)
{
  ethrow_if(not rpath_file_target.is_relative(), "Path '{}' is not relative"_fmt(rpath_file_target));

  switch(op)
  {
    case ns_enum::Op::BIOS: path_file_bios = rpath_file_target; break;
    case ns_enum::Op::CORE: path_file_core = rpath_file_target; break;
    case ns_enum::Op::ROM: path_file_rom = rpath_file_target; break;
    default: throw std::runtime_error("Invalid item to make default");
  } // switch
} // set_default() }}}

// append() {{{
void Project::append(ns_enum::Op const op, fs::path const& rpath_file_target)
{
  ethrow_if(not rpath_file_target.is_relative(), "Path '{}' is not relative"_fmt(rpath_file_target));

  switch(op)
  {
    case ns_enum::Op::BIOS: paths_file_bios.push_back(rpath_file_target); break;
    case ns_enum::Op::CORE: paths_file_core.push_back(rpath_file_target); break;
    case ns_enum::Op::ROM: paths_file_rom.push_back(rpath_file_target); break;
    default: throw std::runtime_error("Invalid item to append");
  } // switch
} // append() }}}

// erase() {{{
void Project::erase(ns_enum::Op const op, fs::path const& rpath_file_target)
{
  ethrow_if(not rpath_file_target.is_relative(), "Path '{}' is not relative"_fmt(rpath_file_target));

  switch(op)
  {
    case ns_enum::Op::BIOS:
      if( rpath_file_target == path_file_bios ) { path_file_bios = ""; }
      std::erase_if(paths_file_bios, [&](auto&& e){ return e == rpath_file_target; });
      break;
    case ns_enum::Op::CORE:
      if( rpath_file_target == path_file_core ) { path_file_core = ""; }
      std::erase_if(paths_file_core, [&](auto&& e){ return e == rpath_file_target; });
      break;
    case ns_enum::Op::ROM:
      if( rpath_file_target == path_file_rom ) { path_file_rom = ""; }
      std::erase_if(paths_file_rom, [&](auto&& e){ return e == rpath_file_target; });
      break;
    case ns_enum::Op::ICON:
      if( rpath_file_target == path_file_icon ) { path_file_icon = ""; }
      break;
    default: throw std::runtime_error("Invalid item to delete");
  } // switch
} // erase() }}}

// find_directory() {{{
fs::path Project::find_directory(ns_enum::Op const op)
{
  switch(op)
  {
    case ns_enum::Op::CONFIG: return path_dir_config; break;
    case ns_enum::Op::DATA: return path_dir_data; break;
    case ns_enum::Op::BIOS: return path_dir_bios; break;
    case ns_enum::Op::ROM: return path_dir_rom; break;
    case ns_enum::Op::CORE: return path_dir_core; break;
    case ns_enum::Op::KEYS: return path_dir_keys; break;
    case ns_enum::Op::LINUX: return path_dir_linux; break;
    default: throw std::runtime_error("Invalid item to get directory for");
  } // switch
} // find_directory() }}}

// find_file() {{{
fs::path Project::find_file(ns_enum::Op const op)
{
  switch(op)
  {
    case ns_enum::Op::BIOS: return path_file_bios; break;
    case ns_enum::Op::CORE: return path_file_core; break;
    case ns_enum::Op::ROM: return path_file_rom; break;
    case ns_enum::Op::ICON: return path_file_icon; break;
    default: throw std::runtime_error("Invalid item to get directory for");
  } // switch
} // find_file() }}}

// find_file() {{{
std::vector<fs::path> const& Project::find_files(ns_enum::Op const op)
{
  switch(op)
  {
    case ns_enum::Op::BIOS: return paths_file_bios; break;
    case ns_enum::Op::CORE: return paths_file_core; break;
    case ns_enum::Op::ROM: return paths_file_rom; break;
    default: throw std::runtime_error("Invalid item to get directory for");
  } // switch
} // find_file() }}}

// init_impl() {{{
void init_impl(fs::path const& path_dir_project, ns_enum::Platform const& platform)
{
  // Configure data directory names
  fs::path path_dir_config   = "config";
  fs::path path_dir_data     = "data";
  fs::path path_dir_rom      = "rom";
  fs::path path_dir_core     = "core";
  fs::path path_dir_bios     = "bios";
  fs::path path_dir_keys     = "keys";
  fs::path path_dir_linux    = "linux";

  // Create directories
  ns_fs::ns_path::dir_create<true>(path_dir_project);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_config);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_data);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_rom);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_core);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_bios);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_keys);
  ns_fs::ns_path::dir_create<true>(path_dir_project / path_dir_linux);

  // Log created directories
  ns_log::write('i', "path_dir_config       :", path_dir_config);
  ns_log::write('i', "path_dir_data         :", path_dir_data);
  ns_log::write('i', "path_dir_rom          :", path_dir_rom);
  ns_log::write('i', "path_dir_core         :", path_dir_core);
  ns_log::write('i', "path_dir_bios         :", path_dir_bios);
  ns_log::write('i', "path_dir_keys         :", path_dir_keys);
  ns_log::write('i', "path_dir_linux        :", path_dir_linux);


  // Set project data
  ns_db::from_file_project([&](auto&& db_project)
  {
    db_project("project")         = path_dir_project.filename();
    db_project("platform")        = ns_enum::to_string(platform);
    db_project("path_dir_config") = path_dir_config;
    db_project("path_dir_data")   = path_dir_data;
    db_project("path_dir_bios")   = path_dir_bios;
    db_project("path_dir_rom")    = path_dir_rom;
    db_project("path_dir_core")   = path_dir_core;
    db_project("path_dir_keys")   = path_dir_keys;
    db_project("path_dir_linux")  = path_dir_linux;
    db_project("path_file_bios")  = "";
    db_project("path_file_core")  = "";
    db_project("path_file_icon")  = "";
    db_project("path_file_rom")   = "";
    db_project("paths_file_bios") = std::vector<fs::path>();
    db_project("paths_file_core") = std::vector<fs::path>();
    db_project("paths_file_rom")  = std::vector<fs::path>();
  }
  , ns_db::Mode::CREATE);
} // init_impl() }}}

// read_impl() {{{
Project read_impl(fs::path path_file_db)
{
  Project project(path_file_db);
  ns_db::from_file(path_file_db, [&](auto&& db)
  {
    project.project         = fs::path{db["project"]};
    project.platform        = ns_enum::from_string<ns_enum::Platform>(fs::path{db["platform"]});
    project.path_dir_config = fs::path{db["path_dir_config"]};
    project.path_dir_data   = fs::path{db["path_dir_data"]};
    project.path_dir_bios   = fs::path{db["path_dir_bios"]};
    project.path_dir_rom    = fs::path{db["path_dir_rom"]};
    project.path_dir_core   = fs::path{db["path_dir_core"]};
    project.path_dir_keys   = fs::path{db["path_dir_keys"]};
    project.path_dir_linux  = fs::path{db["path_dir_linux"]};
    project.path_file_bios  = fs::path{db["path_file_bios"]};
    project.path_file_core  = fs::path{db["path_file_core"]};
    project.path_file_icon  = fs::path{db["path_file_icon"]};
    project.path_file_rom   = fs::path{db["path_file_rom"]};
    project.paths_file_bios = db["paths_file_bios"].template to_vector<fs::path>();
    project.paths_file_core = db["paths_file_core"].template to_vector<fs::path>();
    project.paths_file_rom  = db["paths_file_rom"].template to_vector<fs::path>();
  }, ns_db::Mode::READ);
  return project;
} // read_impl() }}}

// write_impl() {{{
void write_impl(Project const& project)
{
  ns_db::from_file(project.m_path_file_db, [&](auto&& db)
  {
    db("project") = project.project;
    db("platform") = ns_enum::to_string(project.platform);
    db("path_dir_config") = project.path_dir_config;
    db("path_dir_data") = project.path_dir_data;
    db("path_dir_bios") = project.path_dir_bios;
    db("path_dir_rom") = project.path_dir_rom;
    db("path_dir_core") = project.path_dir_core;
    db("path_dir_keys") = project.path_dir_keys;
    db("path_dir_linux") = project.path_dir_linux;
    db("path_file_bios")  = project.path_file_bios;
    db("path_file_core")  = project.path_file_core;
    db("path_file_icon")  = project.path_file_icon;
    db("path_file_rom")   = project.path_file_rom;
    db("paths_file_bios") = project.paths_file_bios;
    db("paths_file_core") = project.paths_file_core;
    db("paths_file_rom")  = project.paths_file_rom;
  }, ns_db::Mode::CREATE);
} // write_impl() }}}

} // namespace

// init() {{{
inline std::error<std::string> init(fs::path const& path_dir_project, ns_enum::Platform const& platform)
{
  return ns_exception::to_error([&]
  {
    init_impl(path_dir_project, platform);
  });
} // init() }}}

// read() {{{
inline std::expected<Project,std::string> read()
{
  return ns_exception::to_expected([&]
  {
    return read_impl(ns_db::file_project());
  });
} // read() }}}

// write() {{{
inline std::error<std::string> write(Project const& project)
{
  return ns_exception::to_error([&]
  {
    write_impl(project);
  });
} // write() }}}

} // namespace ns_db::ns_project

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
