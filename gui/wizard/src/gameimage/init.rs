use anyhow::anyhow as ah;

use crate::gameimage::gameimage;

// pub fn init() {{{
pub fn init(name : String, platform : String) -> anyhow::Result<()>
{
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec!["init", "--dir", &name, "--platform", &platform ])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not init gameimage project: {}", ret)),
  } // match
} // fn: init }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
