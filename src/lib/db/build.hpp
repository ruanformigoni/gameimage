///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : build
///

#pragma once

#include "../db.hpp"
#include "../../std/env.hpp"
#include "../hope.hpp"

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
    std::string project;
    fs::path path_dir_build;
    fs::path path_dir_cache;
    fs::path path_file_image;
    fs::path path_file_output;
    std::string dist_wine;
    std::vector<Metadata> projects;
    Metadata& find(std::string_view name);
  friend std::expected<Build,std::string> read_impl(fs::path path_file_db);
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
  std::ignore = ns_db::open(path_file_database, [&](ns_db::Db& db)
  {
    // build dir
    db("path_dir_build") = path_dir_build;
    // cache dir
    fs::path path_dir_cache = path_dir_build / "cache";
    db("path_dir_cache") = path_dir_cache;
    // Location of linux image
    db("path_file_image") = path_dir_cache / "linux.flatimage";
    // Location of the output file
    db("path_file_output") = db.template value_or_default<std::string>("path_file_output");
    // Wine distribution
    db("dist_wine") = db.template value_or_default<std::string>("dist_wine", "default");
    // Set as default project
    db("project") = db.template value_or_default<std::string>("project");
    // Projects object
    if ( not db.contains("projects"))
    {
      db("projects") = ns_db::object_t();
    } // if
  }
  , fs::exists(path_file_database)? ns_db::Mode::UPDATE : ns_db::Mode::CREATE);
} // init_impl() }}}

// read_impl() {{{
std::expected<Build,std::string> read_impl(fs::path path_file_db)
{
  return ns_db::open<std::expected<Build,std::string>>(path_file_db, [&](auto&& db) -> std::expected<Build,std::string>
  {
    Build build(path_file_db);
    build.project          = ehope(db.template value<std::string>("project"));
    build.path_dir_build   = ehope(db.template value<fs::path>("path_dir_build"));
    build.path_dir_cache   = ehope(db.template value<fs::path>("path_dir_cache"));
    build.path_file_image  = ehope(db.template value<fs::path>("path_file_image"));
    build.path_file_output = ehope(db.template value<fs::path>("path_file_output"));
    build.dist_wine        = ehope(db.template value<std::string>("dist_wine"));
    auto projects = ehope(db.value("projects"));
    for(auto const& key : projects.keys())
    {
      auto project = ehope(db.value("projects", key));
      Metadata metadata;
      metadata.name                  = key;
      metadata.path_dir_project      = ehope(project.template value<fs::path>("path_dir_project"));
      metadata.path_dir_project_root = ehope(project.template value<fs::path>("path_dir_project_root"));
      metadata.platform              = ns_enum::from_string<ns_enum::Platform>(project.template value_or_default<std::string>("platform"));
      build.projects.push_back(metadata);
    } // for
    return build;
  }, ns_db::Mode::READ).value_or(std::unexpected("Could not read build database"));
} // read_impl() }}}

// write_impl() {{{
void write_impl(Build const& build)
{
  std::ignore = ns_db::open(build.path_file_db, [&](auto&& db)
  {
    db("project") = build.project;
    db("path_dir_build") = build.path_dir_build;
    db("path_dir_cache") = build.path_dir_cache;
    db("path_file_image") = build.path_file_image;
    db("path_file_output") = build.path_file_output;
    db("dist_wine") = build.dist_wine;
    if( build.projects.empty() ) { db("projects") = ns_db::object_t{}; }
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
  return read_impl(fs::path{ns_env::get_or_throw("GIMG_DIR")} / "gameimage.json");
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
