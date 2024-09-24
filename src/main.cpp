///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : main
///

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

// fetch() {{{
void fetch(ns_parser::Parser const& parser)
{
  ns_enum::Platform platform = ns_enum::from_string<ns_enum::Platform>(parser["--platform"]);

  if ( parser.optional("--url-base") )
  {
    ns_fetch::url_set(platform, parser.optional("--url-base"), ns_fetch::UrlType::BASE);
    return;
  } // if

  if ( parser.optional("--url-dwarfs") )
  {
    ns_fetch::url_set(platform, parser.optional("--url-dwarfs"), ns_fetch::UrlType::DWARFS);
    return;
  } // if

  if ( parser.contains("--url-clear") )
  {
    ns_fetch::url_clear(platform);
    return;
  } // if

  if ( parser.contains("--sha") )
  {
    ns_fetch::sha(platform);
    return;
  } // if

  if ( parser.optional("--ipc")  )
  {
    ns_fetch::ipc(platform , parser.optional("--ipc"));
    return;
  } // if

  ns_fetch::fetch(platform, parser.optional("--only-file"));
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
  auto args{parser.remaining()};

  // Check for op
  "No option was specified"_throw_if([&]{ return args.empty(); });

  // Get project
  std::string str_project = ns_db::query(ns_db::file_default(), "project");

  // Get project path
  fs::path path_dir_project = ns_db::query(ns_db::file_default(), str_project, "path_dir_project");

  ns_enum::Op op = ns_enum::from_string<ns_enum::Op>(args.front());
  args.erase(args.begin());

  // Install icon
  if ( op == ns_enum::Op::ICON )
  {
    // Check if has icon path
    "No file name specified for icon"_throw_if([&]{ return args.empty(); });
    // Create icon
    ns_install::icon(args.front());
    return;
  } // if

  // Install item from the remote
  if ( parser.contains("--remote") )
  {
    ns_install::remote(op, args);
    return;
  } // if

  // Remove item
  if ( parser.contains("--remove") )
  {
    ns_install::remove(op, path_dir_project, args);
    return;
  }

  // Install item
  ns_install::install(op, args);

} // install() }}}

// compress() {{{
void compress()
{
  ns_compress::compress();
} // compress() }}}

// search() {{{
void search(ns_parser::Parser const& parser)
{
  if ( parser.contains("--remote") ) 
  {
    ns_search::search_remote(parser.optional("query"), parser.contains("--ipc"));
    return;
  } // if

  ns_search::search_local(parser.optional("query"), parser.contains("--ipc"));
} // search() }}}

// select() {{{
void select(ns_parser::Parser const& parser)
{
  auto args{parser.remaining()};

  // Check for op
  "No option was specified"_throw_if([&]{ return args.empty(); });

  // Get project
  std::string str_project = ns_db::query(ns_db::file_default(), "project");

  // Get project path
  fs::path path_dir_project = ns_db::query(ns_db::file_default(), str_project, "path_dir_project");

  // Parse operation
  ns_enum::Op op = ns_enum::from_string<ns_enum::Op>(args.front());
  args.erase(args.begin());

  // Check for args
  "No argument was passed for the select command"_throw_if([&]{ return args.empty(); });

  // Select
  ns_select::select(op, args.front());
} // select() }}}

// test() {{{
void test()
{
  ns_test::test();
} // test() }}}

// desktop() {{{
void desktop(ns_parser::Parser const& parser)
{
  ns_desktop::desktop(parser["name"], parser["icon"], parser["items"]);
} // desktop() }}}

// package() {{{
void package(ns_parser::Parser const& parser)
{
  ns_package::package(parser["name"]);
} // package() }}}

// main() {{{
int main(int argc, char** argv)
{

  // Init log
  ns_log::init(argc, argv, "gameimage.log");

  // Export path to self directory
  ns_env::set("GIMG_SCRIPT_DIR"
    , ns_fs::ns_path::dir_executable<true>()._ret.c_str()
    , ns_env::Replace::Y
  );

  // Parse args
  ns_parser::Parser parser("GameImage", "Create portable single-file games that work across linux distributions");

  parser.add_subparser(std::make_unique<ns_parser::Fetch>()   );
  parser.add_subparser(std::make_unique<ns_parser::Init>()    );
  parser.add_subparser(std::make_unique<ns_parser::Project>() );
  parser.add_subparser(std::make_unique<ns_parser::Install>() );
  parser.add_subparser(std::make_unique<ns_parser::Compress>());
  parser.add_subparser(std::make_unique<ns_parser::Search>()  );
  parser.add_subparser(std::make_unique<ns_parser::Select>()  );
  parser.add_subparser(std::make_unique<ns_parser::Test>()    );
  parser.add_subparser(std::make_unique<ns_parser::Desktop>() );
  parser.add_subparser(std::make_unique<ns_parser::Package>() );

  // Parse args
  try
  {
    parser.parse_args(argc, argv);
  } // try
  catch(std::exception const& e)
  {
    // Get selected subparser if any
    if ( auto subparser = parser.used_subparser() )
    {
      ns_log::write('i', subparser->get().help());
    }
    else
    {
      ns_log::write('i', parser.help());
    } // else
    ns_log::write('e', e.what());
    return EXIT_FAILURE;
  } // catch

  // Get selected subparser if any
  auto subparser = parser.used_subparser();

  if ( ! subparser )
  {
    ns_log::write('i', parser.help());
    return EXIT_FAILURE;
  } // if


  // Execute selected stage
  try
  {
    switch(subparser->get().enum_stage())
    {
      case ns_enum::Stage::FETCH    : fetch(*subparser); break;
      case ns_enum::Stage::INIT     : init(*subparser); break;
      case ns_enum::Stage::PROJECT  : project(*subparser); break;
      case ns_enum::Stage::INSTALL  : install(*subparser); break;
      case ns_enum::Stage::COMPRESS : compress(); break;
      case ns_enum::Stage::SEARCH   : search(*subparser); break;
      case ns_enum::Stage::SELECT   : select(*subparser); break;
      case ns_enum::Stage::TEST     : test(); break;
      case ns_enum::Stage::DESKTOP  : desktop(*subparser); break;
      case ns_enum::Stage::PACKAGE  : package(*subparser); break;
    } // switch
  } // try
  catch(std::exception const& e)
  {
    ns_log::write('i', subparser->get().help());
    ns_log::write('e', e.what());
    return EXIT_FAILURE;
  } // catch

  return EXIT_SUCCESS;
}
// }}}

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
