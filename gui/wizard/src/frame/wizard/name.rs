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

use crate::frame::wizard;


// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>, title: &str)
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
  frame_icon.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_joystick(10.0).as_str()).unwrap()));
  frame_icon.set_frame(FrameType::NoBox);

  //
  // Game name
  //
  let mut input_name = Input::default()
    .with_size(frame_content.w() - dimm::border()*2, dimm::height_button_wide())
    .below_of(&frame_content, 0)
    .with_align(Align::Top | Align::Left);
  input_name.set_pos(frame_content.x() + dimm::border()
    , input_name.y() - input_name.h() - dimm::border());

  // // Check if GIMG_NAME exists
  if let Some(env_name) = env::var("GIMG_NAME").ok()
  {
    input_name.set_value(&env_name);
  } // if

  // // Set input_name callback
  input_name.set_callback(|e|
  {
    env::set_var("GIMG_NAME", e.value());
  });

  // Callback to previous
  ret_frame_footer.btn_prev.clone().emit(tx, common::Msg::DrawCreator);

  // Callback to Next
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  let clone_tx = tx.clone();
  ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    // Check for name
    let name = if let Ok(name) = env::var("GIMG_NAME")
    {
      name
    }
    else
    {
      clone_output_status.set_value("Name field is empty");
      println!("Could not fetch GIMG_NAME");
      return;
    }; // else

    // Check for platform
    let platform = if let Ok(platform) = env::var("GIMG_PLATFORM")
    {
      platform
    }
    else
    {
      clone_output_status.set_value("Could not fetch GIMG_PLATFORM");
      println!("Could not fetch GIMG_PLATFORM");
      return;
    }; // else

    // Check for image
    let image = if let Ok(image) = env::var("GIMG_IMAGE")
    {
      image
    }
    else
    {
      clone_output_status.set_value("Could not fetch GIMG_IMAGE");
      println!("Could not fetch GIMG_IMAGE");
      return;
    }; // else

    // Init project
    if let Err(e) = common::gameimage_cmd(vec!["init".to_string()
      , "--dir".to_string()
      , name.clone()
      , "--platform".to_string()
      , platform
      , "--image".to_string()
      , image
    ])
    {
      let msg = format!("Could not execute backend: {}", e.to_string());
      clone_output_status.set_value(&msg);
      println!("{}", &msg);
      return;
    }

    // Export project dir
    if let Ok(env_dir) = env::var("GIMG_DIR")
    {
      env::set_var("GIMG_PROJECT", env_dir + "/" + name.as_str());
    } // if

    // Go to next frame
    clone_tx.send(common::Msg::DrawRetroarchIcon);
  });
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
