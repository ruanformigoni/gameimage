///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : subprocess
// @created     : Saturday Jan 20, 2024 18:55:15 -03
///

#pragma once

#include <vector>
#include <string>
#include <filesystem>
#include <signal.h>
#include <sys/wait.h>

#include <boost/process.hpp>
#include <fmt/ranges.h>

#include "db.hpp"
#include "log.hpp"

#include "../common.hpp"

#include "../std/filesystem.hpp"
#include "../std/string.hpp"
#include "../std/env.hpp"

namespace ns_subprocess
{

namespace fs = std::filesystem;
namespace proc = boost::process;

// Concepts
template<typename T>
concept StringVector = std::same_as<std::decay_t<T>, std::vector<std::string>>;

// Forwards declarations
inline void wait(fs::path path_file);

// enum class SubProcessOptions {{{
enum class SubProcessOptions
{
  NONE     = 0,
  PRINT    = 1 << 0,
  WAITFILE = 1 << 1,
  CHECKERR = 1 << 2,
}; // enum

constexpr inline SubProcessOptions operator|(SubProcessOptions a, SubProcessOptions b)
{
  return static_cast<SubProcessOptions>(static_cast<int>(a) | static_cast<int>(b));
}

inline SubProcessOptions operator&(SubProcessOptions a, SubProcessOptions b) {
  return static_cast<SubProcessOptions>(static_cast<int>(a) & static_cast<int>(b));
}

// enum class SubProcessOptions }}}

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

// sync() {{{
template<SubProcessOptions options = SubProcessOptions::PRINT | SubProcessOptions::WAITFILE, typename... Args>
decltype(auto) sync(fs::path path_file, Args&&... args)
{
  // Wait for busy file
  if constexpr ( ns_enum::check_and(options, SubProcessOptions::WAITFILE) )
  {
    wait(path_file);
  } // if

  struct ret_t
  {
    std::stringstream ss_stdout{};
    std::stringstream ss_stderr{};
    int exit_code{};
  };

  ret_t data;
  
  // Process arguments
  std::vector<std::string> arguments;
  (subprocess_arg(arguments, std::forward<Args>(args)), ...);

  proc::ipstream pipe_stream_stdout;
  proc::ipstream pipe_stream_stderr;

  // Must include wine inside flatimage
  proc::child child(path_file.string()
    , proc::args(arguments)
    , proc::std_out > pipe_stream_stdout
    , proc::std_err > pipe_stream_stderr);

  auto t1 = std::thread([&]
  {
    for(std::string line; pipe_stream_stdout && std::getline(pipe_stream_stdout, line) && !line.empty();)
    {
      data.ss_stdout << line << '\n';
      if constexpr ( ns_enum::check_and(options, SubProcessOptions::PRINT) )
      {
        ns_log::write('i', "[o] :: ", line);
      } // if
    } // for
  }); // t1

  auto t2 = std::thread([&]
  {
    for(std::string line; pipe_stream_stderr && std::getline(pipe_stream_stderr, line) && !line.empty();)
    {
      data.ss_stderr << line;
      if constexpr ( ns_enum::check_and(options, SubProcessOptions::PRINT) )
      {
        ns_log::write('i', "[e] :: ", line);
      } // if
    } // for
  }); // t1

  child.wait();

  t1.join();
  t2.join();

  // Save return
  data.exit_code = child.exit_code();

  if ( child.exit_code() != 0 && ns_enum::check_and(options, SubProcessOptions::CHECKERR) )
  {
    ns_log::write('e', "Command did not exit successfully: '{} {}'"_fmt(path_file, ns_string::from_container(arguments)));
  } // if
  else if ( ns_enum::check_and(options, SubProcessOptions::PRINT) )
  {
    ns_log::write('i', "Finished Command: '{} {}'"_fmt(path_file, ns_string::from_container(arguments)));
  } // else

  // Wait for busy file
  if constexpr ( ns_enum::check_and(options, SubProcessOptions::WAITFILE) )
  {
    wait(path_file);
  } // if

  return data;
} // function: sync }}}

// wait() {{{
inline void wait(fs::path path_file)
{
  // Check if is regular file
  ns_fs::ns_path::file_exists<true>(path_file);

  // Find lsof in PATH
  fs::path path_lsof;
  "Could not find lsof in PATH"_throw_if([&]
  {
    path_lsof = proc::search_path("lsof").string();
    return ! ns_fs::ns_path::file_exists<false>(path_lsof)._bool;
  });

  // Get pids
  auto ret = sync<SubProcessOptions::NONE>(path_lsof, "-t", path_file);

  // Parse into pid vec
  std::vector<pid_t> pids;
  for(std::string line; std::getline(ret.ss_stdout, line);)
  {
    pids.push_back(std::atoi(line.c_str()));
    ns_log::write('i', "Wait for pid ", pids.back());
  } // for

  // Wait for pids
  auto start{std::chrono::high_resolution_clock::now()};
  
  namespace chrono = std::chrono;

  for(chrono::seconds elapsed = chrono::duration_cast<chrono::seconds>(chrono::high_resolution_clock::now() - start)
    ; ! pids.empty()
    ;)
  {
    // Update elapsed time
    elapsed = std::chrono::duration_cast<std::chrono::seconds>(std::chrono::high_resolution_clock::now() - start);

    ns_log::write('i', "Elapsed: ", elapsed.count());

    // Check if has passed limit
    if ( elapsed >= std::chrono::seconds{1800} )
    {
      std::ranges::for_each(pids, [](pid_t pid)
      {
        kill(pid, SIGKILL);
        ns_log::write('i', "Kill pid ", pid);
      });
      break;
    } // if

    // Process exists if kill != 0
    if (pid_t curr = kill(pids.back(), 0); curr != 0 )
    {
      ns_log::write('i', "Pid ", pids.back(), " finished");
      pids.pop_back();
    } // if

    // Wait before retry
    std::this_thread::sleep_for(std::chrono::seconds{1});
  } // while
} // wait() }}}


} // namespace ns_subprocess

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
