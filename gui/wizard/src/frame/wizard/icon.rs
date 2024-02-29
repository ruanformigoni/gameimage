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
use crate::log;
use crate::scaling;


// get_icon() {{{
fn get_icon() -> anyhow::Result<PathBuf>
{
  Ok((env::var("GIMG_PROJECT")? + "/icon/icon.png").into())
} // fn: get_icon }}}

// set_image() {{{
fn set_image(mut frame : Frame) -> anyhow::Result<()>
{
  // Get image
  let path_icon = get_icon()?;

  // Resize
  let path_icon_resized = PathBuf::from(path_icon.clone())
    .parent()
    .unwrap()
    .join("icon.wizard.resized.png");
  common::image_resize(path_icon_resized.clone(), path_icon, frame.w() as u32, frame.h() as u32);

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
      log!("Could not load png icon: {}", e);
    },
  } // if

  Err(ah!("Could not set cover frame image"))
} // set_image() }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>
  , title: &str
  , msg_prev : common::Msg
  , msg_next : common::Msg)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_header = ret_frame_header.frame.clone();
  let frame_content = ret_frame_header.frame_content.clone();
  let frame_footer = ret_frame_footer.frame.clone();

  // Scale icon image size
  let f_scale = |val: i32| -> i32
  {
    (val as f32 * scaling::factor().unwrap_or(1.0)) as i32
  };

  // Create icon box
  let mut frame_icon = Frame::default()
    .with_size(f_scale(150), f_scale(225))
    .center_of(&frame_content);
  frame_icon.set_pos(frame_icon.x(), frame_icon.y() - dimm::height_button_wide());
  frame_icon.set_frame(FrameType::BorderBox);

  // Footer callbacks
  ret_frame_footer.btn_prev.clone().emit(tx, msg_prev);

  let clone_tx = tx.clone();
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    if let Ok(path_file_icon) = env::var("GIMG_ICON")
    {
      if ! PathBuf::from(path_file_icon).is_file()
      {
        log!("Icon file is invalid");
        clone_output_status.set_value("Icon file is invalid");
        return;
      } // if
    } // if
    else
    {
      log!("Icon is not set");
      clone_output_status.set_value("Icon is not set");
      return;
    } // else

    clone_tx.send(msg_next);
  });

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
    let choice = file_chooser("Select the icon", "*.{jpg,png}", ".", false);

    if choice.is_none()
    {
      return;
    } // if

    let str_choice = choice.unwrap();

    // Show file path on selector
    e.set_value(str_choice.as_str());

    // Try to install icon
    clone_output_status.set_value("Installing icon...");
    if let Ok(rx_gameimage) = common::gameimage_cmd(vec![
        "install".to_string()
      , "icon".to_string()
      , str_choice.clone()
    ])
    {
      clone_tx.send(common::Msg::WindDeactivate);
      let _ = rx_gameimage.recv();
      clone_tx.send(common::Msg::WindActivate);
    } // if
    else
    {
      clone_output_status.set_value("Could not install icon, use .jpg or .png");
      return;
    } // else

    // Set environment variable
    match get_icon()
    {
      Ok(path_icon) => env::set_var("GIMG_ICON", path_icon),
      Err(e) =>
      {
        log!("Could not get icon path: {}", e);
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
