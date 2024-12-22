use anyhow::anyhow as ah;

use serde_json::json;

use crate::log;
use crate::common;
use crate::gameimage;

// search() {{{
fn search(str_type : &str, use_remote : bool) -> anyhow::Result<Vec<std::path::PathBuf>>
{
  let mut json_args = json!({});
  json_args["op"] = "search".into();
  json_args["search"]["op"] = if use_remote { "remote".into() } else { "local".into() };
  json_args["search"]["query"] = str_type.into();
  // Start backend
  let (rx_msg, rx_code) = match gameimage::gameimage::gameimage_async(vec![&json_args.to_string()])
  {
    Ok((rx_msg, rx_code)) => (rx_msg, rx_code),
    Err(e) => return Err(ah!("Could not start gameimage backend: {}", e)),
  };
  log!("Started backend");
  // Retrieve messages
  let mut vec : Vec<std::path::PathBuf> = vec![];
  while let Ok(msg) = rx_msg.recv()
  {
    vec.push(msg.into());
  } // while
  log!("Finished reading messages");
  match rx_code.recv()
  {
    Ok(code) => match code
    {
      0 => log!("Backend exited successfully"),
      val => log!("Backend exited with code '{}'", val),
    },
    Err(e) => return Err(ah!("Failed to retrieve code from backend: {}", e)),
  } // match
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
