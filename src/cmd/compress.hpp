///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : compress
///

#pragma once

#include "../enum.hpp"

#include "../std/filesystem.hpp"

#include "../lib/db/build.hpp"
#include "../lib/db/project.hpp"
#include "../lib/subprocess.hpp"

namespace ns_compress
{

namespace fs = std::filesystem;

// validate() {{{
inline void validate(ns_db::ns_build::Metadata& db_metadata, ns_db::ns_project::Project& db_project)
{
  // Icon
  "Icon is not installed"_try([&]
  {
    fs::path path_file_icon = db_metadata.path_dir_project / db_project.path_file_icon;
    ns_fs::ns_path::file_exists<true>(path_file_icon);
    ns_log::write('i', "Found icon '", path_file_icon, "'");
  });


  auto f_validate_file_or_directory = [&](ns_enum::Op const& op)
  {
    fs::path path_file = db_metadata.path_dir_project / db_project.find_file(op);
    try
    {
      ns_fs::ns_path::file_exists<true>(path_file);
    } // try
    catch(std::exception const& e)
    {
      ns_fs::ns_path::dir_exists<true>(path_file);
    } // catch
  };

  auto f_validate_files = [&](ns_enum::Op const& op)
  {
    std::vector<fs::path> paths_file = db_project.find_files(op);

    for(auto&& path_file : paths_file)
    {
      "Missing file {} in json for '{}'"_try([&]
      {
        try
        {
          ns_fs::ns_path::file_exists<true>(db_metadata.path_dir_project / path_file);
        } // try
        catch(std::exception const& e)
        {
          ns_fs::ns_path::dir_exists<true>(db_metadata.path_dir_project / path_file);
        } // catch
      }, ns_enum::to_string(op), path_file);
    } // for
  };

  switch(db_metadata.platform)
  {
    case ns_enum::Platform::LINUX:
    case ns_enum::Platform::WINE:
    {
      f_validate_file_or_directory(ns_enum::Op::ROM);
    }
    break;
    case ns_enum::Platform::RETROARCH:
    {
      // default rom
      f_validate_file_or_directory(ns_enum::Op::ROM);
      // default core
      f_validate_file_or_directory(ns_enum::Op::CORE);
      // all roms
      f_validate_files(ns_enum::Op::ROM);
      // all cores
      f_validate_files(ns_enum::Op::CORE);
    } // case
    break;
    case ns_enum::Platform::PCSX2:
    {
      // default rom
      f_validate_file_or_directory(ns_enum::Op::ROM);
      // default bios
      f_validate_file_or_directory(ns_enum::Op::BIOS);
      // all roms
      f_validate_files(ns_enum::Op::ROM);
      // all bios
      f_validate_files(ns_enum::Op::BIOS);
    } // case
    break;
    case ns_enum::Platform::RPCS3:
    {
      // default rom
      f_validate_file_or_directory(ns_enum::Op::ROM);
    } // case
    break;
  } // switch

} // validate() }}}

// compress() {{{
inline decltype(auto) compress()
{
  // Open databases
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Could not open build database");
  auto db_metadata = db_build->find(db_build->project);
  auto db_project = ns_db::ns_project::read();
  ethrow_if(not db_project, "Could not open project database");

  // Validate package by platform
  validate(db_metadata, *db_project);

  // Output file
  fs::path path_file_layer{db_metadata.path_dir_project_root.string() + ".layer"};

  // Erase if exists
  lec(fs::remove, path_file_layer);

  // Log
  ns_log::write('i', "project: ", db_metadata.name);
  ns_log::write('i', "image: ", db_build->path_file_image);
  ns_log::write('i', "layer: ", path_file_layer);

  // Execute portal
  auto f_portal = []<typename... Args>(Args&&... args)
  {
    (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
      .with_piped_outputs()
      .with_args(std::forward<Args>(args)...)
      .spawn()
      .wait();
  };

  // Commit
  f_portal(db_build->path_file_image, "fim-commit");

  // Compress
  f_portal(db_build->path_file_image , "fim-layer" , "create" , db_metadata.path_dir_project_root , path_file_layer);

  ns_log::write('i', "Wrote file to '", path_file_layer, "'");
} // compress() }}}

} // namespace ns_compress

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
