use shared::std::PathBufExt;

use serde_json::json;

use anyhow::anyhow as ah;

use crate::gameimage::gameimage;

// pub fn select() {{{
pub fn select(str_label : &str, path : &std::path::PathBuf) -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "select".into();
  json_args["select"]["op"] = str_label.into();
  json_args["select"]["path_file_target"] = path.string().into();
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not select '{}' '{}' into the image: {}", &str_label, path.string(), ret)),
  } // match
} // fn: select }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
