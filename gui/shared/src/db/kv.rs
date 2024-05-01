use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;

pub type Kv = HashMap<String, String>;

// pub fn read() {{{
pub fn read(db : &PathBuf) -> anyhow::Result<Kv>
{
  let kv : Kv = match File::open(db)
  {
    Ok(file) => serde_json::from_reader(file).unwrap_or(Kv::default()),
    Err(_) => Kv::default(),
  }; // match

  Ok(kv)
} // fn: read }}}

// pub fn write() {{{
pub fn write(db : &PathBuf, key: &String, val: &String) -> anyhow::Result<()>
{
  // Read existing data
  let mut kv : Kv = match File::open(db.clone())
  {
    Ok(file) => serde_json::from_reader(file).unwrap_or(Kv::default()),
    Err(_) => Kv::default(),
  }; // match

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
