use std::env;
use std::fs::File;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json;

// struct Entries {{{
#[derive(Serialize, Deserialize)]
pub struct Entries
{
  pub rom: Option<Vec<String>>,
  pub core: Option<Vec<String>>,
  pub bios: Option<Vec<String>>,
  pub keys: Option<Vec<String>>,
} // Entries }}}

// from_file() {{{
fn from_file() -> anyhow::Result<PathBuf>
{
  let mut path_file : PathBuf = env::var("GIMG_DIR")?.into();
  path_file.push("gameimage.search.json");

  Ok(path_file)
} // from_file() }}}

// read() {{{
fn read() -> anyhow::Result<Entries>
{
  let path_file = from_file()?;

  let file = File::open(path_file)?;

  let entries : Entries = serde_json::from_reader(file)?;

  Ok(entries)
} // fn: read }}}

// get() {{{
pub fn get() -> anyhow::Result<Entries>
{
  read()
} // get() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
