use crate::log;
use crate::common;
use crate::gameimage;
use crate::lib;

use anyhow::anyhow as ah;

// fetch() {{{
pub fn fetch(platform: common::Platform) -> anyhow::Result<i32>
{
  let args = vec!["fetch", "--platform", platform.as_str() ];
  match gameimage::gameimage::gameimage_sync(args)
  {
    0 => { log!("Fetch on backend finished successfully"); Ok(0) },
    rc => { return Err(ah!("Failed to execute fetch on backend with {}", rc)); },
  } // match
} // fetch() }}}

// installed() {{{
pub fn installed() -> anyhow::Result<Vec<common::Platform>>
{
  let mut out : Vec<common::Platform> = vec![];
  // Start ipc
  let ipc = lib::ipc::Ipc::new(gameimage::gameimage::binary()?, || {})?;
  // Start backend
  gameimage::gameimage::gameimage_async(vec!["fetch", "--ipc=installed"])?;
  // Read messages
  while let Ok(msg) = ipc.recv()
  {
    match common::Platform::from_str(&msg)
    {
      Some(platform) => out.push(platform),
      None => log!("Invalid platform '{}", msg),
    }
  } // while
  Ok(out)
} // installed() }}}

// fetchlist() {{{
pub fn fetchlist() -> anyhow::Result<i32>
{
  match gameimage::gameimage::gameimage_sync(vec!["fetch", "--fetchlist"])
  {
    0 => { log!("Fetch on backend finished successfully"); Ok(0)},
    rc => { log!("Failed to execute fetch on backend with {}", rc); Ok(rc)},
  } // match
} // fetchlist() }}}

// validate() {{{
pub fn validate() -> anyhow::Result<i32>
{
  // let platform = gameimage::gameimage::platform()?;
  //
  // let rc = gameimage::gameimage::gameimage_sync(vec!
  // [
  //   "fetch"
  //   , "--platform", &platform
  //   , "--sha"
  // ]);
  //
  // if rc == 0 { return Ok(rc); }
  //
  // Err(ah!("Exit with error code {}", rc))
  Ok(0)
} // validate() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
