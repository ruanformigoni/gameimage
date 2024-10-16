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

#include "../std/filesystem.hpp"

#include "../lib/log.hpp"

namespace ns_db
{

namespace
{

namespace fs = std::filesystem;


using json_t = nlohmann::json;
using Exception = json_t::exception;

template<typename T>
concept IsString =
     std::convertible_to<std::decay_t<T>, std::string>
  or std::constructible_from<std::string, std::decay_t<T>>;

// class JsonIterator {{{
template<typename IteratorType>
class JsonIterator
{
  private:
    IteratorType m_it;

  public:
    using iterator_category = std::forward_iterator_tag;
    using value_type = typename IteratorType::value_type;
    using difference_type = typename IteratorType::difference_type;
    using pointer = typename IteratorType::pointer;
    using reference = typename IteratorType::reference;

    // Construct with an nlohmann::json iterator
    explicit JsonIterator(IteratorType it) : m_it(it) {}

    // Increment operators
    JsonIterator& operator++() { ++m_it; return *this; }
    JsonIterator operator++(int) { JsonIterator tmp = *this; ++(*this); return tmp; }

    // Dereference operators
    reference operator*() const { return *m_it; }
    pointer operator->() const { return &(*m_it); }

    // Comparison operators
    bool operator==(const JsonIterator& other) const { return m_it == other.m_it; }
    bool operator!=(const JsonIterator& other) const { return m_it != other.m_it; }
}; // }}}

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
    Mode m_mode;

    Db(std::reference_wrapper<json_t> json);

    json_t& data();
    json_t& data() const;
  public:
    // Iterators
    using iterator = JsonIterator<json_t::iterator>;
    using const_iterator = JsonIterator<json_t::const_iterator>;
    const_iterator cbegin() const { return const_iterator(data().cbegin()); }
    const_iterator cend() const { return const_iterator(data().cend()); }
    iterator begin() { return iterator(data().begin()); }
    iterator end() { return iterator(data().end()); }
    const_iterator begin() const { return const_iterator(data().cbegin()); }
    const_iterator end() const { return const_iterator(data().cend()); }

    // Constructors
    Db() = delete;
    Db(Db const&) = delete;
    Db(Db&&) = delete;
    Db(fs::path t, Mode mode);

    // Destructors
    ~Db();

    // Access
    decltype(auto) items() const;
    template<bool _throw = true, IsString T>
    bool contains(T&& t) const;
    template<typename T = std::string>
    std::vector<T> to_vector() const;
    bool empty() const;

    // Modifying
    template<IsString T>
    bool erase(T&& t);

    // Operators
    operator std::string() const;
    operator fs::path() const;
    operator json_t() const;
    Db operator=(Db const&) = delete;
    Db operator=(Db&&) = delete;
    template<IsString T>
    Db const& operator[](T&& t) const;
    template<IsString T>
    Db operator()(T&& t);
    template<IsString T>
    T operator=(T&& t);
    template<IsString T>
    std::vector<T> operator=(std::vector<T> const& t);
    template<IsString T>
    Db& operator|=(T&& t);

    friend std::ostream& operator<<(std::ostream& os, Db const& db);
}; // class: Db }}}

// Constructors {{{
inline Db::Db(std::reference_wrapper<json_t> json)
{
  m_json = json;
} // Json

inline Db::Db(fs::path t, Mode mode)
  : m_path_file_db(t)
  , m_mode(mode)
{
  ns_log::write('i', "Open file '", m_path_file_db, "'", " as ", ns_enum::to_string(mode));

  auto f_parse_file = [](std::string const& name_file, std::ifstream const& f)
  {
    // Read to string
    std::string contents = ns_string::to_string(f.rdbuf());
    // Validate contents
    if ( ! json_t::accept(contents) )
    {
      // Failed to parse
      "Failed to parse db '{}', will create if mode is write"_throw(name_file);
    } // if
    // Parse contents
    return json_t::parse(contents);
  };

  // Open file on read mode
  if ( mode == Mode::READ or mode == Mode::UPDATE )
  {
    // Open target file as read
    std::ifstream file(t, std::ios::in);
    // Check for failure
    "Failed to open '{}'"_throw_if([&]{ return ! file.good(); }, t);
    // Try to parse
    m_json = f_parse_file(t.string(), file);
    // Close file
    file.close();
    return;
  } // if

  if ( mode == Mode::CREATE )
  {
    // Print file name
    ns_log::write('i', "Creating db file '", t);
    // Create empty json
    m_json = json_t::parse("{}");
    return;
  } // else

  "Invalid open mode in db"_throw();
} // Db

// }}}

// Destructors {{{

inline Db::~Db()
{
  if ( ( m_mode == Mode::UPDATE or m_mode == Mode::CREATE ) && std::holds_alternative<json_t>(m_json))
  {
    std::ofstream file(m_path_file_db, std::ios::trunc);
    "Failed to open '{}' for writing"_throw_if([&]{ return ! file.good(); }, m_path_file_db);
    file << std::setw(2) << std::get<json_t>(m_json);
    file.close();
  } // if
} // Db

// }}}

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
inline json_t& Db::data() const
{
  return const_cast<Db*>(this)->data();
} // data() const }}}

// items() {{{
inline decltype(auto) Db::items() const
{
  return data().items();
} // items() }}}

// to_vector() {{{
template<typename T>
std::vector<T> Db::to_vector() const
{
  json_t& json = data();
  ethrow_if(not json.is_array(), "Tried to access non-array as array in DB");
  std::vector<T> vector;
  std::for_each(json.begin(), json.end(), [&](std::string e){ vector.push_back(T{e}); });
  return vector;
} // to_vector() }}}

// contains() {{{
template<bool _throw, IsString T>
bool Db::contains(T&& t) const
{
  auto&& json = data();

  if ( json.contains(t) )
  {
    return true;
  } // if

  if constexpr ( _throw ) { "'{}' not found in json"_throw(t); } // if 

  return false;
} // contains() }}}

// empty() {{{
inline bool Db::empty() const
{
  return data().empty();
} // empty() }}}

// operator::string() {{{
inline Db::operator std::string() const
{
  return data();
} // operator::string() }}}

// operator::fs::path() {{{
inline Db::operator fs::path() const
{
  return data();
} // operator::fs::path() }}}

// operator::json_t() {{{
inline Db::operator json_t() const
{
  return data();
} // operator::fs::path() }}}

// operator[] {{{
// Key exists and is accessed
template<IsString T>
Db const& Db::operator[](T&& t) const
{
  static std::unique_ptr<Db> db;

  json_t& json = data();

  // Check if key is present
  if ( ! json.contains(t) )
  {
    "Key '{}' not present in db file"_throw(t);
  } // if

  // Access key
  try
  {
    db = std::unique_ptr<Db>(new Db{std::reference_wrapper<json_t>(json[t])});
  } // try
  catch(std::exception const& e)
  {
    "Failed to parse key '{}': {}"_throw(t, e.what());
  } // catch

  // Unreachable, used to suppress no return warning
  return *db;
} // operator[] }}}

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

  // When key was found, returns 1
  return json.erase(key) == 1;
} // erase() }}}

// operator=(IsString) {{{
template<IsString T>
T Db::operator=(T&& t)
{
  data() = t;
  return t;
} // operator=(IsString) }}}

// operator=(vector<path>) {{{
template<IsString T>
inline std::vector<T> Db::operator=(std::vector<T> const& t)
{
  data() = t;
  return t;
} // operator=(vector<path>) }}}

// operator|= {{{
template<IsString T>
Db& Db::operator|=(T&& t)
{
  auto& json = data();
  if ( std::find_if(json.cbegin()
    , json.cend()
    , [&](auto&& e){ return std::string{e} == t; }) == json.cend() )
  {
    json.push_back(std::forward<T>(t));
  } // if
  return *this;
  // else
} // operator|= }}}

// operator<< {{{
inline std::ostream& operator<<(std::ostream& os, Db const& db)
{
  os << db.data();
  return os;
} // operator<< }}}

// from_file() {{{
template<IsString T, typename F>
void from_file(T&& t, F&& f, Mode mode)
{
  // Create DB
  Db db = Db(std::forward<T>(t), mode);
  // Access
  f(db);
} // function: from_file }}}

// to_file() {{{
template<IsString T>
void to_file(Db const& json, T&& t)
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
  fs::path path_project;

  from_file(file_default(), [&](auto&& db)
  { 
    std::string str_project = db["project"];
    path_project = std::string(db["projects"][str_project]["path_dir_project"]);
  }, Mode::READ);

  return path_project /= "gameimage.json";
} // file_project() }}}

// from_file_default() {{{
template<typename F>
inline void from_file_default(F&& f, Mode mode)
{
  from_file(file_default(), f, mode);
} // function: from_file_default }}}

// from_file_project() {{{
template<typename F>
inline void from_file_project(F&& f, Mode mode)
{
  from_file(file_project(), f, mode);
} // function: from_file_project }}}

// query() {{{
template<typename F, typename... Args>
inline std::string query(F&& file, Args... args)
{
  std::string ret;

  auto f_access_impl = [&]<typename T, typename U>(T& ref_db, U&& u)
  {
    ref_db = std::reference_wrapper(ref_db.get()[u]);
  }; // f_access

  auto f_access = [&]<typename T, typename... U>(T& ref_db, U&&... u)
  {
    ( f_access_impl(ref_db, std::forward<U>(u)), ... );
  }; // f_access

  from_file(file, [&]<typename T>(T&& db)
  {
    // Get a ref to db
    auto ref_db = std::reference_wrapper<Db const>(db);

    // Update the ref to the selected query object
    f_access(ref_db, std::forward<Args>(args)...);

    // Assign result
    ret = ref_db.get();
  }, Mode::READ);

  return ret;
} // query() }}}

} // namespace ns_db

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
