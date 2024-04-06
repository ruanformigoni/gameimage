use std::env;
use std::path::PathBuf;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  button::Button,
  group::PackType,
  frame::Frame,
  enums::{FrameType,Color},
};

use crate::dimm;
use crate::frame;
use crate::common;
use crate::log;
use crate::common::PathBufExt;
use crate::common::FltkSenderExt;

// pub fn test() {{{
pub fn test(tx: Sender<common::Msg>
  , title: &str
  , msg_prev: common::Msg
  , _msg_curr: common::Msg
  , msg_next: common::Msg)
{
  let mut frame = Frame::default()
    .with_size(dimm::width(), dimm::height());
  frame.set_frame(FrameType::BorderBox);
  frame.set_type(PackType::Vertical);

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

  // Set previous/next frame
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), msg_prev);
  ret_frame_footer.btn_next.clone().emit(tx.clone(), msg_next);

  // Add a 'test' button
  let mut btn_test = Button::default()
    .with_size(dimm::width_button_wide(), dimm::height_button_wide())
    .with_label("Test");
  btn_test.set_label_size(dimm::height_text());
  btn_test.clone().center_x(&frame_content);
  btn_test.set_pos(btn_test.x(), ret_frame_footer.btn_next.y());
  btn_test.set_color(Color::Green);

  let mut term = frame::term::Term::new(dimm::border()
    , frame_content.w() - dimm::border()*2
    , frame_content.h() - dimm::border()*2
    , frame_content.x() + dimm::border()
    , frame_content.y() + dimm::border());

  let clone_tx = tx.clone();
  btn_test.set_callback(move |_|
  {
    clone_tx.send_awake(common::Msg::WindDeactivate);

    let path_gimg_backend = if let Ok(var) = env::var("GIMG_BACKEND")
    {
      PathBuf::from(var)
    } // if
    else
    {
      log!("Could not fetch GIMG_BACKEND var");
      return;
    }; // else

    let _ = term.dispatch(vec![path_gimg_backend.string().as_str(), "test"]
      , move |_|
      {
        clone_tx.send_awake(common::Msg::WindActivate);
      }
    );
  });
} // fn test() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
