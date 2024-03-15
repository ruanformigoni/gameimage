use anyhow::anyhow as ah;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::common::PathBufExt;

// struct Entry {{{
#[derive(Clone, Serialize, Deserialize)]
pub struct Entry
{
  #[serde(rename = "path-build")]
  pub path_build: Option<PathBuf>,
  pub project: Option<PathBuf>,
  pub projects: Option<Vec<PathBuf>>,
} // Entry }}}

// to_absolute_paths() {{{
fn to_absolute_paths(mut entry : Entry) -> anyhow::Result<Entry>
{
  // Get build dir path
  let path_build = entry.path_build.clone().ok_or(ah!("Could not get path to build dir"))?;
  // It should be absolute
  if ! path_build.is_absolute() { return Err(ah!("Could dir path should be absolute")); }
  // Update default project path to absolute path
  entry.project = Some(entry.project.ok_or(ah!("Could not get path to rom file"))?.prepend(&path_build));

  Ok(entry)
} // to_absolute_paths() }}}

// get() {{{
pub fn get() -> anyhow::Result<Entry>
{
  // GIMG_DIR should contain the path to the build dir
  let path_file_db : PathBuf = env::var("GIMG_DIR")?.into();

  // Try to open the gameimage.json file in it
  let file = File::open(path_file_db.join("gameimage.json"))?;

  // Parse
  let entry : Entry = serde_json::from_reader(file)?;

  // Return with absolute paths
  Ok(to_absolute_paths(entry)?)
} // fn: get }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
