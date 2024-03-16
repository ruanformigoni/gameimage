use std::path::PathBuf;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  input::FileInput,
  frame::Frame,
  dialog::file_chooser,
  enums::{Align,FrameType},
};

use anyhow::anyhow as ah;

use crate::dimm;
use crate::frame;
use crate::common;
use crate::common::PathBufExt;
use crate::log;

// set_image_preview() {{{
fn set_image_preview(mut frame : Frame, path_file_icon : PathBuf) -> anyhow::Result<()>
{
  // Resize to preview how it looks
  let path_icon_resized = PathBuf::from(path_file_icon.clone())
    .parent()
    .unwrap()
    .join("icon.wizard.resized.png");

  // Do the actual resizing
  if let Err(e) = common::image_resize(path_icon_resized.clone()
    , path_file_icon.clone()
    , frame.w() as u32, frame.h() as u32)
  {
    log!("Failed to resize image {} with error {}", path_file_icon.string(), e);
  } // if

  // Set as icon in frame
  match fltk::image::PngImage::load(path_icon_resized)
  {
    Ok(png_image) =>
    {
      frame.set_image_scaled(Some(png_image));
      frame.redraw();
      fltk::app::awake();
      return Ok(())
    },
    Err(e) =>
    {
      log!("Could not load png icon: {}", e);
    },
  } // if

  Err(ah!("Could not set cover frame image"))
} // set_image_preview() }}}

// pub fn desktop() {{{
pub fn desktop(tx: Sender<common::Msg>, title: &str)
{
  // Enter the build directory
  if let Err(e) = common::dir_build()
  {
    log!("Err: {}", e.to_string());
  } // if

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

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
    let str_choice = if let Some(choice) = file_chooser("Select the icon", "*.{jpg,png}", ".", false)
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

    // Disable window
    clone_tx.send(common::Msg::WindDeactivate);

    let clone_frame_icon = frame_icon.clone();
    let mut clone_output_status = clone_output_status.clone();
    std::thread::spawn(move ||
    {
      // Set as desktop entry icon for image
      // Wait for message & check return value
      if common::gameimage_sync(vec!["desktop", &str_choice]) != 0
      {
        log!("Failed to execute desktop command on backend");
        clone_tx.send(common::Msg::WindActivate);
        return;
      } // if

      // Set preview image
      if let Err(e) = set_image_preview(clone_frame_icon.clone(), str_choice.into())
      {
        clone_output_status.set_value(format!("Failed to load icon image into preview with error {}", e).as_str());
      } // if
      else
      {
        clone_output_status.set_value("Set preview image");
      } // else

      // Re-activate window
      clone_tx.send(common::Msg::WindActivate);
    }); // std::thread
  }); // callback
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
