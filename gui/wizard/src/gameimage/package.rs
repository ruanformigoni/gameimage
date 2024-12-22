use anyhow::anyhow as ah;

use serde_json::json;

use crate::gameimage::gameimage;

// pub fn package() {{{
pub fn package(name: &str, projects : Vec<String>) -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "package".into();
  json_args["package"]["name"] = name.into();
  json_args["package"]["projects"] = projects.clone().into();
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not include projects '{}' into the image: {}", projects.join(":"), ret)),
  } // match
} // fn: package }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
