use std::ffi::CString;

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
  let msgid = match Ipc::get_msgid(libc::IPC_CREAT)
  {
    Ok(msgid) => msgid,
    Err(e) => return Err(ah!("Failed to create message queue: {}", e)),
  };

  Ok(Ipc { msgid })
} // }}}

// pub fn recv() {{{
pub fn recv(&self) -> anyhow::Result<String>
{
  let mut buf: MsgBuf = unsafe { std::mem::zeroed() };

  let ret = unsafe
  {
    libc::msgrcv(self.msgid
      , &mut buf as *mut MsgBuf as *mut libc::c_void
      , buf.mtext.len() as libc::size_t
      , 0
      , libc::MSG_NOERROR)
  };

  if ret == -1
  {
    return Err(ah!("Could not recover message: {}", std::io::Error::last_os_error()));
  } // if

  let ret = ret as usize;
  let bytes = &buf.mtext[..ret];

  let str_slice = match std::str::from_utf8(bytes) {
    Ok(s) => s,
    Err(e) => return Err(ah!("Received message is not valid UTF-8: {}", e)),
  };

  Ok(str_slice.to_owned())
} // }}}

// fn get_msgid() {{{
fn get_msgid(flags: i32) -> anyhow::Result<i32>
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
  log!("Frontend key is: {}", key);

  match unsafe { libc::msgget(key, 0o666 | flags) }
  {
    -1 => return Err(ah!("Message queue does not yet exist, no need to close: {}", errno::errno())),
    msgid => Ok(msgid),
  }
} // fn get_msgid() }}}

// pub fn close() {{{
pub fn close()
{
  let msgid = match Ipc::get_msgid(0)
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
