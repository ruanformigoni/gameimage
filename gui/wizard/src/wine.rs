use std::path::PathBuf;

use fltk::{
  button::Button,
  group::{Group, PackType},
  prelude::{GroupExt, WidgetExt},
  window::Window,
  enums::{FrameType,Color},
  frame::Frame,
  misc::Progress,
};

use url as Url;

use crate::common;
use crate::dimm;
use crate::download;

pub fn frame_select_executable(mut wind : Window)
{
  wind.clear();
  wind.begin();

  let mut group = Group::default().with_size(dimm::WIDTH, dimm::HEIGHT);
  group.set_frame(FrameType::FlatBox);

  let mut frame = Frame::default()
    .with_size(dimm::WIDTH, dimm::HEIGHT)
    .with_label("");
  frame.set_frame(FrameType::NoBox);
  frame.set_type(PackType::Vertical);

  // let radio_button = RadioButton::new(dimm::BORDER,dimm::BORDER,60,40,"Radio gaga");

  group.end();

  wind.end();
}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
