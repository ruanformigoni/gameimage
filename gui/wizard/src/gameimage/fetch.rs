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

// url_clear() {{{
pub fn url_clear() -> anyhow::Result<i32>
{
  let platform = gameimage::gameimage::platform()?;

  let rc = gameimage::gameimage::gameimage_sync(vec!
  [
    "fetch"
    , "--platform", &platform
    , "--url-clear"
  ]);

  if rc != 0 { return Err(ah!("backend exited with non-zero code: {}", rc)); } // if

  Ok(rc)
} // url_clear() }}}

// set_url() {{{
fn set_url(str_type : &str, str_url : &str) -> anyhow::Result<i32>
{
  let platform = gameimage::gameimage::platform()?;

  let rc = gameimage::gameimage::gameimage_sync(vec!
  [
    "fetch"
    , "--platform", &platform
    , &str_type, &str_url
  ]);

  if rc != 0 { return Err(ah!("backend exited with non-zero code: {}", rc)); } // if

  Ok(rc)
} // set_url() }}}

// set_url_base() {{{
pub fn set_url_base(str_url : &str) -> anyhow::Result<i32>
{
  Ok(set_url("--url-base", str_url)?)
} // set_url_base() }}}

// set_url_dwarfs() {{{
pub fn set_url_dwarfs(str_url : &str) -> anyhow::Result<i32>
{
  Ok(set_url("--url-dwarfs", str_url)?)
} // set_url_dwarfs() }}}

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
