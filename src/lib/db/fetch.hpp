///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : fetch
///

#pragma once

#include "../hope.hpp"
#include "../db.hpp"
#include "../../macro.hpp"

namespace ns_db::ns_fetch
{

namespace
{

// Forward declarations
class Fetch;
std::expected<Fetch, std::string> read_impl(fs::path const& path_file_db);

// platforms() {{{
struct CoreUrl { std::string core; std::string url; };
struct Platform
{
  private:
    std::map<std::string,std::string> m_url_layer;
    std::vector<CoreUrl> m_vec_core_url;
  public:
    virtual std::string get_layer(std::string identifier = "default")
    {
      ethrow_if(not m_url_layer.contains(identifier), "Layer '{}' not found for platform"_fmt(identifier));
      return m_url_layer[identifier];
    };
    virtual std::vector<CoreUrl> get_cores() const { return m_vec_core_url; };
    virtual ~Platform() {};
  friend std::expected<Fetch, std::string> read_impl(fs::path const& path_file_db);
};
struct Linux     : public Platform {};
struct Retroarch : public Platform {};
struct Pcsx2     : public Platform {};
struct Rpcs3     : public Platform {};
struct Wine      : public Platform {};
// platforms() }}}

// class Fetch {{{
class Fetch
{
  private:
    std::unique_ptr<Linux> m_linux = std::make_unique<Linux>();
    std::unique_ptr<Retroarch> m_retroarch = std::make_unique<Retroarch>();
    std::unique_ptr<Pcsx2> m_pcsx2 = std::make_unique<Pcsx2>();
    std::unique_ptr<Rpcs3> m_rpcs3 = std::make_unique<Rpcs3>();
    std::unique_ptr<Wine> m_wine = std::make_unique<Wine>();
    std::string m_version;
    Fetch() = default;
  public:
    std::unique_ptr<Platform> get_platform(ns_enum::Platform platform) const
    {
      switch (platform)
      {
        case ns_enum::Platform::LINUX     : return std::make_unique<Platform>(*m_linux);
        case ns_enum::Platform::RETROARCH : return std::make_unique<Platform>(*m_retroarch);
        case ns_enum::Platform::PCSX2     : return std::make_unique<Platform>(*m_pcsx2);
        case ns_enum::Platform::RPCS3     : return std::make_unique<Platform>(*m_rpcs3);
        case ns_enum::Platform::WINE      : return std::make_unique<Platform>(*m_wine);
      } // switch
      throw std::runtime_error("Unknown platform");
    } // get_platform

  friend std::expected<Fetch, std::string> read_impl(fs::path const& path_file_db);
}; // class Fetch }}}

// read_impl() {{{
inline std::expected<Fetch, std::string> read_impl(fs::path const& path_file_db)
{
  return ns_db::from_file<std::expected<Fetch, std::string>>(path_file_db, [&](auto&& db) -> std::expected<Fetch, std::string>
  {
    Fetch fetch;
    // Linux
    fetch.m_linux->m_url_layer["default"] = ehope(db.template value<std::string>("linux", "layer"));
    // Pcsx2
    fetch.m_pcsx2->m_url_layer["default"] = ehope(db.template value<std::string>("pcsx2", "layer"));
    // Rpcs3
    fetch.m_rpcs3->m_url_layer["default"] = ehope(db.template value<std::string>("rpcs3", "layer"));
    // Wine
    auto layers = ehope(db.value("wine", "layer"));
    for(auto const& key : layers.keys())
    {
      fetch.m_wine->m_url_layer[key] = ehope(db.template value<std::string>("wine", "layer", key));
    } // for
    // Retroarch
    fetch.m_retroarch->m_url_layer["default"] = ehope(db.template value<std::string>("retroarch", "layer"));
    auto cores = ehope(db.value("retroarch", "core"));
    for(auto const& key : cores.keys())
    {
      if (auto value = db.template value<std::string>("retroarch", "core", key))
      {
        fetch.m_retroarch->m_vec_core_url.push_back(CoreUrl{ key, *value });
      } // if
      else
      {
        ns_log::write('e', "Could not get value for key '", key, "'");
      } // else
    } // for
    return fetch;
  }, ns_db::Mode::READ).value_or(std::unexpected("Could not read 'fetch' database"));
} // read_impl() }}}

} // namespace

// read() {{{
inline std::expected<Fetch, std::string> read(fs::path const& path_file_db)
{
  return read_impl(path_file_db);
} // read() }}}

} // namespace ns_db::ns_fetch

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
