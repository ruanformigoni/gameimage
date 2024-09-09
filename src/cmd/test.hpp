///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : test
///

#pragma once

#include "../lib/db.hpp"
#include "../lib/subprocess.hpp"

namespace ns_test
{

namespace fs = std::filesystem;

// test() {{{
inline decltype(auto) test()
{
  // Current application
  std::string str_app;

  // Path to flatimage
  fs::path path_file_flatimage;

  // Path to project
  fs::path path_dir_project;

  // Path to boot file
  fs::path path_file_boot;

  // Get default path
  ns_db::from_file_default([&](auto&& db)
  {
    // Current application
    str_app = db["project"];

    // Path to flatimage
    path_file_flatimage = ns_fs::ns_path::file_exists<true>(db[str_app]["path_file_image"])._ret;

    // Path to project
    path_dir_project = ns_fs::ns_path::dir_exists<true>(db[str_app]["path_dir_project"])._ret;

    // Path to boot file
    path_file_boot = ns_fs::ns_path::file_exists<true>(path_dir_project / "boot")._ret;
  }
  , ns_db::Mode::READ);

  // Start application
  ns_subprocess::sync("/fim/static/fim_portal", path_file_flatimage, "fim-exec", path_file_boot);
} // test() }}}
 
} // namespace ns_test

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
