use std::iter;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// struct Entry {{{
#[derive(Serialize, Deserialize)]
struct Entry
{
  pub paths: Vec<String>,
  pub urls: Vec<String>,
} // Entry }}}

// from_file() {{{
fn from_file() -> anyhow::Result<PathBuf>
{
  let mut path_file : PathBuf = env::var("GIMG_DIR")?.into();
  path_file.push("gameimage.fetch.json");

  Ok(path_file)
} // from_file() }}}

// read() {{{
pub fn read() -> anyhow::Result<HashMap<String,String>>
{
  let path_file = from_file()?;

  let file = File::open(path_file)?;

  let entry : Entry = serde_json::from_reader(file)?;

  let mut map : HashMap<String,String> = HashMap::new();

  for (path, url) in iter::zip(entry.paths, entry.urls)
  {
    map.insert(path, url);
  } // for

  Ok(map)
} // fn: read }}}

// get() {{{
pub fn get() -> anyhow::Result<HashMap<String,String>>
{
  read()
} // get() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
