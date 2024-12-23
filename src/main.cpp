///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : main
///

#include <fmt/ranges.h>
#include <matchit.h>
#include <magic_enum/magic_enum.hpp>
#include <easylogging++.h>
#include <variant>

#include "common.hpp"
#include "enum.hpp"
#include "macro.hpp"

#include "cmd/fetch.hpp"
#include "cmd/init.hpp"
#include "cmd/project.hpp"
#include "cmd/install.hpp"
#include "cmd/compress.hpp"
#include "cmd/search.hpp"
#include "cmd/select.hpp"
#include "cmd/test.hpp"
#include "cmd/desktop.hpp"
#include "cmd/package.hpp"

#include "std/env.hpp"

#include "lib/log.hpp"
#include "lib/parser.hpp"


// Start logging
INITIALIZE_EASYLOGGINGPP

// init() {{{
void init(ns_parser::Init const& parser)
{
  switch(parser.op)
  {
    case ns_parser::OpInit::BUILD: ns_init::build(parser.path_dir_build.value()); break;
    case ns_parser::OpInit::PROJECT: ns_init::project(parser.name.value(), parser.platform.value()); break;
  };
} // init() }}}

// fetch() {{{
void fetch(ns_parser::Fetch const& parser)
{
  switch(parser.op)
  {
    case ns_parser::OpFetch::SOURCES: elog_unexpected(ns_fetch::sources()); break;
    case ns_parser::OpFetch::INSTALLED:
    {
      // Get installed platforms
      auto vec_platform = ns_fetch::installed();
      // Send platforms
      std::ranges::for_each(vec_platform | std::views::transform([](auto&& e){ return ns_enum::to_string_lower(e); })
        , [&](auto&& e) { ns_ipc::ipc().send(e); }
      );
    } // if
    break;
    case ns_parser::OpFetch::SHA: elog_unexpected(ns_fetch::sha(parser.platform.value())); break;
    case ns_parser::OpFetch::FETCH: elog_unexpected(ns_fetch::fetch(parser.platform.value())); break;
  } // Switch
} // fetch() }}}

// project() {{{
void project(ns_parser::Project const& parser)
{
  switch( parser.op )
  {
    case ns_parser::OpProject::SET: elog_unexpected(ns_project::set(parser.name)); break;
    case ns_parser::OpProject::DEL: elog_unexpected(ns_project::del(parser.name)); break;
  } // switch
} // project() }}}

// install() {{{
void install(ns_parser::Install const& parser)
{
  // Read database
  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Error to open build database '{}'"_fmt(db_build.error()));
  auto db_metadata = db_build->find(db_build->project);
  // Get parsed elements
  auto [op, sub_op, args] = parser;
  // Execute selected operation
  switch(op)
  {
    case ns_parser::OpInstall::INSTALL: ns_install::install(sub_op, args); break;
    case ns_parser::OpInstall::REMOTE: ns_install::remote(sub_op, args); break;
    case ns_parser::OpInstall::REMOVE: ns_install::remove(sub_op, db_metadata.path_dir_project, args); break;
  };
} // install() }}}

// compress() {{{
void compress()
{
  ns_compress::compress();
} // compress() }}}

// search() {{{
void search(ns_parser::Search const& parser)
{
  switch (parser.op)
  {
    case ns_parser::OpSearch::REMOTE: ns_search::search_remote(parser.query); break;
    case ns_parser::OpSearch::LOCAL: ns_search::search_local(parser.query); break;
  } // switch
} // search() }}}

// select() {{{
void select(ns_parser::Select const& parser)
{
  ns_select::select(parser.op, parser.path_file_target);
} // select() }}}

// test() {{{
void test()
{
  ns_test::test();
} // test() }}}

// desktop() {{{
void desktop(ns_parser::Desktop const& parser)
{
  switch(parser.op)
  {
    case ns_parser::OpDesktop::ICON: ns_desktop::icon(parser.path_file_icon.value()); break;
    case ns_parser::OpDesktop::SETUP: ns_desktop::desktop(parser.name.value(), parser.items.value()); break;
  }
} // desktop() }}}

// package() {{{
void package(ns_parser::Package const& parser)
{
  ns_package::package(parser.name, parser.projects);
} // package() }}}

// parse() {{{
int parse(int argc, char** argv)
{
  // Parse arguments
  auto parsed = ns_parser::parse(argc, argv);
  ereturn_if(not parsed, parsed.error(), EXIT_FAILURE);
  // Call functions
  if ( auto* cmd = std::get_if<ns_parser::Fetch>(&parsed.value()) )
  {
    fetch(*cmd);
  } // if
  else if ( auto* cmd = std::get_if<ns_parser::Init>(&parsed.value()) )
  {
    init(*cmd);
  } // else if
  else if ( auto* cmd = std::get_if<ns_parser::Project>(&parsed.value()) )
  {
    project(*cmd);
  } // else if
  else if ( auto* cmd = std::get_if<ns_parser::Install>(&parsed.value()) )
  {
    install(*cmd);
  } // else if
  else if ( std::get_if<ns_parser::Compress>(&parsed.value()) )
  {
    compress();
  } // else if
  else if ( auto* cmd = std::get_if<ns_parser::Search>(&parsed.value()) )
  {
    search(*cmd);
  } // else if
  else if ( auto* cmd = std::get_if<ns_parser::Select>(&parsed.value()) )
  {
    select(*cmd);
  } // else if
  else if ( std::get_if<ns_parser::Test>(&parsed.value()) )
  {
    test();
  } // else if
  else if ( auto* cmd = std::get_if<ns_parser::Desktop>(&parsed.value()) )
  {
    desktop(*cmd);
  } // else if
  else if ( auto* cmd = std::get_if<ns_parser::Package>(&parsed.value()) )
  {
    package(*cmd);
  } // else if
  return EXIT_SUCCESS;
} // parse() }}}

// main() {{{
int main(int argc, char** argv)
{
  // Init log
  ns_log::init(argc, argv, "gameimage.log");
  // Set layers directory if possible
  if (auto db_build = ns_db::ns_build::read(); db_build.has_value() )
  {
    ns_env::set("FIM_DIRS_LAYER", db_build->path_dir_cache, ns_env::Replace::Y);
  } // if
  // Parse commands
  try
  {
    return parse(argc, argv);
  } // try
  catch(std::exception const& e)
  {
    std::cerr << "Exception: " << e.what() << '\n';
    return EXIT_FAILURE;
  } // catch
}
// }}}

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
