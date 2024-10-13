use anyhow::anyhow as ah;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use shared::std::PathBufExt;

// struct Entry {{{
#[derive(Clone, Serialize, Deserialize)]
pub struct Entry
{
pub project: PathBuf, // path to default project
pub path_dir_build: PathBuf, // path to build dir
pub projects: Vec<PathBuf>, // list of projects by name
#[serde(flatten)]
pub dynamic_projects: Option<HashMap<String, EntryDetails>>,
} // Entry }}}

// struct EntryDetails {{{
#[derive(Clone, Serialize, Deserialize)]
pub struct EntryDetails {
  pub path_dir_project: PathBuf,
  pub path_dir_project_root: PathBuf,
  pub path_file_image: PathBuf,
  pub platform: String,
} // EntryDetails }}}

impl Entry
{

// get_project_dir() {{{
pub fn get_project_dir(&self, name_project : &str) -> anyhow::Result<PathBuf>
{
  Ok(self
    .dynamic_projects
    .clone()
    .ok_or(ah!("Project list is empty"))?
    .get(name_project)
    .ok_or(ah!("Project not found in projects list"))?
    .path_dir_project
    .clone())
} // get_project_dir() }}}

}

// get_current_project_dir() {{{
pub fn get_current_project_dir() -> anyhow::Result<PathBuf>
{
  let db_global = read()?;
  let name_project = db_global.project.string();

  Ok(db_global
    .dynamic_projects
    .clone()
    .ok_or(ah!("Project list is empty"))?
    .get(&name_project)
    .ok_or(ah!("Project not found in projects list"))?
    .path_dir_project
    .clone())
} // get_current_project_dir() }}}

// read() {{{
pub fn read() -> anyhow::Result<Entry>
{
  // GIMG_DIR should contain the path to the build dir
  let path_file_db : PathBuf = env::var("GIMG_DIR")?.into();

  // Try to open the gameimage.json file in it
  let file = File::open(path_file_db.join("gameimage.json"))?;

  // Parse
  let entry : Entry = serde_json::from_reader(file)?;

  // Return with absolute paths
  Ok(entry)
} // fn: read }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
