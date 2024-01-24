// Gui
use fltk::prelude::*;
use fltk::{
  output::Output,
  group::PackType,
  button::Button,
  frame::Frame,
  enums::{Align,FrameType,Color},
};

use crate::dimm;

#[derive(Clone)]
pub struct RetFrameHeader
{
  pub frame: Frame,
  pub frame_content : Frame,
  pub header: Frame,
  pub sep: Frame,
} // struct RetFrameHeader

#[derive(Clone)]
pub struct RetFrameFooter
{
  pub frame: Frame,
  pub output_status: Output,
  pub btn_next: Button,
  pub btn_prev: Button,
  pub sep: Frame,
} // struct RetFrameHeader

// pub fn frame_header() {{{
pub fn frame_header(title: &str) -> RetFrameHeader
{
  let mut frame = Frame::default()
    .with_size(dimm::width(), dimm::height_header())
    .with_pos(0,0);
  frame.set_type(PackType::Vertical);

  // Header
  let mut header = Frame::new(dimm::border()
    , dimm::border()
    , dimm::width()-dimm::border()*2
    , dimm::height_button_wide()
    , title);
  header.set_frame(FrameType::NoBox);
  header.set_label_size((dimm::height_text() as f32 * 1.5) as i32);

  // Separator
  let mut sep = Frame::default()
    .with_size(dimm::width() - dimm::border()*2, dimm::height_sep())
    .below_of(&header, dimm::border());
  sep.set_frame(FrameType::BorderBox);

  let mut frame_content = Frame::default()
    .with_size(dimm::width(), dimm::height() - dimm::height_header() - dimm::height_footer())
    .below_of(&sep, 0);
  frame_content.set_pos(0, frame_content.y());

  RetFrameHeader{ frame, frame_content, header, sep }
} // }}}

// pub fn frame_footer() {{{
pub fn frame_footer() -> RetFrameFooter
{
  let mut frame = Frame::default()
    .with_size(dimm::width(), dimm::height_footer())
    .with_pos(0, dimm::posy_footer());
  // frame.set_color(Color::Green);
  frame.set_type(PackType::Vertical);

  // Status bar
  let mut output_status = Output::default()
    .with_size(dimm::width(), dimm::height_status())
    .with_align(Align::Left)
    .with_pos(0, dimm::height() - dimm::height_status());
  output_status.set_text_size(dimm::height_text());
  output_status.deactivate();

  // Continue
  let mut btn_next = Button::default()
    .with_size(dimm::WIDTH_BUTTON_WIDE, dimm::HEIGHT_BUTTON_WIDE)
    .with_label("Next")
    .above_of(&output_status, dimm::BORDER);
  btn_next.set_pos(dimm::width() - dimm::WIDTH_BUTTON_WIDE - dimm::BORDER, btn_next.y());
  btn_next.set_color(Color::Blue);

  // Prev
  let mut btn_prev = Button::default()
    .with_size(dimm::WIDTH_BUTTON_WIDE, dimm::HEIGHT_BUTTON_WIDE)
    .with_label("Prev")
    .above_of(&output_status, dimm::BORDER);
  btn_prev.set_pos(dimm::BORDER, btn_prev.y());
  btn_prev.set_color(Color::Background);

  // Separator
  let mut sep = Frame::default()
    .above_of(&btn_next, dimm::BORDER)
    .with_size(dimm::WIDTH - dimm::BORDER*2, dimm::height_sep());
  sep.set_frame(FrameType::BorderBox);
  sep.set_pos(dimm::BORDER, sep.y());

  RetFrameFooter { frame, output_status, btn_next, btn_prev, sep }

} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
