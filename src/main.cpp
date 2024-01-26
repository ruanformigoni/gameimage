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
#include "cmd/default.hpp"
#include "cmd/install.hpp"
#include "cmd/compress.hpp"

#include "lib/log.hpp"
#include "lib/parser.hpp"

// #include "package.hpp"

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

// def() {{{
void def(ns_parser::Parser const& parser)
{
  ns_default::set(parser["default"]);
} // def() }}}

// install() {{{
void install(ns_parser::Parser const& parser)
{
  ns_json::Json json = ns_json::from_default_file();
  std::string str_platform = json[json["default"]]["platform"];
  ns_install::install(ns_json::from_default_file(), parser.remaining());
} // install() }}}

// compress() {{{
void compress()
{
  ns_json::Json json = ns_json::from_default_file();

  std::string str_app = json["default"];
  std::string str_platform = json[str_app]["platform"];

  switch(ns_enum::from_string<ns_enum::Platform>(str_platform))
  {
    case ns_enum::Platform::WINE:
      ns_compress::compress();
      break;
    case ns_enum::Platform::RETROARCH:
      break;
    case ns_enum::Platform::PCSX2:
      break;
    case ns_enum::Platform::RPCS3:
      break;
    case ns_enum::Platform::YUZU:
      break;
  } // switch

} // compress() }}}

// main() {{{
int main(int argc, char** argv)
{

  // Init log
  ns_log::init(argc, argv);

  if ( argc < 2 )
  {
    ns_log::write('e', "No arguments provided for GameImage");
  } // if

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
      match::pattern | "default"  = [&]{ parser = std::make_unique<ns_parser::Default>("default");   },
      match::pattern | "install"  = [&]{ parser = std::make_unique<ns_parser::Install>("install");   },
      match::pattern | "compress" = [&]{ parser = std::make_unique<ns_parser::Compress>("compress"); },
      match::pattern | match::_   = [&]{ "Invalid stage '{}'"_throw(str_stage); }
    );
    // Parse args
    parser->parse_args(argc-1, argv+1);
  } // try
  catch(std::exception const& e)
  {
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
      case ns_enum::Stage::DEFAULT:
      {
        def(*parser);
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
    //           break;
    //         case ns_enum::Stage::INSTALL:
    //           ns_install::install(parser.m_stage->path_init(), parser.m_stage->remaining());
    //           break;
    //         case ns_enum::Stage::PACKAGE:
    //           ns_package::package(parser.m_stage->path_init(), parser.m_stage->path_target());
    //           break;
      break;
    } // switch
  } // try
  catch(std::exception const& e)
  {
    ns_log::write('e', e.what());
  } // catch

  return EXIT_SUCCESS;
}
// }}}

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
