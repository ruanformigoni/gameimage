use anyhow::anyhow as ah;
use std::env;
use std::fs::File;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::db::global;
use crate::common::PathBufExt;

// struct Entry {{{
#[derive(Clone, Serialize, Deserialize)]
pub struct Entry
{
  pub path_dir_self: Option<PathBuf>,
  #[serde(rename = "project")]
  pub project: Option<String>,
  #[serde(rename = "path-file-icon")]
  pub path_file_icon: Option<PathBuf>,
  #[serde(rename = "path-file-rom")]
  pub path_file_rom: Option<PathBuf>,
  #[serde(rename = "path-file-core")]
  pub path_file_core: Option<PathBuf>,
  #[serde(rename = "path-file-bios")]
  pub path_file_bios: Option<PathBuf>,
  #[serde(rename = "platform")]
  pub platform: Option<String>,
} // Entry }}}

pub type Entries = Vec<Entry>;

// to_absolute_paths() {{{
fn to_absolute_paths(mut entry : Entry) -> anyhow::Result<Entry>
{
  entry.path_dir_self = Some(global::get()?
    .path_build
    .ok_or(ah!("Could not read build dir"))?
    .join(entry.project.clone().ok_or(ah!("Could not read project name"))?));

  let path_dir_self = entry.path_dir_self.clone().unwrap();
  let f_to_absolute = |field : Option<PathBuf>| -> Option<PathBuf>
  {
    if field.is_none()
    {
      return None;
    } // if

    Some(field.unwrap().prepend(&path_dir_self))
  };

  entry.path_file_rom = f_to_absolute(entry.path_file_rom);
  entry.path_file_core = f_to_absolute(entry.path_file_core);
  entry.path_file_bios = f_to_absolute(entry.path_file_bios);
  entry.path_file_icon = f_to_absolute(entry.path_file_icon);

  Ok(entry)
} // to_absolute_paths() }}}

// list() {{{

// List all projects
pub fn list() -> anyhow::Result<Entries>
{
  let mut entries : Entries = Vec::new();

  for path_dir_project in global::get()?
    .projects
    .ok_or(ah!("Could not read projects from global database"))?
  {
    let file = File::open(path_dir_project.join("gameimage.json").clone())?;

    // Get project entry
    let entry : Entry = serde_json::from_reader(file)?;

    // Include in vec
    entries.push(to_absolute_paths(entry)?);
  } // for

  Ok(entries)
} // fn: list }}}

// pub fn current() {{{

// Reads the current project entry
pub fn current() -> anyhow::Result<Entry>
{
  // Get global info
  let global = global::get()?;

  // Get the current project
  let path_dir_project = global.project.ok_or(ah!("Could not get project dir"))?;

  // Path to file with the project info
  let path_file_project = path_dir_project.join("gameimage.json");

  // Read file
  let file = File::open(path_file_project.clone())?;

  // Parse from json
  let project : Entry = serde_json::from_reader(file)?;

  Ok(to_absolute_paths(project)?)
} // current() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
