#![allow(dead_code)]
#![allow(unused_variables)]

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


// get_icon() {{{
fn get_icon() -> anyhow::Result<PathBuf>
{
  Ok((env::var("GIMG_PROJECT")? + "/icon/icon.png").into())
} // fn: get_icon }}}

// set_image() {{{
fn set_image(mut frame : Frame) -> anyhow::Result<()>
{
  let path_icon = get_icon()?;
  match fltk::image::PngImage::load(path_icon)
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
} // set_image() }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>, title: &str)
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
  ret_frame_footer.btn_prev.clone().emit(tx, common::Msg::DrawRetroarchName);
  ret_frame_footer.btn_next.clone().emit(tx, common::Msg::DrawRetroarchRom);

  // Icon
  let mut input_icon = FileInput::default()
    .with_size(frame_content.w() - dimm::border()*2, dimm::height_button_wide() + dimm::border())
    .below_of(&frame_content, 0)
    .with_align(Align::Top | Align::Left);
  input_icon.set_pos(frame_content.x() + dimm::border()
    , input_icon.y() - input_icon.h() - dimm::border());
  input_icon.set_readonly(true);

  // Check if GIMG_ICON exists
  if let Some(env_icon) = env::var("GIMG_ICON").ok()
  {
    // Set value of select field
    input_icon.set_value(&env_icon);
    // Update preview
    if let Err(e) = set_image(frame_icon.clone())
    {
      ret_frame_footer.output_status
        .clone()
        .set_value(format!("Failed to load preview: {}", e.to_string()).as_str());
    } // if
  } // if

  // // Set input_icon callback
  let clone_tx = tx.clone();
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  input_icon.set_callback(move |e|
  {
    let choice = file_chooser("Select the icon", "*.jpg|*.png|*.svg", ".", false);

    if choice.is_none()
    {
      return;
    } // if

    let str_choice = choice.unwrap();

    // Show file path on selector
    e.set_value(str_choice.as_str());

    // Try to install icon
    clone_output_status.set_value("Installing icon...");
    if let Err(e) = common::gameimage_cmd(vec![
        "install".to_string()
      , "icon".to_string()
      , str_choice.clone()
    ])
    {
      clone_output_status.set_value("Could not install icon, use .jpg or .png");
      return;
    } // if

    // Set environment variable
    match get_icon()
    {
      Ok(path_icon) => env::set_var("GIMG_ICON", path_icon),
      Err(e) =>
      {
        println!("Could not get icon path: {}", e);
        clone_output_status.set_value(format!("Could not get icon path: {}", e).as_str());
      }
    } // if

    // Set preview image
    if let Err(e) = set_image(frame_icon.clone())
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
