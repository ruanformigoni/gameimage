///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : parser
// @created     : Saturday Jan 20, 2024 23:29:45 -03
///

#pragma once

#include <argparse/argparse.hpp>

#include "../enum.hpp"
#include "../common.hpp"

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
    Parser const& subparser(std::string const& name)
    {
      return *m_map_subparser.at(name);
    } // subparser

    // Check current stage
    std::optional<std::reference_wrapper<Parser>> used_subparser()
    {
      auto it = std::ranges::find_if(m_map_subparser, [&](auto&& e){ return m_parser->is_subcommand_used(e.second->parser()); });

      // Print help message on no command used
      if ( it == std::ranges::end(m_map_subparser) )
      {
        return std::nullopt;
      } // if

      return *(it->second);
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
      // Set custom base url
      m_parser->add_argument("--url-base")
        .action([&](std::string const& s){ m_map_option_value["--url-base"]=s; })
        .help("Set custom url for the base");
      // Set custom dwarfs url
      m_parser->add_argument("--url-dwarfs")
        .action([&](std::string const& s){ m_map_option_value["--url-dwarfs"]=s; })
        .help("Set custom url to for dwarfs");
      // Clear custom urls url
      m_parser->add_argument("--url-clear")
        .default_value(false)
        .implicit_value(true)
        .action([&](std::string const& s){ m_map_option_value["--url-clear"]=s; })
        .help("Clear custom urls");
      // Set platform
      m_parser->add_argument("--platform")
        .action([&](std::string const& s){ m_map_option_value["--platform"]=s; })
        .help("Specity the platform to download the flatimage");
      // Only download provided file
      m_parser->add_argument("--only-file")
        .action([&](std::string const& s){ m_map_option_value["--only-file"]=s; })
        .help("Only downloads the specified file");
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
    Init() : Parser("init", "Init a novel project")
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::INIT;
      // Set platform
      m_parser->add_argument("--platform")
        .action([&](std::string const& s){ m_map_option_value["--platform"]=s; })
        .required()
        .help("The platform to init the new directory");
      // Set directory name
      m_parser->add_argument("--dir")
        .action([&](std::string const& s){ m_map_option_value["--dir"]=s; })
        .required()
        .help("The directory to init the application");
      // Set path to image
      m_parser->add_argument("--image")
        .action([&](std::string const& s){ m_map_option_value["--image"]=s; })
        .required()
        .help("The flatimage to configure and package the program");
    } // Init
}; // class: Init }}}

// class Project {{{
class Project final : public Parser
{
  public:
    Project() : Parser("project", "Select the default project")
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::PROJECT;
      // Set default project
      m_parser->add_argument("project")
        .action([&](std::string const& s){ m_map_option_value["project"]=s; })
        .required()
        .help("Sets the current application to configure");
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
    Desktop() : Parser("desktop", "Configure desktop integration")
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::DESKTOP;

      // Set args
      m_parser->add_argument("icon")
        .action([&](std::string const& s){ m_map_option_value["icon"]=s; })
        .required()
        .help("Path to the file to use as icon");

      // Set args
      m_parser->add_argument("items")
        .action([&](std::string const& s){ m_map_option_value["items"]=s; })
        .required()
        .help("Items to enable in desktop integration [entry,mimetype,icon]");
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

      // Set args
      m_parser->add_argument("name")
        .action([&](std::string const& s){ m_map_option_value["name"]=s; })
        .required()
        .help("Name of the project to include in the image");
    } // Package
}; // class: Package }}}

} // namespace ns_parser

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
