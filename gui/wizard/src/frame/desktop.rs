#![allow(warnings)]

use std::env;
use std::path::PathBuf;
use std::fs::File;

// Gui
use fltk::prelude::*;
use fltk::{
  app::{Sender,Receiver},
  window::Window,
  text::{TextBuffer,TextDisplay},
  menu::MenuButton,
  button::Button,
  group::Group,
  image::SharedImage,
  input::{Input,FileInput},
  group::PackType,
  frame::Frame,
  dialog::{file_chooser,dir_chooser},
  enums::{Align,FrameType,Color},
  misc::Progress,
};

use url as Url;
use anyhow;
use anyhow::anyhow as ah;

use crate::dimm;
use crate::frame;
use crate::common;
use crate::db;
use crate::download;
use crate::svg;

// set_image_preview() {{{
fn set_image_preview(mut frame : Frame, path_file_icon : PathBuf) -> anyhow::Result<()>
{
  // Resize to preview how it looks
  let path_icon_resized = PathBuf::from(path_file_icon.clone())
    .parent()
    .unwrap()
    .join("icon.wizard.resized.png");

  // Do the actual resizing
  common::image_resize(path_icon_resized.clone(), path_file_icon, frame.w() as u32, frame.h() as u32);

  // Set as icon in frame
  match fltk::image::PngImage::load(path_icon_resized)
  {
    Ok(png_image) =>
    {
      frame.set_image_scaled(Some(png_image));
      frame.redraw();
      return Ok(())
    },
    Err(e) =>
    {
      println!("Could not load png icon: {}", e);
    },
  } // if

  Err(ah!("Could not set cover frame image"))
} // set_image_preview() }}}

// pub fn desktop() {{{
pub fn desktop(tx: Sender<common::Msg>, title: &str)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_header = ret_frame_header.frame.clone();
  let frame_content = ret_frame_header.frame_content.clone();
  let frame_footer = ret_frame_footer.frame.clone();

  // Create icon box
  let mut frame_icon = Frame::default()
    .with_size(150, 225)
    .center_of(&frame_content);
  frame_icon.set_pos(frame_icon.x(), frame_icon.y() - dimm::height_button_wide());
  frame_icon.set_frame(FrameType::BorderBox);

  // Footer callbacks
  ret_frame_footer.btn_prev.clone().hide();
  ret_frame_footer.btn_next.clone().emit(tx.clone(), common::Msg::DrawName);

  // Icon
  let mut input_icon = FileInput::default()
    .with_size(frame_content.w() - dimm::border()*2, dimm::height_button_wide() + dimm::border())
    .below_of(&frame_content, 0)
    .with_align(Align::Top | Align::Left);
  input_icon.set_pos(frame_content.x() + dimm::border()
    , input_icon.y() - input_icon.h() - dimm::border());
  input_icon.set_readonly(true);

  // Set input_icon callback
  let clone_tx = tx.clone();
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  input_icon.set_callback(move |e|
  {
    let str_choice = if let Some(choice) = file_chooser("Select the icon", "*.jpg|*.png|*.svg", ".", false)
    {
      choice
    } // if
    else
    {
      clone_output_status.set_value("No file selected");
      return;
    }; // else

    // Show file path on selector
    e.set_value(str_choice.as_str());

    // Try to install icon
    clone_output_status.set_value("Installing icon...");

    // Set as desktop entry icon for image
    if let Err(e) = common::gameimage_cmd(vec!["desktop".to_string(), str_choice.clone()])
    {
      clone_output_status.set_value("Could not install icon, use .jpg or .png");
      return;
    } // if

    // Set preview image
    if let Err(e) = set_image_preview(frame_icon.clone(), str_choice.into())
    {
      clone_output_status.set_value("Failed to load icon image into preview");
    } // if
    else
    {
      clone_output_status.set_value("Set preview image");
    } // else
  });
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
