use std::env;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  input::Input,
  frame::Frame,
  enums::{Align,FrameType},
};

use anyhow::anyhow as ah;

use shared::fltk::SenderExt;
use shared::svg;
use shared::{column,row,fixed};

use crate::dimm;
use crate::frame;
use crate::common;
use crate::log_status;
use crate::gameimage;

// fn name_next() {{{
fn name_next() -> anyhow::Result<()>
{
  // Check for name
  let name = match env::var("GIMG_NAME")
  {
    Ok(name) => name,
    Err(e) => return Err(ah!("Could not fetch GIMG_NAME: {}", e)),
  };
  // Check for platform
  let platform = match frame::platform::PLATFORM.lock()
  {
    Ok(guard) => match guard.clone()
    {
      Some(platform) => platform,
      None => return Err(ah!("No platform selected")),
    },
    Err(e) => return Err(ah!("Could not lock platform: {}", e)),
  };
  // Init project
  match gameimage::init::project(name, platform.as_str().to_string())
  {
    Ok(_) => (),
    Err(e) => return Err(ah!("Could not init project: {}", e)),
  } // match

  Ok(())
} // fn name_next() }}}

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>
  , title: &str
  , msg_prev: common::Msg
  , msg_next: common::Msg)
{
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);
  // Layout
  column!(col,
    col.add(&Frame::default());
    row!(row_icon,
      row_icon.add(&Frame::default());
      fixed!(row_icon, frame_icon, Frame::default(), 225);
      row_icon.add(&Frame::default());
    );
    col.fixed(&row_icon, 150);
    col.add(&Frame::default());
    fixed!(col, input_name, Input::default(), dimm::height_button_wide());
  );
  // Configure icon box
  let mut frame_icon = frame_icon.clone();
  frame_icon.set_frame(FrameType::NoBox);
  frame_icon.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_joystick(10.0).as_str()).unwrap()));
  // Game name
  let mut input_name = input_name.clone()
    .with_align(Align::Top | Align::Left);
  input_name.set_pos(ui.group.x(), input_name.y() - input_name.h());
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
        else if c == ':' { Some('-') }
        else if c == ' ' { Some('-') }
        else { None }
      })
      .collect()
  };
  // Check if GIMG_NAME exists
  let env_name = f_sanitize(env::var("GIMG_NAME").unwrap_or_default());
  env::set_var("GIMG_NAME", &env_name);
  input_name.set_value(&env_name);
  // Set input_name callback
  input_name.handle(move |input,ev|
  {
    if ev == fltk::enums::Event::KeyUp
    {
      env::set_var("GIMG_NAME", f_sanitize(input.value()));
      return true;
    } // if
    return false;
  });
  // Callback to previous
  ui.btn_prev.clone().emit(tx, msg_prev);
  // Callback to Next
  let clone_tx = tx.clone();
  let clone_msg_next = msg_next.clone();
  ui.btn_next.clone().set_callback(move |_|
  {
    clone_tx.send_awake(common::Msg::WindDeactivate);
    std::thread::spawn(move ||
    {
      match name_next()
      {
        Ok(()) => tx.send_activate(clone_msg_next),
        Err(e) => { clone_tx.send_awake(common::Msg::WindActivate); log_status!("{}", e); }
      }
    });
  });
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
