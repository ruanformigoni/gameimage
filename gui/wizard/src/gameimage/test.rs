use anyhow::anyhow as ah;

use serde_json::json;

use crate::log;
use crate::common;
use crate::gameimage::gameimage;

// pub fn test() {{{
pub fn test() -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "test".into();
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => { log!("test returned successfully"); return Ok(()) },
    ret => return Err(ah!("test returned with error: {}", ret)),
  } // match
} // fn: test }}}
