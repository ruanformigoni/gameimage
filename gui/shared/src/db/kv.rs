use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;

pub type Kv = HashMap<String, String>;

// pub fn read() {{{
pub fn read(db : &PathBuf) -> anyhow::Result<Kv>
{
  Ok(serde_json::from_reader(File::open(db)?)?)
} // fn: read }}}

// pub fn write() {{{
pub fn write(db : &PathBuf, key: &String, val: &String) -> anyhow::Result<()>
{
  let mut kv : Kv = serde_json::from_reader(File::open(db.clone())?)?;

  // Append
  kv.insert(key.clone(), val.clone());

  // Write to file
  write!(File::create(&db)?, "{}", serde_json::to_string(&kv)?)?;

  Ok(())
} // fn: write }}}

// pub fn erase() {{{
pub fn erase(db : &PathBuf, key: String) -> anyhow::Result<()>
{
  // Read current
  let mut kv = read(db)?;

  // Erase key
  kv.remove(&key);

  // Write to file
  write!(File::create(db)?, "{}", serde_json::to_string(&kv)?)?;

  Ok(())
} // erase() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
