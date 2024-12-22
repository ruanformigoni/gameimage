///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : db
///

#pragma once

#include <filesystem>
#include <fstream>
#include <nlohmann/json.hpp>
#include <variant>

#include "../common.hpp"
#include "../enum.hpp"
#include "../macro.hpp"

#include "log.hpp"

namespace ns_db
{

using object_t = nlohmann::basic_json<>::object_t;
class Db;

namespace
{

namespace fs = std::filesystem;


using json_t = nlohmann::json;
using Exception = json_t::exception;

template<typename T>
concept IsString =
     std::convertible_to<std::decay_t<T>, std::string>
  or std::constructible_from<std::string, std::decay_t<T>>;

} // anonymous namespace

// enum class Mode {{{
enum class Mode
{
  READ,
  CREATE,
  UPDATE,
};
// }}}

// class Db {{{
class Db
{
  private:
    std::variant<json_t, std::reference_wrapper<json_t>> m_json;
    fs::path m_path_file_db;

    json_t& data();
    json_t const& data() const;
  public:
    // Constructors
    Db(std::reference_wrapper<json_t> json) noexcept;
    // Element access
    [[nodiscard]] std::vector<std::string> keys() const noexcept;
    template<typename V = Db, IsString... Ks>
    [[nodiscard]] std::expected<V,std::string> value(Ks&&... ks) noexcept;
    template<typename V>
    [[nodiscard]] V value_or_default(IsString auto&& k, V&& v = V{}) const noexcept;
    template<typename F, IsString Ks>
    [[nodiscard]] decltype(auto) apply(F&& f, Ks&& ks);
    // Capacity
    [[nodiscard]] bool empty() const noexcept;
    // Lookup
    template<IsString T>
    [[nodiscard]] bool contains(T&& t) const noexcept;
    // Modifiers
    template<IsString T>
    [[nodiscard]] bool erase(T&& t);
    // Operators
    template<typename T>
    T operator=(T&& t);
    template<IsString T>
    [[nodiscard]] Db operator()(T&& t);
    // Friends
    friend std::ostream& operator<<(std::ostream& os, Db const& db);
}; // class: Db }}}

// Constructors {{{
inline Db::Db(std::reference_wrapper<json_t> json) noexcept
{
  m_json = json;
} // Constructors }}}

// data() {{{
inline json_t& Db::data()
{
  if (std::holds_alternative<std::reference_wrapper<json_t>>(m_json))
  {
    return std::get<std::reference_wrapper<json_t>>(m_json).get();
  } // if

  return std::get<json_t>(m_json);
} // data() }}}

// data() const {{{
inline json_t const& Db::data() const
{
  return const_cast<Db*>(this)->data();
} // data() const }}}

// keys() {{{
inline std::vector<std::string> Db::keys() const noexcept
{
  return data().items()
    | std::views::transform([](auto&& e){ return e.key(); })
    | std::ranges::to<std::vector<std::string>>();
} // keys() }}}

// contains() {{{
template<IsString T>
bool Db::contains(T&& t) const noexcept
{
  return data().contains(t);
} // contains() }}}

// empty() {{{
inline bool Db::empty() const noexcept
{
  return data().empty();
} // empty() }}}

// value() {{{
template<typename V, IsString... Ks>
std::expected<V, std::string> Db::value(Ks&&... ks) noexcept
{
  if constexpr ( sizeof...(ks) == 0 )
  {
    json_t& json = data();
    return ( json.is_string() )? std::expected<V,std::string>(std::string{json})
      : std::unexpected("Json element is not a string");
  } // if
  else
  {
    // Use keys to access nested json elements
    auto f_access_impl = [&]<typename T, typename U>(T& value, U&& u)
    {
      // ns_log::write('i', "Access '{}'"_fmt(u));
      // Check if json is still valid
      qreturn_if(not value.has_value());
      // Get json database
      json_t& db = value->get();
      // Access value
      value = ( db.contains(u) )?
          std::expected<std::reference_wrapper<json_t>,std::string>(std::reference_wrapper(db[u]))
        : std::unexpected("Could not access key '{}' in database"_fmt(u));
    }; // f_access
    auto f_access = [&]<typename T, typename... U>(T& value, U&&... u)
    {
      ( f_access_impl(value, std::forward<U>(u)), ... );
    }; // f_access
    std::expected<std::reference_wrapper<json_t>,std::string> expected_json = std::reference_wrapper(data());
    f_access(expected_json, std::forward<Ks>(ks)...);
    // Check if access was successful
    // Return a novel db with a non-owning view over the accessed data or use to create a type V
    if constexpr ( std::same_as<V,Db> )
    {
      return Db{expected_json->get()};
    } // if
    else if constexpr ( ns_concept::IsVector<V> )
    {
      json_t json = expected_json->get();
      qreturn_if(not json.is_array(), std::unexpected("Tried to create array with non-array entry"));
      return std::ranges::subrange(json.begin(), json.end())
        | std::views::transform([](auto&& e){ return typename std::remove_cvref_t<V>::value_type(e); })
        | std::ranges::to<V>();
    } // if
    else
    {
      return ns_exception::to_expected([&]{ return V{expected_json->get()}; });
    } // else
  } // else
} // value() }}}

// value_or_default() {{{
template<typename V>
V Db::value_or_default(IsString auto&& k, V&& v) const noexcept
{
  // Read current json
  json_t json = data();
  // Check if has value
  ereturn_if(not json.contains(k), "Could not find '{}' in database"_fmt(k), v);
  // Access value
  json = json[k];
  // Return range or string type
  if constexpr ( ns_concept::IsVector<V> )
  {
    ereturn_if(not json.is_array(), "Tried to access non-array as array in DB with key '{}'"_fmt(k), v);
    return std::ranges::subrange(json.begin(), json.end())
      | std::views::transform([](auto&& e){ return typename std::remove_cvref_t<V>::value_type(e); })
      | std::ranges::to<V>();
  } // if
  else
  {
    return V{json};
  } // else
} // value_or_default() }}}

// apply() {{{
// Key exists or is created, and is accessed
template<typename F, IsString Ks>
decltype(auto) Db::apply(F&& f, Ks&& ks)
{
  if (auto access = value(std::forward<Ks>(ks)))
  {
    return ns_exception::to_expected([&]{ return f(*access); });
  } // if
  return std::unexpected("Could not apply function");
} // apply() }}}

// operator() {{{
// Key exists or is created, and is accessed
template<IsString T>
Db Db::operator()(T&& t)
{
  json_t& json = data();

  // Access key
  try
  {
    return Db{std::reference_wrapper<json_t>(json[t])};
  } // try
  catch(std::exception const& e)
  {
    "Failed to parse key '{}': {}"_throw(t, e.what());
  } // catch

  // Unreachable, used to suppress no return warning
  return Db{std::reference_wrapper<json_t>(json[t])};
} // operator() }}}

// erase() {{{
template<IsString T>
bool Db::erase(T&& t)
{
  json_t& json = data();

  auto key = ns_string::to_string(t);

  if ( json.is_array() )
  {
    // Search in array & erase if there is a match
    auto it_search = std::find(json.begin(), json.end(), key);
    if ( it_search == json.end() ) { return false; }
    json.erase(std::distance(json.begin(), it_search));
    return true;
  }

  // Erase returns the number of elements removed
  return json.erase(key) > 1;
} // erase() }}}

// operator= {{{
template<typename T>
T Db::operator=(T&& t)
{
  data() = t;
  return t;
} // operator= }}}

// operator<< {{{
inline std::ostream& operator<<(std::ostream& os, Db const& db)
{
  os << db.data();
  return os;
} // operator<< }}}

// from_file() {{{
template<typename Ret = void>
auto from_file(IsString auto&& t, auto&& f, Mode mode) -> std::expected<Ret, std::string>
{
  fs::path path_file_db{ns_string::to_string(t)};
  // Parse a file
  auto f_parse_file = [](std::ifstream const& f) -> std::optional<json_t>
  {
    // Read to string
    std::string contents = ns_string::to_string(f.rdbuf());
    // Validate contents
    qreturn_if (json_t::accept(contents),  json_t::parse(contents));
    // Failed to parse
    return std::nullopt;
  };
  // Write a file
  auto f_write_file = [](fs::path const& path_file_db, json_t const& json, Mode const& mode)
  {
    if ( mode == Mode::READ ) { return; }
    std::ofstream file(path_file_db, std::ios::trunc);
    ereturn_if(not file.is_open(), "Failed to open '{}' for writing");
    file << std::setw(2) << json;
  };
  // Create json object
  json_t json;
  if ( mode == Mode::READ or mode == Mode::UPDATE )
  {
    // Open target file as read
    std::ifstream file(path_file_db, std::ios::in);
    // Check for failure
    qreturn_if(not file.is_open(), std::unexpected("Failed to open '{}'"_fmt(path_file_db)));
    // Try to parse
    auto optional_json = f_parse_file(file);
    qreturn_if(not optional_json, std::unexpected("Failed to parse db '{}'"_fmt(path_file_db)));
    json = *optional_json;
  } // if
  else
  {
    // Print file name
    ns_log::write('i', "Creating db file '", path_file_db);
    // Create empty json
    json = json_t::parse("{}");
  } // else
  // Create DB
  Db db = Db(std::reference_wrapper(json));
  // Access
  if constexpr ( std::same_as<Ret,void> )
  {
    f(db);
    f_write_file(path_file_db, json, mode);
    return std::expected<void,std::string>{};
  } // if
  else
  {
    auto ret = f(db);
    f_write_file(path_file_db, json, mode);
    return ret;
  } // else

} // function: from_file }}}

// from_string() {{{
template<typename Ret = void, ns_concept::AsString S>
auto from_string(S&& s, auto&& f) -> std::expected<Ret, std::string>
{
  // Create json object
  json_t json;
  // Create empty json
  json = json_t::parse(ns_string::to_string(s));
  // Create DB
  Db db = Db(std::reference_wrapper(json));
  // Access
  if constexpr ( std::same_as<Ret,void> )
  {
    f(db);
    return std::expected<void,std::string>{};
  } // if
  else
  {
    auto ret = f(db);
    return ret;
  } // else

} // function: from_string }}}

} // namespace ns_db

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
