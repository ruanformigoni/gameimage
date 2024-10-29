use std::env;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  input::Input,
  frame::Frame,
  enums::{Align,FrameType},
};

use shared::fltk::SenderExt;
use shared::svg;

use crate::dimm;
use crate::frame;
use crate::common;
use crate::log;
use crate::log_return_void;
use crate::gameimage;

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>
  , title: &str
  , msg_prev: common::Msg
  , msg_next: common::Msg)
{
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
  // Game name
  //
  let mut input_name = Input::default()
    .with_size(frame_content.w() - dimm::border()*2, dimm::height_button_wide())
    .below_of(&frame_content, 0)
    .with_align(Align::Top | Align::Left);
  input_name.set_pos(frame_content.x() + dimm::border()
    , input_name.y() - input_name.h() - dimm::border());
  let _ = input_name.take_focus();

  // Sanitize game name
  let f_sanitize = |input : String| -> String
  {
    input
      .chars()
      .filter_map(|c|
      {
        if c.is_alphanumeric() { Some(c) }
        else if c == '-' { Some(c) }
        else if c == '_' { Some(c) }
        else if c == ' ' { Some('-') }
        else { None }
      })
      .collect()
  };

  // // Check if GIMG_NAME exists
  if let Some(mut env_name) = env::var("GIMG_NAME").ok()
  {
    env_name = f_sanitize(env_name);
    env::set_var("GIMG_NAME", &env_name);
    input_name.set_value(&env_name);
  } // if

  // // Set input_name callback
  input_name.set_callback(move |e|
  {
    env::set_var("GIMG_NAME", f_sanitize(e.value()));
  });

  // Callback to previous
  ret_frame_footer.btn_prev.clone().emit(tx, msg_prev);

  // Callback to Next
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  let clone_tx = tx.clone();
  let clone_msg_next = msg_next.clone();
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
      log!("Could not fetch GIMG_NAME");
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
      log!("Could not fetch GIMG_PLATFORM");
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
      log!("Could not fetch GIMG_IMAGE");
      return;
    }; // else

    // Init project
    clone_tx.send_awake(common::Msg::WindDeactivate);
    std::thread::spawn(move ||
    {
      match gameimage::init::init(name, platform, image)
      {
        Ok(_) => (),
        Err(e) => { clone_tx.send_awake(common::Msg::WindActivate); log_return_void!("{}", e); }
      } // match

      // Go to next frame
      clone_tx.send_awake(clone_msg_next);
    });
  });
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
