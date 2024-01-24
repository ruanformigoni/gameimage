use std::io::prelude::*;
use regex::Regex;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use anyhow::anyhow as ah;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Var
{
  pub key: String,
  pub val: String,
} // Var

#[derive(Serialize, Deserialize)]
pub struct Vars 
{
  pub env: Vec<Var>,
} // Vars

// from_file() {{{
fn from_file() -> anyhow::Result<PathBuf>
{
  let mut path_db : PathBuf = env::var("GIMG_LAUNCHER_ROOT")?.into();
  path_db.push("gameimage.env.json");

  Ok(path_db)
} // from_file() }}}

// read() {{{
fn read() -> anyhow::Result<Vars>
{
  let path_db = from_file()?;

  let vars : Vars = match File::open(path_db)
  {
    Ok(file) => serde_json::from_reader(file).unwrap_or(Vars{ env: vec![] }),
    Err(_) => Vars{ env: vec![] },
  }; // match

  Ok(vars)
} // fn: read }}}

// write() {{{
fn write(key: String, val: String) -> anyhow::Result<()>
{
  let regex_key = Regex::new(r"^[_[:alpha:]][_[:alnum:]]*$")?;

  // Validate key
  if ! regex_key.is_match(key.as_str())
  {
    return Err(ah!("Invalid characters in key value"));
  } // if

  let path_db = from_file()?;

  // Read existing data
  let mut vars : Vars = match File::open(path_db.clone())
  {
    Ok(file) => serde_json::from_reader(file).unwrap_or(Vars{ env: vec![] }),
    Err(_) => Vars{ env: vec![] },
  }; // match

  // Append
  vars.env.push(Var{ key, val });

  // Write to file
  write!(File::create(&path_db)?, "{}", serde_json::to_string(&vars)?)?;

  Ok(())
} // fn: write }}}

// erase() {{{
fn erase(key: String) -> anyhow::Result<()>
{ 
  // Read current
  let mut vars = read()?;

  // Erase key
  vars.env = vars.env.into_iter().filter(move |e|{ e.key != key }).collect();

  // Write to file
  write!(File::create(&from_file()?)?, "{}", serde_json::to_string(&vars)?)?;

  Ok(())
} // erase() }}}

// get() {{{
pub fn get() -> anyhow::Result<Vars>
{
  read()
} // get() }}}

// set() {{{
pub fn set(key : String, val: String) -> anyhow::Result<()>
{
  write(key, val)
} // set() }}}

// del() {{{
pub fn del(key : String) -> anyhow::Result<()>
{
  erase(key)
} // set() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
