use anyhow::anyhow as ah;

use shared::std::PathBufExt;

use crate::common;
use crate::log;
use crate::gameimage::gameimage;

// pub fn desktop() {{{
pub fn desktop(name: &str, items: &str) -> anyhow::Result<()>
{
  log!("Integration items: {}", items);
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec!["desktop", "setup", name, items])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not include {} into the image: {}", name, ret)),
  } // match
} // fn: desktop }}}

// pub fn icon() {{{
pub fn icon(path : &std::path::PathBuf) -> anyhow::Result<()>
{
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec!["desktop", "icon", &path.string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not setup desktop icon {}: {}", path.string(), ret)),
  } // match
} // fn: icon }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
