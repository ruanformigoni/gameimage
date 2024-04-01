use std::env;
use std::fs::File;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

// struct Entries {{{
#[derive(Serialize, Deserialize)]
pub struct Entries
{
  pub rom: Option<Vec<PathBuf>>,
  pub core: Option<Vec<PathBuf>>,
  pub bios: Option<Vec<PathBuf>>,
  pub keys: Option<Vec<PathBuf>>,
} // Entries }}}

// from_file() {{{
fn from_file() -> anyhow::Result<PathBuf>
{
  let mut path_file : PathBuf = env::var("GIMG_DIR")?.into();
  path_file.push("gameimage.search.json");

  Ok(path_file)
} // from_file() }}}

// read() {{{
pub fn read() -> anyhow::Result<Entries>
{
  Ok(serde_json::from_reader(File::open(from_file()?)?)?)
} // fn: read }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
