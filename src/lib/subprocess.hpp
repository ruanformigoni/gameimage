///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : subprocess
// @created     : Saturday Jan 20, 2024 18:55:15 -03
///

#pragma once

#include <vector>
#include <string>
#include <filesystem>

#include <boost/process.hpp>
#include <fmt/ranges.h>

#include "../common.hpp"


#include "log.hpp"

namespace ns_subprocess
{

namespace fs = std::filesystem;
namespace proc = boost::process;

// Concepts
template<typename T>
concept StringVector = std::same_as<std::decay_t<T>, std::vector<std::string>>;

// subprocess_arg() {{{
// Process args which can be either std::string or std::vector<std::string>
template<typename T>
void subprocess_arg(std::vector<std::string>& arguments, T&& arg)
{
  if constexpr (StringVector<T>)
  {
    arguments.insert(arguments.end(), std::make_move_iterator(arg.begin()), std::make_move_iterator(arg.end()));
  }
  else if constexpr ( std::is_convertible_v<T, std::string> )
  {
    arguments.emplace_back(arg);
  } // else
} // }}}

// subprocess() {{{
template<typename... Args>
void subprocess(fs::path binary, Args&&... args)
{
  // Process arguments
  std::vector<std::string> arguments;
  (subprocess_arg(arguments, std::forward<Args>(args)), ...);

  proc::ipstream pipe_stream_stdout;
  proc::ipstream pipe_stream_stderr;

  // Must include wine inside flatimage
  proc::child child(binary.string()
    , proc::args(arguments)
    , proc::std_out > pipe_stream_stdout
    , proc::std_err > pipe_stream_stderr);

  auto t1 = std::thread([&]
  {
    for(std::string line; pipe_stream_stdout && std::getline(pipe_stream_stdout, line) && !line.empty();)
    {
    ns_log::write('i', "[subprocess o] :: ", line);
    } // for
  }); // t1

  auto t2 = std::thread([&]
  {
    for(std::string line; pipe_stream_stderr && std::getline(pipe_stream_stderr, line) && !line.empty();)
    {
    ns_log::write('i', "[subprocess e] :: ", line);
    } // for
  }); // t1

  t1.join();
  t2.join();

  child.wait();

  if ( child.exit_code() != 0 )
  {
    "Command did not exit successfully: '{} {}'"_throw(binary, arguments);
  } // if
} // function: subprocess }}}

} // namespace ns_subprocess

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
