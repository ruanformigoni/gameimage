use anyhow::anyhow as ah;

use serde_json::json;

use shared::std::PathBufExt;

use crate::gameimage::gameimage;

// pub fn desktop() {{{
pub fn desktop(name: &str, items: &str) -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "desktop".into();
  json_args["desktop"]["op"] = "setup".into();
  json_args["desktop"]["name"] = name.into();
  json_args["desktop"]["items"] = items.into();
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not include {} into the image: {}", name, ret)),
  } // match
} // fn: desktop }}}

// pub fn icon() {{{
pub fn icon(path : &std::path::PathBuf) -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "desktop".into();
  json_args["desktop"]["op"] = "icon".into();
  json_args["desktop"]["path_file_icon"] = path.string().into();
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not setup desktop icon {}: {}", path.string(), ret)),
  } // match
} // fn: icon }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
