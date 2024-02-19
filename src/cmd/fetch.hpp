///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : fetch
///

#pragma once

#include <fcntl.h>
#include <cpr/cpr.h>
#include <fmt/ranges.h>

#include "../common.hpp"
#include "../enum.hpp"

#include "../std/fifo.hpp"

#include "../lib/subprocess.hpp"
#include "../lib/log.hpp"
#include "../lib/db.hpp"

namespace ns_fetch
{

namespace fs = std::filesystem;

// struct DataDownload {{{
struct DataDownload
{
  fs::path path_file;
}; // }}}

// dispatch_progress() {{{
inline void dispatch_progress(fs::path const& path_fifo, std::string const& data)
{
  // Create fifo
  fifo::create(path_fifo.c_str());
  // Push data to fifo
  fifo::push(path_fifo.c_str(), fmt::format("{}\n", data));
  // Print percentage
  ns_log::write('i', "Download progress ", path_fifo.filename().string(), ": ", data, "%");
} // }}}

// fetch_callback() {{{
// Progress callback function
inline bool fetch_callback(cpr::cpr_off_t downloadTotal, cpr::cpr_off_t downloadNow, cpr::cpr_off_t, cpr::cpr_off_t, intptr_t userdata)
{
  static std::chrono::steady_clock::time_point prev = std::chrono::steady_clock::now();
  std::chrono::steady_clock::time_point now = std::chrono::steady_clock::now();
  std::chrono::milliseconds timeDiff = duration_cast<std::chrono::milliseconds>(now - prev);

  // Create fifo with file basename
  DataDownload* data = reinterpret_cast<DataDownload*>(userdata);

	if (downloadTotal > 0 && timeDiff.count() >= 2000)
	{
    prev = now;
    // Update progress
    int percentage = static_cast<int>((downloadNow * 100) / downloadTotal);
    dispatch_progress(data->path_file, std::to_string(percentage));
	}
	return true; // Return false to cancel the download
} // }}}

// fetch_file_from_url() {{{
inline void fetch_file_from_url(fs::path const& path_file, cpr::Url const& url)
{
  // Try to open destination file
  auto ofile = std::ofstream{path_file, std::ios::binary};
  // Open file error
  if ( ! ofile.good() ) { "Failed to open file '{}' for writing"_throw(path_file); }
  // Access data from callback
  fs::path path_fifo_progress(fmt::format("/tmp/gameimage/fifo/fetch.progress.{}", path_file.filename().string()));
  DataDownload data { .path_file = path_fifo_progress };
  // Fetch file
  cpr::Response r = cpr::Download(ofile, url, cpr::ProgressCallback{fetch_callback, reinterpret_cast<intptr_t>(&data)});
  // Check for success
  if ( r.status_code != 200 )
  {
    "Failure to fetch file '{}' with code '{}'"_throw(path_file, r.status_code);
  }
  // Set to progress 100%
  dispatch_progress(path_fifo_progress, "100");
  // Make file executable
  using std::filesystem::perms;
  fs::permissions(path_file, perms::owner_all | perms::group_all | perms::others_read);
} // }}}

// check_sha_from_url() {{{
inline void check_sha_from_url(fs::path const& path_file, cpr::Url const& url)
{
  // Find sha256sum binary
  fs::path path_bin_sha256sum;
  "Could not find sha256sum in PATH"_throw_if([&]
  {
    path_bin_sha256sum = boost::process::search_path("sha256sum").string();
    return ! ns_fs::ns_path::file_exists<false>(path_bin_sha256sum)._bool;
  });
  ns_log::write('i', "sha256sum binary path: ", path_bin_sha256sum);

  // SHA url is url + sha256sum
  cpr::Url url_checksum = url.str() + ".sha256sum";

  // SHA file name is file name + sha256sum
  fs::path path_sha256sum = path_file.string() + ".sha256sum";

  // Fetch SHA file
  fetch_file_from_url(path_sha256sum, url_checksum);

  // Check SHA
  if (auto ret_proc = ns_subprocess::sync(path_bin_sha256sum, "-c", path_sha256sum);
    ret_proc.exit_code != 0)
  {
    "SHA failed for "_throw(path_file);
  } // if

  ns_log::write('i', "SHA passed for ", path_file);
} // }}}

// fetch_to_file() {{{
inline void fetch_to_file(ns_enum::Platform const& platform
  , fs::path path_dest
  , bool is_check_sha
  , std::optional<fs::path> opt_path_json)
{
  // Log mode
  if ( is_check_sha )
  {
    ns_log::write('i', "Only checking SHA");
  } // if

  if ( opt_path_json )
  {
    ns_log::write('i', "Only writting json");
  } // if

  // Fetch a file
  auto f_fetch = [&](fs::path path_file, cpr::Url url, bool is_check_sha = false, bool to_json = false)
  {
    // Check if should only append json
    if ( to_json )
    {
      ns_db::from_file(*opt_path_json,
      [&](ns_db::Db&& db)
      {
        db("paths") |= path_file.c_str();
        db("urls") |= url.c_str();
      }, std::ios_base::out);
      return;
    } // if

    try
    {
      check_sha_from_url(path_file, url);
    }
    // Re-download if SHA failed, and json write is disabled
    catch(std::exception const& e)
    {
      // Only check SHA
      if ( is_check_sha )
      {
        "Failed to check SHA for file '{}'"_throw(path_file);
      }
      // Re-download on failure
      ns_log::write('i', "Failed to check SHA for file ");
      fetch_file_from_url(path_file, url);
    }
  };

  // Erase previous dry run file if exists
  if (opt_path_json.has_value()
    && ns_fs::ns_path::file_exists<false>(*opt_path_json)._bool)
  {
    fs::remove(*opt_path_json);
  } // if

  // Create temporary fetch dir
  fs::create_directories(GIMG_PATH_JSON_FETCH);

  // Fetch file list
  auto path_json = fs::path{GIMG_PATH_JSON_FETCH} /= "fetch.json";
  f_fetch(path_json
    , cpr::Url{"https://gist.githubusercontent.com/ruanformigoni/e6f023c9d071e24fc95a50c14c06c88b/raw/75b98364d6dfb95fc1e263bb5055f027ada3c63e/fetch.json"});

  // Set temporary directory
  fs::path dir_dest = path_dest.parent_path();

  // Create temporary directory
  fs::create_directories(dir_dest);

  // Helper to downloads/merge files
  auto f_fetch_by_platform = [&](auto&& db_fetch, ns_enum::Platform platform)
  {
    // Create platform string
    auto str_platform = ns_string::to_lower(ns_enum::to_string(platform));

    // Determine paths for base and platform
    fs::path path_platform = fs::path{dir_dest} / "{}.dwarfs"_fmt(str_platform);
    fs::path path_base_tarball = fs::path{dir_dest} / "{}.tar.xz"_fmt(str_platform);

    // Fetch base and platform
    f_fetch(path_platform, cpr::Url{db_fetch["dwarfs"][str_platform]}, is_check_sha, opt_path_json.has_value());
    f_fetch(path_base_tarball, cpr::Url{db_fetch["base"][str_platform]}, is_check_sha, opt_path_json.has_value());

    // Check if is dry run, if so stop here
    if ( opt_path_json.has_value() ) { return; }

    // Find tar in PATH
    fs::path path_tar;
    "Could not find tar in PATH"_throw_if([&]
    {
      path_tar = boost::process::search_path("tar").string();
      return ! ns_fs::ns_path::file_exists<false>(path_tar)._bool;
    });

    // Get file name inside the tarball
    std::string tar_name_file =
    [&]
    {
      auto ret = ns_subprocess::sync(path_tar, "-tf", path_base_tarball);
      std::string file_name;
      std::getline(ret.ss_stdout, file_name);
      return file_name;
    }();
    ns_log::write('i', "Tarball contains '{}'"_fmt(tar_name_file));

    // Extract base
    ns_subprocess::sync(path_tar, "-xf", path_base_tarball, tar_name_file);

    // Move to target name
    fs::path path_base = fs::path{dir_dest} / "{}.flatimage"_fmt(str_platform);
    fs::rename(tar_name_file, path_base);
    ns_log::write('i', "Rename from '{}' to '{}'"_fmt(tar_name_file, path_base));

    // Merge files
    ns_subprocess::sync(path_base, "fim-dwarfs-add", path_platform, "/fim/mount/{}"_fmt(str_platform));

    // Move to target
    fs::rename(path_base, path_dest);
  };

  // Open file list
  ns_db::from_file(path_json, [&]<typename T>(T&& db_fetch)
  {
    f_fetch_by_platform(std::forward<T>(db_fetch), platform);
  }, std::ios::in);

} // fetch_to_file() }}}

// fetch() {{{
inline void fetch(std::string str_platform
  , fs::path path_file_name
  , bool is_check_sha
  , std::optional<fs::path> opt_path_json)
{
  // Validate input
  ns_enum::Platform platform = ns_enum::from_string<ns_enum::Platform>(str_platform);
  fs::path path_image        = ns_fs::ns_path::dir_parent_exists<true>(path_file_name)._ret;

  // Log
  ns_log::write('i', "platform: ", str_platform);
  ns_log::write('i', "image: ", path_image);

  // Fetch files
  ns_fetch::fetch_to_file(platform, path_image, is_check_sha, opt_path_json);
} // fetch() }}}

} // namespace ns_fetch

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
