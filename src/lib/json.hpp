///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : json
///

#pragma once

#include <filesystem>
#include <fstream>
#include <nlohmann/json.hpp>

#include "../common.hpp"

#include "../std/filesystem.hpp"

namespace ns_json
{

namespace fs = std::filesystem;

using json = nlohmann::json;
using Exception = json::exception;

template<typename T>
concept IsString =
     std::convertible_to<std::decay_t<T>, std::string>
  or std::constructible_from<std::string, std::decay_t<T>>;

// class Json {{{
class Json
{
  private:
    json m_json;

  public:
    Json()
    {} // Json

    Json(Json const& json)
    {
      m_json = json.m_json;
    } // Json

    template<IsString T>
    Json(T&& t)
    {
      std::ifstream ifile{t};
      if ( ! ifile.good() )
      {
        "Failed to open '{}'"_throw(t);
      } // if
      m_json = json::parse(ifile);
    } // Json

    operator std::string() const
    {
      return m_json;
    } // operator std::string

    template<IsString T>
    json& operator[](T&& t)
    {
      if ( ! m_json.contains(std::forward<T>(t)) )
      {
        "Key '{}' not present in json file"_fmt(t);
      } // if
      return m_json[std::forward<T>(t)];
    } // operator[]

    template<IsString T>
    T operator=(T&& t)
    {
      m_json[t];
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
std::ostream& operator<<(std::ostream& os, Json const& json)
{
  os << json.m_json;
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
fs::path default_file()
{
  return fs::current_path() /= "gameimage.json";
} // default_file() }}}

// from_default_file() {{{
Json from_default_file()
{
  return from_file(default_file());
} // function: from_default_file }}}

// to_default_file() {{{
void to_default_file(Json const& json)
{
  to_file(json, default_file());
} // function: to_default_file }}}

} // namespace ns_json

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
