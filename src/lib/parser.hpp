///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : parser
// @created     : Saturday Jan 20, 2024 23:29:45 -03
///

#pragma once

#include <filesystem>

#include "../enum.hpp"
#include "../common.hpp"
#include "../lib/db.hpp"
#include "../lib/hope.hpp"
#include "../cmd/desktop.hpp"

namespace ns_parser
{

namespace
{

namespace fs = std::filesystem;

} // namespace

enum class Op
{
  FETCH,
  INIT,
  PROJECT,
  INSTALL,
  COMPRESS,
  SEARCH,
  SELECT,
  TEST,
  DESKTOP,
  PACKAGE,
};

// parse_init() {{{

enum class OpInit { BUILD, PROJECT };
struct Init
{
  OpInit op;
  std::optional<fs::path> path_dir_build;
  std::optional<std::string> name;
  std::optional<ns_enum::Platform> platform;
};

[[nodiscard]] inline std::expected<Init, std::string> parse_init(auto& db)
{
  Init init;
  std::string op_init = ehope(db.template value<std::string>("init", "op"));
  switch(ns_enum::from_string<OpInit>(op_init))
  {
    case OpInit::BUILD:
    {
      init.op = OpInit::BUILD;
      init.path_dir_build = ehope(db.template value<std::string>("init", "path_dir_build"));
    } // case
    break;
    case OpInit::PROJECT:
    {
      auto platform = ehope(db.template value<std::string>("init", "platform"));
      init.op = OpInit::PROJECT;
      init.name = ehope(db.template value<std::string>("init", "name"));
      init.platform = ns_enum::from_string<ns_enum::Platform>(platform);
    } // case
    break;
  } // switch
  return init;
} // parse_init() }}}

// parse_fetch() {{{
enum class OpFetch { SOURCES, FETCH, INSTALLED, SHA, };
struct Fetch
{
  OpFetch op;
  std::optional<ns_enum::Platform> platform;
};

[[nodiscard]] inline std::expected<Fetch, std::string> parse_fetch(auto& db)
{
  Fetch fetch;
  std::string op_fetch = ehope(db.template value<std::string>("fetch", "op"));
  switch(ns_enum::from_string<OpFetch>(op_fetch))
  {
    case OpFetch::SOURCES:
    {
      fetch.op = OpFetch::SOURCES;
    } // case
    break;
    case OpFetch::FETCH:
    {
      std::string platform = ehope(db.template value<std::string>("fetch", "platform"));
      fetch.op = OpFetch::FETCH;
      fetch.platform = ns_enum::from_string<ns_enum::Platform>(platform);
    } // case
    break;
    case OpFetch::INSTALLED:
    {
      fetch.op = OpFetch::INSTALLED;
    }
    break;
    case OpFetch::SHA:
    {
      std::string platform = ehope(db.template value<std::string>("fetch", "platform"));
      fetch.op = OpFetch::SHA;
      fetch.platform = ns_enum::from_string<ns_enum::Platform>(platform);
    }
    break;
  } // switch
  return fetch;
}
// parse_fetch() }}}

// parse_project() {{{
enum class OpProject { SET, DEL };
struct Project
{
  OpProject op;
  std::string name;
};

[[nodiscard]] inline std::expected<Project, std::string> parse_project(auto& db)
{
  Project project;
  std::string op_project = ehope(db.template value<std::string>("project", "op"));
  std::string name_project = ehope(db.template value<std::string>("project", "name"));
  switch(ns_enum::from_string<OpProject>(op_project))
  {
    case OpProject::SET: project.op = OpProject::SET; project.name = name_project; break;
    case OpProject::DEL: project.op = OpProject::DEL; project.name = name_project; break;
  } // switch
  return project;
} // parse_project() }}}

// parse_install() {{{
enum class OpInstall { INSTALL, REMOTE, REMOVE };
struct Install
{
  OpInstall op;
  ns_enum::Op sub_op;
  std::vector<std::string> args;
};

[[nodiscard]] inline std::expected<Install, std::string> parse_install(auto& db)
{
  Install install;
  std::string op_install = ehope(db.template value<std::string>("install", "op"));
  std::string sub_op_install = ehope(db.template value<std::string>("install", "sub_op"));
  install.op = ns_enum::from_string<OpInstall>(op_install);
  install.sub_op = ns_enum::from_string<ns_enum::Op>(sub_op_install);
  install.args = ehope(db.template value<std::vector<std::string>>("install", "args"));
  return install;
} // parse_install() }}}

// parse_compress() {{{
struct Compress
{
};

[[nodiscard]] inline std::expected<Compress, std::string> parse_compress([[maybe_unused]] auto&)
{
  return Compress{};
} // parse_compress() }}}

// parse_search() {{{
enum class OpSearch { REMOTE, LOCAL };
struct Search
{
  OpSearch op;
  std::string query;
};

[[nodiscard]] inline std::expected<Search, std::string> parse_search(auto& db)
{
  Search search;
  std::string op_search = ehope(db.template value<std::string>("search", "op"));
  search.op = ns_enum::from_string<OpSearch>(op_search);
  search.query = ehope(db.template value<std::string>("search", "query"));
  return search;
} // parse_search() }}}

// parse_select() {{{
struct Select
{
  ns_enum::Op op;
  fs::path path_file_target;
};

[[nodiscard]] inline std::expected<Select, std::string> parse_select(auto& db)
{
  Select select;
  std::string op_select = ehope(db.template value<std::string>("select", "op"));
  select.op = ns_enum::from_string<ns_enum::Op>(op_select);
  select.path_file_target = ehope(db.template value<std::string>("select", "path_file_target"));
  return select;
} // parse_search() }}}

// parse_test() {{{
struct Test
{
};

[[nodiscard]] inline std::expected<Test, std::string> parse_test([[maybe_unused]] auto&)
{
  return Test{};
} // parse_test() }}}

// parse_desktop() {{{
enum class OpDesktop { ICON, SETUP };
struct Desktop
{
  OpDesktop op;
  std::optional<fs::path> path_file_icon;
  std::optional<std::string> name;
  std::optional<std::vector<ns_desktop::IntegrationItems>> items;
};

[[nodiscard]] inline std::expected<Desktop, std::string> parse_desktop(auto& db)
{
  Desktop desktop;
  std::string op_desktop = ehope(db.template value<std::string>("desktop", "op"));
  desktop.op = ns_enum::from_string<OpDesktop>(op_desktop);
  switch( desktop.op )
  {
    case OpDesktop::ICON:
    {
      desktop.path_file_icon = ehope(db.template value<std::string>("desktop", "path_file_icon"));
    } // case
    break;
    case OpDesktop::SETUP:
    {
      desktop.name = ehope(db.template value<std::string>("desktop", "name"));
      std::string str_items = ehope(db.template value<std::string>("desktop", "items"));
      desktop.items = ns_vector::from_string<std::vector<ns_desktop::IntegrationItems>>(str_items
        , ','
        , [](auto&& e){ return ns_enum::from_string<ns_desktop::IntegrationItems>(e); }
      );
    }
    break;
  } // switch
  return desktop;
} // parse_desktop() }}}

// parse_package() {{{
struct Package
{
  std::string name;
  std::vector<std::string> projects;
};

[[nodiscard]] inline std::expected<Package, std::string> parse_package(auto& db)
{
  Package package;
  package.name = ehope(db.template value<std::string>("package", "name"));
  package.projects = ehope(db.template value<std::vector<std::string>>("package", "projects"));
  return package;
} // parse_package() }}}

using Command = std::variant<Init,Fetch,Project,Install,Compress,Search,Select,Test,Desktop,Package>;

// parse() {{{
[[nodiscard]] inline std::expected<Command, std::string> parse(int argc, char** argv)
{
  using Ret = std::expected<Command,std::string>;
  // Check for argument count
  qreturn_if(argc < 2, std::unexpected("Incorrect number of arguments"));
  // Open json argument
  return ns_db::from_string<Ret>(argv[1], [](ns_db::Db& db) -> Ret
  {
    Command command;
    auto op = ehope(db.template value<std::string>("op"));
    switch(ns_enum::from_string<Op>(op))
    {
      case Op::FETCH: command = ehope(parse_fetch(db)); break;
      case Op::INIT: command = ehope(parse_init(db)); break;
      case Op::PROJECT: command = ehope(parse_project(db)); break;
      case Op::INSTALL: command = ehope(parse_install(db)); break;
      case Op::COMPRESS: command = ehope(parse_compress(db)); break;
      case Op::SEARCH: command = ehope(parse_search(db)); break;
      case Op::SELECT: command = ehope(parse_select(db)); break;
      case Op::TEST: command = ehope(parse_test(db)); break;
      case Op::DESKTOP: command = ehope(parse_desktop(db)); break;
      case Op::PACKAGE: command = ehope(parse_package(db)); break;
    } // switch
    return command;
  }).value();
} // parse() }}}

} // namespace ns_parser

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
