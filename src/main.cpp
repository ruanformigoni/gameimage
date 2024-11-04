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

#include "lib/log.hpp"
#include "lib/parser.hpp"


// Start logging
INITIALIZE_EASYLOGGINGPP

// fetch() {{{
void fetch(ns_parser::Parser const& parser)
{
  ns_enum::IpcQuery entry_ipc_query = parser.contains("--ipc")?
      ns_enum::from_string<ns_enum::IpcQuery>(*parser.optional("--ipc"))
    : ns_enum::IpcQuery::NONE;

  if ( parser.optional("--fetchlist")  )
  {
    auto error = ns_fetch::fetchlist();
    elog_if(error, *error);
    return;
  } // if

  if ( entry_ipc_query == ns_enum::IpcQuery::INSTALLED )
  {
    // Get installed platforms
    auto vec_platform = ns_fetch::installed();
    // Send platforms
    std::ranges::for_each(vec_platform | std::views::transform([](auto&& e){ return ns_enum::to_string_lower(e); })
      , [&](auto&& e) { ns_ipc::ipc().send(e); }
    );
    return;
  } // if

  ns_enum::Platform platform = ns_enum::from_string<ns_enum::Platform>(parser["--platform"]);

  if ( parser.contains("--sha") )
  {
    auto error = ns_fetch::sha(platform);
    elog_if(error, *error);
    return;
  } // if

  if ( entry_ipc_query != ns_enum::IpcQuery::NONE )
  {
    ns_fetch::ipc(platform, entry_ipc_query);
    return;
  } // if

  ns_fetch::fetch(platform);
} // fetch() }}}

// init() {{{
void init(ns_parser::Parser const& parser)
{
  if ( parser.contains("--build") )
  {
    ns_init::build(parser["--build"]);
  } // if
  else
  {
    ns_init::project(parser.optional("--name").value()
      , parser.optional("--platform").value()
    );
  } // else
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

  auto db_build = ns_db::ns_build::read();
  ethrow_if(not db_build, "Error to open build database '{}'"_fmt(db_build.error()));
  auto db_metadata = db_build->find(db_build->project);

  ns_enum::Op op = ns_enum::from_string<ns_enum::Op>(args.front());
  args.erase(args.begin());

  // Install icon
  if ( op == ns_enum::Op::ICON )
  {
    // Check if has icon path
    "No file name specified for icon"_throw_if([&]{ return args.empty(); });
    // Create icon
    ns_install::icon(db_metadata, args.front());
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
    ns_install::remove(op, db_metadata.path_dir_project, args);
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
    ns_search::search_remote(parser.optional("query"));
    return;
  } // if

  ns_search::search_local(parser.optional("query"));
} // search() }}}

// select() {{{
void select(ns_parser::Parser const& parser)
{
  auto args{parser.remaining()};

  // Check for op
  "No option was specified"_throw_if([&]{ return args.empty(); });

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
  ns_package::package(parser["name"], parser["projects"]);
} // package() }}}

// main() {{{
int main(int argc, char** argv)
{

  // Init log
  ns_log::init(argc, argv, "gameimage.log");

  // Set layers directory
  ns_env::set("FIM_DIRS_LAYER", fs::current_path() / "cache", ns_env::Replace::Y);

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
