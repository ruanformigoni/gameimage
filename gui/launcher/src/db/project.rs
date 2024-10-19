use std::path::PathBuf;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

// struct Project {{{
#[derive(Clone, Serialize, Deserialize)]
pub struct Project
{
  pub project        : String,
  pub platform       : String,
  pub path_file_icon : Option<PathBuf>,
  pub path_file_rom  : Option<PathBuf>,
  pub path_file_core : Option<PathBuf>,
  pub path_file_bios : Option<PathBuf>,
  #[serde(skip)]
  path_file_db       : PathBuf,
} // struct Project }}}

// pub fn read() {{{
pub fn read(path_file_db : &PathBuf) -> anyhow::Result<Project>
{
  let mut project : Project = serde_json::from_reader(std::fs::File::open(path_file_db)?)?;
  project.path_file_db = path_file_db.into(); 
  Ok(project)
} // fn: read }}}

// pub fn write() {{{
#[allow(dead_code)]
pub fn write(project : Project) -> anyhow::Result<()>
{
  write!(std::fs::File::create(&project.path_file_db)?, "{}", serde_json::to_string(&project)?)?;
  Ok(())
} // fn: write }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
