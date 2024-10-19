use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

use shared::std::PathBufExt;

use anyhow::anyhow as ah;

use crate::common;
use crate::db;

pub struct Data
{
  pub platform : anyhow::Result<common::Platform>,
  pub path_root : PathBuf,
  pub path_icon : PathBuf,
  pub path_icon_grayscale : PathBuf,
  pub path_boot : PathBuf,
} // Data

// fn mount() {{{
fn mount(path_root : PathBuf) -> anyhow::Result<Data>
{
  let db_project = db::project::read(&path_root.join("gameimage.json"))?;
  let path_icon = path_root.join("icon/icon.png");
  let path_icon_grayscale = path_root.join("icon").join("icon.grayscale.png");
  let path_boot = path_root.join("boot");
  let platform = common::Platform::from_str(&db_project.platform);
  if path_icon.exists() && path_boot.exists()
  {
    return Ok(Data{ platform, path_boot, path_root, path_icon, path_icon_grayscale })
  } // if
  Err(ah!("Could not include project from '{}'", path_root.string()))
} // fn mount() }}}

// pub fn mounts() {{{
pub fn mounts() -> anyhow::Result<Vec<Data>>
{
  let vec_entries : Vec<DirEntry> = fs::read_dir("/opt/gameimage-games")?
    .filter_map(|e| { e.ok() })
    .filter(|e|{ e.path().is_dir() })
    .collect();

  let mut vec_data : Vec<Data> = vec![];

  for entry in vec_entries
  {
    match mount(entry.path())
    {
      Ok(data) => vec_data.push(data),
      Err(e) => eprintln!("{}", e),
    }
  } // for

  // Sort
  vec_data.sort_by(|a, b| return a.path_root.string().partial_cmp(&b.path_root.string()).unwrap());

  Ok(vec_data)
} // mounts() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
