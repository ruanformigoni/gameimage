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
#include "../lib/log.hpp"
#include "../lib/db/fetch.hpp"
#include "../lib/sha.hpp"

inline constexpr const char* URL_FETCH = "https://192.168.0.16:1170/fetch.json";

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

// struct fetchlist_layer_ret_t {{{
struct fetchlist_layer_ret_t
{
  fs::path path;
  cpr::Url url;
}; // }}}

// get_path_fetchlist() {{{
[[nodiscard]] inline fs::path get_path_fetchlist()
{
  return fs::current_path() / "fetch.json";
} // get_path_fetchlist() }}}

// get_path_file_image() {{{
[[nodiscard]] inline fs::path get_path_file_image(ns_enum::Platform const& platform)
{
  return fs::current_path() / ( ns_enum::to_string_lower(platform) + ".flatimage" );
} // }}}

// get_path_dir_cache() {{{
[[nodiscard]] inline fs::path get_path_dir_cache()
{
  return fs::current_path() / "cache";
} // }}}

// fetch_file_from_url() {{{
[[nodiscard]] inline std::expected<fs::path, std::string> fetch_file_from_url(fs::path const& path_file
  , cpr::Url const& url
  , std::optional<std::string> key_ipc = std::nullopt)
{
  ns_log::write('i', "Fetch file '", url.c_str(), "' to '", path_file, "'");
  // Create upper directories
  lec(fs::create_directories, path_file.parent_path());
  // Try to open destination file
  auto ofile = std::ofstream{path_file, std::ios::binary};
  // Check if file is open
  qreturn_if(not ofile.is_open(), std::unexpected("Failed to open file '{}' for writing"_fmt(path_file)));
  // Initialize IPC
  std::unique_ptr<ns_ipc::Ipc> ptr_ipc = nullptr;
  ns_log::exception([&]{ ptr_ipc = std::make_unique<ns_ipc::Ipc>(key_ipc.value_or(path_file)); });
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
[[nodiscard]] inline std::error<std::string> fetch_on_failed_check(fs::path const& path_file
  , cpr::Url const& url
  , std::optional<std::string> key_ipc = std::nullopt)
{
  qreturn_if(check_file(path_file, url), std::nullopt);

  ns_log::write('i', "Failed to check SHA for file ", path_file);

  if(auto expected_path_file = fetch_file_from_url(path_file, url, key_ipc); not expected_path_file )
  {
    return expected_path_file.error();
  } // if

  return std::nullopt;
} // }}}

// fetchlist_layer() {{{
[[nodiscard]] inline std::expected<fetchlist_layer_ret_t, std::string> fetchlist_layer(ns_enum::Platform const& platform)
{
  // Temporary file with fetch list
  fs::path path_file_fetchlist = get_path_fetchlist();
  // Open file as database
  auto database = ns_db::ns_fetch::read(path_file_fetchlist);
  ethrow_if(not database, database.error());
  // Fetch layer url
  std::string str_url_layer = database
      ->get_platform(platform)
      ->get_layer((platform == ns_enum::Platform::WINE)? ns_env::get_or_else("GIMG_WINE_DIST", "umu-proton-ge") : "default");
  // Show url
  ns_log::write('i', "url to fetch: ", str_url_layer);
  fs::path path_dir_dst = (platform == ns_enum::Platform::LINUX)?
      path_file_fetchlist.parent_path() / "cache/linux.flatimage"
    : path_file_fetchlist.parent_path() / "cache/{}.layer"_fmt(ns_enum::to_string_lower(platform));
  // Create destination / url pair
  return fetchlist_layer_ret_t { .path = path_dir_dst, .url = cpr::Url(str_url_layer), };
} // fetchlist_layer() }}}

// fetch_layer() {{{
[[nodiscard]] inline std::expected<fetchlist_layer_ret_t, std::string> fetch_layer(ns_enum::Platform platform)
{
  // Resolve URL
  auto expected_path_and_url_layer = fetchlist_layer(platform);
  qreturn_if(not expected_path_and_url_layer, std::unexpected(expected_path_and_url_layer.error()));
  auto [path_target, url] = *expected_path_and_url_layer;
  // Fetch
  auto error_fetch = fetch_on_failed_check(path_target, url);
  qreturn_if(error_fetch, std::unexpected(*error_fetch));
  // Send 100% completion
  ns_ipc::Ipc(path_target).send(100);
  // Return fetched path and url
  return *expected_path_and_url_layer;
} // fetch() }}}

} // anonymous namespace

// fetch_cores() {{{
inline std::expected<std::vector<ns_db::ns_fetch::CoreUrl>,std::string> fetch_cores()
{
  // Define sources file
  fs::path path_file_fetchlist = get_path_fetchlist();
  // Fetch from remote
  if ( auto expected = fetch_file_from_url(path_file_fetchlist, cpr::Url{URL_FETCH}); not expected)
  {
    return std::unexpected(expected.error());
  } // if
  // Open as a database
  auto database = ns_db::ns_fetch::read(path_file_fetchlist);
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
inline void fetch(ns_enum::Platform platform)
{
  std::expected<fetchlist_layer_ret_t, std::string> fetchlist_layer;
  fetchlist_layer = fetch_layer(platform);
  ereturn_if(not fetchlist_layer, fetchlist_layer.error());
} // fetch() }}}

// installed() {{{
inline std::vector<ns_enum::Platform> installed()
{
  // Gather
  std::error_code ec;
  auto platforms = fs::directory_iterator(get_path_dir_cache(), ec)
    | std::views::filter([](auto&& e){ return fs::is_regular_file(e) and ns_enum::is_enum_entry<ns_enum::Platform>(e.path().stem()); })
    | std::views::transform([](auto&& e){ return ns_enum::from_string<ns_enum::Platform>(e.path().stem()); })
    | std::ranges::to<std::vector<ns_enum::Platform>>();
  std::ranges::sort_unique(platforms);
  return platforms;
} // installed() }}}

// sha() {{{
inline std::error<std::string> sha(ns_enum::Platform platform)
{
  // Create image path
  fs::path path_file_image = get_path_file_image(platform);

  // Log
  ns_log::write('i', "platform: ", ns_enum::to_string_lower(platform));
  ns_log::write('i', "image: ", path_file_image);
  ns_log::write('i', "Only checking SHA");

  // Get layer
  auto expected_path_and_url_layer = fetchlist_layer(platform);
  qreturn_if(expected_path_and_url_layer, expected_path_and_url_layer.error());

  // Check sha for layer
  qreturn_if(not check_file(expected_path_and_url_layer->path, expected_path_and_url_layer->url)
    , "Failed to check file '{}'"_fmt(expected_path_and_url_layer->path)
  );

  return std::nullopt;
} // sha() }}}

// ipc() {{{
inline void ipc(ns_enum::Platform platform , ns_enum::IpcQuery entry_ipc_query)
{
  // Use self as IPC reference
  fs::path path_file_ipc = ns_fs::ns_path::file_self<true>()._ret;

  // Create image path
  fs::path path_file_image = get_path_file_image(platform);
  fs::path path_dir_image = path_file_image.parent_path();

  // Open IPC
  ns_ipc::Ipc ipc(path_file_ipc, true);
  ns_log::write('i', "Path to ipc reference file: '", path_file_ipc, "'");

  // Send layer path or url
  auto expected_path_and_url_layer = fetchlist_layer(platform);
  ethrow_if(not expected_path_and_url_layer, expected_path_and_url_layer.error());
  ipc.send((entry_ipc_query == ns_enum::IpcQuery::FILES)? expected_path_and_url_layer->path.string() : expected_path_and_url_layer->url.str());
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
