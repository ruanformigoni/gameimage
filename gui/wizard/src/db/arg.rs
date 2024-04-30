use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;
use anyhow::anyhow as ah;

use crate::db;

pub type Args = HashMap<PathBuf, String>;

// from_file() {{{
fn from_file() -> anyhow::Result<PathBuf>
{
  let entry = db::global::read()?;

  let path_file_environment = entry
      .path_dir_build
      .ok_or(ah!("Could not read build dir to set env"))?
      .join(entry.project.ok_or(ah!("Could not read project dir to set env"))?)
      .join("gameimage.wine.args.json");

  Ok(path_file_environment)
} // from_file() }}}

// pub fn read() {{{
pub fn read() -> anyhow::Result<Args>
{
  let path_db = from_file()?;

  let args : Args = match File::open(path_db)
  {
    Ok(file) => serde_json::from_reader(file).unwrap_or(Args::default()),
    Err(_) => Args::default(),
  }; // match

  Ok(args)
} // fn: read }}}

// pub fn write() {{{
pub fn write(path: &PathBuf, val: &String) -> anyhow::Result<()>
{
  let path_db = from_file()?;

  // Read existing data
  let mut args : Args = match File::open(path_db.clone())
  {
    Ok(file) => serde_json::from_reader(file).unwrap_or(Args::default()),
    Err(_) => Args::default(),
  }; // match

  // Append
  args.insert(path.clone(), val.clone());

  // Write to file
  write!(File::create(&path_db)?, "{}", serde_json::to_string(&args)?)?;

  Ok(())
} // fn: write }}}

// pub fn erase() {{{
pub fn erase(path: PathBuf) -> anyhow::Result<()>
{ 
  // Read current
  let mut args = read()?;

  // Erase path
  args.remove(&path);

  // Write to file
  write!(File::create(&from_file()?)?, "{}", serde_json::to_string(&args)?)?;

  Ok(())
} // erase() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
