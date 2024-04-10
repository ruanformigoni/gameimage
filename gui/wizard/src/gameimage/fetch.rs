use anyhow::anyhow as ah;

use crate::log;
use crate::common;
use crate::lib;
use crate::gameimage;

macro_rules! log_return
{
  ($($arg:tt)*) => { { log!($($arg)*); return Err(ah!($($arg)*)); } }
} // log_return

// query() {{{
fn query(str_query : &str) -> anyhow::Result<Vec<String>>
{
  let binary = gameimage::gameimage::binary()?;
  let platform = gameimage::gameimage::platform()?;

  let _ = gameimage::gameimage::gameimage_async(vec!
  [
    "fetch"
    , "--platform", &platform
    , "--ipc", &str_query
  ]);

  let ipc = match lib::ipc::Ipc::new(binary, || {})
  {
    Ok(ipc) => ipc,
    Err(e) => { log_return!("Could not create ipc instance: {}", e); },
  }; // match

  let mut vec = vec![];
  while let Ok(msg) = ipc.recv()
  {
    vec.push(msg);
  } // while

  Ok(vec)
} // query() }}}

// query_urls() {{{
pub fn query_urls() -> anyhow::Result<Vec<String>>
{
  Ok(query("urls")?)
} // query_urls() }}}

// query_files() {{{
pub fn query_files() -> anyhow::Result<Vec<String>>
{
  Ok(query("files")?)
} // query_files() }}}

// validate() {{{
pub fn validate() -> anyhow::Result<i32>
{
  let platform = gameimage::gameimage::platform()?;

  let rc = gameimage::gameimage::gameimage_sync(vec!
  [
    "fetch"
    , "--platform", &platform
    , "--sha"
  ]);

  if rc == 0 { return Ok(rc); }

  Err(ah!("Exit with error code {}", rc))
} // validate() }}}

// configure() {{{
pub fn configure() -> anyhow::Result<i32>
{
  let platform = gameimage::gameimage::platform()?;

  let rc = gameimage::gameimage::gameimage_sync(vec!
  [
    "fetch"
    , "--platform", &platform
  ]);

  if rc == 0 { return Ok(rc); }

  Err(ah!("Exit with error code {}", rc))
} // configure() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
