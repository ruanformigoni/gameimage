///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : package
///

#pragma once

#include <filesystem>
#include <set>

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

// package_config() {{{
inline void package_config(fs::path const& path_dir_home_src
  , fs::path const& path_dir_home_dst
  , std::set<ns_enum::Platform> const& set_platforms)
{
  auto f_copy_recursive = [](fs::path const& path_dir_src, fs::path const& path_dir_dst)
  {
    for(auto&& path_file_src : fs::recursive_directory_iterator(path_dir_src)
      | std::views::filter([](auto&& e){ return not fs::is_directory(e); })
      | std::views::transform([](auto&& e){ return e.path(); }))
    {
      fs::path path_file_dst = path_dir_dst / fs::relative(path_file_src, path_dir_src);
      ns_log::exception([&]{ fs::create_directories(path_file_dst.parent_path()); });
      ns_log::exception([&]{ fs::copy(path_file_src, path_file_dst, fs::copy_options::overwrite_existing | fs::copy_options::copy_symlinks); });
    } // for
  };

  if ( set_platforms.contains(ns_enum::Platform::RPCS3) )
  {
    f_copy_recursive(path_dir_home_src / ".config/rpcs3", path_dir_home_dst / ".config/rpcs3");
  } // if

  if ( set_platforms.contains(ns_enum::Platform::PCSX2) )
  {
    f_copy_recursive(path_dir_home_src / ".config/PCSX2", path_dir_home_dst / ".config/PCSX2");
  } // if

  if ( set_platforms.contains(ns_enum::Platform::RETROARCH) )
  {
    f_copy_recursive(path_dir_home_src / ".config/retroarch", path_dir_home_dst / ".config/retroarch");
  } // if
} // package_config() }}}

// package_platforms() {{{
inline void package_platforms(std::set<ns_enum::Platform> const& set_platforms, ns_db::ns_build::Build& db_build)
{
  for(auto&& platform : set_platforms)
  {
    fs::path path_file_layer = db_build.path_dir_cache / "{}.layer"_fmt(ns_enum::to_string_lower(platform));
    portal(db_build.path_file_output, "fim-layer", "add", path_file_layer);
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
    ns_fs::ns_path::file_exists<true>(db_build.path_file_output);
    // Get path to the compressed layer to include in the image
    fs::path path_file_layer = ns_fs::ns_path::file_exists<true>(db_metadata.path_dir_project_root.string() + ".layer")._ret;
    // Include layer in the image
    portal(db_build.path_file_output, "fim-layer", "add", path_file_layer);
  } // for
} // package_project() }}}

// package() {{{
inline void package(std::string const& str_name, std::string const& str_projects)
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

  // Get list of projects
  auto vec_project = ns_vector::from_string(str_projects, ':');

  // Get list of platforms
  std::set<ns_enum::Platform> set_platforms = db_build->projects
    | std::views::filter([&](ns_db::ns_build::Metadata const& e){ return std::ranges::contains(vec_project, e.name); })
    | std::views::transform([](ns_db::ns_build::Metadata const& e){ return e.platform; })
    | std::ranges::to<std::set<ns_enum::Platform>>();

  // Create path to target image
  db_build->path_file_output = db_build->path_dir_build.parent_path() / (str_name + ".flatimage");
  ns_db::ns_build::write(*db_build);

  // Package configuration files
  fs::path path_dir_home_src = db_build->path_file_image.parent_path()
    / (std::string{"."} + db_build->path_file_image.filename().string() + std::string{".config"})
    / "overlays/upperdir/home/gameimage";
  fs::path path_dir_home_dst = db_build->path_file_output.parent_path()
    / (std::string{"."} + db_build->path_file_output.filename().string() + std::string{".config"})
    / "overlays/upperdir/home/gameimage";
  package_config(path_dir_home_src, path_dir_home_dst, set_platforms);

  // Copy image to output location
  fs::copy_file(db_build->path_file_image, db_build->path_file_output, fs::copy_options::overwrite_existing);

  // Include platforms
  package_platforms(set_platforms, *db_build);

  // Include projects
  package_projects(vec_project, *db_build);

  // Include launcher inside game image
  portal(db_build->path_file_output, "fim-exec", "cp", path_file_launcher_dst, "/fim/static/gameimage-launcher");

  // Set boot command
  portal(db_build->path_file_output, "fim-boot", "/bin/bash", "-c", R"(/fim/static/gameimage-launcher "$@")", "--");

  // Enable notify-send
  portal(db_build->path_file_output, "fim-notify", "on");

  // Commit changes into the image
  portal(db_build->path_file_output , "fim-commit");

} // package() }}}

} // namespace ns_package

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
