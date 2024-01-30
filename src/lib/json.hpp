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
        // Open file as read
        std::ifstream ifile{t};
        if ( ! ifile.good() )
        {
          "Failed to open '{}'"_throw(t);
        } // if

        // Parse json
        m_json = json_t::parse(ifile);
      } // try
      catch(std::exception const& e)
      {
        "Could not open file '{}': {}"_throw(t, e.what());
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

    // Key exists and is accessed
    template<IsString T>
    Json operator[](T&& t)
    {
      json_t& json = data();

      // Check if key is present
      if ( ! json.contains(t) )
      {
        "Key '{}' not present in json file"_throw(t);
      } // if

      // Access key
      try
      {
        return Json{std::reference_wrapper<json_t>(json[t])};
      } // try
      catch(std::exception const& e)
      {
        "Failed to parse json key '{}': {}"_throw(t, e.what());
      } // catch

      // Unreachable, used to suppress no return warning
      return {};
    } // operator[]

    // Key exists or is created, and is accessed
    template<IsString T>
    Json operator()(T&& t)
    {
      json_t& json = data();

      // Access key
      try
      {
        return Json{std::reference_wrapper<json_t>(json[t])};
      } // try
      catch(std::exception const& e)
      {
        "Failed to parse json key '{}': {}"_throw(t, e.what());
      } // catch

      // Unreachable, used to suppress no return warning
      return {};
    } // operator()

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

// file_default() {{{
inline fs::path file_default()
{
  return fs::current_path() /= "gameimage.json";
} // file_default() }}}

// file_project() {{{
inline fs::path file_project()
{
  Json json = from_file(file_default());
  std::string str_project = json["project"];
  fs::path path_project = json[str_project]["path-app"];
  return path_project /= "gameimage.json";
} // file_project() }}}

// from_file_default() {{{
inline Json from_file_default()
{
  return from_file(file_default());
} // function: from_file_default }}}

// from_file_project() {{{
inline Json from_file_project()
{
  return from_file(file_project());
} // function: from_file_project }}}

// to_file_default() {{{
inline void to_file_default(Json const& json)
{
  to_file(json, file_default());
} // function: to_file_default }}}

// to_file_project() {{{
inline void to_file_project(Json const& json)
{
  to_file(json, file_project());
} // function: to_file_project }}}

} // namespace ns_json

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
