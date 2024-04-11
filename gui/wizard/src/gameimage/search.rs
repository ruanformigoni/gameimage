use anyhow::anyhow as ah;

use crate::log;
use crate::common;
use crate::lib;
use crate::gameimage;

macro_rules! log_return
{
  ($($arg:tt)*) => { { log!($($arg)*); return Err(ah!($($arg)*)); } }
} // log_return

// search() {{{
fn search(str_type : &str, use_remote : bool) -> anyhow::Result<Vec<std::path::PathBuf>>
{
  let binary = gameimage::gameimage::binary()?;

  let mut args = vec! [ "search" , "--ipc" ];
  if use_remote { args.push("--remote"); }
  args.push(str_type);
  let _ = gameimage::gameimage::gameimage_async(args);
  log!("Started backend");

  let ipc = match lib::ipc::Ipc::new(binary, || {})
  {
    Ok(ipc) => ipc,
    Err(e) => { log_return!("Could not create ipc instance: {}", e); },
  }; // match
  log!("Started search ipc");

  let mut vec : Vec<std::path::PathBuf> = vec![];
  while let Ok(msg) = ipc.recv()
  {
    vec.push(msg.into());
  } // while
  log!("Finished reading messages");

  Ok(vec)
} // search() }}}

// search_local() {{{
pub fn search_local(str_type : &str) -> anyhow::Result<Vec<std::path::PathBuf>>
{
  Ok(search(str_type, false)?)
} // search_local() }}}

// search_remote() {{{
pub fn search_remote(str_type : &str) -> anyhow::Result<Vec<std::path::PathBuf>>
{
  Ok(search(str_type, true)?)
} // search_remote() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
