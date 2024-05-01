use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;
use anyhow::anyhow as ah;

use crate::db;

pub type Kv = HashMap<PathBuf, String>;

// from_file() {{{
fn from_file(db : &str) -> anyhow::Result<PathBuf>
{
  let entry = db::global::read()?;

  let path_file_environment = entry
      .path_dir_build
      .ok_or(ah!("Could not read build dir to set env"))?
      .join(entry.project.ok_or(ah!("Could not read project dir to set env"))?)
      .join(db);

  Ok(path_file_environment)
} // from_file() }}}

// pub fn read() {{{
pub fn read(db : &str) -> anyhow::Result<Kv>
{
  let path_db = from_file(db)?;

  let kv : Kv = match File::open(path_db)
  {
    Ok(file) => serde_json::from_reader(file).unwrap_or(Kv::default()),
    Err(_) => Kv::default(),
  }; // match

  Ok(kv)
} // fn: read }}}

// pub fn write() {{{
pub fn write(db : &str, path: &PathBuf, val: &String) -> anyhow::Result<()>
{
  let path_db = from_file(db)?;

  // Read existing data
  let mut kv : Kv = match File::open(path_db.clone())
  {
    Ok(file) => serde_json::from_reader(file).unwrap_or(Kv::default()),
    Err(_) => Kv::default(),
  }; // match

  // Append
  kv.insert(path.clone(), val.clone());

  // Write to file
  write!(File::create(&path_db)?, "{}", serde_json::to_string(&kv)?)?;

  Ok(())
} // fn: write }}}

// pub fn erase() {{{
pub fn erase(db : &str, path: PathBuf) -> anyhow::Result<()>
{ 
  // Read current
  let mut kv = read(db.clone())?;

  // Erase path
  kv.remove(&path);

  // Write to file
  write!(File::create(&from_file(db)?)?, "{}", serde_json::to_string(&kv)?)?;

  Ok(())
} // erase() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
