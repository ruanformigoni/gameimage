use anyhow::anyhow as ah;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

// struct Entry {{{
#[derive(Clone, Serialize, Deserialize)]
pub struct Entry
{
  pub project: String, // path to default project
  pub path_dir_build: PathBuf, // path to build dir
  pub path_dir_cache: PathBuf, // path to build dir
  pub path_file_image: PathBuf, // path to main flatimage
  pub path_file_output: PathBuf, // path to output file
  pub dist_wine: String, // Current wine distribution
  pub projects: HashMap<String, EntryDetails>,
} // Entry }}}

// struct EntryDetails {{{
#[derive(Clone, Serialize, Deserialize)]
pub struct EntryDetails {
  pub path_dir_project: PathBuf,
  pub path_dir_project_root: PathBuf,
  pub platform: String,
} // EntryDetails }}}

impl Entry
{

// get_project_dir() {{{
pub fn get_project_dir(&self, name_project : &str) -> anyhow::Result<PathBuf>
{
  Ok(self.projects.get(name_project).ok_or(ah!("Key '{}' not found in projects list", name_project))?.path_dir_project.clone())
} // get_project_dir() }}}

}

// get_current_project() {{{
pub fn get_current_project() -> anyhow::Result<EntryDetails>
{
  let db_global = read()?;
  let name_project = db_global.project;
  Ok(db_global.projects.get(&name_project).ok_or(ah!("Project not found in projects list"))?.clone())
} // get_current_project() }}}

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

// write() {{{
pub fn write(entry: &Entry) -> anyhow::Result<()>
{
  // GIMG_DIR should contain the path to the build dir
  let path_file_db : PathBuf = env::var("GIMG_DIR")?.into();
  // Try to open the gameimage.json file in it
  let file = File::create(path_file_db.join("gameimage.json"))?;
  // Parse
  serde_json::to_writer_pretty(file, entry)?;
  Ok(())
} // fn: write }}}

// update() {{{
pub fn update<F>(f: F) -> anyhow::Result<()>
  where F: FnOnce(Entry) -> Entry
{
  // Read
  let entry = read()?;
  // Update
  let entry = f(entry);
  // Write
  write(&entry)?;
  Ok(())
} // fn: update }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
