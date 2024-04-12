use shared::std::PathBufExt;
use shared::std::VecExt;

use anyhow::anyhow as ah;

use crate::gameimage::gameimage;

// pub fn icon() {{{
pub fn icon(path : &std::path::PathBuf) -> anyhow::Result<()>
{
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec!["install", "icon", &path.string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not install icon '{}' into the image: {}", path.string(), ret)),
  } // match
} // fn: icon }}}

// pub fn install() {{{
pub fn install(str_type : &str, vec_path_files : Vec<String>) -> anyhow::Result<()>
{
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec!["install", &str_type].append_strings(vec_path_files).as_str_slice())
  {
    0 => Ok(()),
    ret => Err(ah!("Could not install files: {}", ret)),
  } // match
} // fn: install }}}

// pub fn remote() {{{
pub fn remote(str_type : &str, vec_path_files : Vec<String>) -> anyhow::Result<()>
{
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec!["install", "--remote", &str_type].append_strings(vec_path_files).as_str_slice())
  {
    0 => Ok(()),
    ret => Err(ah!("Could not install remote files: {}", ret)),
  } // match
} // fn: remote }}}

// pub fn remove() {{{
pub fn remove(str_type : &str, vec_path_files : Vec<String>) -> anyhow::Result<()>
{
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec!["install", "--remove", &str_type].append_strings(vec_path_files).as_str_slice())
  {
    0 => Ok(()),
    ret => Err(ah!("Could not remove files: {}", ret)),
  } // match
} // fn: remove }}}

// pub fn gui() {{{
pub fn gui() -> anyhow::Result<()>
{
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec!["install", "gui"])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not install files: {}", ret)),
  } // match
} // fn: install }}}

// pub fn winetricks() {{{
pub fn winetricks(vec_path_files : Vec<String>) -> anyhow::Result<()>
{
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec!["install", "winetricks"].append_strings(vec_path_files).as_str_slice())
  {
    0 => Ok(()),
    ret => Err(ah!("Could not install files: {}", ret)),
  } // match
} // fn: winetricks }}}

// pub fn wine() {{{
pub fn wine(vec_path_files : Vec<String>) -> anyhow::Result<()>
{
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec!["install", "wine"].append_strings(vec_path_files).as_str_slice())
  {
    0 => Ok(()),
    ret => Err(ah!("Could not install files: {}", ret)),
  } // match
} // fn: wine }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
