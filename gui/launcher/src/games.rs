use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

use shared::std::PathBufExt;

use anyhow::anyhow as ah;

use crate::common;
use crate::db;

pub struct Game
{
  pub platform : common::Platform,
  pub path_root : PathBuf,
  pub path_icon : PathBuf,
  pub path_icon_grayscale : PathBuf,
  pub path_boot : PathBuf,
} // Game

// pub fn launch() {{{
pub fn launch()
{
  let _ =  std::process::Command::new("sh")
    .args(["-c", "$GIMG_LAUNCHER_BOOT"])
    .stdout(std::process::Stdio::inherit())
    .stderr(std::process::Stdio::inherit())
    .output();
} // fn: launch }}}

// pub fn select() {{{
pub fn select(game: &Game)
{
  std::env::set_var("GIMG_PLATFORM", game.platform.as_str());
  std::env::set_var("GIMG_LAUNCHER_BOOT", game.path_boot.to_str().unwrap_or(""));
  std::env::set_var("GIMG_LAUNCHER_ROOT", game.path_root.to_str().unwrap_or(""));
  std::env::set_var("GIMG_LAUNCHER_IMG", game.path_icon.to_str().unwrap_or(""));
  std::env::set_var("GIMG_LAUNCHER_IMG_GRAYSCALE", game.path_icon_grayscale.to_str().unwrap_or(""));
} // fn: select }}}

// pub fn select_by_index() {{{
pub fn select_by_index(index: usize) -> anyhow::Result<()>
{
  select(games()?.get(index).ok_or(ah!("Index out of bounds"))?);
  Ok(())
} // fn: select_by_index }}}

// fn game() {{{
fn game(path_root : PathBuf) -> anyhow::Result<Game>
{
  let db_project = db::project::read(&path_root.join("gameimage.json"))?;
  let path_icon = path_root.join("icon/icon.png");
  let path_icon_grayscale = path_root.join("icon").join("icon.grayscale.png");
  let path_boot = path_root.join("boot");
  let platform = common::Platform::from_str(&db_project.platform)?;
  if path_icon.exists() && path_boot.exists()
  {
    return Ok(Game{ platform, path_boot, path_root, path_icon, path_icon_grayscale })
  } // if
  Err(ah!("Could not include project from '{}'", path_root.string()))
} // fn game() }}}

// pub fn games() {{{
pub fn games() -> anyhow::Result<Vec<Game>>
{
  let vec_entries : Vec<DirEntry> = fs::read_dir("/opt/gameimage-games")?
    .filter_map(|e| { e.ok() })
    .filter(|e|{ e.path().is_dir() })
    .collect();

  let mut vec_data : Vec<Game> = vec![];

  for entry in vec_entries
  {
    match game(entry.path())
    {
      Ok(game) => vec_data.push(game),
      Err(e) => eprintln!("{}", e),
    }
  } // for

  // Sort
  vec_data.sort_by(|a, b| return a.path_root.string().partial_cmp(&b.path_root.string()).unwrap());

  Ok(vec_data)
} // games() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
