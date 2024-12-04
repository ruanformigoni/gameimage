use std::collections::HashMap;
use std::sync::{Mutex,LazyLock};

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  enums::{Align,FrameType,Color},
  group::PackType,
};

use clown::clown;

use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;
use shared::{tabs,rescope,hover_blink,hseparator_fixed,column,row,add,fixed,scroll,hpack};

use crate::db;
use crate::dimm;
use crate::common;
use crate::log;
use crate::log_err_status;
use crate::gameimage;

pub static HASH_PLATFORM_MSG: LazyLock<HashMap<common::Platform, common::Msg>> = LazyLock::new(||
{
  let mut m = HashMap::new();
  m.insert(common::Platform::Wine, common::Msg::DrawWineName);
  m.insert(common::Platform::Linux, common::Msg::DrawLinuxName);
  m.insert(common::Platform::Retroarch, common::Msg::DrawRetroarchName);
  m.insert(common::Platform::Pcsx2, common::Msg::DrawPcsx2Name);
  m.insert(common::Platform::Rcps3, common::Msg::DrawRpcs3Name);
  m
});

pub static HASH_PLATFORM_DESCR: LazyLock<HashMap<&'static str, &'static str>> =LazyLock::new(||
{
  let mut m = HashMap::new();
  m.insert("linux", " Linux - Play linux native games (required)");
  m.insert("wine", " Wine - Play windows games");
  m.insert("pcsx2", " Pcsx2 - Play playstation 2 games");
  m.insert("rpcs3", " Rcps3 - Play playstation 3 games");
  m.insert("retroarch", " Retroarch - Play games from retro consoles");
  m
});

pub static PLATFORM: LazyLock<Mutex<Option<common::Platform>>> = LazyLock::new(|| Mutex::new(None));

pub static DIST_WINE: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));

// fn fetch_backend() {{{
fn fetch_backend(tx: Sender<common::Msg>
  , platform: common::Platform
  , mut widget_progress: fltk::misc::Progress)
{
  tx.send_awake(common::Msg::WindDeactivate);
  let clone_tx = tx.clone();
  let f_progress = move |rx: std::sync::mpsc::Receiver<String>|
  {
    while let Ok(msg) = rx.recv()
    {
      match msg.parse::<f64>()
      {
        Ok(progress) => widget_progress.set_value(progress),
        Err(e) => { log!("Could not convert progress to float: {}", e); return; },
      }; // match
    }; // match
  };
  std::thread::spawn(move ||
  {
    match gameimage::fetch::fetch(platform, f_progress)
    {
      Ok(_) => log!("Successfully fetched file"),
      Err(e) =>
      {
        fltk::dialog::alert_default(&format!("Failed to fetch file: {}", e));
        log!("Failed to fetch file: {}", e);
      },
    }; // match
    clone_tx.send_activate(common::Msg::DrawPlatform);
  });
} // fn fetch_backend() }}}

// fn platform_add() {{{
fn platform_add(tx: Sender<common::Msg>
  , platform: common::Platform
  , is_installed: bool) -> fltk::group::Flex
{
  row!(row,
    row.set_spacing(dimm::border());
    add!(row, prog, shared::fltk::progress::progress());
    fixed!(row
      , btn_platform
      , if is_installed { shared::fltk::button::rect::arrow_forward() } else {  shared::fltk::button::rect::cloud() }
      , dimm::width_button_rec()
    );
  );
  // Configure progress bar
  let mut prog = prog.clone()
    .with_label(HASH_PLATFORM_DESCR.get(platform.as_str()).unwrap_or(&""))
    .with_align(Align::Left | Align::Inside)
    .with_color(Color::BackGround)
    .with_color_selected(Color::Blue);
  if is_installed{ prog.set_value(100.0); }
  // Configure start button
  hover_blink!(btn_platform);
  btn_platform.clone()
    .with_color(if is_installed { Color::Green } else { Color::Blue })
    .with_callback(#[clown] move |_|
    {
      if is_installed
      {
        *PLATFORM.lock().unwrap() = Some(honk!(platform).clone());
        tx.send_awake(*HASH_PLATFORM_MSG.get(&platform).unwrap());
      }
      else
      {
        fetch_backend(tx, platform.clone(), prog.clone());
      } // else
    });
  row
} // fn platform_add() }}}

// fn platform_add_wine() {{{
fn platform_add_wine(tx: Sender<common::Msg>
  , distributions: &HashMap<String,String>
  , mut is_installed: bool) -> fltk::group::Flex
{
  column!(col,
    col.set_spacing(dimm::border());
    row!(row,
      row.set_spacing(dimm::border());
      add!(row, prog, shared::fltk::progress::progress());
      fixed!(row
        , btn_fetch
        , if is_installed { shared::fltk::button::rect::arrow_forward() } else {  shared::fltk::button::rect::cloud() }
        , dimm::width_button_rec()
      );
    );
    col.fixed(&row, dimm::height_button_wide());
    fixed!(col, menubutton, fltk::menu::MenuButton::default(), dimm::height_button_wide());
  );

  let dist_wine_db = db::global::read().map(|e| e.dist_wine).unwrap_or(String::from("default"));
  {
    let mut dist_wine_current = DIST_WINE.lock().unwrap();
    // Initialize value
    if *dist_wine_current == "" { *dist_wine_current = dist_wine_db.clone(); }
    // Check if dropdown menu selection differs from database reference
    is_installed = is_installed && *dist_wine_current == dist_wine_db;
  }
  // Configure progress bar
  let mut prog = prog.clone()
    .with_label(HASH_PLATFORM_DESCR.get("wine").unwrap_or(&""))
    .with_align(Align::Left | Align::Inside)
    .with_frame(FrameType::BorderBox)
    .with_color(Color::BackGround)
    .with_color_selected(Color::Blue);
  if is_installed{ prog.set_value(100.0); }
  // Configure fetch button
  hover_blink!(btn_fetch);
  btn_fetch.clone()
    .with_color(if is_installed { Color::Green } else { Color::Blue })
    .with_callback(#[clown] move |_|
    {
      if is_installed
      {
        match PLATFORM.lock()
        {
          Ok(mut guard) => *guard = Some(common::Platform::Wine),
          Err(e) => log!("Could not lock platform: {}", e),
        } // match
        tx.send_awake(*HASH_PLATFORM_MSG.get(&common::Platform::Wine).unwrap());
      }
      else
      {
        if let Err(e) = db::global::update(#[clown] |mut db|{ db.dist_wine = DIST_WINE.lock().unwrap().clone(); db })
        {
          log!("Could not update wine distribution: {}", e);
        } // if
        fetch_backend(tx, common::Platform::Wine, prog.clone());
      } // else
    });
  // Create distribution dropdown menu
  let mut menubutton = menubutton.clone()
    .with_frame(FrameType::FlatBox)
    .with_color(Color::BackGround.lighter())
    .with_callback(move |e|
    {
      let choice = if let Some(choice) = e.choice() { choice } else { return; };
      *DIST_WINE.lock().unwrap() = choice;
      tx.send(common::Msg::DrawPlatform);
    });
  menubutton.add_choice(&distributions.keys().map(|e| e.clone()).collect::<Vec<String>>().join("|"));
  menubutton.set_label(&DIST_WINE.lock().unwrap().clone());
  col
} // fn platform_add_wine() }}}

// fn platform_list() {{{
fn platform_list(tx: Sender<common::Msg>, widget: &fltk::group::Flex) -> anyhow::Result<()>
{
  let vec_platforms = gameimage::fetch::installed()?;
  let db_fetch = db::fetch::read()?;
  let mut col = fltk::group::Flex::new(widget.x(), widget.y(), widget.w(), widget.h(), "");
  col.set_type(PackType::Vertical);
  col.set_spacing(dimm::border());
  let row_linux         = platform_add(tx, common::Platform::Linux, vec_platforms.contains(&common::Platform::Linux));
  let mut row_rpcs3     = platform_add(tx, common::Platform::Rcps3, vec_platforms.contains(&common::Platform::Rcps3));
  let mut row_retroarch = platform_add(tx, common::Platform::Retroarch, vec_platforms.contains(&common::Platform::Retroarch));
  let mut row_pcsx2     = platform_add(tx, common::Platform::Pcsx2, vec_platforms.contains(&common::Platform::Pcsx2));
  let mut row_wine      = platform_add_wine(tx, &db_fetch.wine.layer, vec_platforms.contains(&common::Platform::Wine));
  col.fixed(&row_linux, dimm::height_button_wide());
  col.fixed(&row_rpcs3, dimm::height_button_wide());
  col.fixed(&row_retroarch, dimm::height_button_wide());
  col.fixed(&row_pcsx2, dimm::height_button_wide());
  col.fixed(&row_wine, dimm::height_button_wide()*2 + dimm::border());
  if ! vec_platforms.contains(&common::Platform::Linux)
  {
    row_rpcs3.deactivate();
    row_retroarch.deactivate();
    row_pcsx2.deactivate();
    row_wine.deactivate();
  } // if
  col.end();

  Ok(())
} // fn platform_list() }}}

// pub fn platform() {{{
pub fn platform(tx: Sender<common::Msg>, title: &str)
{
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);
  // Enter the build directory
  log_err_status!(common::dir_build());
  // Configure buttons
  ui.btn_prev.clone().emit(tx, common::Msg::DrawCreator);
  ui.btn_next.clone().hide();
  // List platforms to fetch
  log_err_status!(platform_list(tx, &ui.group));
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
