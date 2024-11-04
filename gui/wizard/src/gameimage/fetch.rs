use std::sync::{mpsc,Arc,Mutex};
use crate::log;
use crate::common;
use crate::gameimage;

use anyhow::anyhow as ah;

// fetch() {{{
pub fn fetch<F>(platform: common::Platform, f: F) -> anyhow::Result<i32>
  where F: FnMut(mpsc::Receiver<String>) + Send + 'static
{
  let args = vec!["fetch", "--platform", platform.as_str() ];
  match gameimage::gameimage::gameimage_sync_ipc(args, f)
  {
    0 => { log!("Fetch on backend finished successfully"); Ok(0) },
    rc => { return Err(ah!("Failed to execute fetch on backend with {}", rc)); },
  } // match
} // fetch() }}}

// installed() {{{
pub fn installed() -> anyhow::Result<Vec<common::Platform>>
{
  let arc_platforms : Arc<Mutex<Vec<common::Platform>>> = Arc::new(Mutex::new(vec![]));

  // Start backend
  let clone_arc_platforms = arc_platforms.clone();
  gameimage::gameimage::gameimage_sync_ipc(vec!["fetch", "--ipc=installed"], move |rx|
  {
    while let Ok(msg) = rx.recv()
    {
      match clone_arc_platforms.lock()
      {
        Ok(mut guard) => match common::Platform::from_str(&msg)
        {
          Some(platform) => guard.push(platform),
          None => log!("Invalid platform: {}", msg),
        }
        Err(e) => log!("Could not lock installed vec: {}", e),
      };
    } // while
  });

  match arc_platforms.clone().lock()
  {
    Ok(platforms) => Ok(platforms.clone()),
    Err(e) => return Err(ah!("Could not lock platforms: {}", e)),
  }
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

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
