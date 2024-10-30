use anyhow::anyhow as ah;

use crate::gameimage::gameimage;

// pub fn package() {{{
pub fn package(projects : &str) -> anyhow::Result<()>
{
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec!["package", &projects])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not include project '{}' into the image: {}", projects, ret)),
  } // match
} // fn: package }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
