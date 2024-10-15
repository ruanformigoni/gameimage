///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : global
///

#pragma once

#include "../db.hpp"

namespace ns_db::ns_global
{

namespace
{

namespace fs = std::filesystem;

class Db
{
  private:
    fs::path path_dir_build;
    std::string project;
    std::vector<std::string> projects;
    Db() = default;
  public:
    Db(Db const&) = delete;
    Db(Db&&) = delete;
    Db& operator=(Db const&) = delete;
    Db& operator=(Db&&) = delete;
}; // Db

// init_impl() {{{
inline void init_impl(fs::path const& path_dir_build
  , fs::path const& path_dir_project
  , fs::path const& path_dir_project_root 
  , fs::path const& path_file_image
  , ns_enum::Platform const& platform)
{
  ns_db::from_file_default([&](auto&& db_global)
  {
    // project name is Dir name
    std::string str_name = path_dir_project.filename();

    // build dir
    db_global("path_dir_build") = path_dir_build;

    // Set as default project
    db_global("project") = str_name;

    // Append to project list
    db_global("projects") |= str_name;

    // Set data
    db_global(str_name)("path_file_image")       = path_file_image;
    db_global(str_name)("path_dir_project")      = path_dir_project;
    db_global(str_name)("path_dir_project_root") = path_dir_project_root;
    db_global(str_name)("platform")              = ns_enum::to_string(platform);
  }
  , fs::exists(ns_db::file_default())? ns_db::Mode::UPDATE : ns_db::Mode::CREATE);
  
} // init_impl() }}}

} // namespace

// init() {{{
[[nodiscard]] inline std::error<std::string> init(fs::path const& path_dir_build
  , fs::path const& path_dir_project
  , fs::path const& path_dir_project_root 
  , fs::path const& path_file_image
  , ns_enum::Platform const& platform)
{
  return ns_exception::to_error([&]
  { 
    init_impl(path_dir_build
      , path_dir_project
      , path_dir_project_root
      , path_file_image
      , platform
    );
 });
} // init() }}}

} // namespace ns_db::ns_global

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
