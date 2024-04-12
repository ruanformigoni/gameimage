use shared::std::PathBufExt;

use anyhow::anyhow as ah;

use crate::gameimage::gameimage;

// pub fn desktop() {{{
pub fn desktop(path : &std::path::PathBuf) -> anyhow::Result<()>
{
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec!["desktop", &path.string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not include {} into the image: {}", path.string(), ret)),
  } // match
} // fn: desktop }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
