use shared::std::PathBufExt;

use anyhow::anyhow as ah;

use crate::gameimage::gameimage;

// pub fn select() {{{
pub fn select(str_label : &str, path : &std::path::PathBuf) -> anyhow::Result<()>
{
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec!["select", &str_label, &path.string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not select '{}' '{}' into the image: {}", &str_label, path.string(), ret)),
  } // match
} // fn: select }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
