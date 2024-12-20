///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : log
// @created     : Sunday Jan 21, 2024 14:17:24 -03
///

#pragma once

#include <filesystem>
#include <easylogging++.h>
#include <fmt/core.h>

#include "../std/concepts.hpp"
#include "../std/exception.hpp"

namespace ns_log
{

namespace fs = std::filesystem;

// logger_file() {{{
inline void logger_file(fs::path const& path_file_log)
{
  // Configure
  el::Configurations default_conf;
  // To default
  default_conf.setToDefault();
  // Values are always std::string
  default_conf.set(el::Level::Global
    , el::ConfigurationType::Format
    , "%levshort :: %msg");
  // Configuration file
  default_conf.set(el::Level::Global
    , el::ConfigurationType::Filename
    , path_file_log);
  default_conf.set(el::Level::Global
    , el::ConfigurationType::ToStandardOutput
    , "false");
  // default logger uses default configurations
  el::Loggers::reconfigureLogger("default", default_conf);
} // logger_file() }}}

// logger_stdout() {{{
inline void logger_stdout()
{
  // Configure
  el::Configurations default_conf;
  // To default
  default_conf.setToDefault();
  // Values are always std::string
  default_conf.set(el::Level::Global
    , el::ConfigurationType::Format
    , "%levshort :: %msg");
  default_conf.set(el::Level::Global
    , el::ConfigurationType::ToStandardOutput
    , "true");
  // default logger uses default configurations
  el::Loggers::reconfigureLogger("term", default_conf);
} // logger_stdout() }}}

// init() {{{
inline void init(int argc
  , char** argv
  , fs::path path_file_log)
{
  // Remove log file if exists
  fs::remove(path_file_log);
  // Start easylogging
  START_EASYLOGGINGPP(argc, argv);
  // Create loggers
  logger_file(path_file_log);
  logger_stdout();
  // Try to make canonical path for log file
  try
  {
    path_file_log = fs::canonical(path_file_log);
    LOG(INFO) << fmt::format(fmt::runtime("Log file {}"), path_file_log.string());
  } // try
  catch (std::exception const& e)
  {
    LOG(ERROR) << fmt::format("Could not make canonical path for log file '{}'", path_file_log.c_str());
  } // catch: 
} // init() }}}

// write() {{{
template<ns_concept::StreamInsertable... T>
void write(char level, T&&... t)
{
  std::stringstream ss;

  if constexpr ( sizeof...(t) > 0 )
  {
    ( ss << ... << t );
  } // if

  switch (level)
  {
    case 'i':
    for(std::string line; std::getline(ss, line);)
    {
      CLOG(INFO, "term")  << line;
      CLOG(INFO, "default")  << line;
    }
    break;
    case 'e':
    for(std::string line; std::getline(ss, line);)
    {
      CLOG(INFO, "term")  << line;
      CLOG(INFO, "default")  << line;
    }; break;
    case 'd':
    for(std::string line; std::getline(ss, line);)
    {
      CLOG(INFO, "default")  << line;
    }; break;
  } // switch: level
} // write() }}}

// fn: exception {{{
inline void exception(auto&& fn)
{
  if (auto expected = ns_exception::to_expected(fn); not expected)
  {
    write('e', expected.error());
  } // if
} // }}}

// fn: ec {{{
template<typename F, typename... Args>
inline auto ec(F&& fn, Args&&... args) -> std::invoke_result_t<F, Args...>
{
  std::error_code ec;
  if constexpr ( std::same_as<void,std::invoke_result_t<F, Args...>> )
  {
    fn(std::forward<Args>(args)..., ec);
    if ( ec ) { ns_log::write('e', ec.message()); } // if
  }
  else
  {
    auto ret = fn(std::forward<Args>(args)..., ec);
    if ( ec ) { ns_log::write('e', ec.message()); } // if
    return ret;
  } // else
} // }}}

} // namespace ns_log

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
