///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : package
///

#pragma once

#include <filesystem>

#include "lib/subprocess.hpp"

namespace ns_package
{

namespace fs = std::filesystem;

// package() {{{
inline void package(fs::path path_setup, fs::path path_out)
{
  // Install path
  fs::path path_flatimage  = fs::path{path_setup} /= "AppDir/usr/bin/base.flatimage";

  // mkdwarfs command
  auto f_mkdwarfs = [&]<typename... _Args>(_Args&&... args)
  {
    ns_subprocess::subprocess(path_flatimage, "fim-exec", "mkdwarfs", std::forward<_Args>(args)...);
  };

  // Create dwarfs filesystem
  f_mkdwarfs("-i", path_setup, "-o", path_out);

} // package() }}}

} // namespace ns_package

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
