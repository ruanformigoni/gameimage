///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : log
// @created     : Sunday Jan 21, 2024 14:17:24 -03
///

#pragma once

#include <filesystem>
#include <easylogging++.h>

#include "../common.hpp"

namespace ns_log
{

namespace fs = std::filesystem;

inline void init(int argc, char** argv)
{
  START_EASYLOGGINGPP(argc, argv);
  // Logfile
  const char * str_file_log = "gameimage.log";
  // Configure
  el::Configurations default_conf;
  // To default
  default_conf.setToDefault();
  // Values are always std::string
  default_conf.set(el::Level::Info
    , el::ConfigurationType::Format
    , "%datetime{%H:%m:%s} %level :: %msg");
  default_conf.set(el::Level::Error
    , el::ConfigurationType::Format
    , "%datetime{%H:%m:%s} %level :: %msg");
  default_conf.set(el::Level::Debug
    , el::ConfigurationType::Format
    , "%datetime{%H:%m:%s} %level :: %msg");
  // Configuration file
  default_conf.set(el::Level::Info
    , el::ConfigurationType::Filename
    , str_file_log);
  default_conf.set(el::Level::Error
    , el::ConfigurationType::Filename
    , str_file_log);
  default_conf.set(el::Level::Debug
    , el::ConfigurationType::Filename
    , str_file_log);
  // default logger uses default configurations
  el::Loggers::reconfigureLogger("default", default_conf);
  // Try to make canonical path for log file
  try
  {
    fs::path path_file_log = fs::canonical(str_file_log);
    LOG(INFO) << "Log file {}"_fmt(path_file_log.string());
  } // try
  catch (std::exception const& e)
  {
    LOG(ERROR) << "Could not make canonical path for log file"_fmt(str_file_log);
  } // catch: 
  // To set GLOBAL configurations you may use
  el::Loggers::reconfigureLogger("default", default_conf);
} // function: init

template<std::convertible_to<std::string>... T>
void write(char level, T&&... t)
{
  std::stringstream ss;

  ( ss << ... << t );

  switch (level)
  {
    case 'i': LOG(INFO)  << ss.str(); break;
    case 'e': LOG(ERROR) << ss.str(); break;
    case 'd': LOG(DEBUG) << ss.str(); break;
  } // switch: level
} // function: write

} // namespace ns_log

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
