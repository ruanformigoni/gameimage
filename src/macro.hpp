///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : macro
///

#pragma once

// Ec wrapper
#define lec(fun, ...) \
  ns_log::ec([]<typename... Args>(Args&&... args){ return fun(std::forward<Args>(args)...); }, __VA_ARGS__)

// Throw
#define qthrow_if(cond, msg) \
  if (cond) { throw std::runtime_error(msg); }

#define dthrow_if(cond, msg) \
  if (cond) { ns_log::write('d', msg); throw std::runtime_error(msg); }

#define ithrow_if(cond, msg) \
  if (cond) { ns_log::write('i', msg); throw std::runtime_error(msg); }

#define ethrow_if(cond, msg) \
  if (cond) { ns_log::write('e', msg); throw std::runtime_error(msg); }

// Exit
#define qexit_if(cond, ret) \
  if (cond) { exit(ret); }

#define dexit_if(cond, msg, ret) \
  if (cond) { ns_log::write('d', msg); exit(ret); }

#define iexit_if(cond, msg, ret) \
  if (cond) { ns_log::write('i', msg); exit(ret); }

#define eexit_if(cond, msg, ret) \
  if (cond) { ns_log::write('e', msg); exit(ret); }

// Return
#define qreturn_if(cond, ...) \
  if (cond) { return __VA_ARGS__; }

#define dreturn_if(cond, msg, ...) \
  if (cond) { ns_log::write('d', msg); return __VA_ARGS__; }

#define ireturn_if(cond, msg, ...) \
  if (cond) { ns_log::write('i', msg); return __VA_ARGS__; }

#define ereturn_if(cond, msg, ...) \
  if (cond) { ns_log::write('e', msg); return __VA_ARGS__; }

// Break
#define qbreak_if(cond) \
  if ( (cond) ) { break; }

#define ebreak_if(cond, msg) \
  if ( (cond) ) { ns_log::write('e', msg); break; }

#define ibreak_if(cond, msg) \
  if ( (cond) ) { ns_log::write('i', msg); break; }

#define dbreak_if(cond, msg) \
  if ( (cond) ) { ns_log::write('d', msg); break; }

// Continue
#define qcontinue_if(cond) \
  if ( (cond) ) { continue; }

#define econtinue_if(cond, msg) \
  if ( (cond) ) { ns_log::write('e', msg); continue; }

#define icontinue_if(cond, msg) \
  if ( (cond) ) { ns_log::write('i', msg); continue; }

#define dcontinue_if(cond, msg) \
  if ( (cond) ) { ns_log::write('d', msg); continue; }

// Conditional log
#define elog_if(cond, msg) \
  if ( (cond) ) { ns_log::write('e', msg); }

#define ilog_if(cond, msg) \
  if ( (cond) ) { ns_log::write('i', msg); }

#define dlog_if(cond, msg) \
  if ( (cond) ) { ns_log::write('d', msg); }