// Gui
use fltk::prelude::*;
use fltk::{
  output::Output,
  group::PackType,
  button::Button,
  frame::Frame,
  enums::{Align,FrameType,Color},
};

use shared::fltk::WidgetExtExtra;

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
  let tx = crate::GUI.lock().unwrap().tx.clone();

  let mut frame = Frame::default()
    .with_size(dimm::width_wizard(), dimm::height_header())
    .with_pos(0,0);
  frame.set_type(PackType::Vertical);

  let mut row = fltk::group::Flex::default()
    .with_size(frame.w() - dimm::border()*2, dimm::height_button_wide())
    .with_pos(dimm::border(), dimm::border())
    .with_type(fltk::group::FlexType::Row);
  // Show terminal button
  let mut btn_term = shared::fltk::button::rect::terminal()
    .with_color(Color::Blue);
  btn_term.emit(tx, crate::common::Msg::ToggleTerminal);
  row.fixed(&btn_term, btn_term.w());
  // Header
  let mut header = Frame::default()
    .with_size(0, dimm::height_button_wide())
    .with_align(Align::Inside | Align::Center)
    .with_label(title);
  header.set_label_size((dimm::height_text() as f32 * 1.5) as i32);
  row.add_resizable(&header);
  row.end();

  // Separator
  let mut sep = Frame::default()
    .with_size(dimm::width_wizard() - dimm::border()*2, dimm::height_sep())
    .below_of(&row, dimm::border());
  sep.set_frame(FrameType::BorderBox);

  let mut frame_content = Frame::default()
    .with_size(dimm::width_wizard(), dimm::height_wizard() - dimm::height_header() - dimm::height_footer())
    .below_of(&sep, 0);
  frame_content.set_pos(0, frame_content.y());

  RetFrameHeader{ frame, frame_content, header, sep }
} // }}}

// pub fn frame_footer() {{{
pub fn frame_footer() -> RetFrameFooter
{
  let mut frame = Frame::default()
    .with_size(dimm::width_wizard(), dimm::height_footer())
    .with_pos(0, dimm::posy_footer());
  // frame.set_color(Color::Green);
  frame.set_type(PackType::Vertical);

  // Status bar
  let mut output_status = Output::default()
    .with_size(dimm::width_wizard(), dimm::height_status())
    .with_align(Align::Left)
    .with_pos(0, dimm::height_wizard() - dimm::height_status());
  output_status.set_text_size(dimm::height_text());
  output_status.deactivate();

  // Continue
  let mut btn_next = Button::default()
    .with_size(dimm::width_button_wide(), dimm::height_button_wide())
    .with_label("Next")
    .above_of(&output_status, dimm::border());
  btn_next.set_pos(dimm::width_wizard() - dimm::width_button_wide() - dimm::border(), btn_next.y());
  btn_next.set_color(Color::Blue);

  // Prev
  let mut btn_prev = Button::default()
    .with_size(dimm::width_button_wide(), dimm::height_button_wide())
    .with_label("Prev")
    .above_of(&output_status, dimm::border());
  btn_prev.set_pos(dimm::border(), btn_prev.y());
  btn_prev.set_color(Color::Background);

  // Separator
  let mut sep = Frame::default()
    .above_of(&btn_next, dimm::border())
    .with_size(dimm::width_wizard() - dimm::border()*2, dimm::height_sep());
  sep.set_frame(FrameType::BorderBox);
  sep.set_pos(dimm::border(), sep.y());

  RetFrameFooter { frame, output_status, btn_next, btn_prev, sep }

} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
