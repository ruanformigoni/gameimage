use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

use shared::std::PathBufExt;

use anyhow::anyhow as ah;

use crate::common;

pub struct Data
{
  pub platform : anyhow::Result<common::Platform>,
  pub path_root : PathBuf,
  pub path_icon : PathBuf,
  pub path_icon_grayscale : PathBuf,
  pub path_boot : PathBuf,
} // Data

// get_path_db() {{{
fn get_path_db(path_dir_root : &std::path::PathBuf) -> anyhow::Result<std::path::PathBuf>
{
  let mut path_db = path_dir_root.clone();
  path_db.push("gameimage.json");
  Ok(path_db)
} // get_path_db() }}}

// get_platform() {{{
fn get_platform(path_dir_root : &std::path::PathBuf) -> anyhow::Result<common::Platform>
{
  Ok(common::Platform::from_str(&shared::db::kv::read(&get_path_db(&path_dir_root)?)?
    .get("platform").ok_or(ah!("Key not found"))?)?)
} // get_platform() }}}

// pub fn mounts() {{{
pub fn mounts() -> anyhow::Result<Vec<Data>>
{
  let vec_entries : Vec<DirEntry> = fs::read_dir("/fim/mount")?
    .filter_map(|e| { e.ok() })
    .filter(|e|{ e.path().is_dir() })
    .collect();

  let mut vec_pairs : Vec<Data> = vec![];

  for entry in vec_entries
  {
    let path_root = entry.path();
    let path_icon = path_root.join("icon").join("icon.png");
    let path_icon_grayscale = path_root.join("icon").join("icon.grayscale.png");
    let path_boot = path_root.join("boot");
    let platform = get_platform(&path_root);
    if path_icon.exists() && path_boot.exists()
    {
      vec_pairs.push(Data{ platform, path_boot, path_root, path_icon, path_icon_grayscale });
    } // if
  } // for

  // Sort
  vec_pairs.sort_by(|a, b| return a.path_root.string().partial_cmp(&b.path_root.string()).unwrap());

  Ok(vec_pairs)
} // mounts() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
