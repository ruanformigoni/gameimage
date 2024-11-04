use std::ffi::{CStr, CString};

use anyhow::anyhow as ah;

use shared::std::PathBufExt;

use crate::log;
use crate::common;
use crate::gameimage;

// struct MsgBuf {{{
#[repr(C)]
struct MsgBuf
{
  mtype: libc::c_long,
  mtext: [u8; 1024],
} // struct MsgBuf }}}

// pub struct Ipc {{{
pub struct Ipc
{
  msgid : i32,
} // struct Ipc }}}

impl Ipc
{

// pub fn new() {{{
pub fn new() -> anyhow::Result<Ipc>
{
  log!("Starting ipc");

  // Get path to backend binary
  let path_file_backend = gameimage::gameimage::binary()?;

  // Error out if file not exists
  if ! path_file_backend.exists()
  {
    return Err(ah!("File to create ipc communication does not exist"));
  } // if

  // Wait for backend to create fifo
  let cstr_path = match CString::new(path_file_backend.string().clone())
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
    match libc::msgrcv(self.msgid
      , &mut buf as *mut MsgBuf as *mut libc::c_void
      , std::mem::size_of::<[libc::c_char; 1024]>() as libc::size_t
      , 0
      , libc::MSG_NOERROR)
    {
      -1 =>
      {
        let cstr_msg_err = CString::new("Could not recover message").unwrap_or_default();
        log!("Could not recover message");
        libc::perror(cstr_msg_err.as_ptr());
        return Err(ah!("Could not recover message"));
      },
      _ => (),
    };
  };

  let c_str: &CStr = unsafe { CStr::from_ptr(buf.mtext.as_ptr() as *const libc::c_char) };
  let str_slice: &str = c_str.to_str().unwrap_or("");

  Ok(str_slice.to_owned())
} // }}}

// fn get_msgid() {{{
fn get_msgid() -> anyhow::Result<i32>
{
  let path_file_backend = match gameimage::gameimage::binary()
  {
    Ok(path_file_backend) => path_file_backend,
    Err(e) => return Err(ah!("Could not get path to backend binary for ipc: {}", e)),
  };

  // Wait for backend to create fifo
  let cstr_path = match CString::new(path_file_backend.string().clone())
  {
    Ok(cstr) => cstr,
    Err(e) => return Err(ah!("Could not create C string: {}", e)),
  }; // match

  let key = match unsafe { libc::ftok(cstr_path.as_ptr(), 65) }
  {
    -1 => return Err(ah!("Failed to get key to check message queue: {}", errno::errno())),
    key => key,
  };

  match unsafe { libc::msgget(key, 0o666) }
  {
    -1 => return Err(ah!("Message queue does not yet exist, no need to close: {}", errno::errno())),
    msgid => Ok(msgid),
  }
} // fn get_msgid() }}}

// pub fn close() {{{
pub fn close()
{
  let msgid = match Ipc::get_msgid()
  {
    Ok(msgid) => msgid,
    Err(e) => { log!("Could not retrieve msgid: {}", e); return; }
  };

  match unsafe { libc::msgctl(msgid, libc::IPC_RMID, std::ptr::null_mut()) }
  {
    -1 => log!("Could not close existing message queue"),
    _ => log!("Closed existing message queue"),
  } // match
} // close }}}

} // impl Ipc

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
