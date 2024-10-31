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

use crate::gameimage;
use crate::dimm;
use crate::frame;
use crate::common;
use crate::log;
use shared::svg;
use shared::std::PathBufExt;

// pub fn welcome() {{{
pub fn welcome(tx: Sender<common::Msg>, title: &str)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();
  let frame_footer = ret_frame_footer.frame.clone();

  // Project Logo
  let mut frame_image = Frame::default()
    .with_size(dimm::height_button_rec()*4, dimm::height_button_rec()*4);
  frame_image.set_align(Align::Inside | Align::Bottom);
  frame_image.clone().center_of(&frame_content);
  frame_image.set_pos(frame_image.x(), frame_image.y() - dimm::height_button_wide() - dimm::height_text());
  let mut clone_frame_image = frame_image.clone();
  if let Some(image) = fltk::image::SvgImage::from_data(svg::ICON_GAMEIMAGE).ok()
  {
    clone_frame_image.set_image_scaled(Some(image));
  } // if
  else
  {
    log!("Failed to load icon image");
  } // else

  let mut input_dir = FileInput::default()
    .with_size(dimm::width_wizard() - dimm::border()*2, dimm::height_button_wide() + dimm::height_text())
    .above_of(&frame_footer, dimm::border())
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
  let mut clone_output_status = ret_frame_footer.output_status.clone();
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
  ret_frame_footer.btn_prev.clone().hide();

  // Set callback for next
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  let clone_tx = tx.clone();
  ret_frame_footer.btn_next.clone().set_callback(move |_|
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
      Err(e) => log!("Could not create build directory: {}", e),
    }
    // Init project build directory
    match gameimage::init::build(path_dir_build)
    {
      Ok(()) => (),
      Err(e) => log!("Error to initialize build directory: {}", e)
    }; // match
    // Fetch fetch list
    match gameimage::fetch::fetchlist()
    {
      Ok(code) => log!("Fetch exited with code {}", code),
      Err(e) => log!("Error to initialize build directory: {}", e)
    }; // match
    // Draw creator frame
    clone_tx.send_awake(common::Msg::DrawCreator);
  });
} // fn: welcome }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
