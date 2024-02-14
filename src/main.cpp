///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : main
///

#include <iostream>
#include <fmt/ranges.h>
#include <matchit.h>
#include <magic_enum/magic_enum.hpp>
#include <easylogging++.h>

#include "common.hpp"
#include "enum.hpp"

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
#include "std/filesystem.hpp"

#include "lib/log.hpp"
#include "lib/parser.hpp"


// Start logging
INITIALIZE_EASYLOGGINGPP

namespace match = matchit;

// fetch() {{{
void fetch(ns_parser::Parser const& parser)
{
  ns_fetch::fetch(parser["--platform"], parser["--output-file"]);
} // fetch() }}}

// init() {{{
void init(ns_parser::Parser const& parser)
{
  ns_init::init(parser["--platform"], parser["--dir"], parser["--image"]);
} // init() }}}

// project() {{{
void project(ns_parser::Parser const& parser)
{
  ns_project::set(parser["project"]);
} // project() }}}

// install() {{{
void install(ns_parser::Parser const& parser)
{
  ns_install::install(parser.remaining());
} // install() }}}

// compress() {{{
void compress()
{
  ns_compress::compress();
} // compress() }}}

// search() {{{
void search(ns_parser::Parser const& parser)
{
  ns_search::search(parser.remaining());
} // search() }}}

// select() {{{
void select(ns_parser::Parser const& parser)
{
  ns_select::select(parser.remaining());
} // select() }}}

// test() {{{
void test()
{
  ns_test::test();
} // test() }}}


// desktop() {{{
void desktop(ns_parser::Parser const& parser)
{
  ns_desktop::desktop(parser["icon"]);
} // desktop() }}}

// package() {{{
void package(ns_parser::Parser const& parser)
{
  ns_package::package(parser["dwarfs"]);
} // package() }}}

// main() {{{
int main(int argc, char** argv)
{

  // Init log
  ns_log::init(argc, argv);

  if ( argc < 2 )
  {
    ns_log::write('i', ns_parser::HELP_ALL);
    ns_log::write('e', "No arguments provided for GameImage");
    return EXIT_FAILURE;
  } // if

  // Export path to self directory
  ns_env::set("GIMG_SCRIPT_DIR"
    , ns_fs::ns_path::dir_executable<true>()._ret.c_str()
    , ns_env::Replace::Y
  );

  std::unique_ptr<ns_parser::Parser> parser;

  //
  // Select stage
  //
  std::string str_stage = std::string{argv[1]};
  try
  {
    // Fetch parser for option
    match::match(str_stage)
    (
      match::pattern | "fetch"    = [&]{ parser = std::make_unique<ns_parser::Fetch>("fetch");       },
      match::pattern | "init"     = [&]{ parser = std::make_unique<ns_parser::Init>("init");         },
      match::pattern | "project"  = [&]{ parser = std::make_unique<ns_parser::Project>("project");   },
      match::pattern | "install"  = [&]{ parser = std::make_unique<ns_parser::Install>("install");   },
      match::pattern | "compress" = [&]{ parser = std::make_unique<ns_parser::Compress>("compress"); },
      match::pattern | "search"   = [&]{ parser = std::make_unique<ns_parser::Search>("search");     },
      match::pattern | "select"   = [&]{ parser = std::make_unique<ns_parser::Select>("select");     },
      match::pattern | "test"     = [&]{ parser = std::make_unique<ns_parser::Test>("test");         },
      match::pattern | "desktop"  = [&]{ parser = std::make_unique<ns_parser::Desktop>("desktop");   },
      match::pattern | "package"  = [&]{ parser = std::make_unique<ns_parser::Package>("package");   },
      match::pattern | match::_   = [&]{ "Invalid stage '{}'"_throw(str_stage);                      }
    );
  } // try
  catch(std::exception const& e)
  {
    ns_log::write('i', ns_parser::HELP_ALL);
    ns_log::write('e', e.what());
    return EXIT_FAILURE;
  } // catch

  try
  {
    // Parse args
    parser->parse_args(argc-1, argv+1);
  } // try
  catch(std::exception const& e)
  {
    parser->usage();
    ns_log::write('e', e.what());
    return EXIT_FAILURE;
  } // catch

  //
  // Execute stage
  //
  try
  {
    switch(parser->enum_stage())
    {
      case ns_enum::Stage::FETCH:
      {
        fetch(*parser);
      } // case
      break;
      case ns_enum::Stage::INIT:
      {
        init(*parser);
      } // case
      break;
      case ns_enum::Stage::PROJECT:
      {
        project(*parser);
      } // case
      break;
      case ns_enum::Stage::INSTALL:
      {
        install(*parser);
      } // case
      break;
      case ns_enum::Stage::COMPRESS:
      {
        compress();
      } // case
      break;
      case ns_enum::Stage::SEARCH:
      {
        search(*parser);
      } // case
      break;
      case ns_enum::Stage::SELECT:
      {
        select(*parser);
      } // case
      break;
      case ns_enum::Stage::TEST:
      {
        test();
      } // case
      break;
      case ns_enum::Stage::DESKTOP:
      {
        desktop(*parser);
      } // case
      break;
      case ns_enum::Stage::PACKAGE:
      {
        package(*parser);
      } // case
      break;
    } // switch
  } // try
  catch(std::exception const& e)
  {
    ns_log::write('e', e.what());
    parser->usage();
  } // catch

  return EXIT_SUCCESS;
}
// }}}

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
