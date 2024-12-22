use std::path::PathBuf;

use anyhow::anyhow as ah;

use serde_json::json;

use shared::std::PathBufExt;

use crate::gameimage::gameimage;

// pub fn build() {{{
pub fn build(path_dir_build : PathBuf) -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "init".into();
  json_args["init"]["op"] = "build".into();
  json_args["init"]["path_dir_build"] = path_dir_build.string().into();
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not init gameimage build root: {}", ret)),
  } // match
} // fn: build }}}

// pub fn project() {{{
pub fn project(name : String, platform : String) -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "init".into();
  json_args["init"]["op"] = "project".into();
  json_args["init"]["name"] = name.into();
  json_args["init"]["platform"] = platform.into();
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not init gameimage project: {}", ret)),
  } // match
} // fn: project }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
