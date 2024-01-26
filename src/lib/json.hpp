///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : json
///

#pragma once

#include <filesystem>
#include <fstream>
#include <nlohmann/json.hpp>
#include <variant>

#include "../common.hpp"

#include "../std/filesystem.hpp"

#include "../lib/log.hpp"

namespace ns_json
{

namespace fs = std::filesystem;

using json_t = nlohmann::json;
using Exception = json_t::exception;

template<typename T>
concept IsString =
     std::convertible_to<std::decay_t<T>, std::string>
  or std::constructible_from<std::string, std::decay_t<T>>;

// class Json {{{
class Json
{
  private:
    std::variant<json_t, std::reference_wrapper<json_t>> m_json;

    Json(std::reference_wrapper<json_t> json)
    {
      m_json = json;
    } // Json

    json_t& data()
    {
      if (std::holds_alternative<std::reference_wrapper<json_t>>(m_json))
      {
        return std::get<std::reference_wrapper<json_t>>(m_json).get();
      } // if
      
      return std::get<json_t>(m_json);
    } // get

    json_t data() const
    {
      return const_cast<Json*>(this)->data();
    } // get

  public:
    Json()
      : m_json(json_t{})
    {} // Json

    Json(Json const& json)
    {
      m_json = json.m_json;
    } // Json

    Json(fs::path t)
    {
      try
      {
        std::ifstream ifile{t};
        if ( ! ifile.good() )
        {
          "Failed to open '{}'"_throw(t);
        } // if
        m_json = json_t::parse(ifile);
      } // try
      catch(std::exception const& e)
      {
        "Could not open file '{}'"_throw(t, e.what());
      } // catch
    } // Json

    template<bool _throw = true, IsString T>
    bool contains(T&& t)
    {
      if constexpr ( _throw )
      {
        if ( ! data().contains(t) )
        {
          "'{}' not found in json"_throw(t);
        } // if
      } // if

      return data().contains(t);
    } // function: contains

    operator std::string() const
    {
      return data();
    } // operator std::string

    operator fs::path() const
    {
      return data();
    } // operator fs::path

    template<IsString T>
    Json operator[](T&& t)
    {
      // Check if key is present
      if ( ! data().contains(std::forward<T>(t)) )
      {
        "Key '{}' not present in json file"_throw(t);
      } // if

      // Get reference to current value
      json_t& json = data()[std::forward<T>(t)];

      // Access key
      try
      {
        return Json{std::reference_wrapper<json_t>(json)};
      } // try
      catch(std::exception const& e)
      {
        "Failed to parse json key '{}': {}"_throw(e.what());
      } // catch

      // Unreachable, used to suppress no return warning
      return {};
    } // operator[]

    template<IsString T>
    T operator=(T&& t)
    {
      data() = t;
      return t;
    } // operator=

    Json& operator=(Json json)
    {
      this->m_json = json.m_json;
      return *this;
      // else
    } // operator=

    friend std::ostream& operator<<(std::ostream& os, Json const& json);
}; // class: Json }}}

// operator<< {{{
inline std::ostream& operator<<(std::ostream& os, Json const& json)
{
  os << json.data();
  return os;
} // operator<< }}}

// from_file() {{{
template<IsString T>
Json from_file(T&& t)
{
  return Json(std::forward<T>(t));
} // function: from_file }}}

// to_file() {{{
template<IsString T>
void to_file(Json const& json, T&& t)
{
  std::ofstream ofile_json{t};
  ofile_json << std::setw(2) << json;
  ofile_json.close();
} // function: to_file }}}

// default_file() {{{
inline fs::path default_file()
{
  return fs::current_path() /= "gameimage.json";
} // default_file() }}}

// from_default_file() {{{
inline Json from_default_file()
{
  return from_file(default_file());
} // function: from_default_file }}}

// to_default_file() {{{
inline void to_default_file(Json const& json)
{
  to_file(json, default_file());
} // function: to_default_file }}}

} // namespace ns_json

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
