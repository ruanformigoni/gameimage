///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : build
///

#pragma once

#include "../db.hpp"
#include "../../std/env.hpp"

namespace ns_db::ns_build
{

struct Metadata
{
  std::string name;
  fs::path path_dir_project;
  fs::path path_dir_project_root;
  ns_enum::Platform platform;
};

namespace
{

class Build
{
  private:
    fs::path path_file_db;
    Build(fs::path const& path_file_db)
      : path_file_db(path_file_db)
    {};
  public:
    fs::path path_dir_build;
    std::string project;
    fs::path path_file_image;
    std::vector<Metadata> projects;
    Metadata& find(std::string_view name);
  friend Build read_impl(fs::path path_file_db);
  friend void write_impl(Build const& build);
}; // Build

// find() {{{
Metadata& Build::find(std::string_view name)
{
  auto search = std::ranges::find_if(projects, [&](auto&& e) { return e.name == name; });
  ethrow_if(search == std::ranges::end(projects), "Could not find project '{}'"_fmt(project));
  return *search;
} // find() }}}

namespace fs = std::filesystem;

// init_impl() {{{
inline void init_impl(fs::path const& path_dir_build)
{
  // Create build directory
  fs::create_directories(path_dir_build);
  // Create database
  fs::path path_file_database = path_dir_build / "gameimage.json";
  ns_db::from_file(path_file_database, [&](auto&& db)
  {
    // build dir
    db("path_dir_build") = path_dir_build;

    // Location of linux image
    db("path_file_image") = path_dir_build / "cache/linux.flatimage";

    // Set as default project
    db("project") = "";

    // Projects array
    db("projects") = ns_db::object_t();
  }
  , fs::exists(path_file_database)? ns_db::Mode::UPDATE : ns_db::Mode::CREATE);
} // init_impl() }}}

// read_impl() {{{
Build read_impl(fs::path path_file_db)
{
  Build build(path_file_db);
  ns_db::from_file(path_file_db, [&](auto&& db)
  {
    build.project = db["project"];
    build.path_dir_build = fs::path{db["path_dir_build"]};
    build.path_file_image =  fs::path{db["path_file_image"]};;
    for( auto [name, obj] : db["projects"].items() )
    {
      Metadata metadata;
      metadata.name = name;
      metadata.path_dir_project =  fs::path{obj["path_dir_project"]};
      metadata.path_dir_project_root =  fs::path{obj["path_dir_project_root"]};
      metadata.platform =  ns_enum::from_string<ns_enum::Platform>(obj["platform"]);
      build.projects.push_back(metadata);
    } // for
  }, ns_db::Mode::READ);
  return build;
} // read_impl() }}}

// write_impl() {{{
void write_impl(Build const& build)
{
  ns_db::from_file(build.path_file_db, [&](auto&& db)
  {
    db("project") = build.project;
    db("path_dir_build") = build.path_dir_build;
    db("path_file_image") = build.path_file_image;
    for( auto metadata : build.projects )
    {
      db("projects")(metadata.name)("path_dir_project") = metadata.path_dir_project;
      db("projects")(metadata.name)("path_dir_project_root") = metadata.path_dir_project_root;
      db("projects")(metadata.name)("platform") = ns_enum::to_string(metadata.platform);
    } // for
  }, ns_db::Mode::CREATE);
} // write_impl() }}}

} // namespace

// init() {{{
[[nodiscard]] inline std::error<std::string> init(fs::path const& path_dir_build)
{
  return ns_exception::to_error([&]
  {
    init_impl(path_dir_build);
 });
} // init() }}}

// read() {{{
inline std::expected<Build,std::string> read()
{
  return ns_exception::to_expected([&]
  {
    return read_impl(fs::path{ns_env::get_or_throw("GIMG_DIR")} / "gameimage.json");
  });
} // read() }}}

// write() {{{
inline std::error<std::string> write(Build const& build)
{
  return ns_exception::to_error([&]
  {
    write_impl(build);
  });
} // write() }}}

} // namespace ns_db::ns_build

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
