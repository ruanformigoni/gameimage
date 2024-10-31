///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : test
///

#pragma once

#include "../lib/db/build.hpp"
#include "../lib/subprocess.hpp"

namespace ns_test
{

// test() {{{
inline decltype(auto) test()
{
  // Open db
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Could not open build database");
  auto db_metadata = db_build->find(db_build->project);

  // Start application
  (void) ns_subprocess::Subprocess("/fim/static/fim_portal")
    .with_piped_outputs()
    .with_args(db_build->path_file_image, "fim-exec", db_metadata.path_dir_project / "boot")
    .spawn()
    .wait();
} // test() }}}
 
} // namespace ns_test

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
