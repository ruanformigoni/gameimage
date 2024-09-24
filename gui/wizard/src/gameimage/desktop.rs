use anyhow::anyhow as ah;

use shared::std::PathBufExt;

use crate::common;
use crate::log;
use crate::gameimage::gameimage;

// pub fn desktop() {{{
pub fn desktop(name: &str, path : &std::path::PathBuf, items: &str) -> anyhow::Result<()>
{
  log!("Integration items: {}", items);
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec!["desktop", name, &path.string(), items])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not include {} into the image: {}", path.string(), ret)),
  } // match
} // fn: desktop }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
