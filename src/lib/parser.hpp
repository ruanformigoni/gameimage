///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : parser
// @created     : Saturday Jan 20, 2024 23:29:45 -03
///

#pragma once

#include <argparse/argparse.hpp>

#include "../enum.hpp"
#include "../common.hpp"

#include "../cmd/desktop.hpp"

namespace ns_parser
{

// class Parser {{{
class Parser
{
  protected:
    std::string m_name;
    ns_enum::Stage m_enum_stage;
    std::unique_ptr<argparse::ArgumentParser> m_parser;
    std::map<std::string, std::string> m_map_option_value;
    std::unordered_map<std::string, std::unique_ptr<Parser>> m_map_subparser;
  public:
    Parser(std::string const& name, std::string const& description)
      : m_name(name)
      , m_parser(std::make_unique<argparse::ArgumentParser>(name))
    {
      m_parser->add_description(description);
    }
    virtual ~Parser(){}

    // Return help string
    std::string help() const
    {
      return ns_string::to_string(m_parser->help().rdbuf());
    } // help

    // Return parser name
    std::string const& name() const
    {
      return m_name;
    } // name

    // Check current stage
    ns_enum::Stage enum_stage() const
    {
      return m_enum_stage;
    } // enum_stage

    // Include a subparser
    void add_subparser(std::unique_ptr<Parser> other)
    {
      m_parser->add_subparser(other->parser());
      m_map_subparser.emplace(other->m_name, std::move(other));
    } // add_subparser

    // Get reference of main parser
    argparse::ArgumentParser& parser()
    {
      return *m_parser;
    } // parser

    // Get reference of subparser with provided name
    Parser const& subparser(std::string const& name) const
    {
      return *m_map_subparser.at(name);
    } // subparser

    // Check current stage
    std::optional<std::reference_wrapper<Parser>> used_subparser() const
    {
      auto it = std::ranges::find_if(m_map_subparser, [&](auto&& e){ return m_parser->is_subcommand_used(e.second->parser()); });

      // Print help message on no command used
      if ( it == std::ranges::end(m_map_subparser) )
      {
        return std::nullopt;
      } // if

      return *(it->second);
    } // used_subparser

    // Check current stage
    std::optional<std::string> used_subparser_name() const
    {
      auto subparser = used_subparser();
      ereturn_if(not subparser, "Could not find used subparser", std::nullopt);
      return subparser->get().name();
    } // used_subparser

    // Check if contains value
    bool contains(std::string const& key) const noexcept
    {
      return m_map_option_value.contains(key);
    } // contains

    // Check if contains value
    std::optional<std::string> optional(std::string const& key) const noexcept
    {
      return m_map_option_value.contains(key)
        ? std::make_optional(m_map_option_value.at(key)) : std::nullopt;
    } // optional

    // Fetch value from key
    std::string operator[](std::string const& key) const
    {
      if ( ! m_map_option_value.contains(key) )
      {
        "Argument '{}' not found"_throw(key);
      } // if

      return m_map_option_value.at(key);
    } // operator[]

    // Parse args
    void parse_args(int argc, char** argv)
    {
      m_parser->parse_args(argc, argv);
    } // parse_args

    // Get remaining
    std::vector<std::string> remaining() const
    {
      return m_parser->get<std::vector<std::string>>("args");
    } // function: remaining
}; // class: Parser }}}

// class Fetch {{{
class Fetch final : public Parser
{
  public:
    Fetch() : Parser("fetch", "Fetch an image from the remote")
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::FETCH;
      // Fetch fetchlist
      m_parser->add_argument("--fetchlist")
        .default_value(false)
        .implicit_value(true)
        .action([&](std::string const& s){ m_map_option_value["--fetchlist"]=s; })
        .help("Fetch the remote fetchlist");
      // Set platform
      m_parser->add_argument("--platform")
        .action([&](std::string const& s){ m_map_option_value["--platform"]=s; })
        .help("Specity the platform to download the flatimage");
      // Only list platforms
      m_parser->add_argument("--installed")
        .default_value(false)
        .implicit_value(true)
        .action([&](std::string const& s){ m_map_option_value["--installed"]=s; })
        .help("List currently installed platforms");
      // Only check-sha, do not download
      m_parser->add_argument("--sha")
        .default_value(false)
        .implicit_value(true)
        .action([&](std::string const& s){ m_map_option_value["--sha"]=s; })
        .help("Do not download, only check SHA");
      // Query data with ipc
      m_parser->add_argument("--ipc")
        .action([&](std::string const& s){ m_map_option_value["--ipc"]=s; })
        .help("Query information through ipc (message queues)");
    } // Fetch
}; // class: Fetch }}}

// class Init {{{
class Init final : public Parser
{
  public:
    Init() : Parser("init", "Init the build directory or a novel project")
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::INIT;
      // Set build directory
      m_parser->add_argument("--build")
        .action([&](std::string const& s){ m_map_option_value["--build"]=s; })
        .help("Create the build directory for gameimage");
      // Set platform
      m_parser->add_argument("--platform")
        .action([&](std::string const& s){ m_map_option_value["--platform"]=s; })
        .help("The platform to init the new directory");
      // Set directory name
      m_parser->add_argument("--name")
        .action([&](std::string const& s){ m_map_option_value["--name"]=s; })
        .help("The name of the project");
    } // Init
}; // class: Init }}}

// class Project {{{
class Project final : public Parser
{
  public:
    Project() : Parser("project", "Manage projects")
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::PROJECT;
      // Set default project
      m_parser->add_argument("op")
        .action([&](std::string const& s){ m_map_option_value["op"]=s; })
        .required()
        .help("Operation: set or del");
      m_parser->add_argument("project")
        .action([&](std::string const& s){ m_map_option_value["project"]=s; })
        .required()
        .help("Project name");
    } // Project
}; // class: Project }}}

// class Install {{{
class Install final : public Parser
{
  public:
    Install() : Parser("install", "Install a file to the current project")
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::INSTALL;

      // Install core from a remote
      m_parser->add_argument("--remove")
        .default_value(false)
        .implicit_value(true)
        .action([&](std::string const& s){ m_map_option_value["--remove"]=s; })
        .help("Removes an installed file/dir");

      // Install core from a remote
      m_parser->add_argument("--remote")
        .default_value(false)
        .implicit_value(true)
        .action([&](std::string const& s){ m_map_option_value["--remote"]=s; })
        .help("Install core from remote");

      // Set args
      m_parser->add_argument("args")
        .nargs(argparse::nargs_pattern::at_least_one)
        .remaining()
        .required()
        .help("Install an application into the default directory");
    } // Install
}; // class: Install }}}

// class Compress {{{
class Compress final : public Parser
{
  public:
    Compress() : Parser("compress", "Validate and compress the current project")
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::COMPRESS;
    } // Compress
}; // class: Compress }}}

// class Search {{{
class Search final : public Parser
{
  public:
    Search() : Parser("search", "Search for installed [rom,core,bios,keys]")
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::SEARCH;

      // Search from a remote
      m_parser->add_argument("--remote")
        .default_value(false)
        .implicit_value(true)
        .action([&](std::string const& s){ m_map_option_value["--remote"]=s; })
        .help("Search for core on remote");

      // Sends data with ipc instead of printing to stdout
      m_parser->add_argument("--ipc")
        .default_value(false)
        .implicit_value(true)
        .action([&](std::string const& s){ m_map_option_value["--ipc"]=s; })
        .help("Sends data with Ipc, with the current binary path use to form key");

      // Set args
      auto&& arg_query = m_parser->add_argument("query");
      arg_query.add_choice("rom");
      arg_query.add_choice("bios");
      arg_query.add_choice("core");
      arg_query.add_choice("keys");
      arg_query.action([&](std::string const& s){ m_map_option_value["query"]=s; });
    } // Search
}; // class: Search }}}

// class Select {{{
class Select final : public Parser
{
  public:
    Select() : Parser("select", "Select the default [rom,core,bios,keys]")
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::SELECT;

      // Set args
      m_parser->add_argument("args")
        .nargs(argparse::nargs_pattern::at_least_one)
        .remaining()
        .required()
        .help("Select the subcommand for select");
    } // Select
}; // class: Select }}}

// class Test {{{
class Test final : public Parser
{
  public:
    Test() : Parser("test", "Test the current project")
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::TEST;
    } // Test
}; // class: Test }}}

// class Desktop {{{
class Desktop final : public Parser
{
  public:
    Desktop()
      : Parser("desktop", "Configure desktop integration")
    {
      // Subparsers
      std::unique_ptr<Parser> parser_icon = std::make_unique<Parser>("icon", "Desktop icon integration");
      std::unique_ptr<Parser> parser_integrate = std::make_unique<Parser>("setup", "Desktop entry integration");

      // Set stage
      m_enum_stage = ns_enum::Stage::DESKTOP;

      // Icon command
      parser_icon->parser()
        .add_argument("path")
        .action([&](std::string const& s){ m_map_option_value["path"]=s; })
        .required()
        .help("Path for the icon to integrate");
      this->add_subparser(std::move(parser_icon));

      // Integrate command
      parser_integrate->parser()
        .add_argument("name")
        .action([&](std::string const& s){ m_map_option_value["name"]=s; })
        .help("Set the name of the game");
      parser_integrate->parser()
        .add_argument("items")
        .action([&](std::string const& s){ m_map_option_value["items"]=s; })
        .help("Items to enable in desktop integration [entry,mimetype,icon]");
      this->add_subparser(std::move(parser_integrate));
    } // Desktop
}; // class: Desktop }}}

// class Package {{{
class Package final : public Parser
{
  public:
    Package() : Parser("package", "Package the a compressed project into the current image")
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::PACKAGE;

      // Output file
      m_parser->add_argument("name")
        .action([&](std::string const& s){ m_map_option_value["name"]=s; })
        .required()
        .help("Name of the target file");

      // Set args
      m_parser->add_argument("projects")
        .action([&](std::string const& s){ m_map_option_value["projects"]=s; })
        .required()
        .help("Package a list of projects separated by ':'");
    } // Package
}; // class: Package }}}

} // namespace ns_parser

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
