use anyhow::anyhow as ah;

use shared::std::PathBufExt;

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

  let ipc = match lib::ipc::Ipc::new(binary, || {})
  {
    Ok(ipc) => ipc,
    Err(e) => { log_return!("Could not create ipc instance: {}", e); },
  }; // match
  log!("Started search ipc");

  let _ = gameimage::gameimage::gameimage_async(vec!
  [
    "fetch"
    , "--platform", &platform
    , "--ipc", &str_query
  ]);
  log!("Started backend");

  let mut vec = vec![];
  while let Ok(msg) = ipc.recv()
  {
    vec.push(msg);
  } // while
  log!("Finished reading messages");

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

// set_url_layer() {{{
pub fn set_url_layer(str_url : &str) -> anyhow::Result<i32>
{
  Ok(set_url("--url-layer", str_url)?)
} // set_url_layer() }}}

// fetch() {{{
pub fn fetch(opt_path_file_dst : Option<std::path::PathBuf>) -> anyhow::Result<i32>
{
  let str_platform = gameimage::gameimage::platform()?.to_lowercase();

  let mut args = vec![
      "fetch"
    , "--platform"
    , &str_platform
  ];

  let str_path_file_dst;
  if let Some(path_file_dst) = opt_path_file_dst
  {
    str_path_file_dst = path_file_dst.string();
    args.push("--only-file");
    args.push(&str_path_file_dst);
  } // if

  match gameimage::gameimage::gameimage_sync(args)
  {
    0 => log!("Fetch on backend finished successfully"),
    rc => { log_return!("Failed to execute fetch on backend with {}", rc); },
  } // match

  Ok(0)
} // fetch() }}}

// fetchlist() {{{
pub fn fetchlist() -> anyhow::Result<i32>
{
  match gameimage::gameimage::gameimage_sync(vec!["fetch", "--fetchlist"])
  {
    0 => log!("Fetch on backend finished successfully"),
    rc => { log_return!("Failed to execute fetch on backend with {}", rc); },
  } // match

  Ok(0)
} // fetchlist() }}}

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
