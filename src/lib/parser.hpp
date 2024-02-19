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

// Constants {{{
inline const char* HELP_FETCH
{
  "Usage\n"
  "    :: Short: This is the fetch command, it is used to fetch an image from github\n"
  "    :: Usage: gameimage fetch --platform=[wine,retroarch,pcsx2,rpcs3,yuzu] --output-file=my-image.flatimage --json=some-file.json\n"
  "    :: Example: gameimage fetch --platform=wine --output-file=wine.flatimage\n"
};

inline const char* HELP_INIT
{
  "Usage\n"
  "    :: Short: This is the init command, it creates a new project with the provided image\n"
  "    :: Usage: gameimage init --platform=[wine,retroarch,pcsx2,rpcs3,yuzu] --dir=my-game --image=some-image.flatimage\n"
  "    :: Example: gameimage init --platform=wine --dir=aliens --image=wine.flatimage\n"
};

inline const char* HELP_PROJECT
{
  "Usage\n"
  "    :: Short: This is the project command, it switches between existing projects\n"
  "    :: Usage: gameimage project /path/to/folder\n"
  "    :: Example: gameimage project aliens\n"
};

inline const char* HELP_INSTALL
{
  "Usage\n"
  "    :: Short: This command installs/configures programs into the current project\n"
  "    :: Usage:\n"
  "    ::    :: wine: gameimage install [icon,wine,winetricks,dxvk,vkd3d]\n"
  "    ::    :: retroarch: gameimage install [bios,rom,core]\n"
  "    ::    :: pcsx2: gameimage install [bios,rom]\n"
  "    ::    :: rpcs3: gameimage install [bios,rom]\n"
  "    ::    :: yuzu: gameimage install [rom, keys]\n"
  "    :: Examples:\n"
  "    ::    :: wine:\n"
  "    ::    ::    gameimage install icon ./my-cover.png\n"
  "    ::    ::    gameimage install winetricks dotnet40\n"
  "    ::    ::    gameimage install dxvk\n"
  "    ::    ::    gameimage install vkd3d\n"
  "    ::    ::    gameimage install wine ./my-game.exe\n"
  "    ::    :: retroarch:\n"
  "    ::    ::    gameimage install icon ./my-cover.png\n"
  "    ::    ::    gameimage install bios ./my-bios\n"
  "    ::    ::    gameimage install rom ./my-rom.bin\n"
  "    ::    ::    gameimage install rom ./my-rom.cue\n"
  "    ::    ::    gameimage install core ./my-core.so\n"
  "    ::    :: pcsx2:\n"
  "    ::    ::    gameimage install icon ./my-cover.png\n"
  "    ::    ::    gameimage install bios ./my-bios\n"
  "    ::    ::    gameimage install rom ./my-rom.iso\n"
  "    ::    :: rpcs3:\n"
  "    ::    ::    gameimage install icon ./my-cover.png\n"
  "    ::    ::    gameimage install bios ./my-bios\n"
  "    ::    ::    gameimage install rom\n"
  "    ::    :: yuzu:\n"
  "    ::    ::    gameimage install icon ./my-cover.png\n"
  "    ::    ::    gameimage install bios ./my-bios\n"
  "    ::    ::    gameimage install keys ./my-keys\n"
  "    ::    ::    gameimage install rom\n"
};

inline const char* HELP_COMPRESS
{
  "Usage\n"
  "    :: Short: This command compresses (packages) the current project\n"
  "    :: Usage: gameimage compress\n"
  "    :: Example: gameimage compress\n"
};

inline const char* HELP_SEARCH
{
  "Usage\n"
  "    :: Short: This command searches installed files on the project\n"
  "    :: Usage: gameimage search [rom,core,bios,keys] --json=some-file.json\n"
  "    :: Example: gameimage search rom\n"
};

inline const char* HELP_SELECT
{
  "Usage\n"
  "    :: Short: This command selects the default between installed files\n"
  "    :: Usage: gameimage select [rom,core,bios,keys] \"./some/file.something\"\n"
  "    :: Example: gameimage select rom \"rom/my-rom.zip\"\n"
};

inline const char* HELP_PACKAGE
{
  "Usage\n"
  "    :: Short: This command packages the project into the current image\n"
  "    :: Usage: gameimage package ./path/to/game.dwarfs\n"
  "    :: Example: gameimage package my-game.dwarfs\n"
};

inline const char* HELP_TEST
{
  "Usage\n"
  "    :: Short: Tests the current project\n"
  "    :: Usage: gameimage test\n"
  "    :: Example: gameimage test\n"
};

inline const char* HELP_DESKTOP
{
  "Usage\n"
  "    :: Short: Setup desktop entry and file manager icon\n"
  "    :: Usage: gameimage desktop ./path/to/icon.png\n"
  "    :: Example: gameimage desktop ./my-image.png\n"
};

inline const char* HELP_ALL
{
  "Usage\n"
  "    :: Welcome to gameimage!\n"
  "    :: This program packages games into a portable linux binary\n"
  "    :: fetch    - Fetch an image from github\n"
  "    :: init     - Init a new game project\n"
  "    :: project  - Select the default game project\n"
  "    :: install  - Install a file/rom to the current project\n"
  "    :: test     - Test the current project\n"
  "    :: compress - Validate and compress the current project\n"
  "    :: search   - Search for installed [rom,core,bios,key]\n"
  "    :: select   - Select the default [rom,core,bios,key]\n"
  "    :: package  - Package the a compressed project into the current image\n"
  "    :: desktop  - Enable desktop integration for gameimage\n"
};
// }}}

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
    }

    // Command usage
    virtual void usage() const noexcept = 0;

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
    } // contains

    // Fetch value from key
    std::string operator[](std::string const& key) const
    {
      if ( ! m_map_option_value.contains(key) )
      {
        "Argument '{}' not found"_throw(key);
        usage();
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
        .help("Specity the platform to download the flatimage");
      // Set output file
      m_parser.add_argument("--output-file")
        .action([&](std::string const& s){ m_map_option_value["--output-file"]=s; })
        .help("Specity the output file name for the flatimage");
      // Only check-sha, do not download
      m_parser .add_argument("--sha")
        .default_value(false)
        .implicit_value(true)
        .action([&](std::string const& s){ m_map_option_value["--sha"]=s; })
        .help("Do not download, only check SHA");
      // Only write json, do not download
      m_parser.add_argument("--json")
        .action([&](std::string const& s){ m_map_option_value["--json"]=s; })
        .help("Do not download, save fetch list to json instead");
    } // Fetch
    
    void usage() const noexcept override
    {
      ns_log::write('i', HELP_FETCH);
    } // usage
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

    void usage() const noexcept override
    {
      ns_log::write('i', HELP_INIT);
    } // usage
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

    void usage() const noexcept override
    {
      ns_log::write('i', HELP_PROJECT);
    } // usage
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

    void usage() const noexcept override
    {
      ns_log::write('i', HELP_INSTALL);
    } // usage
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

    void usage() const noexcept override
    {
      ns_log::write('i', HELP_COMPRESS);
    } // usage
}; // class: Compress }}}

// class Search {{{
class Search : public Parser
{
  public:
    Search(std::string name)
      : Parser(name)
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::SEARCH;

      // Write json with search results
      m_parser.add_argument("--json")
        .nargs(1)
        .action([&](std::string const& s){ m_map_option_value["--json"]=s; })
        .help("Save search results to json");

      // Set args
      auto&& arg_query = m_parser.add_argument("query");
      arg_query.add_choice("rom");
      arg_query.add_choice("bios");
      arg_query.add_choice("core");
      arg_query.action([&](std::string const& s){ m_map_option_value["query"]=s; });
    } // Search

    void usage() const noexcept override
    {
      ns_log::write('i', HELP_SEARCH);
    } // usage
}; // class: Search }}}

// class Select {{{
class Select : public Parser
{
  public:
    Select(std::string name)
      : Parser(name)
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::SELECT;

      // Set args
      m_parser.add_argument("args")
        .nargs(argparse::nargs_pattern::at_least_one)
        .remaining()
        .required()
        .help("Select the subcommand for select");
    } // Select

    void usage() const noexcept override
    {
      ns_log::write('i', HELP_SELECT);
    } // usage
}; // class: Select }}}

// class Test {{{
class Test : public Parser
{
  public:
    Test(std::string name)
      : Parser(name)
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::TEST;
    } // Test

    void usage() const noexcept override
    {
      ns_log::write('i', HELP_TEST);
    } // usage
}; // class: Test }}}

// class Desktop {{{
class Desktop : public Parser
{
  public:
    Desktop(std::string name)
      : Parser(name)
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::DESKTOP;

      // Set args
      m_parser.add_argument("icon")
        .action([&](std::string const& s){ m_map_option_value["icon"]=s; })
        .required()
        .help("Path to the file to use as icon");
    } // Desktop

    void usage() const noexcept override
    {
      ns_log::write('i', HELP_DESKTOP);
    } // usage
}; // class: Desktop }}}

// class Package {{{
class Package : public Parser
{
  public:
    Package(std::string name)
      : Parser(name)
    {
      // Set stage
      m_enum_stage = ns_enum::Stage::PACKAGE;

      // Set args
      m_parser.add_argument("dwarfs")
        .action([&](std::string const& s){ m_map_option_value["dwarfs"]=s; })
        .required()
        .help("Path to the dwarfs filesystem to include in the project image");
    } // Package

    void usage() const noexcept override
    {
      ns_log::write('i', HELP_PACKAGE);
    } // usage
}; // class: Package }}}

} // namespace ns_parser

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
