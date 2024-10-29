use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct EntryEmulator
{
  pub layer: String,
} // Entry }}}

#[derive(Clone, Serialize, Deserialize)]
pub struct EntryWine
{
  pub layer: HashMap<String,String>,
} // Entry }}}

// struct Entry {{{
#[derive(Clone, Serialize, Deserialize)]
pub struct Entry
{
  pub version: String,
  pub rpcs3: EntryEmulator,
  pub pcsx2: EntryEmulator,
  pub retroarch: EntryEmulator,
  pub wine: EntryWine,
} // Entry }}}

// read() {{{
pub fn read() -> anyhow::Result<Entry>
{
  // GIMG_DIR should contain the path to the build dir
  let path_file_db : PathBuf = env::var("GIMG_DIR")?.into();

  // Try to open the gameimage.json file in it
  let file = File::open(path_file_db.join("fetch.json"))?;

  // Parse
  let entry : Entry = serde_json::from_reader(file)?;

  // Return with absolute paths
  Ok(entry)
} // fn: read }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
