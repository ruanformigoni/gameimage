use std::sync::{mpsc,Arc,Mutex};

use serde_json::json;

use anyhow::anyhow as ah;

use crate::log;
use crate::common;
use crate::gameimage;

// fetch() {{{
pub fn fetch<F>(platform: common::Platform, f: F) -> anyhow::Result<i32>
  where F: FnMut(mpsc::Receiver<String>) + Send + 'static
{
  let mut json_args = json!({});
  json_args["op"] = "fetch".into();
  json_args["fetch"]["op"] = "fetch".into();
  json_args["fetch"]["platform"] = platform.as_str().into();
  match gameimage::gameimage::gameimage_sync_ipc(vec![&json_args.to_string()], f)
  {
    0 => { log!("Fetch on backend finished successfully"); Ok(0) },
    rc => { return Err(ah!("Failed to execute fetch on backend with {}", rc)); },
  } // match
} // fetch() }}}

// installed() {{{
pub fn installed() -> anyhow::Result<Vec<common::Platform>>
{
  let mut json_args = json!({});
  json_args["op"] = "fetch".into();
  json_args["fetch"]["op"] = "installed".into();
  let arc_platforms : Arc<Mutex<Vec<common::Platform>>> = Arc::new(Mutex::new(vec![]));
  let clone_arc_platforms = arc_platforms.clone();
  gameimage::gameimage::gameimage_sync_ipc(vec![&json_args.to_string()], move |rx|
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

// sources() {{{
pub fn sources() -> anyhow::Result<i32>
{
  let mut json_args = json!({});
  json_args["op"] = "fetch".into();
  json_args["fetch"]["op"] = "sources".into();
  match gameimage::gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => { log!("Fetch on backend finished successfully"); Ok(0)},
    rc => { log!("Failed to execute fetch on backend with {}", rc); Ok(rc)},
  } // match
} // sources() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
