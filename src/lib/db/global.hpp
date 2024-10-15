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

struct Metadata
{
  std::string name;
  fs::path path_dir_project;
  fs::path path_dir_project_root;
  fs::path path_file_image;
  ns_enum::Platform platform;
};

class Global
{
  private:
    fs::path path_file_db;
    Global(fs::path const& path_file_db)
      : path_file_db(path_file_db)
    {};
  public:
    fs::path path_dir_build;
    std::string project;
    std::vector<Metadata> projects;
    Metadata& find(std::string_view name);
  friend Global read_impl(fs::path path_file_db);
}; // Global

// find() {{{
Metadata& Global::find(std::string_view name)
{
  auto search = std::ranges::find_if(projects, [&](auto&& e){ return e.name == name; });
  ethrow_if(search == std::ranges::end(projects), "Could not find project '{}'"_fmt(project));
  return *search;
} // find() }}}

namespace fs = std::filesystem;

// init_impl() {{{
inline void init_impl(fs::path const& path_dir_build
  , fs::path const& path_dir_project
  , fs::path const& path_dir_project_root
  , fs::path const& path_file_image
  , ns_enum::Platform const& platform)
{
  ns_db::from_file_default([&](auto&& db)
  {
    // project name is Dir name
    std::string str_name = path_dir_project.filename();

    // build dir
    db("path_dir_build") = path_dir_build;

    // Set as default project
    db("project") = str_name;

    // Set data
    db("projects")(str_name)("path_file_image")       = path_file_image;
    db("projects")(str_name)("path_dir_project")      = path_dir_project;
    db("projects")(str_name)("path_dir_project_root") = path_dir_project_root;
    db("projects")(str_name)("platform")              = ns_enum::to_string(platform);
  }
  , fs::exists(ns_db::file_default())? ns_db::Mode::UPDATE : ns_db::Mode::CREATE);

} // init_impl() }}}

// read_impl() {{{
Global read_impl(fs::path path_file_db)
{
  Global global(path_file_db);
  ns_db::from_file(path_file_db, [&](auto&& db)
  {
    global.project = db["project"];
    global.path_dir_build = fs::path{db["path_dir_build"]};
    for( auto [name, obj] : db["projects"].items() )
    {
      Metadata metadata;
      metadata.name = name;
      metadata.path_dir_project =  fs::path{obj["path_dir_project"]};
      metadata.path_dir_project_root =  fs::path{obj["path_dir_project_root"]};
      metadata.path_file_image =  fs::path{obj["path_file_image"]};
      metadata.platform =  ns_enum::from_string<ns_enum::Platform>(obj["platform"]);
      global.projects.push_back(metadata);
    } // for
  }, ns_db::Mode::READ);
  return global;
} // read_impl() }}}

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

// read() {{{
inline std::expected<Global,std::string> read(fs::path const& path_dir_project)
{
  return ns_exception::to_expected([&]
  {
    return read_impl(path_dir_project);
  });
} // read() }}}

} // namespace ns_db::ns_global

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
