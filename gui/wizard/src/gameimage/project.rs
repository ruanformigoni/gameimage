use anyhow::anyhow as ah;

use crate::gameimage::gameimage;

// pub fn set() {{{
#[allow(dead_code)] pub fn set(str_name: &str) -> anyhow::Result<()>
{
  match gameimage::gameimage_sync(vec!["project", "set", str_name])
  {
    0 => Ok(()),
    ret => Err(ah!("Project command failed with return code: {}", ret)),
  } // match
} // fn: select }}}

// pub fn del() {{{
pub fn del(str_name: &str) -> anyhow::Result<()>
{
  match gameimage::gameimage_sync(vec!["project", "del", str_name])
  {
    0 => Ok(()),
    ret => Err(ah!("Project command failed with return code: {}", ret)),
  } // match
} // fn: select }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
