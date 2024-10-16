use anyhow::anyhow as ah;
use std::fs::File;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::db::global;
use crate::common;
use shared::std::PathBufExt;
use crate::log;

pub enum EntryName
{
  PathFileIcon,
  PathFileRom,
  PathFileCore,
  PathFileBios,
}

// struct Entry {{{
#[derive(Clone, Serialize, Deserialize)]
pub struct Entry
{
  project        : String,
  platform       : String,
  path_file_icon : Option<PathBuf>,
  path_file_rom  : Option<PathBuf>,
  path_file_core : Option<PathBuf>,
  path_file_bios : Option<PathBuf>,
} // Entry

impl Entry
{

pub fn get_project(&self) -> anyhow::Result<String>
{
  Ok(self.project.clone())
} // project

pub fn get_platform(&self) -> anyhow::Result<String>
{
  Ok(self.platform.clone())
} // project

pub fn get_dir_self(&self) -> anyhow::Result<PathBuf>
{
  // Get the build dir
  let db = global::read()?;

  // Get project name
  let name_project = match self.get_project()
  {
    Ok(name_project) => name_project,
    Err(e) => return Err(ah!("Could not get project name: {}", e)),
  };

  // Return project dir
  Ok(db.get_project_dir(&name_project)?)
}

pub fn get_path_absolute(&self, entry: EntryName) -> anyhow::Result<PathBuf>
{
  // Get project dir == build_dir / project_name
  let project_dir_self = self.get_dir_self()?;

  let f_to_absolute = |entry : &Option<PathBuf>| -> Option<PathBuf>
  {
    match entry
    {
      Some(pathbuf) => Some(project_dir_self.join(pathbuf)),
      None => None,
    }
  };

  let ok_path_file_absolute = match entry
  {
    EntryName::PathFileIcon => f_to_absolute(&self.path_file_icon),
    EntryName::PathFileRom  => f_to_absolute(&self.path_file_rom),
    EntryName::PathFileCore => f_to_absolute(&self.path_file_core),
    EntryName::PathFileBios => f_to_absolute(&self.path_file_bios),
  }; // match

  Ok(ok_path_file_absolute.ok_or(ah!("Could not read absolute path"))?)
} // dir_absolute

pub fn get_path_relative(&self, entry: EntryName) -> anyhow::Result<PathBuf>
{
  let some_path_project_relative = match entry
  {
    EntryName::PathFileIcon => self.path_file_icon.clone(),
    EntryName::PathFileRom  => self.path_file_rom.clone(),
    EntryName::PathFileCore => self.path_file_core.clone(),
    EntryName::PathFileBios => self.path_file_bios.clone(),
  }; // match

  Ok(some_path_project_relative.ok_or(ah!("Could not read relative path"))?)
} // get_dir_relative

}
// struct Entry }}}

pub type Entries = Vec<Entry>;

// list() {{{

// List all projects
pub fn list() -> anyhow::Result<Entries>
{
  let mut entries : Entries = Vec::new();
  let db_global = global::read()?;

  for project in db_global.projects.clone()
  {
    let (_, data) = project.clone();

    // Expected json file
    let path_file_json = data.path_dir_project.join("gameimage.json");

    // Open project file
    let file = match File::open(&path_file_json)
    {
      Ok(file) => file,
      Err(e) => { log!("Could not open file '{}' with error '{}'", path_file_json.string(), e); continue; }
    };

    // Get project entry
    let entry : Entry = match serde_json::from_reader(file)
    {
      Ok(entry) => entry,
      Err(_) => continue,
    };

    // Include in vec
    entries.push(entry);
  } // for

  Ok(entries)
} // fn: list }}}

// pub fn current() {{{

// Reads the current project entry
pub fn current() -> anyhow::Result<Entry>
{
  // Get global info
  let global = global::read()?;

  // Get the current project
  let path_file_project = global.get_project_dir(&global.project)?.join("gameimage.json");

  // Read file
  let file = File::open(&path_file_project)?;

  // Read entry
  Ok(serde_json::from_reader(file)?)
} // current() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
