use shared::std::PathBufExt;

use serde_json::json;

use anyhow::anyhow as ah;

use crate::gameimage::gameimage;

// pub fn icon() {{{
pub fn icon(path : &std::path::PathBuf) -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "install".into();
  json_args["install"]["op"] = "install".into();
  json_args["install"]["sub_op"] = "icon".into();
  json_args["install"]["args"] = vec![path.string()].into();
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not install icon '{}' into the image: {}", path.string(), ret)),
  } // match
} // fn: icon }}}

// pub fn install() {{{
pub fn install(str_type : &str, vec_path_files : Vec<String>) -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "install".into();
  json_args["install"]["op"] = "install".into();
  json_args["install"]["sub_op"] = str_type.into();
  json_args["install"]["args"] = vec_path_files.into();
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not install files: {}", ret)),
  } // match
} // fn: install }}}

// pub fn remote() {{{
pub fn remote(str_type : &str, vec_path_files : Vec<String>) -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "install".into();
  json_args["install"]["op"] = "remote".into();
  json_args["install"]["sub_op"] = str_type.into();
  json_args["install"]["args"] = vec_path_files.into();
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not install remote files: {}", ret)),
  } // match
} // fn: remote }}}

// pub fn remove() {{{
pub fn remove(str_type : &str, vec_path_files : Vec<String>) -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "install".into();
  json_args["install"]["op"] = "remove".into();
  json_args["install"]["sub_op"] = str_type.into();
  json_args["install"]["args"] = vec_path_files.into();
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not remove files: {}", ret)),
  } // match
} // fn: remove }}}

// pub fn gui() {{{
pub fn gui() -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "install".into();
  json_args["install"]["op"] = "install".into();
  json_args["install"]["sub_op"] = "gui".into();
  json_args["install"]["args"] = Vec::<String>::new().into();
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not install files: {}", ret)),
  } // match
} // fn: install }}}

// pub fn winetricks() {{{
pub fn winetricks(vec_path_files : Vec<String>) -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "install".into();
  json_args["install"]["op"] = "install".into();
  json_args["install"]["sub_op"] = "winetricks".into();
  json_args["install"]["args"] = vec_path_files.into();
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not install files: {}", ret)),
  } // match
} // fn: winetricks }}}

// pub fn wine() {{{
pub fn wine(vec_path_files : Vec<String>) -> anyhow::Result<()>
{
  let mut json_args = json!({});
  json_args["op"] = "install".into();
  json_args["install"]["op"] = "install".into();
  json_args["install"]["sub_op"] = "wine".into();
  json_args["install"]["args"] = vec_path_files.into();
  match gameimage::gameimage_sync(vec![&json_args.to_string()])
  {
    0 => Ok(()),
    ret => Err(ah!("Could not install files: {}", ret)),
  } // match
} // fn: wine }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
