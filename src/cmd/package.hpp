///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : package
///

#pragma once

#include <filesystem>

#include "../lib/subprocess.hpp"
#include "../lib/db/build.hpp"

namespace ns_package
{

namespace fs = std::filesystem;

// portal() {{{
template<typename... Args>
decltype(auto) portal(Args&&... args)
{
  (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
    .with_piped_outputs()
    .with_args(std::forward<Args>(args)...)
    .spawn()
    .wait();
} // portal() }}}

// package_platforms() {{{
inline void package_platforms(std::vector<std::string> const& vec_project
  , fs::path const& path_file_image
  , ns_db::ns_build::Build& db_build)
{
  // Get unique platform list
  auto vec_platforms = db_build.projects
    | std::views::filter([&](ns_db::ns_build::Metadata const& e){ return std::ranges::contains(vec_project, e.name); })
    | std::views::transform([](ns_db::ns_build::Metadata const& e){ return e.platform; })
    | std::ranges::to<std::vector<ns_enum::Platform>>();
  std::ranges::sort_unique(vec_platforms);
  // Include each platform in the image
  for(auto&& platform : vec_platforms)
  {
    fs::path path_file_layer = db_build.path_dir_cache / "{}.layer"_fmt(ns_enum::to_string_lower(platform));
    portal(path_file_image, "fim-layer", "add", path_file_layer);
  } // for
} // package_platforms() }}}

// package_project() {{{
inline void package_projects(std::vector<std::string> const& vec_project, ns_db::ns_build::Build& db_build)
{
  for(auto&& project : vec_project)
  {
    // Get project metadata
    auto db_metadata = db_build.find(project);
    // Verify that image exists
    ns_fs::ns_path::file_exists<true>(db_build.path_file_image);
    // Get path to the compressed layer to include in the image
    fs::path path_file_layer = ns_fs::ns_path::file_exists<true>(db_metadata.path_dir_project_root.string() + ".layer")._ret;
    // Include layer in the image
    portal(db_build.path_file_image, "fim-layer", "add", path_file_layer);
  } // for
} // package_project() }}}

// package() {{{
inline void package(std::string const& str_projects)
{
  // Open databases
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Could not open build database");

  // Verify that directory exists
  ns_fs::ns_path::dir_exists<true>(db_build->path_dir_build);

  // Copy launcher to outside wizard image
  auto path_file_launcher_src = ns_subprocess::search_path("gameimage-launcher");
  ethrow_if(not path_file_launcher_src, "Could not find gameimage-launcher in PATH");
  fs::path path_file_launcher_dst = db_build->path_dir_build / "gameimage-launcher";
  fs::copy_file(*path_file_launcher_src
    , path_file_launcher_dst
    , fs::copy_options::overwrite_existing
  );

  auto vec_project = ns_vector::from_string(str_projects, ':');

  // Include platforms
  package_platforms(vec_project, db_build->path_file_image, *db_build);

  // Include projects
  package_projects(vec_project, *db_build);

  // Include launcher inside game image
  portal(db_build->path_file_image, "fim-exec", "cp", path_file_launcher_dst, "/fim/static/gameimage-launcher");

  // Set boot command
  portal(db_build->path_file_image, "fim-boot", "/bin/bash", "-c", R"(/fim/static/gameimage-launcher "$@")", "--");

  // Enable notify-send
  portal(db_build->path_file_image, "fim-notify", "on");

  // Commit changes into the image
  portal(db_build->path_file_image , "fim-commit");

} // package() }}}

} // namespace ns_package

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
