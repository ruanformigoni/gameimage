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
    std::map<std::string, std::string> m_map_option_value;
    argparse::ArgumentParser m_parser;
    ns_enum::Stage m_enum_stage;
  public:
    // Set parent
    Parser(std::string name)
      : m_parser(name)
    {
      m_enum_stage = ns_enum::Stage::NONE;
    }

    // Fetch value from key
    std::string operator[](std::string const& key) const
    {
      if ( ! m_map_option_value.contains(key) )
      {
        "Argument '{}' not found"_throw(key);
      } // if

      return m_map_option_value.at(key);
    } // operator[]

    // Check current stage
    ns_enum::Stage enum_stage()
    {
      return m_enum_stage;
    } // enum_stage

    // Parse args
    void parse_args(int argc, char** argv)
    {
      m_parser.parse_args(argc, argv);
    } // parse_args

    // Get remaining
    std::vector<std::string> remaining() const
    {
      return m_parser.get<std::vector<std::string>>("args");
    } // function: remaining
}; // class: Parser }}}

// class Fetch {{{
class Fetch : public Parser
{
  public:
    Fetch(std::string name)
      : Parser(name)
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::FETCH;
      // Set platform
      m_parser.add_argument("--platform")
        .action([&](std::string const& s){ m_map_option_value["--platform"]=s; })
        .required()
        .help("Specity the platform to download the flatimage");
      // Set output file
      m_parser.add_argument("--output-file")
        .action([&](std::string const& s){ m_map_option_value["--output-file"]=s; })
        .required()
        .help("Specity the output file name for the flatimage");
    } // Fetch
}; // class: Fetch }}}

// class Init {{{
class Init : public Parser
{
  public:
    Init(std::string name)
      : Parser(name)
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::INIT;
      // Set platform
      m_parser.add_argument("--platform")
        .action([&](std::string const& s){ m_map_option_value["--platform"]=s; })
        .required()
        .help("The platform to init the new directory");
      // Set directory name
      m_parser.add_argument("--dir")
        .action([&](std::string const& s){ m_map_option_value["--dir"]=s; })
        .required()
        .help("The directory to init the application");
      // Set path to image
      m_parser.add_argument("--image")
        .action([&](std::string const& s){ m_map_option_value["--image"]=s; })
        .required()
        .help("The flatimage to configure and package the program");
    } // Init
}; // class: Init }}}

// class Project {{{
class Project : public Parser
{
  public:
    Project(std::string name)
      : Parser(name)
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::PROJECT;
      // Set default project
      m_parser.add_argument("project")
        .action([&](std::string const& s){ m_map_option_value["project"]=s; })
        .required()
        .help("Sets the current application to configure");
    } // Project
}; // class: Project }}}

// class Install {{{
class Install : public Parser
{
  public:
    Install(std::string name)
      : Parser(name)
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::INSTALL;

      // Set args
      m_parser.add_argument("args")
        .nargs(argparse::nargs_pattern::at_least_one)
        .remaining()
        .required()
        .help("Install an application into the default directory");
    } // Install
}; // class: Install }}}

// class Compress {{{
class Compress : public Parser
{
  public:
    Compress(std::string name)
      : Parser(name)
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::COMPRESS;
    } // Compress
}; // class: Compress }}}

// class Target {{{
class Target : public Parser
{
  public:
    Target(std::string name)
      : Parser(name)
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::TARGET;

      // Set args
      m_parser.add_argument("args")
        .nargs(argparse::nargs_pattern::at_least_one)
        .remaining()
        .required()
        .help("Select the subcommand for target");
    } // Target
}; // class: Target }}}

} // namespace ns_parser

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
