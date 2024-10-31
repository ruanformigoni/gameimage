use std::path::PathBuf;
use anyhow::anyhow as ah;
use shared::std::PathBufExt;

use crate::gameimage::gameimage;

// pub fn build() {{{
pub fn build(path_dir_build : PathBuf) -> anyhow::Result<()>
{
  match gameimage::gameimage_sync(vec!["init", "--build", &path_dir_build.string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not init gameimage build root: {}", ret)),
  } // match
} // fn: build }}}

// pub fn project() {{{
pub fn project(name : String, platform : String) -> anyhow::Result<()>
{
  match gameimage::gameimage_sync(vec!["init", "--name", &name, "--platform", &platform ])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not init gameimage project: {}", ret)),
  } // match
} // fn: project }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
