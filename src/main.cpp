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
#include "validate.hpp"

#include "cmd/fetch.hpp"
#include "cmd/init.hpp"
#include "cmd/default.hpp"
#include "cmd/install.hpp"

#include "lib/log.hpp"
#include "lib/parser.hpp"

// #include "package.hpp"

// Start logging
INITIALIZE_EASYLOGGINGPP

namespace match = matchit;

namespace fs = std::filesystem;

// fetch() {{{
void fetch(ns_parser::Parser const& parser)
{
  ns_enum::Platform platform = ns_validate::platform(parser["--platform"]);
  fs::path path_output       = ns_validate::path_parent_exists(parser["--output-file"]);
  path_output                = ns_validate::path_file_valid(path_output);

  ns_fetch::fetch(platform, path_output);
} // function: fetch }}}

// init() {{{
void init(ns_parser::Parser const& parser)
{
  ns_enum::Platform platform = ns_validate::platform(parser["--platform"]);
  fs::path path_app          = ns_validate::path_parent_exists(parser["--dir"]);
  fs::path path_image        = ns_validate::path_file_exists(parser["--image"]);

  ns_init::init(platform, path_app, path_image);
} // function: fetch }}}

// def() {{{
void def(ns_parser::Parser const& parser)
{
  ns_default::set(parser["default"]);
} // function: fetch }}}

// install() {{{
void install(ns_parser::Parser const& parser)
{
  auto json = ns_json::from_default_file()["--platform"];
  auto str_json = ns_common::to_string(json);
  auto enum_platform = ns_enum::from_string<ns_enum::Platform>(str_json);

  switch(enum_platform)
  {

  } // switch

  ns_install::wine(parser.remaining());
} // function: fetch }}}

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
      match::pattern | "fetch"   = [&]{ parser = std::make_unique<ns_parser::Fetch>("fetch"); },
      match::pattern | "init"    = [&]{ parser = std::make_unique<ns_parser::Init>("init"); },
      match::pattern | "default" = [&]{ parser = std::make_unique<ns_parser::Default>("default"); },
      match::pattern | "install" = [&]{ parser = std::make_unique<ns_parser::Install>("install"); },
      match::pattern | match::_  = [&]
      {
        throw std::runtime_error("Invalid stage '{}'"_fmt(str_stage));
      }
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
