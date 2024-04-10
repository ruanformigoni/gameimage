use std::ffi::{CStr, CString};

use anyhow::anyhow as ah;

use crate::log;
use crate::common;
use crate::common::PathBufExt;

// struct MsgBuf {{{
#[repr(C)]
struct MsgBuf
{
  mtype: libc::c_long,
  mtext: [libc::c_char; 10240],
} // struct MsgBuf }}}

// pub struct Ipc {{{
pub struct Ipc
{
  msgid : i32,
} // struct Ipc }}}

impl Ipc
{

// pub fn new() {{{
pub fn new(path : std::path::PathBuf) -> anyhow::Result<Ipc>
{
  let mut time_elapsed = std::time::Duration::from_secs(0);
  let time_limit = std::time::Duration::from_secs(120);

  // Wait for file to exist if it does not
  while ! path.exists() && time_elapsed < time_limit
  {
    std::thread::sleep(std::time::Duration::from_millis(100));
    time_elapsed += std::time::Duration::from_millis(100);
  } // while
  
  // If ran into timeout, then fail
  if ! path.exists()
  {
    return Err(ah!("Timeout reached to create ipc communication"));
  } // if

  // Wait for backend to create fifo
  let cstr_path = match CString::new(path.string().clone())
  {
    Ok(cstr) => cstr,
    Err(e) =>
    {
      log!("Could not create C string: {}", e);
      return Err(ah!("Could not create C string: {}", e));
    },
  }; // match

  log!("Create/Access message queue on {}", cstr_path.clone().into_string().unwrap_or(String::new()));

  let key = match unsafe { libc::ftok(cstr_path.as_ptr(), 65) }
  {
    -1 =>
    {
      let cstr_msg_err = CString::new("Failed to get key from message queue").unwrap_or_default();
      log!("Failed to get key from message queue: {}", errno::errno());
      unsafe { libc::perror(cstr_msg_err.as_ptr()); }
      return Err(ah!("Failed to get key from message queue: {}", errno::errno()));
    },
    key => key,
  };
  log!("Frontend key is: {}", key);

  let msgid = match unsafe { libc::msgget(key, 0o666 | libc::IPC_CREAT) }
  {
    -1 =>
    {
      let cstr_msg_err = CString::new("Failed to get message queue from key").unwrap_or_default();
      log!("Failed to get message queue from key: {}", errno::errno());
      unsafe { libc::perror(cstr_msg_err.as_ptr()); }
      return Err(ah!("Failed to get message queue from key: {}", errno::errno()));
    },
    msgid => msgid,
  };
  log!("Frontend msgid is: {}", msgid);

  Ok(Ipc { msgid })
} // }}}

// pub fn recv() {{{
pub fn recv(&self) -> anyhow::Result<String>
{
  let mut buf: MsgBuf = unsafe { std::mem::zeroed() };

  unsafe
  {
    match libc::msgrcv(self.msgid,
      &mut buf as *mut MsgBuf as *mut libc::c_void,
      std::mem::size_of::<[libc::c_char; 10240]>() as libc::size_t,
      0,
      libc::MSG_NOERROR,)
    {
      -1 =>
      {
        let cstr_msg_err = CString::new("Could not recover message").unwrap_or_default();
        log!("Could not recover message");
        libc::perror(cstr_msg_err.as_ptr());
        return Err(ah!("Could not recover message"));
      },
      rc => rc,
    }
  };

  let c_str: &CStr = unsafe { CStr::from_ptr(buf.mtext.as_ptr()) };
  let str_slice: &str = c_str.to_str().unwrap_or("");

  Ok(str_slice.to_owned())
} // }}}

} // impl Ipc

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
