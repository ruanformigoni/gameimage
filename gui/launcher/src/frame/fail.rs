use fltk::{
  group::PackType,
  prelude::WidgetExt,
  enums::FrameType,
  frame::Frame,
};

use shared::dimm;

pub struct RetFrameBase
{
  pub frame_base : Frame,
} // Ret


// fn: new {{{
pub fn new(width : i32, height : i32, border : i32) -> RetFrameBase
{
  let mut frame_base = Frame::default().with_size(width, height);
  frame_base.set_type(PackType::Vertical);
  frame_base.set_frame(FrameType::FlatBox);

  Frame::default()
    .with_size(width - border*2, dimm::height_text())
    .with_pos(border, height / 2 - dimm::height_text() / 2)
    .with_label("No game found inside this image");

  RetFrameBase{ frame_base }
} // fn: new }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
