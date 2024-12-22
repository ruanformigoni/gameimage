use anyhow::anyhow as ah;

use serde_json::json;

use crate::gameimage::gameimage;

// pub fn set() {{{
#[allow(dead_code)] pub fn set(str_name: &str) -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "project".into();
  json_args["project"]["op"] = "set".into();
  json_args["project"]["name"] = str_name.into();
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Project command failed with return code: {}", ret)),
  } // match
} // fn: select }}}

// pub fn del() {{{
pub fn del(str_name: &str) -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "project".into();
  json_args["project"]["op"] = "del".into();
  json_args["project"]["name"] = str_name.into();
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Project command failed with return code: {}", ret)),
  } // match
} // fn: select }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
