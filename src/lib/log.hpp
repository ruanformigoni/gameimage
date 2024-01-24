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

namespace ns_log
{

namespace fs = std::filesystem;

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
}

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
}

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
} // function: init

template<ns_concept::StreamInsertable... T>
void write(char level, T&&... t)
{
  std::stringstream ss;

  ( ss << ... << t );

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
} // function: write

} // namespace ns_log

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
