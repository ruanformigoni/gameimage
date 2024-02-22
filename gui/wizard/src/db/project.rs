use anyhow::anyhow as ah;
use std::env;
use std::fs::File;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json;

// struct Entry {{{
#[derive(Clone, Serialize, Deserialize)]
pub struct Entry
{
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

// struct Projects {{{
#[derive(Clone, Serialize, Deserialize)]
pub struct Projects
{
  pub project: PathBuf,
} // Projects }}}

// from_dirs() {{{
fn from_dirs() -> anyhow::Result<Vec<PathBuf>>
{
  let path_dir : PathBuf = env::var("GIMG_DIR")?.into();

  let files : Vec<PathBuf> = fs::read_dir(path_dir.clone())?
    .filter_map(|e| e.ok())
    // Ends with dwarfs
    .filter(|e| e.path().extension().is_some_and(|e|{ e.to_str() == Some("dwarfs")  }))
    // Pop extension
    .filter_map(|e|
    {
      if   let Some(parent) = e.path().parent()
        && let Some(stem) = e.path().file_stem()
      {
        let mut path_buf = parent.to_path_buf();
        path_buf.push(stem);
        return Some(path_buf)
      }
      None
    })
    // Exists as directory
    .filter(|e| e.is_dir() )
    // Contains gameimage.json
    .filter_map(|e| Some(e.join("gameimage.json")) )
    // gameimage.json is file
    .filter(|e| e.is_file() )
    .collect();

  Ok(files)
} // from_dirs() }}}

// read() {{{
fn read() -> anyhow::Result<Entries>
{
  let mut entries = Entries::new();

  for path_file in from_dirs()?
  {
    let file = File::open(path_file.clone())?;
    let mut entry : Entry = serde_json::from_reader(file)?;
    // Get game directory
    let path_dir = path_file
      .parent()
      .ok_or(ah!("Failed to get parent for path_file"))?;

    // Prepend path to another path and return a copy
    let f_prepend_path = |path_pre : PathBuf, path_base : Option<PathBuf>| -> Option<PathBuf>
    {
      if let Some(path) = path_base
      {
        return Some(path_pre.join(path));
      }
      None
    };

    entry.path_file_icon = f_prepend_path(path_dir.to_path_buf(), entry.path_file_icon);
    entry.path_file_rom = f_prepend_path(path_dir.to_path_buf(), entry.path_file_rom);

    // Include in vec
    entries.push(entry);
  } // for

  Ok(entries)
} // fn: read }}}

// get() {{{
pub fn get() -> anyhow::Result<Entries>
{
  read()
} // get() }}}

// pub fn dir() {{{

// Reads the directory of the current project
pub fn dir() -> anyhow::Result<PathBuf>
{
  // Fetch the directory with the current projects
  let path_dir : PathBuf = env::var("GIMG_DIR")?.into();

  // Path to file with the projects info
  let path_file_projects = path_dir.join("gameimage.json");

  // Read file
  let file = File::open(path_file_projects.clone())?;

  // Parse from json
  let projects : Projects = serde_json::from_reader(file)?;

  Ok(path_dir.join(PathBuf::from(projects.project)))
} // dir() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
