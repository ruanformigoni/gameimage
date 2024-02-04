///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : package
///

#pragma once

#include <filesystem>

#include "../lib/subprocess.hpp"
#include "../lib/json.hpp"

namespace ns_package
{

namespace fs = std::filesystem;

// package() {{{
inline void package(fs::path path_file_dwarfs)
{
  // Get current project
  ns_json::Json json = ns_json::from_file_default();
  std::string str_project = json["project"];

  // Image path
  fs::path path_image = json[str_project]["path-image"];

  // Verify that image exists
  ns_fs::ns_path::file_exists<true>(path_image);

  // Verify that dwarfs exists
  ns_fs::ns_path::file_exists<true>(path_file_dwarfs);

  // Get stem of dwarfs file
  std::string str_stem_dwarfs = path_file_dwarfs.stem();

  // Get path to the dwarfs mount point
  fs::path path_dir_mount = fs::path{"/fim/mount/"} / str_stem_dwarfs;

  // Include in image
  ns_subprocess::sync(path_image
    , "fim-dwarfs-add"
    , path_file_dwarfs
    , path_dir_mount);

  // Setup overlayfs
  std::string str_path_overlayfs = R"('"$FIM_FILE_BINARY".config/overlays/')";
  str_path_overlayfs += str_stem_dwarfs;
  ns_subprocess::sync(path_image
    , "fim-dwarfs-overlayfs"
    , str_stem_dwarfs
    , str_path_overlayfs
  );

  // Get path to boot file inside the dwarfs filesystem
  fs::path path_file_boot = path_dir_mount / "boot";

  // Set boot command
  ns_subprocess::sync(path_image, "fim-cmd", path_file_boot);
  

} // package() }}}

} // namespace ns_package

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
