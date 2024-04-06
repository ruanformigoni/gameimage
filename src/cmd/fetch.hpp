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
#include "../lib/sha.hpp"
#include "../lib/tar.hpp"

inline const char* FETCH_URL = "https://raw.githubusercontent.com/gameimage/runners/master/fetch.json";

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
  ns_log::write('i', "Fetch file '", url.c_str(), "' to '", path_file, "'");
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
inline void check_file_from_sha(fs::path const& path_file_to_check, cpr::Url const& url)
{
  // SHA file name is file name + sha256sum
  fs::path path_file_sha256sum = path_file_to_check.string() + ".sha256sum";

  // Fetch SHA file if not exists
  // SHA url is url + sha256sum
  fetch_file_from_url(path_file_sha256sum, url.str() + ".sha256sum");

  // Check SHA
  if ( not ns_sha::check_256sum(path_file_to_check, path_file_sha256sum))
  {
    "SHA failed for {}"_throw(path_file_to_check);
  } // if

  ns_log::write('i', "SHA passed for ", path_file_to_check);
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

// struct fetchlist_base_ret_t {{{
struct fetchlist_base_ret_t
{
  fs::path path;
  cpr::Url url;
}; // }}}

// fetchlist_base() {{{
inline decltype(auto) fetchlist_base(ns_enum::Platform const& platform, fs::path const& path_dir_fetch)
{
  // Create dir
  ns_fs::ns_path::dir_create<true>(path_dir_fetch);

  // Temporary file with fetch list
  auto path_json = path_dir_fetch / "fetch.base.json";

  // Fetch list
  fetch_file_from_url(path_json, cpr::Url{FETCH_URL});

  // Create platform string
  auto str_platform = ns_string::to_lower(ns_enum::to_string(platform));

  // Select wine distribution
  std::string str_url_base = ns_db::query(path_json, str_platform, "base");

  // Show base url
  ns_log::write('i', "url base  : ", str_url_base);

  return fetchlist_base_ret_t
  {
    .path = fs::path{path_dir_fetch} / "{}.tar.xz"_fmt(str_platform),
    .url = cpr::Url(str_url_base),
  };
} // fetchlist_base() }}}

// struct fetchlist_dwarfs_ret_t {{{
struct fetchlist_dwarfs_ret_t
{
  fs::path path;
  cpr::Url url;
}; // }}}

// fetchlist_dwarfs() {{{
inline decltype(auto) fetchlist_dwarfs(ns_enum::Platform const& platform, fs::path const& path_dir_fetch)
{
  // Create dir
  ns_fs::ns_path::dir_create<true>(path_dir_fetch);

  // Temporary file with fetch list
  auto path_json = path_dir_fetch / "fetch.base.json";

  // Fetch list
  fetch_file_from_url(path_json, cpr::Url{FETCH_URL});

  // Create platform string
  auto str_platform = ns_string::to_lower(ns_enum::to_string(platform));

  // Select wine distribution
  std::string str_url_dwarfs;
  if ( const char* dist = ns_env::get("GIMG_WINE_DIST"); platform == ns_enum::Platform::WINE )
  {
    if ( dist != nullptr )
    {
      str_url_dwarfs = ns_db::query(path_json, str_platform, "dwarfs", dist);
    } // if
    else
    {
      str_url_dwarfs = ns_db::query(path_json, str_platform, "dwarfs", "default");
    } // else
  } // if
  else
  {
    str_url_dwarfs = ns_db::query(path_json, str_platform, "dwarfs");
  } // else

  ns_log::write('i', "url dwarfs: ", str_url_dwarfs);

  return fetchlist_dwarfs_ret_t
  {
    .path = fs::path{path_dir_fetch} / "{}.dwarfs"_fmt(str_platform),
    .url = cpr::Url(str_url_dwarfs),
  };
} // fetchlist_dwarfs() }}}

// tarball_extract_flatimage() {{{
inline void tarball_extract_flatimage(fs::path const& path_file_tarball, fs::path const& path_file_out)
{
  auto archive_files = ns_tar::list(path_file_tarball.c_str());

  if ( archive_files.empty() )
  {
    ns_log::write('e', "Empty archive'", path_file_tarball, "'");
    return;
  } // if

  auto it_str_file_name = std::ranges::find_if(archive_files, [](auto&& e){ return e.ends_with(".flatimage"); });

  if ( it_str_file_name == std::ranges::end(archive_files) )
  {
    ns_log::write('e', "Could not find the flatimage file in '", path_file_tarball, "'");
    return;
  } // if

  ns_log::write('i', "Tarball contains file '{}'"_fmt(*it_str_file_name));

  // Extract base
  ns_tar::extract(path_file_tarball.c_str(), ns_tar::Opts{0, *it_str_file_name});

  // Move to target name
  fs::rename(*it_str_file_name, path_file_out);
  ns_log::write('i', "Extracted file ", path_file_out);
} // tarball_extract_flatimage() }}}

// dwarfs_create() {{{
inline void dwarfs_create(fs::path const& path_dir_src, std::string path_file_target)
{
  // Compress
  ns_subprocess::sync(ns_subprocess::search_path("mkdwarfs")
    , "-f"
    , "-i"
    , path_dir_src
    , "-o"
    , path_file_target
  );
} // dwarfs_create() }}}

// merge_base_and_dwarfs() {{{
inline void merge_base_and_dwarfs(std::string str_platform
  , fs::path const& path_file_image
  , fs::path const& path_file_dwarfs)
{
  // Merge files
  ns_subprocess::sync(path_file_image, "fim-dwarfs-add", path_file_dwarfs, "/fim/mount/{}"_fmt(str_platform));
} // merge_base_and_dwarfs() }}}

// fetch_base() {{{
decltype(auto) fetch_base(ns_enum::Platform platform
  , std::optional<cpr::Url> const& url
  , fs::path path_dir_dst)
{
  ns_log::write('i', "Downloading file: '", path_dir_dst);

  fetchlist_base_ret_t path_and_url_base;

  // Fetch from custom url
  if ( url.has_value() )
  {
    ns_log::write('i', "Download url: '", url->c_str());
    path_and_url_base = { path_dir_dst / "{}.tar.xz"_fmt(ns_enum::to_string(platform)), *url };
  } // if
  // Fetch from fetchlist
  else
  {
    path_and_url_base = fetchlist_base(platform, path_dir_dst);
    ns_log::write('i', "Download url: '", path_and_url_base.url.c_str());
  } // else

  fetch_file_from_url_on_failed_sha(path_and_url_base.path, path_and_url_base.url);

  return path_and_url_base;
} // fetch_base() }}}

// fetch_dwarfs() {{{
decltype(auto) fetch_dwarfs(ns_enum::Platform platform
  , std::optional<cpr::Url> const& url
  , fs::path path_dir_dst)
{
  enum class FileType
  {
    Dwarfs,
    Tarball,
  };

  FileType ft;

  std::string str_platform = ns_enum::to_string_lower(platform);

  fetchlist_dwarfs_ret_t path_and_url_dwarfs;

  // Custom URL
  if ( url.has_value() )
  {
    if ( std::string{url->c_str()}.ends_with(".tar.xz") ) 
    {
      ft = FileType::Tarball;
      path_and_url_dwarfs = {path_dir_dst / "{}.tar.xz"_fmt(str_platform) , *url};
    } // if
    else if ( std::string{url->c_str()}.ends_with(".dwarfs") ) 
    {
      ft = FileType::Dwarfs;
      path_and_url_dwarfs = {path_dir_dst / "{}.dwarfs"_fmt(str_platform) , *url};
    } // else if
    else
    {
      throw std::runtime_error("Unsupported file type for download");
    } // else
  } // if
  else
  {
    ft = FileType::Dwarfs;
    path_and_url_dwarfs = fetchlist_dwarfs(platform, path_dir_dst);
  } // else

  // Fetch from url to target directory path
  fetch_file_from_url_on_failed_sha(path_and_url_dwarfs.path, path_and_url_dwarfs.url);

  // If is a dwarfs file, then it is ok to finish
  if ( ft == FileType::Dwarfs ) { return path_and_url_dwarfs; }

  // It is a tarball, requires to create the dwarfs file
  uint32_t components_to_strip{0};

  // Check if tarball contains the required files
  if ( platform == ns_enum::Platform::WINE )
  {
    auto opt_path_parent = ns_tar::find(path_and_url_dwarfs.path, "bin/wine");
    if ( not opt_path_parent.has_value() )
    {
      "Could not find '{}' inside tarball '{}'"_throw("bin/wine", path_and_url_dwarfs.path);
    } // if
    components_to_strip = std::distance(opt_path_parent->begin(), opt_path_parent->end());
    ns_log::write('i', "Components to strip: ", components_to_strip);
  } // if
  else
  {
    "Custom download not implemented for {}"_throw(str_platform);
  } // else

  // Extract tarball to "platform" directory
  ns_tar::extract(path_and_url_dwarfs.path, ns_tar::Opts{components_to_strip, std::nullopt, str_platform});

  // Fetch runner script
  if ( platform == ns_enum::Platform::WINE )
  {
    // Download wine.sh script into the binary directory
    fetch_file_from_url(fs::path(str_platform) / "bin" / "wine.sh"
      , "https://raw.githubusercontent.com/gameimage/runners/master/wine/wine.sh"
    );
  } // switch
  else
  {
    "Custom download not implemented for {}"_throw(str_platform);
  } // else

  // Create dwarfs filesystem
  dwarfs_create(str_platform, str_platform + ".dwarfs");

  return path_and_url_dwarfs;
} // fetch_dwarfs() }}}

} // anonymous namespace

// cores_list() {{{
inline decltype(auto) cores_list(fs::path const& path_dir_fetch)
{
  // Temporary file with fetch list
  fs::path path_file_json = path_dir_fetch / "fetch.cores.json";

  // Fetch fetch list
  fetch_file_from_url(path_file_json, cpr::Url{FETCH_URL});

  struct Ret
  {
    std::string core;
    std::string url;
  };

  std::vector<Ret> vector_cores;

  // Get cores
  ns_db::from_file(path_file_json, [&](auto&& db)
  {
    for( auto const& [key, value] : db["retroarch"]["core"].items() )
    {
      vector_cores.push_back(Ret{ns_string::to_string(key), ns_string::to_string(value)});
    }
  }, ns_db::Mode::READ);

  // Return cores
  return vector_cores;
} // get_files_by_platform() }}}

// fetch() {{{
inline void fetch(ns_enum::Platform platform
    , fs::path path_file_image
    , std::optional<cpr::Url> const& url_base = std::nullopt
    , std::optional<cpr::Url> const& url_dwarfs = std::nullopt)
{
  std::string str_platform = ns_enum::to_string_lower(platform);

  // Validate input
  path_file_image = ns_fs::ns_path::dir_parent_exists<true>(path_file_image)._ret;
  fs::path path_dir_image = ns_fs::ns_path::dir_exists<true>(path_file_image.parent_path())._ret;

  // Log
  ns_log::write('i', "platform: ", str_platform);
  ns_log::write('i', "image: ", path_file_image);

  // Fetch base
  auto ret_fetch_base = fetch_base(platform, url_base, path_dir_image);

  // Extract base
  tarball_extract_flatimage(ret_fetch_base.path, path_file_image);

  // No need to merge anything, just use the tarball
  if ( platform == ns_enum::Platform::LINUX ) { return; }

  // Fetch dwarfs file
  auto ret_fetch_dwarfs = fetch_dwarfs(platform, url_dwarfs, path_dir_image);

  // Set file name to platform + dwarfs
  ret_fetch_dwarfs.path.remove_filename();
  ret_fetch_dwarfs.path /= str_platform + ".dwarfs";

  // Merge base and dwarfs
  merge_base_and_dwarfs(ns_enum::to_string_lower(platform)
    , path_file_image
    , ret_fetch_dwarfs.path);
} // fetch() }}}

// sha() {{{
inline void sha(ns_enum::Platform platform, fs::path path_file_image)
{
  // Validate input
  path_file_image = ns_fs::ns_path::dir_parent_exists<true>(path_file_image)._ret;

  // Log
  ns_log::write('i', "platform: ", ns_enum::to_string(platform));
  ns_log::write('i', "image: ", path_file_image);
  ns_log::write('i', "Only checking SHA");

  // Get base
  auto path_and_url_base = fetchlist_base(platform, path_file_image.parent_path());

  // Check sha for base
  check_file_from_sha(path_and_url_base.path, path_and_url_base.url);

  // Linux does not have a separate dwarfs file
  if ( platform == ns_enum::Platform::LINUX ) { return; }

  // Get dwarfs
  auto path_and_url_dwarfs = fetchlist_dwarfs(platform, path_file_image.parent_path());

  // Check sha for dwarfs
  check_file_from_sha(path_and_url_dwarfs.path, path_and_url_dwarfs.url);
} // sha() }}}

// json() {{{
inline void json(ns_enum::Platform platform
  , fs::path path_file_image
  , fs::path path_json)
{
  // Remove if exists
  fs::remove(path_json);

  // Validate input
  path_file_image = ns_fs::ns_path::dir_parent_exists<true>(path_file_image)._ret;

  // Log
  ns_log::write('i', "platform: ", ns_enum::to_string(platform));
  ns_log::write('i', "image: ", path_file_image);

  // Get url and save path to base
  ns_log::write('i', "Writting json for base");
  auto path_and_url_base = fetchlist_base(platform, path_file_image.parent_path());
  auto path_file_base = path_and_url_base.path;
  auto url_base = path_and_url_base.url;

  ns_db::from_file(path_json, [&](auto&& db)
  {
    db("paths") |= path_file_base.c_str();
    db("urls")  |= url_base.c_str();
  }, ns_db::Mode::CREATE);

  if ( platform == ns_enum::Platform::LINUX ) { return; }

  // Get url and save path to dwarfs
  ns_log::write('i', "Writting json for dwarfs");
  auto path_and_url_dwarfs = fetchlist_dwarfs(platform, path_file_image.parent_path());
  auto path_file_dwarfs = path_and_url_dwarfs.path;
  auto url_dwarfs = path_and_url_dwarfs.url;

  ns_db::from_file(path_json, [&](auto&& db)
  {
    db("paths") |= path_file_dwarfs.c_str();
    db("urls")  |= url_dwarfs.c_str();
  }, ns_db::Mode::UPDATE);

} // json() }}}

} // namespace ns_fetch

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
