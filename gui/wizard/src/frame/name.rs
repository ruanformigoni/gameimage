use std::env;
use std::path::PathBuf;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  input::Input,
  frame::Frame,
  enums::{Align,FrameType},
};

use shared::fltk::SenderExt;

use crate::dimm;
use crate::frame;
use crate::log;
use shared::svg;
use crate::common;
use shared::std::PathBufExt;

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>, title: &str)
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
  frame_icon.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_joystick(10.0).as_str()).unwrap()));
  frame_icon.set_frame(FrameType::NoBox);

  //
  // Image name
  //
  let mut input_name = Input::default()
    .with_size(frame_content.w() - dimm::border()*2, dimm::height_button_wide())
    .below_of(&frame_content, 0)
    .with_align(Align::Top | Align::Left);
  input_name.set_pos(frame_content.x() + dimm::border()
    , input_name.y() - input_name.h() - dimm::border());

  // Sanitize image name
  let f_sanitize = |mut input : String| -> Option<String>
  {
    input = input
      .chars()
      .filter_map(|c|
      {
        if c.is_alphanumeric() { Some(c) }
        else if c == '-' { Some(c) }
        else if c == '_' { Some(c) }
        else if c == ' ' { Some('-') }
        else { None }
      })
      .collect();

    if input.is_empty() { return None }

    Some(input)
  };

  // Set prev to desktop
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), common::Msg::DrawDesktop);

  // Callback to Next
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  let clone_tx = tx.clone();
  let clone_input_name = input_name.clone();
  ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    // Check for name
    let str_name = if let Some(name) = f_sanitize(clone_input_name.value())
    {
      name + ".flatimage"
    }
    else
    {
      clone_output_status.set_value("Name field is empty");
      return;
    }; // else

    // Get src image path
    let path_file_image_src =
      if let Ok(env_image) = env::var("GIMG_IMAGE")
      && let Ok(env_dir) = env::var("GIMG_DIR")
    {
      PathBuf::from(env_dir).join(env_image)
    }
    else
    {
      clone_output_status.set_value("Could not fetch GIMG_IMAGE or GIMG_DIR");
      return;
    }; // else

    // Create destination image path
    let path_file_image_dst = if let Some(path_file_image_dst) = path_file_image_src
        //      -
        // ../build/ image.flatimage
        .parent()
        //  -
        // ../ build/image.flatimage
        .and_then(|e|{ e.parent() })
        // ../new-name.flatimage
        .and_then(|e|{ Some(e.join(str_name)) })
    {
      path_file_image_dst
    }
    else
    {
      clone_output_status.set_value("Could not formulate destination path");
      return;
    }; // else

    // Move image
    if let Err(e) = std::fs::rename(path_file_image_src, &path_file_image_dst)
    {
      let msg = format!("Could not move image: {}", e.to_string());
      clone_output_status.set_value(&msg);
      log!("{}", &msg);
      return;
    } // if

    // Save location in var to display in draw finish
    env::set_var("GIMG_FINISH_LOCATION", &path_file_image_dst.string());

    // Go to next frame
    clone_tx.send_awake(common::Msg::DrawFinish);
  });
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
