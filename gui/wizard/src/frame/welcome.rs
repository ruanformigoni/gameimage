use std::env;
use std::path::PathBuf;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  input::FileInput,
  frame::Frame,
  dialog::dir_chooser,
  enums::Align,
};

use shared::fltk::SenderExt;
use anyhow::anyhow as ah;

use crate::db;
use crate::gameimage;
use crate::dimm;
use crate::common;
use crate::log_status;
use shared::svg;
use shared::std::PathBufExt;
use shared::fltk::WidgetExtExtra;

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

// pub fn welcome() {{{
pub fn welcome(tx: Sender<common::Msg>, title: &str)
{
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);

  // Project Logo
  let mut frame_image = Frame::default()
    .with_size(dimm::height_button_rec()*4, dimm::height_button_rec()*4);
  frame_image.set_align(Align::Inside | Align::Bottom);
  frame_image.clone().center_of(&ui.group);
  frame_image.set_pos(frame_image.x(), frame_image.y() - dimm::height_button_wide() - dimm::height_text());
  let mut clone_frame_image = frame_image.clone();
  if let Some(image) = fltk::image::SvgImage::from_data(svg::ICON_GAMEIMAGE).ok()
  {
    clone_frame_image.set_image_scaled(Some(image));
  } // if
  else
  {
    log_status!("Failed to load icon image");
  } // else

  let mut input_dir = FileInput::default()
    .with_size(dimm::width_wizard() - dimm::border()*2, dimm::height_button_wide() + dimm::height_text())
    .bottom_left_of(&ui.group, 0)
    .with_align(Align::Top | Align::Left)
    .with_label("Select The Directory for GameImage's Temporary Files");
  input_dir.set_pos(dimm::border(), input_dir.y());
  input_dir.set_readonly(true);

  // Check if GIMG_DIR exists
  if let Some(env_dir_build) = env::var("GIMG_DIR").ok()
  {
    input_dir.set_value(&env_dir_build);
  } // if

  // Set input_dir callback
  let mut clone_output_status = ui.status.clone();
  input_dir.set_callback(move |e|
  {
    let mut path_selected = if let Some(value) = dir_chooser("Select the build directory", "", false)
    {
      PathBuf::from(value)
    }
    else
    {
      clone_output_status.set_value("No file selected");
      return;
    }; // if

    // Set build dir as chosen dir + /build
    path_selected = path_selected.join("build");

    // Update chosen dir in selection bar
    e.set_value(&path_selected.string());

    // Set env var to build dir
    env::set_var("GIMG_DIR", &path_selected.string());
  });

  // First frame, no need for prev
  ui.btn_prev.clone().hide();

  // Set callback for next
  let mut clone_output_status = ui.status.clone();
  let clone_tx = tx.clone();
  ui.btn_next.clone().set_callback(move |_|
  {
    let path_dir_build = match env::var("GIMG_DIR")
    {
      Ok(value) => PathBuf::from(value),
      Err(e) => { clone_output_status.set_value(&format!("Invalid temporary files directory: {}", e)); return; }
    }; // if
    // Create build directory
    match std::fs::create_dir_all(&path_dir_build)
    {
      Ok(()) => (),
      Err(e) => log_status!("Could not create build directory: {}", e),
    }
    // Init project build directory
    match gameimage::init::build(path_dir_build)
    {
      Ok(()) => (),
      Err(e) => log_status!("Error to initialize build directory: {}", e)
    }; // match
    // Fetch fetch list
    match gameimage::fetch::fetchlist()
    {
      Ok(code) => log_status!("Fetch exited with code {}", code),
      Err(e) => log_status!("Error to initialize build directory: {}", e)
    }; // match
    // Check if version matches
    if let Err(e) = check_version()
    {
      log_status!("{}", e);
      fltk::dialog::message_default(&format!("{}", e));
      clone_tx.send_awake(common::Msg::WindActivate);
      return;
    } // if
    // Draw creator frame
    clone_tx.send_awake(common::Msg::DrawCreator);
  });
} // fn: welcome }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
