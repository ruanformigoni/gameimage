///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : subprocess
///

#pragma once

#include <cstring>
#include <functional>
#include <sys/wait.h>
#include <csignal>
#include <vector>
#include <string>
#include <unistd.h>
#include <sys/types.h>
#include <sys/prctl.h>
#include <ranges>

#include "log.hpp"
#include "../common.hpp"
#include "../macro.hpp"
#include "../std/vector.hpp"

namespace ns_subprocess
{

namespace
{

namespace fs = std::filesystem;

} // namespace

// search_path() {{{
inline std::optional<std::string> search_path(std::string const& s)
{
  const char* cstr_path = getenv("PATH");
  ereturn_if(cstr_path == nullptr, "PATH: Could not read PATH", std::nullopt);

  std::string str_path{cstr_path};
  auto view = str_path | std::views::split(':');
  auto it = std::find_if(view.begin(), view.end(), [&](auto&& e)
  {
    ns_log::write('i', "PATH: Check for ", fs::path(e.begin(), e.end()) / s);
    return fs::exists(fs::path(e.begin(), e.end()) / s);
  });

  if (it != view.end())
  {
    auto result = fs::path((*it).begin(), (*it).end()) / s;
    ns_log::write('i', "PATH: Found '", result);
    return result;
  } // if

  ns_log::write('i', "PATH: Could not find ", s);
  return std::nullopt;
} // search_path()}}}

// class Subprocess {{{
class Subprocess
{
  private:
    std::string m_program;
    std::vector<std::string> m_args;
    std::vector<std::string> m_env;
    std::optional<pid_t> m_opt_pid;
    std::vector<pid_t> m_vec_pids_pipe;
    std::optional<std::function<void(std::string)>> m_fstdout;
    std::optional<std::function<void(std::string)>> m_fstderr;
    bool m_with_piped_outputs;
    std::optional<pid_t> m_die_on_pid;

    [[nodiscard]] Subprocess& with_pipes_parent(int pipestdout[2], int pipestderr[2]);
    void with_pipes_child(int pipestdout[2], int pipestderr[2]);
    void die_on_pid(pid_t pid);
  public:
    template<ns_concept::AsString T>
    [[nodiscard]] Subprocess(T&& t);
    ~Subprocess();

    Subprocess(Subprocess const&) = delete;
    Subprocess& operator=(Subprocess const&) = delete;

    Subprocess(Subprocess&&) = delete;
    Subprocess& operator=(Subprocess&&) = delete;


    [[nodiscard]] Subprocess& env_clear();

    template<ns_concept::AsString K, ns_concept::AsString V>
    [[nodiscard]] Subprocess& with_var(K&& k, V&& v);

    template<ns_concept::AsString K>
    [[nodiscard]] Subprocess& rm_var(K&& k);

    [[nodiscard]] std::optional<pid_t> get_pid();

    void kill(int signal);

    template<typename Arg, typename... Args>
    requires (sizeof...(Args) > 0)
    Subprocess& with_args(Arg&& arg, Args&&... args);

    template<typename T>
    [[nodiscard]] Subprocess& with_args(T&& t);

    template<typename Arg, typename... Args>
    requires (sizeof...(Args) > 0)
    Subprocess& with_env(Arg&& arg, Args&&... args);

    template<typename T>
    [[nodiscard]] Subprocess& with_env(T&& t);

    [[nodiscard]] Subprocess& with_die_on_pid(pid_t pid);

    [[nodiscard]] Subprocess& with_piped_outputs();

    template<typename F>
    [[nodiscard]] Subprocess& with_stdout_handle(F&& f);

    template<typename F>
    [[nodiscard]] Subprocess& with_stderr_handle(F&& f);

    [[nodiscard]] Subprocess& spawn();

    [[nodiscard]] std::optional<int> wait();
}; // Subprocess }}}

// Subprocess::Subprocess {{{
template<ns_concept::AsString T>
Subprocess::Subprocess(T&& t)
  : m_program(ns_string::to_string(t))
  , m_fstdout([](auto&& e){ ns_log::write('i', "[o]: ", e); })
  , m_fstderr([](auto&& e){ ns_log::write('i', "[e]: ", e); })
  , m_with_piped_outputs(false)
{
  // argv0 is program name
  m_args.push_back(m_program);
  // Copy environment
  for(char** i = environ; *i != nullptr; ++i)
  {
    m_env.push_back(*i);
  } // for
} // Subprocess }}}

// Subprocess::~Subprocess {{{
inline Subprocess::~Subprocess()
{
  (void) this->wait();
} // Subprocess::~Subprocess }}}

// env_clear() {{{
inline Subprocess& Subprocess::env_clear()
{
  m_env.clear();
  return *this;
} // env_clear() }}}

// with_var() {{{
template<ns_concept::AsString K, ns_concept::AsString V>
Subprocess& Subprocess::with_var(K&& k, V&& v)
{
  (void) rm_var(k);
  m_env.push_back("{}={}"_fmt(k,v));
  return *this;
} // with_var() }}}

// rm_var() {{{
template<ns_concept::AsString K>
Subprocess& Subprocess::rm_var(K&& k)
{
  // Find variable
  auto it = std::ranges::find_if(m_env, [&](std::string const& e)
  {
    auto vec = ns_vector::from_string(e, '=');
    qreturn_if(vec.empty(), false);
    return vec.front() == k;
  });

  // Erase if found
  if ( it != std::ranges::end(m_env) )
  {
    ns_log::write('i', "Erased var entry: ", *it);
    m_env.erase(it);
  } // if

  return *this;
} // rm_var() }}}

// get_pid() {{{
inline std::optional<pid_t> Subprocess::get_pid()
{
  return this->m_opt_pid;
} // get_pid() }}}

// kill() {{{
inline void Subprocess::kill(int signal)
{
  if ( auto opt_pid = this->get_pid(); opt_pid )
  {
    ::kill(*opt_pid, signal);
  } // if
} // kill() }}}

// with_args() {{{
template<typename Arg, typename... Args>
requires (sizeof...(Args) > 0)
Subprocess& Subprocess::with_args(Arg&& arg, Args&&... args)
{
  return with_args(std::forward<Arg>(arg)).with_args(std::forward<Args>(args)...);
} // with_args }}}

// with_args() {{{
template<typename T>
Subprocess& Subprocess::with_args(T&& t)
{
  if constexpr ( std::same_as<std::remove_cvref_t<T>, std::string> )
  {
    this->m_args.push_back(std::forward<T>(t));
  } // if
  else if constexpr ( ns_concept::IterableConst<T> )
  {
    std::copy(t.begin(), t.end(), std::back_inserter(m_args));
  } // else if
  else if constexpr ( ns_concept::AsString<T> )
  {
    this->m_args.push_back(ns_string::to_string(std::forward<T>(t)));
  } // else if
  else
  {
    static_assert(false, "Could not determine argument type");
  } // else

  return *this;
} // with_args }}}

// with_env() {{{
template<typename Arg, typename... Args>
requires (sizeof...(Args) > 0)
Subprocess& Subprocess::with_env(Arg&& arg, Args&&... args)
{
  return with_env(std::forward<Arg>(arg)).with_env(std::forward<Args>(args)...);
} // with_env }}}

// with_env() {{{
template<typename T>
Subprocess& Subprocess::with_env(T&& t)
{
  auto f_erase_existing = [this](auto&& entries)
  {
    for (auto&& entry : entries)
    {
      auto parts = ns_vector::from_string(entry, '=');
      econtinue_if(parts.size() < 2, "Entry '{}' is not valid"_fmt(entry));
      std::string key = parts.front();
      (void) this->rm_var(key);
    } // for
  };

  if constexpr ( std::same_as<std::remove_cvref_t<T>, std::string> )
  {
    f_erase_existing(std::vector<std::string>{t});
    this->m_env.push_back(std::forward<T>(t));
  } // if
  else if constexpr ( ns_concept::IterableForward<T> )
  {
    f_erase_existing(t);
    std::ranges::copy(t, std::back_inserter(m_env));
  } // else if
  else if constexpr ( ns_concept::AsString<T> )
  {
    auto entry = ns_string::to_string(std::forward<T>(t));
    f_erase_existing(std::vector<std::string>{entry});
    this->m_env.push_back(entry);
  } // else if
  else
  {
    static_assert(false, "Could not determine argument type");
  } // else

  return *this;
} // with_env }}}

// with_die_on_pid() {{{
inline Subprocess& Subprocess::with_die_on_pid(pid_t pid)
{
  m_die_on_pid = pid;
  return *this;
} // with_die_on_pid }}}

// with_piped_outputs() {{{
inline Subprocess& Subprocess::with_piped_outputs()
{
  m_with_piped_outputs = true;
  return *this;
} // with_piped_outputs() }}}

// with_pipes_parent() {{{
inline Subprocess& Subprocess::with_pipes_parent(int pipestdout[2], int pipestderr[2])
{
  // Close write end
  ereturn_if(close(pipestdout[1]) == -1, "pipestdout[1]: {}"_fmt(strerror(errno)), *this);
  ereturn_if(close(pipestderr[1]) == -1, "pipestderr[1]: {}"_fmt(strerror(errno)), *this);

  auto f_read_pipe = [this](int id_pipe, std::string_view prefix, auto&& f)
  {
    // Fork
    pid_t ppid = getpid();
    pid_t pid = fork();
    ereturn_if(pid < 0, "Could not fork '{}'"_fmt(strerror(errno)));
    // Parent ends here
    if (pid > 0 )
    {
      m_vec_pids_pipe.push_back(pid);
      return;
    } // if
    // Die with parent
    eabort_if(prctl(PR_SET_PDEATHSIG, SIGKILL) < 0, strerror(errno));
    eabort_if(::kill(ppid, 0) < 0, "Parent died, prctl will not have effect: {}"_fmt(strerror(errno)));
    // Check if 'f' is defined
    if ( not f ) { f = [&](auto&& e)
    {
      ns_log::write('i', prefix, "(", m_program, "): ", e); };
    } // if
    // Apply f to incoming data from pipe
    char buffer[1024];
    ssize_t count;
    while ((count = read(id_pipe, buffer, sizeof(buffer))) != 0)
    {
      // Failed to read
      ebreak_if(count == -1, "broke parent read loop: {}"_fmt(strerror(errno)));
      // Split newlines and print each line with prefix
      std::ranges::for_each(std::string(buffer, count)
          | std::views::split('\n')
          | std::views::filter([&](auto&& e){ return not e.empty(); })
        , [&](auto&& e){ (*f)(std::string{e.begin(), e.end()});
      });
    } // while
    close(id_pipe);
    // Exit normally
    exit(0);
  };
  // Create pipes from fifo to ostream
  f_read_pipe(pipestdout[0], "stdout", this->m_fstdout);
  f_read_pipe(pipestderr[0], "stderr", this->m_fstderr);

  return *this;
} // with_pipes_parent() }}}

// with_pipes_child() {{{
inline void Subprocess::with_pipes_child(int pipestdout[2], int pipestderr[2])
{
  // Close read end
  ereturn_if(close(pipestdout[0]) == -1, "pipestdout[0]: {}"_fmt(strerror(errno)));
  ereturn_if(close(pipestderr[0]) == -1, "pipestderr[0]: {}"_fmt(strerror(errno)));

  // Make the opened pipe the replace stdout
  ereturn_if(dup2(pipestdout[1], STDOUT_FILENO) == -1, "dup2(pipestdout[1]): {}"_fmt(strerror(errno)));
  ereturn_if(dup2(pipestderr[1], STDERR_FILENO) == -1, "dup2(pipestderr[1]): {}"_fmt(strerror(errno)));

  // Close original write end after duplication
  ereturn_if(close(pipestdout[1]) == -1, "pipestdout[1]: {}"_fmt(strerror(errno)));
  ereturn_if(close(pipestderr[1]) == -1, "pipestderr[1]: {}"_fmt(strerror(errno)));
} // with_pipes_child() }}}

// die_on_pid() {{{
inline void Subprocess::die_on_pid(pid_t pid)
{
  // Set death signal when pid dies
  ereturn_if(prctl(PR_SET_PDEATHSIG, SIGKILL) < 0, strerror(errno));
  // Abort if pid is not running
  if (::kill(pid, 0) < 0)
  {
    ns_log::write('e', "Parent died, prctl will not have effect: ", strerror(errno));
    std::abort();
  } // if
  // Log pid and current pid
  ns_log::write('i', getpid(), "dies with ", pid);
} // die_on_pid() }}}

// with_stdout_handle() {{{
template<typename F>
Subprocess& Subprocess::with_stdout_handle(F&& f)
{
  this->m_fstdout = f;
  return *this;
} // with_stdout_handle() }}}

// with_stderr_handle() {{{
template<typename F>
Subprocess& Subprocess::with_stderr_handle(F&& f)
{
  this->m_fstderr = f;
  return *this;
} // with_stderr_handle }}}

// wait() {{{
inline std::optional<int> Subprocess::wait()
{
  // Check if pid is valid
  ereturn_if( not m_opt_pid or *m_opt_pid <= 0, "Invalid pid to wait for", std::nullopt);

  // Wait for current process
  int status;
  waitpid(*m_opt_pid, &status, 0);

  // Send SIGTERM for reader forks
  std::ranges::for_each(m_vec_pids_pipe, [](pid_t pid){ ::kill(pid, SIGTERM); });

  // Wait for forks
  std::ranges::for_each(m_vec_pids_pipe, [](pid_t pid){ waitpid(pid, nullptr, 0); });

  return (WIFEXITED(status))? std::make_optional(WEXITSTATUS(status)) : std::nullopt;
} // wait() }}}

// spawn() {{{
inline Subprocess& Subprocess::spawn()
{
  // Log
  ns_log::write('i', "Spawn command: ", ns_string::from_container(m_args));

  int pipestdout[2];
  int pipestderr[2];

  // Create pipe
  ereturn_if(pipe(pipestdout), strerror(errno), *this);
  ereturn_if(pipe(pipestderr), strerror(errno), *this);

  // Ignore on empty vec_argv
  if ( m_args.empty() )
  {
    ns_log::write('i', "No arguments to spawn subprocess");
    return *this;
  } // if

  // Create child
  m_opt_pid = fork();

  // Failed to fork
  ereturn_if(*m_opt_pid == -1, "Failed to fork", *this);

  // Setup pipe on child and parent
  // On parent, return exit code of child
  if ( *m_opt_pid > 0 )
  {
    if ( m_with_piped_outputs )
    {
      return with_pipes_parent(pipestdout, pipestderr);
    } // if
    else
    {
      return *this;
    } // else
  } // if

  // On child, just setup the pipe
  if ( m_with_piped_outputs && *m_opt_pid == 0)
  {
    // this is non-blocking, setup pipes and perform execve afterwards
    with_pipes_child(pipestdout, pipestderr);
  } // else

  // Check if should die with pid
  if ( m_die_on_pid )
  {
    die_on_pid(*m_die_on_pid);
  } // if

  // Create arguments for execve
  auto argv_custom = std::make_unique<const char*[]>(m_args.size() + 1);

  // Copy arguments
  std::ranges::transform(m_args, argv_custom.get(), [](auto&& e) { return e.c_str(); });

  // Set last entry to nullptr
  argv_custom[m_args.size()] = nullptr;

  // Set environment variables
  std::ranges::for_each(m_env, [](auto&& e)
  {
    auto entry = ns_vector::from_string(e, '=');
    ereturn_if(entry.size() < 2, "Invalid environment variable '{}'"_fmt(e));
    auto key = entry.front();
    entry.erase(entry.begin());
    setenv(key.c_str(), ns_string::from_container(entry).c_str(), 1);
  });

  // Create environment for execve
  auto envp_custom = std::make_unique<const char*[]>(m_env.size() + 1);

  // Copy variables
  std::transform(m_env.begin(), m_env.end(), envp_custom.get(), [](auto&& e) { return e.c_str(); });

  // Set last entry to nullptr
  envp_custom[m_env.size()] = nullptr;

  // Perform execve
  execve(m_program.c_str(), (char**) argv_custom.get(), (char**) envp_custom.get());

  // Log error
  ns_log::write('i', "execve() failed: ", strerror(errno));

  // Child should stop here
  std::abort();
} // spawn() }}}

// wait_busy_file() {{{
inline std::optional<std::string> wait_busy_file(fs::path const& path_file_target)
{
  auto path_file_lsof = search_path("lsof");
  qreturn_if(not path_file_lsof, std::optional("Could not locate lsof binary"));

  while(true)
  {
    auto ret = Subprocess(*path_file_lsof)
      .with_piped_outputs()
      .with_args(path_file_target)
      .spawn()
      .wait();
    ebreak_if(not ret, "Failed to query status for busy file");
    dbreak_if(*ret != 0, "break, file is not busy");
  } // while

  return std::nullopt;
} // wait_busy_file()}}}

} // namespace ns_subprocess

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
