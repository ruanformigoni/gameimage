use std::env;
use std::path::PathBuf;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  image::SharedImage,
  input::FileInput,
  group::PackType,
  frame::Frame,
  dialog::dir_chooser,
  enums::{Align,FrameType},
};

use crate::dimm;
use crate::frame;
use crate::common;

// pub fn welcome() {{{
pub fn welcome(tx: Sender<common::Msg>, title: &str)
{
  let mut frame = Frame::default()
    .with_size(dimm::width(), dimm::height());
  frame.set_type(PackType::Vertical);

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_header = ret_frame_header.frame.clone();
  let frame_footer = ret_frame_footer.frame.clone();

  let mut frame_content = Frame::default()
    .with_size(dimm::width(), dimm::height() - dimm::height_header() - dimm::height_footer())
    .below_of(&frame_header, 0);
  frame_content.set_type(PackType::Vertical);
  
  // Project Logo
  let mut frame_image = Frame::default()
    .with_size(dimm::height_button_rec()*4, dimm::height_button_rec()*4);
  frame_image.set_align(Align::Inside | Align::Bottom);
  frame_image.clone().center_of(&frame_content);
  frame_image.set_pos(frame_image.x(), frame_image.y() - dimm::height_button_wide() - dimm::height_text());
  let mut clone_frame_image = frame_image.clone();
  let _ = SharedImage::load("/tmp/gameimage/gameimage.png")
    .and_then(move |mut img|
    {
      img.scale(clone_frame_image.w(), clone_frame_image.h(), true, true);
      clone_frame_image.set_image(Some(img.clone()));
      Ok(img)
    });

  let mut input_dir = FileInput::default()
    .with_size(dimm::WIDTH - dimm::BORDER*2, dimm::HEIGHT_BUTTON_WIDE + dimm::HEIGHT_TEXT)
    .above_of(&frame_footer, dimm::border())
    .with_align(Align::Top | Align::Left)
    .with_label("Select The Directory for GameImage's Temporary Files");
  input_dir.set_pos(dimm::border(), input_dir.y());
  input_dir.set_readonly(true);

  // // Check if GIMG_DIR exists
  if let Some(env_dir_build) = env::var("GIMG_DIR").ok()
  {
    input_dir.set_value(&env_dir_build);
  } // if

  // // Set input_dir callback
  input_dir.set_callback(|e|
  {
    let choice = dir_chooser("Select the build directory", "", false);
    let str_choice = choice.unwrap_or(String::from(""));
    e.set_value(str_choice.as_str());
    env::set_var("GIMG_DIR", str_choice.as_str());
  });

  // First frame, no need for prev
  ret_frame_footer.btn_prev.clone().hide();

  // Set callback for next
  let mut clone_btn_next = ret_frame_footer.btn_next.clone();
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  let clone_tx = tx.clone();
  clone_btn_next.set_callback(move |_|
  {
    let env_gimg_dir = env::var("GIMG_DIR").ok();

    if env_gimg_dir.is_none()
    {
      clone_output_status.set_value("Invalid temporary files directory");
      return;
    } // if

    if ! PathBuf::from(&env_gimg_dir.unwrap()).exists()
    {
      clone_output_status.set_value("Selected path does not exist");
      return;
    } // if

    clone_tx.send(common::Msg::DrawPlatform);
  });
} // fn: welcome }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
