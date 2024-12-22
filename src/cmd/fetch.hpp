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
#include "../lib/db/build.hpp"
#include "../lib/sha.hpp"

inline constexpr const char* URL_FETCH = "https://raw.githubusercontent.com/gameimage/runners/refs/heads/master/fetch/gameimage-1.6.x.json";

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

// struct sources_layer_ret_t {{{
struct sources_layer_ret_t
{
  fs::path path;
  cpr::Url url;
}; // }}}

// get_path_sources() {{{
[[nodiscard]] inline std::expected<fs::path, std::string> get_path_sources()
{
  auto db_build = ns_db::ns_build::read();
  qreturn_if(not db_build, std::unexpected(db_build.error()));
  return db_build->path_dir_build / "fetch.json";
} // get_path_sources() }}}

// fetch_file_from_url() {{{
[[nodiscard]] inline std::expected<fs::path, std::string> fetch_file_from_url(fs::path const& path_file
  , cpr::Url const& url
  , bool send_ipc = true)
{
  ns_log::write('i', "Fetch file '", url.c_str(), "' to '", path_file, "'");
  // Create upper directories
  lec(fs::create_directories, path_file.parent_path());
  // Try to open destination file
  auto ofile = std::ofstream{path_file, std::ios::binary};
  // Check if file is open
  qreturn_if(not ofile.is_open(), std::unexpected("Failed to open file '{}' for writing"_fmt(path_file)));
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
      if ( send_ipc ){  ns_ipc::ipc().send(percentage); }
      // Log
      ns_log::write('i', "Download progress: ", percentage, "%");
    }
    return true; // Return false to cancel the download
  }; //
  // Fetch file
  cpr::Response r = cpr::Download(ofile, url, cpr::ProgressCallback{fetch_callback, reinterpret_cast<intptr_t>(&ofile)});
  // Check for success
  if ( r.status_code != 200 )
  {
    // Remove partial file
    ofile.close();
    fs::remove(path_file);
    // Return failure
    return std::unexpected("Failure to fetch file '{}' with code '{}'"_fmt(path_file, r.status_code));
  } // if
  // Set to progress 100%
  if ( send_ipc ) { ns_ipc::ipc().send(100); }
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
  if ( auto expected256 = fetch_file_from_url(path_file_src.string() + ".sha256sum", url.str() + ".sha256sum", false) )
  {
    path_file_sha = *expected256;
    sha_type = ns_sha::SHA_TYPE::SHA256;
  } // if
  else if ( auto expected512 = fetch_file_from_url(path_file_src.string() + ".sha512sum", url.str() + ".sha512sum", false) )
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
  , cpr::Url const& url)
{
  qreturn_if(check_file(path_file, url), std::nullopt);

  ns_log::write('i', "Failed to check SHA for file ", path_file);

  if(auto expected_path_file = fetch_file_from_url(path_file, url); not expected_path_file )
  {
    return expected_path_file.error();
  } // if

  return std::nullopt;
} // }}}

// sources_layer() {{{
[[nodiscard]] inline std::expected<sources_layer_ret_t, std::string> sources_layer(ns_enum::Platform const& platform)
{
  // Temporary file with fetch list
  auto opt_path_file_sources = get_path_sources();
  qreturn_if(not opt_path_file_sources, std::unexpected(opt_path_file_sources.error()));
  // Open file as database
  auto db_fetch = ns_db::ns_fetch::read(*opt_path_file_sources);
  ethrow_if(not db_fetch, db_fetch.error());
  // Get wine distribution
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, db_build.error());
  // Fetch layer url
  std::string str_url_layer = db_fetch
      ->get_platform(platform)
      ->get_layer((platform == ns_enum::Platform::WINE)? db_build->dist_wine : "default");
  // Show url
  ns_log::write('i', "url to fetch: ", str_url_layer);
  fs::path path_dir_dst = (platform == ns_enum::Platform::LINUX)?
      opt_path_file_sources->parent_path() / "cache/linux.flatimage"
    : opt_path_file_sources->parent_path() / "cache/{}.layer"_fmt(ns_enum::to_string_lower(platform));
  // Create destination / url pair
  return sources_layer_ret_t { .path = path_dir_dst, .url = cpr::Url(str_url_layer), };
} // sources_layer() }}}

// fetch_layer() {{{
[[nodiscard]] inline std::expected<sources_layer_ret_t, std::string> fetch_layer(ns_enum::Platform platform)
{
  // Resolve URL
  auto expected_path_and_url_layer = sources_layer(platform);
  qreturn_if(not expected_path_and_url_layer, std::unexpected(expected_path_and_url_layer.error()));
  auto [path_target, url] = *expected_path_and_url_layer;
  // Fetch
  auto error_fetch = fetch_on_failed_check(path_target, url);
  qreturn_if(error_fetch, std::unexpected(*error_fetch));
  // Send 100% completion
  ns_ipc::ipc().send(100);
  // Return fetched path and url
  return *expected_path_and_url_layer;
} // fetch() }}}

} // anonymous namespace

// fetch_cores() {{{
[[nodiscard]] inline std::expected<std::vector<ns_db::ns_fetch::CoreUrl>,std::string> fetch_cores()
{
  // Define sources file
  auto opt_path_file_sources = get_path_sources();
  qreturn_if(not opt_path_file_sources, std::unexpected(opt_path_file_sources.error()));
  // Fetch from remote
  if ( auto expected = fetch_file_from_url(*opt_path_file_sources, cpr::Url{URL_FETCH}); not expected)
  {
    return std::unexpected(expected.error());
  } // if
  // Open as a database
  auto database = ns_db::ns_fetch::read(*opt_path_file_sources);
  ethrow_if(not database, database.error());
  // Return cores
  return database->get_platform(ns_enum::Platform::RETROARCH)->get_cores();
} // fetch_cores() }}}

// sources() {{{
[[nodiscard]] inline std::expected<void,std::string> sources()
{
  // Define sources file
  auto opt_path_file_sources = get_path_sources();
  qreturn_if(not opt_path_file_sources, std::unexpected(opt_path_file_sources.error()));
  auto expected = fetch_file_from_url(*opt_path_file_sources, cpr::Url{URL_FETCH});
  qreturn_if(not expected, std::unexpected(expected.error()));
  return {};
} // sources() }}}

// fetch() {{{
[[nodiscard]] inline std::expected<void,std::string> fetch(ns_enum::Platform platform)
{
  std::expected<sources_layer_ret_t, std::string> sources_layer;
  sources_layer = fetch_layer(platform);
  qreturn_if(not sources_layer, std::unexpected(sources_layer.error()));
  return {};
} // fetch() }}}

// installed() {{{
[[nodiscard]] inline std::vector<ns_enum::Platform> installed()
{
  // Get path to cache directory
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build.has_value(), "Could not get cache directory: {}"_fmt(db_build.error()));
  // Gather installed platforms
  std::error_code ec;
  auto platforms = fs::directory_iterator(db_build->path_dir_cache, ec)
    | std::views::filter([](auto&& e){ return fs::is_regular_file(e) and ns_enum::is_enum_entry<ns_enum::Platform>(e.path().stem()); })
    | std::views::transform([](auto&& e){ return ns_enum::from_string<ns_enum::Platform>(e.path().stem()); })
    | std::ranges::to<std::vector<ns_enum::Platform>>();
  std::ranges::sort_unique(platforms);
  return platforms;
} // installed() }}}

// sha() {{{
[[nodiscard]] inline std::expected<void,std::string> sha(ns_enum::Platform platform)
{
  // Log
  ns_log::write('i', "platform: ", ns_enum::to_string_lower(platform));
  ns_log::write('i', "Only checking SHA");
  // Get layer
  auto path_and_url_layer = ehope(sources_layer(platform));
  // Check sha for layer
  qreturn_if(not check_file(path_and_url_layer.path, path_and_url_layer.url)
    , std::unexpected("Failed to check file '{}'"_fmt(path_and_url_layer.path))
  );
  return {};
} // sha() }}}

} // namespace ns_fetch

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
