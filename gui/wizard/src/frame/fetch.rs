use std::collections::HashMap;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  enums::{Align,FrameType,Color},
  group::PackType,
};

use lazy_static::lazy_static;
use anyhow::anyhow as ah;

use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;

use crate::db;
use crate::dimm;
use crate::frame;
use crate::common;
use crate::log;
use crate::gameimage;

lazy_static!
{
  pub static ref HASH_PLATFORM_DESCR: HashMap<&'static str, &'static str> =
  {
    let mut m = HashMap::new();
    m.insert("linux", " Linux - Play linux native games (required)");
    m.insert("wine", " Wine - Play windows games");
    m.insert("pcsx2", " Pcsx2 - Play playstation 2 games");
    m.insert("rpcs3", " Rcps3 - Play playstation 3 games");
    m.insert("retroarch", " Retroarch - Play games from retro consoles");
    m
  };
  pub static ref HASH_DESC_PLATFORM: HashMap<&'static str, &'static str> =
  {
    let mut m = HashMap::new();
    for (key, value) in HASH_PLATFORM_DESCR.iter() { m.insert(*value, *key); }
    m
  };
}

// check_version() {{{
fn check_version() -> anyhow::Result<()>
{
  let db_fetch = match db::fetch::read()
  {
    Ok(db) => db,
    Err(e) => return Err(ah!("error: could not read fetch.json, backend failed? No internet? '{}", e)),
  }; // match

  let version = db_fetch.version;
  if ! version.starts_with("1.5")
  {
    return Err(ah!("error: you should update to version {}", version));
  } // if

  Ok(())
} // check_version() }}}

// fn fetch_backend() {{{
fn fetch_backend(tx: Sender<common::Msg>, platform: common::Platform)
{
  tx.send_awake(common::Msg::WindDeactivate);
  let clone_tx = tx.clone();
  std::thread::spawn(move ||
  {
    match gameimage::fetch::fetch(platform)
    {
      Ok(_) => log!("Successfully fetched file"),
      Err(e) => log!("Failed to fetch file: {}", e),
    }; // match
    clone_tx.send_awake(common::Msg::WindActivate);
    clone_tx.send_awake(common::Msg::DrawFetch);
  });
} // fn fetch_backend() }}}

// fn fetch_add_wine() {{{
fn fetch_add_wine(tx: Sender<common::Msg>
  , widget: &fltk::widget::Widget, distributions: &HashMap<String,String>
  , is_installed: bool) -> fltk::group::Flex
{
  let mut col = fltk::group::Flex::new(widget.x() + dimm::border()
    , widget.y() + dimm::border()
    , widget.w() - dimm::border()*2
    , dimm::height_button_wide()*2 + dimm::border()
    , ""
  );
  col.set_type(PackType::Vertical);
  col.set_spacing(dimm::border());
  // Row with platform description and fetch button
  let mut row = fltk::group::Flex::new(col.x(), col.y(), col.w() , dimm::height_button_wide() , "");
  row.set_type(PackType::Horizontal);
  row.set_spacing(dimm::border());
  {
    // Create progress bar
    let mut prog = fltk::misc::Progress::default()
      .with_label(HASH_PLATFORM_DESCR.get("wine").unwrap_or(&""))
      .with_align(Align::Left | Align::Inside)
      .with_frame(FrameType::BorderBox)
      .with_color(Color::BackGround)
      .with_color_selected(Color::Blue);
    if is_installed{ prog.set_value(100.0); }
    // Create start button
    let f_button = if is_installed { || { shared::fltk::button::rect::refresh() } } else { || { shared::fltk::button::rect::cloud() } };
    let btn_fetch = f_button()
      .with_color(if is_installed { Color::Blue } else { Color::Green })
      .with_focus(false)
      .with_callback(move |_| fetch_backend(tx, common::Platform::Wine));
    row.fixed(&btn_fetch, dimm::width_button_rec());
    row.end();
  }
  col.fixed(&row, dimm::height_button_wide());
  // Create distribution dropdown menu
  let mut menubutton = fltk::menu::MenuButton::default()
    .with_frame(FrameType::BorderBox)
    .with_color(Color::BackGround)
    .with_color_selected(Color::Blue)
    .with_focus(false)
    .with_callback(|e|
    {
      let choice = if let Some(choice) = e.choice() { choice } else { return; };
      std::env::set_var("GIMG_WINE_DIST", &choice);
      e.set_label(&choice);
    });
  menubutton.add_choice(&distributions.keys().map(|e| e.clone()).collect::<Vec<String>>().join("|"));
  // Init value for menubutton
  match std::env::var("GIMG_WINE_DIST")
  {
    Ok(var) => menubutton.set_label(&var),
    Err(_) =>
    {
      std::env::set_var("GIMG_WINE_DIST", "default");
      menubutton.set_label("default");
    }
  } // match
  col.fixed(&menubutton, dimm::height_button_wide());
  col.end();
  col
} // fn fetch_add_wine() }}}

// fn fetch_add() {{{
fn fetch_add(tx: Sender<common::Msg>
  , widget: &fltk::widget::Widget,platform: common::Platform
  , is_installed: bool) -> fltk::group::Flex
{
  let mut row = fltk::group::Flex::new(widget.x() + dimm::border()
    , widget.y() + dimm::border()
    , widget.w() - dimm::border()*2
    , dimm::height_button_wide()
    , ""
  );
  row.set_type(PackType::Horizontal);
  row.set_spacing(dimm::border());
  // Create progress bar
  let mut prog = fltk::misc::Progress::default()
    .with_label(HASH_PLATFORM_DESCR.get(platform.as_str()).unwrap_or(&""))
    .with_align(Align::Left | Align::Inside)
    .with_frame(FrameType::BorderBox)
    .with_color(Color::BackGround)
    .with_color_selected(Color::Blue);
  if is_installed { prog.set_value(100.0); }
  // Create start button
  let f_button = if is_installed { || { shared::fltk::button::rect::refresh() } } else { || { shared::fltk::button::rect::cloud() } };
  let btn_fetch = f_button()
    .with_color(if is_installed { Color::Blue } else { Color::Green })
    .with_focus(false)
    .with_callback(move |_| fetch_backend(tx, platform.clone()));
  row.fixed(&btn_fetch, dimm::width_button_rec());
  row.end();
  row
} // fn fetch_add() }}}

// fn fetch_list() {{{
fn fetch_list(tx: Sender<common::Msg>, widget: &fltk::widget::Widget) -> anyhow::Result<()>
{
  let vec_platforms = gameimage::fetch::installed()?;
  let db_fetch = db::fetch::read()?;
  let mut col = fltk::group::Flex::new(widget.x() + dimm::border()
    , widget.y() + dimm::border()
    , widget.w() - dimm::border()*2
    , widget.h() - dimm::border()*2
    , ""
  );
  col.set_type(PackType::Vertical);
  col.set_frame(FrameType::BorderBox);
  col.set_spacing(dimm::border());
  col.set_margin(dimm::border());
  let row_linux         = fetch_add(tx, &col.as_base_widget(), common::Platform::Linux, vec_platforms.contains(&common::Platform::Linux));
  let mut row_rpcs3     = fetch_add(tx, &col.as_base_widget(), common::Platform::Rcps3, vec_platforms.contains(&common::Platform::Rcps3));
  let mut row_retroarch = fetch_add(tx, &col.as_base_widget(), common::Platform::Retroarch, vec_platforms.contains(&common::Platform::Retroarch));
  let mut row_pcsx2     = fetch_add(tx, &col.as_base_widget(), common::Platform::Pcsx2, vec_platforms.contains(&common::Platform::Pcsx2));
  let mut row_wine      = fetch_add_wine(tx, &col.as_base_widget(), &db_fetch.wine.layer, vec_platforms.contains(&common::Platform::Wine));
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
} // fn fetch_list() }}}

// pub fn fetch() {{{
pub fn fetch(tx: Sender<common::Msg>, title: &str)
{
  // Enter the build directory
  if let Err(e) = common::dir_build() { log!("{}", e); }
  // Create frame from template
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();
  // Configure buttons
  ret_frame_footer.btn_prev.clone().emit(tx, common::Msg::DrawCreator);
  ret_frame_footer.btn_next.clone().hide();
  // Clone content frame
  let frame_content = ret_frame_header.frame_content.clone();
  // List platforms to fetch
  if let Err(e) = fetch_list(tx, &frame_content.as_base_widget()) { log!("{}", e); };
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
