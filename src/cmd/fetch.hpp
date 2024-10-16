///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : fetch
///

#pragma once

#include <fcntl.h>
#include <cpr/cpr.h>
#include <fmt/ranges.h>
#include <expected>

#include "fetch/check.hpp"

#include "../common.hpp"
#include "../macro.hpp"
#include "../enum.hpp"

#include "../lib/ipc.hpp"
#include "../std/env.hpp"
#include "../lib/subprocess.hpp"
#include "../lib/log.hpp"
#include "../lib/db/fetch.hpp"
#include "../lib/sha.hpp"

inline constexpr const char* URL_FETCH = "https://raw.githubusercontent.com/gameimage/runners/refs/heads/master/fetch/gameimage-1.4.x.json";

namespace ns_fetch
{

namespace fs = std::filesystem;

// enum class UrlType {{{
enum class UrlType
{
  LAYER,
  BASE,
}; // }}}

// anonymous namespace
namespace
{

// struct fetchlist_base_ret_t {{{
struct fetchlist_base_ret_t
{
  fs::path path;
  cpr::Url url;
}; // }}}

// struct fetchlist_layer_ret_t {{{
struct fetchlist_layer_ret_t
{
  fs::path path;
  cpr::Url url;
}; // }}}

// enum class IpcQuery {{{
enum class IpcQuery
{
  FILES,
  URLS,
}; // }}}

// get_path_fetchlist() {{{
inline fs::path get_path_fetchlist()
{
  return fs::current_path() / "fetch.json";
} // get_path_fetchlist() }}}

// get_path_file_image() {{{
decltype(auto) get_path_file_image(ns_enum::Platform const& platform)
{
  return fs::current_path() / ( ns_enum::to_string_lower(platform) + ".flatimage" );
} // }}}

// fetch_file_from_url() {{{
[[nodiscard]] inline std::expected<fs::path, std::string> fetch_file_from_url(fs::path const& path_file, cpr::Url const& url)
{
  ns_log::write('i', "Fetch file '", url.c_str(), "' to '", path_file, "'");
  // Try to open destination file
  auto ofile = std::ofstream{path_file, std::ios::binary};
  // Check if file is open
  qreturn_if(not ofile.is_open(), std::unexpected("Failed to open file '{}' for writing"_fmt(path_file)));
  // Initialize IPC
  std::unique_ptr<ns_ipc::Ipc> ptr_ipc = nullptr;
  ns_log::exception([&]{ ptr_ipc = std::make_unique<ns_ipc::Ipc>(path_file); });
  // fetch_callback
  auto fetch_callback = [&](cpr::cpr_off_t downloadTotal, cpr::cpr_off_t downloadNow, cpr::cpr_off_t, cpr::cpr_off_t, intptr_t)
  {
    static std::chrono::steady_clock::time_point prev = std::chrono::steady_clock::now();
    std::chrono::steady_clock::time_point now = std::chrono::steady_clock::now();
    std::chrono::milliseconds timeDiff = duration_cast<std::chrono::milliseconds>(now - prev);

    if (downloadTotal > 0 && timeDiff.count() >= 2000)
    {
      prev = now;
      // Update progress
      int percentage = static_cast<int>((downloadNow * 100) / downloadTotal);
      // Send progress to watching processes
      if ( ptr_ipc ){ ptr_ipc->send(percentage); }
      // Log
      ns_log::write('i', "Download progress: ", percentage, "%");
    }
    return true; // Return false to cancel the download
  }; //
  // Fetch file
  cpr::Response r = cpr::Download(ofile, url, cpr::ProgressCallback{fetch_callback, reinterpret_cast<intptr_t>(&ofile)});
  // Check for success
  qreturn_if(r.status_code != 200,  std::unexpected("Failure to fetch file '{}' with code '{}'"_fmt(path_file, r.status_code)));
  // Set to progress 100%
  if ( ptr_ipc ){ ptr_ipc->send(100); }
  ns_log::write('i', "Download progress: 100%");
  // Make file executable
  using std::filesystem::perms;
  std::error_code ec;
  fs::permissions(path_file, perms::owner_all | perms::group_all | perms::others_read, ec);
  elog_if(ec, "Failed to change permissions of file '{}': '{}'"_fmt(path_file, ec.message()));
  // Success
  return path_file;
} // }}}

// check_file() {{{
[[nodiscard]] bool check_file(fs::path path_file_src, cpr::Url url)
{
  fs::path path_file_sha;
  ns_sha::SHA_TYPE sha_type;

  // Fetch SHA
  if ( auto expected256 = fetch_file_from_url(path_file_src.string() + ".sha256sum", url.str() + ".sha256sum") )
  {
    path_file_sha = *expected256;
    sha_type = ns_sha::SHA_TYPE::SHA256;
  } // if
  else if ( auto expected512 = fetch_file_from_url(path_file_src.string() + ".sha512sum", url.str() + ".sha512sum") )
  {
    path_file_sha = *expected512;
    sha_type = ns_sha::SHA_TYPE::SHA512;
  } // if
  else
  {
    ns_log::write('e', expected256.error());
    ns_log::write('e', expected512.error());
    return false;
  } // else

  // Check
  return ns_fetch::ns_check::check_file(path_file_src, path_file_sha, sha_type, url);
} // check_file() }}}

// fetch_on_failed_check() {{{
[[nodiscard]] inline std::error<std::string> fetch_on_failed_check(fs::path const& path_file, cpr::Url const& url)
{
  if ( check_file(path_file, url) )
  {
    ns_ipc::Ipc(path_file).send(100);
    return std::nullopt;
  } // if

  ns_log::write('i', "Failed to check SHA for file ", path_file);

  if(auto expected_path_file = fetch_file_from_url(path_file, url); not expected_path_file )
  {
    return expected_path_file.error();
  } // if

  return std::nullopt;
} // }}}

// fetchlist_base() {{{
[[nodiscard]] inline std::expected<fetchlist_base_ret_t, std::string> fetchlist_base(ns_enum::Platform const& platform)
{
  // Create platform string
  std::string str_platform = ns_enum::to_string_lower(platform);
  // Temporary file with fetch list
  fs::path path_file_fetch = get_path_fetchlist();
  // Open file as database
  auto database = ns_db::ns_fetch::read(path_file_fetch);
  ethrow_if(not database, database.error());
  // Select the base in terms of platform
  std::string str_url_base = database->get_platform(platform)->get_base();
  // Show base url
  ns_log::write('i', "url base  : ", str_url_base);
  // Create destination / url pair
  return fetchlist_base_ret_t
  {
    .path = fs::path{path_file_fetch.parent_path()} / "{}.flatimage"_fmt(str_platform),
    .url = cpr::Url(str_url_base),
  };
} // fetchlist_base() }}}

// fetchlist_layer() {{{
[[nodiscard]] inline std::expected<fetchlist_layer_ret_t, std::string> fetchlist_layer(ns_enum::Platform const& platform)
{
  // Temporary file with fetch list
  fs::path path_file_fetch = get_path_fetchlist();
  // Open file as database
  auto database = ns_db::ns_fetch::read(path_file_fetch);
  ethrow_if(not database, database.error());
  // Fetch layer url
  std::string str_url_layer = database
      ->get_platform(platform)
      ->get_layer((platform == ns_enum::Platform::WINE)? ns_env::get_or_else("GIMG_WINE_DIST", "umu-proton-ge") : "default");
  // Show url
  ns_log::write('i', "url layer: ", str_url_layer);
  // Create destination / url pair
  return fetchlist_layer_ret_t
  {
    .path = fs::path{path_file_fetch.parent_path()} / "{}.layer"_fmt(ns_enum::to_string_lower(platform)),
    .url = cpr::Url(str_url_layer),
  };
} // fetchlist_layer() }}}

// merge_base_and_layer() {{{
inline void merge_base_and_layer(fs::path const& path_file_image , fs::path const& path_file_layer)
{
  (void) ns_subprocess::Subprocess(path_file_image)
    .with_piped_outputs()
    .with_args("fim-layer", "add", path_file_layer)
    .spawn()
    .wait();
} // merge_base_and_layer() }}}

// fetch_base() {{{
[[nodiscard]] inline std::expected<fetchlist_base_ret_t, std::string> fetch_base(ns_enum::Platform platform)
{
  // Resolve URL
  auto expected_path_and_url_base = fetchlist_base(platform);
  qreturn_if(not expected_path_and_url_base, std::unexpected(expected_path_and_url_base.error()));
  // Fetch
  auto error_fetch = fetch_on_failed_check(expected_path_and_url_base->path, expected_path_and_url_base->url);
  qreturn_if(error_fetch, std::unexpected(*error_fetch));
  // Return fetched path and url
  return *expected_path_and_url_base;
} // fetch() }}}

// fetch_layer() {{{
[[nodiscard]] inline std::expected<fetchlist_layer_ret_t, std::string> fetch_layer(ns_enum::Platform platform)
{
  // Resolve URL
  auto expected_path_and_url_layer = fetchlist_layer(platform);
  qreturn_if(not expected_path_and_url_layer, std::unexpected(expected_path_and_url_layer.error()));
  // Fetch
  auto error_fetch = fetch_on_failed_check(expected_path_and_url_layer->path, expected_path_and_url_layer->url);
  qreturn_if(error_fetch, std::unexpected(*error_fetch));
  // Return fetched path and url
  return *expected_path_and_url_layer;
} // fetch() }}}

} // anonymous namespace

// fetch_cores() {{{
inline std::expected<std::vector<ns_db::ns_fetch::CoreUrl>,std::string> fetch_cores()
{
  // Define sources file
  fs::path path_file_fetch = get_path_fetchlist();
  // Fetch from remote
  if ( auto expected = fetch_file_from_url(path_file_fetch, cpr::Url{URL_FETCH}); not expected)
  {
    return std::unexpected(expected.error());
  } // if
  // Open as a database
  auto database = ns_db::ns_fetch::read(path_file_fetch);
  ethrow_if(not database, database.error());
  // Return cores
  return database->get_platform(ns_enum::Platform::RETROARCH)->get_cores();
} // fetch_cores() }}}

// fetchlist() {{{
inline std::error<std::string> fetchlist()
{
  auto expected = fetch_file_from_url(get_path_fetchlist(), cpr::Url{URL_FETCH});
  qreturn_if(not expected, expected.error());
  return std::nullopt;
} // fetchlist() }}}

// fetch() {{{
inline void fetch(ns_enum::Platform platform, std::optional<fs::path> const& only_file = std::nullopt)
{
  // Create image path
  fs::path path_file_image = get_path_file_image(platform);
  fs::path path_dir_image = path_file_image.parent_path();

  if ( only_file and only_file->string().ends_with(".layer") )
  {
    auto expected = fetch_layer(platform);
    elog_if(not expected, expected.error());
    return;
  } // if

  if ( only_file and only_file->string().ends_with(".flatimage") )
  {
    auto expected = fetch_base(platform);
    elog_if(not expected, expected.error());
    return;
  } // if

  // Verify & configure base
  auto expected_path_and_url_base = fetch_base(platform);
  ereturn_if(not expected_path_and_url_base, expected_path_and_url_base.error());

  // No layer for linux
  if ( platform == ns_enum::Platform::LINUX ) { return; }

  // Verify & configure layer
  auto expected_path_and_url_layer = fetch_layer(platform);
  ereturn_if(not expected_path_and_url_layer, expected_path_and_url_layer.error());

  // Merge base and layer
  merge_base_and_layer(path_file_image, expected_path_and_url_layer->path);
} // fetch() }}}

// sha() {{{
inline std::error<std::string> sha(ns_enum::Platform platform)
{
  // Create image path
  fs::path path_file_image = get_path_file_image(platform);
  fs::path path_dir_image = path_file_image.parent_path();

  // Log
  ns_log::write('i', "platform: ", ns_enum::to_string_lower(platform));
  ns_log::write('i', "image: ", path_file_image);
  ns_log::write('i', "Only checking SHA");

  // Get base
  auto expected_path_and_url_base = fetchlist_base(platform);
  qreturn_if(expected_path_and_url_base, expected_path_and_url_base.error());

  // Check sha for base
  qreturn_if(not check_file(expected_path_and_url_base->path, expected_path_and_url_base->url)
    , "Failed to check file '{}'"_fmt(expected_path_and_url_base->path)
  );

  // Linux does not have a separate layer file
  if ( platform == ns_enum::Platform::LINUX ) { return std::nullopt; }

  // Get layer
  auto expected_path_and_url_layer = fetchlist_layer(platform);
  qreturn_if(expected_path_and_url_layer, expected_path_and_url_layer.error());

  // Check sha for layer
  qreturn_if(not check_file(expected_path_and_url_base->path, expected_path_and_url_base->url)
    , "Failed to check file '{}'"_fmt(expected_path_and_url_base->path)
  );

  return std::nullopt;
} // sha() }}}

// ipc() {{{
inline void ipc(ns_enum::Platform platform , std::optional<std::string> query)
{
  // Use self as IPC reference
  fs::path path_file_ipc = ns_fs::ns_path::file_self<true>()._ret;

  // Create image path
  fs::path path_file_image = get_path_file_image(platform);
  fs::path path_dir_image = path_file_image.parent_path();

  // Open IPC
  ns_ipc::Ipc ipc(path_file_ipc, true);
  ns_log::write('i', "Path to ipc reference file: '", path_file_ipc, "'");

  // Check if query exists
  ethrow_if( not query.has_value(), "No query provided for IPC");

  // Get query
  IpcQuery ipc_query = ns_enum::from_string<IpcQuery>(ns_string::to_upper(*query));

  // Send base path or url
  auto expected_path_and_url_base = fetchlist_base(platform);
  ethrow_if(not expected_path_and_url_base, expected_path_and_url_base.error());
  ipc.send((ipc_query == IpcQuery::FILES)? expected_path_and_url_base->path.string() : expected_path_and_url_base->url.str());

  if ( platform == ns_enum::Platform::LINUX ) { return; }

  // Send layer path or url
  auto expected_path_and_url_layer = fetchlist_layer(platform);
  ethrow_if(not expected_path_and_url_layer, expected_path_and_url_layer.error());
  ipc.send((ipc_query == IpcQuery::FILES)? expected_path_and_url_layer->path.string() : expected_path_and_url_layer->url.str());
} // ipc() }}}

// url_clear() {{{
inline void url_clear(ns_enum::Platform platform)
{
  // Create image path
  fs::path path_file_image = get_path_file_image(platform);
  fs::path path_dir_image = path_file_image.parent_path();
  fs::path path_file_json = path_dir_image / "gameimage.fetch.json";
  fs::remove(path_file_json);
} // url_clear() }}}

} // namespace ns_fetch

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
