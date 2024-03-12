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

// anonymous namespace
namespace
{

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

// check_file_from_sha() {{{
inline void check_file_from_sha(fs::path const& path_file, cpr::Url const& url)
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

  // Fetch SHA file if not exists
  if ( ! ns_fs::ns_path::file_exists<false>(path_sha256sum)._bool )
  {
    fetch_file_from_url(path_sha256sum, url_checksum);
  } // if

  // Check SHA
  if (auto ret_proc = ns_subprocess::sync(path_bin_sha256sum, "-c", path_sha256sum);
    ret_proc.exit_code != 0)
  {
    "SHA failed for "_throw(path_file);
  } // if

  ns_log::write('i', "SHA passed for ", path_file);
} // }}}

// fetch_file_from_url_on_failed_sha() {{{
inline void fetch_file_from_url_on_failed_sha(fs::path const& path_file, cpr::Url const& url)
{
  try
  {
    check_file_from_sha(path_file, url);
  }
  // Re-download if SHA failed, and json write is disabled
  catch(std::exception const& e)
  {
    // Re-download on failure
    ns_log::write('i', "Failed to check SHA for file ", path_file);
    fetch_file_from_url(path_file, url);
  }
} // }}}

// list_base_and_dwarfs() {{{
inline decltype(auto) list_base_and_dwarfs(ns_enum::Platform const& platform
  , fs::path const& path_dir_dst)
{
  struct Ret
  {
    fs::path path_file_dwarfs;
    fs::path path_file_base;
    cpr::Url url_dwarfs;
    cpr::Url url_base;
  };

  // Create parent directories directory
  fs::create_directories(path_dir_dst);

  // Temporary file with fetch list
  auto path_json = path_dir_dst / "fetch.base.json";

  // Fetch fetch list
  fetch_file_from_url(path_json, cpr::Url{"https://gist.githubusercontent.com/ruanformigoni/e6f023c9d071e24fc95a50c14c06c88b/raw/665c7f36bd823f319b554a96859c5acad5aa852d/fetch.json"});

  // Create platform string
  auto str_platform = ns_string::to_lower(ns_enum::to_string(platform));

  return Ret
  {
    .path_file_dwarfs = fs::path{path_dir_dst} / "{}.dwarfs"_fmt(str_platform),
    .path_file_base = fs::path{path_dir_dst} / "{}.tar.xz"_fmt(str_platform),
    .url_dwarfs = cpr::Url(ns_db::query(path_json, "dwarfs", str_platform)),
    .url_base = cpr::Url(ns_db::query(path_json, "base", str_platform)),
  };
} // get_files_by_platform() }}}

// merge_base_and_dwarfs() {{{
inline void merge_base_and_dwarfs(std::string str_platform
  , fs::path const& path_file_base
  , fs::path const& path_file_dwarfs
  , fs::path const& path_file_out)
{
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
      auto ret = ns_subprocess::sync(path_tar, "-tf", path_file_base);
      std::string file_name;
      std::getline(ret.ss_stdout, file_name);
      return file_name;
    }();
    ns_log::write('i', "Tarball contains '{}'"_fmt(tar_name_file));

    // Extract base
    ns_subprocess::sync(path_tar, "-xf", path_file_base, tar_name_file);

    // Move to target name
    fs::rename(tar_name_file, path_file_out);
    ns_log::write('i', "Extracted file ", path_file_out);

    // Merge files
    ns_subprocess::sync(path_file_out, "fim-dwarfs-add", path_file_dwarfs, "/fim/mount/{}"_fmt(str_platform));
} // merge_base_and_dwarfs() }}}

} // anonymous namespace

// cores_list() {{{
inline decltype(auto) cores_list(fs::path const& path_dir_dst)
{
  // Create parent directories directory
  fs::create_directories(path_dir_dst);

  // Temporary file with fetch list
  auto path_json = path_dir_dst / "fetch.cores.json";

  // Fetch fetch list
  fetch_file_from_url(path_json, cpr::Url{"https://gist.githubusercontent.com/ruanformigoni/e6f023c9d071e24fc95a50c14c06c88b/raw/665c7f36bd823f319b554a96859c5acad5aa852d/fetch.json"});

  struct Ret
  {
    std::string core;
    std::string url;
  };

  std::vector<Ret> vector_cores;

  // Get cores
  ns_db::from_file(path_json, [&](auto&& db)
  {
    for( auto const& [key, value] : db["retroarch"].items() )
    {
      vector_cores.push_back(Ret{ns_common::to_string(key), ns_common::to_string(value)});
    }
  });

  // Return cores
  return vector_cores;
} // get_files_by_platform() }}}

// base_fetch() {{{
inline void base_fetch(ns_enum::Platform platform, fs::path path_file_name)
{
  // Validate input
  fs::path path_image = ns_fs::ns_path::dir_parent_exists<true>(path_file_name)._ret;

  // Log
  ns_log::write('i', "platform: ", ns_enum::to_string(platform));
  ns_log::write('i', "image: ", path_image);

  // Get files and destination paths to download
  auto fetch_paths_and_urls = list_base_and_dwarfs(platform, path_image.parent_path());

  // base_fetch base and dwarfs
  fetch_file_from_url_on_failed_sha(fetch_paths_and_urls.path_file_base, fetch_paths_and_urls.url_base);
  fetch_file_from_url_on_failed_sha(fetch_paths_and_urls.path_file_dwarfs, fetch_paths_and_urls.url_dwarfs);

  // Merge base and dwarfs
  merge_base_and_dwarfs(ns_string::to_lower(ns_enum::to_string(platform))
    , fetch_paths_and_urls.path_file_base
    , fetch_paths_and_urls.path_file_dwarfs
    , path_image);
} // base_fetch() }}}

// base_sha() {{{
inline void base_sha(ns_enum::Platform platform, fs::path path_file_name)
{
  // Validate input
  fs::path path_image = ns_fs::ns_path::dir_parent_exists<true>(path_file_name)._ret;

  // Log
  ns_log::write('i', "platform: ", ns_enum::to_string(platform));
  ns_log::write('i', "image: ", path_image);

  // Get files and destination paths to download
  auto fetch_paths_and_urls = list_base_and_dwarfs(platform, path_image.parent_path());

  // Check SHA only
  ns_log::write('i', "Only checking SHA");
  check_file_from_sha(fetch_paths_and_urls.path_file_base, fetch_paths_and_urls.url_base);
  check_file_from_sha(fetch_paths_and_urls.path_file_dwarfs, fetch_paths_and_urls.url_dwarfs);
} // base_sha() }}}

// base_json() {{{
inline void base_json(ns_enum::Platform platform, fs::path path_file_name, fs::path path_json)
{
  // Validate input
  fs::path path_image = ns_fs::ns_path::dir_parent_exists<true>(path_file_name)._ret;

  // Log
  ns_log::write('i', "platform: ", ns_enum::to_string(platform));
  ns_log::write('i', "image: ", path_image);

  // Get files and destination paths to download
  auto fetch_paths_and_urls = list_base_and_dwarfs(platform, path_image.parent_path());

  ns_log::write('i', "Only writting json for base");
  fs::remove(path_json);
  ns_db::from_file(ns_fs::ns_path::file_create<true>(path_json)._ret, [&](auto& db)
  {
    db("paths") |= fetch_paths_and_urls.path_file_base.c_str();
    db("paths") |= fetch_paths_and_urls.path_file_dwarfs.c_str();
    db("urls") |= fetch_paths_and_urls.url_base.c_str();
    db("urls") |= fetch_paths_and_urls.url_dwarfs.c_str();
  }, std::ios_base::out);

} // fetch() }}}

} // namespace ns_fetch

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
