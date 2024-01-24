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
#include "../lib/json.hpp"

namespace ns_fetch
{

namespace fs = std::filesystem;

// struct DataDownload {{{
struct DataDownload
{
  fs::path path_file;
}; // }}}

// dispatch_progress() {{{
void dispatch_progress(fs::path const& path_fifo, std::string const& data)
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
bool fetch_callback(cpr::cpr_off_t downloadTotal, cpr::cpr_off_t downloadNow, cpr::cpr_off_t, cpr::cpr_off_t, intptr_t userdata)
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

// fetch_to_file() {{{
void fetch_to_file(ns_enum::Platform const& platform, fs::path path_dest)
{
  // Fetch a file
  auto f_fetch = [](fs::path path, cpr::Url url)
  {
    // Try to open destination file
    auto ofile = std::ofstream{path, std::ios::binary};
    // Open file error
    if ( ! ofile.good() ) { "Failed to open file '{}' for writing"_throw(path); }
    // Access data from callback
    fs::path path_fifo_progress(fmt::format("/tmp/gameimage/fifo/fetch.progress.{}", path.filename().string()));
    DataDownload data { .path_file = path_fifo_progress };
    // Fetch file
    cpr::Response r = cpr::Download(ofile, url, cpr::ProgressCallback{fetch_callback, reinterpret_cast<intptr_t>(&data)});
    // Check for success
    if ( r.status_code != 200 )
    {
      "Failure to fetch file {} with code {}"_throw(path, r.status_code);
    }
    // Set to progress 100%
    dispatch_progress(path_fifo_progress, "100");
    // Make file executable
    using std::filesystem::perms;
    fs::permissions(path, perms::owner_all | perms::group_all | perms::others_read);
  };

  // Create temporary fetch dir
  fs::create_directories(GIMG_PATH_JSON_FETCH);

  // Fetch file list
  auto path_json = fs::path{GIMG_PATH_JSON_FETCH} /= "fetch.json";
  f_fetch(path_json
    , cpr::Url{"https://gist.githubusercontent.com/ruanformigoni/e6f023c9d071e24fc95a50c14c06c88b/raw/31b73dfe42bd5741d63114d5140b8d56b65f81be/fetch.json"}
  );

  // Set temporary directory
  fs::path dir_dest = path_dest.parent_path();

  // Create temporary directory
  fs::create_directories(dir_dest);

  // Open file list
  ns_json::Json json_fetch = ns_json::from_file(path_json);

  // Fetch tools by platform
  switch(platform)
  {
    case ns_enum::Platform::WINE:
    {
      // Determine paths for base and wine
      fs::path path_wine = fs::path{dir_dest} /= "opt.dwarfs";
      fs::path path_base = fs::path{dir_dest} /= "base.flatimage";
      f_fetch(path_wine, cpr::Url{json_fetch["wine-tkg"]});
      f_fetch(path_base, cpr::Url{json_fetch["base-wine"]});
      // Merge files
      ns_subprocess::subprocess(path_base, "fim-include-path", path_wine, "/opt.dwarfs");
      // Move to target
      fs::rename(path_base, path_dest);
      // Remove dwarfs file
      fs::remove(path_wine);
    } // case
    break;
    case ns_enum::Platform::RETROARCH:
      f_fetch(fs::path{dir_dest} /= "retroarch" , cpr::Url{json_fetch["base-retroarch"]});
      break;
    case ns_enum::Platform::PCSX2:
      f_fetch(fs::path{dir_dest} /= "pcsx2" , cpr::Url{json_fetch["base-pcsx2"]});
      break;
    case ns_enum::Platform::RPCS3:
      f_fetch(fs::path{dir_dest} /= "rpcs3" , cpr::Url{json_fetch["base-rpcs3"]});
      break;
    case ns_enum::Platform::YUZU:
      f_fetch(fs::path{dir_dest} /= "yuzu" , cpr::Url{json_fetch["base-yuzu"]});
      break;
  }

} // fetch_to_file() }}}

// fetch() {{{
void fetch(std::string str_platform, fs::path str_name_file)
{
  // Validate input
  ns_enum::Platform platform = ns_enum::from_string<ns_enum::Platform>(str_platform);
  fs::path path_image       = ns_fs::ns_path::file_exists<true>(str_name_file)._ret;

  // Log
  ns_log::write('i', "platform: ", str_platform);
  ns_log::write('i', "image: ", path_image);

  // Fetch files
  try
  {
    ns_fetch::fetch_to_file(platform, fs::path(path_image));
  }
  catch(std::exception const& e)
  {
    fmt::println(stderr, "{}", e.what());
  }
} // fetch() }}}

} // namespace ns_fetch

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
