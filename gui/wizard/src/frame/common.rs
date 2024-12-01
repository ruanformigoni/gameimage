// Gui
use fltk::prelude::*;
use fltk::{
  output::Output,
  group::PackType,
  frame::Frame,
  group::Group,
  enums::{Align,Color},
};

use shared::fltk::WidgetExtExtra;

use crate::dimm;
use crate::frame;

// pub fn frame_header() {{{
pub fn frame_header(str_title: &str)
{
  let mut frame = Frame::default()
    .with_size(dimm::width_wizard(), dimm::height_header())
    .with_pos(0,0);
  frame.set_type(PackType::Vertical);

  let mut row = fltk::group::Flex::default()
    .row()
    .with_size(frame.w() - dimm::border()*2, dimm::height_button_wide())
    .with_pos(dimm::border(), dimm::border());
  // Show terminal button
  let mut btn_term = shared::fltk::button::rect::terminal()
    .with_color(Color::Blue);
  row.fixed(&btn_term, btn_term.w());
  // Header
  let mut title = Frame::default()
    .with_id("header_title")
    .with_size(0, dimm::height_button_wide())
    .with_align(Align::Inside | Align::Center)
    .with_label(str_title);
  title.set_label_size((dimm::height_text() as f32 * 1.5) as i32);
  row.add_resizable(&title);
  row.end();

  // Separator
  let mut sep = shared::fltk::separator::horizontal(dimm::width_wizard() - dimm::border()*2)
    .below_of(&row, dimm::border());
  sep.set_pos(dimm::border(), sep.y());

  let (w,h) = (dimm::width_wizard() - dimm::border()*2
      , dimm::height_wizard() - dimm::height_header() - dimm::height_footer() - dimm::border()*2
  );

  let mut group_content = Group::default()
    .with_id("content")
    .with_size(w,h)
    .below_of(&sep, dimm::border());
  let frame_content = Frame::default()
    .with_size(w,h)
    .below_of(&sep, dimm::border());
  group_content.add(&frame_content);
  group_content.end();

  let mut term = frame::term::Term::new_with_id("term_log", 0, w, h, sep.x(), sep.y() + sep.h() + dimm::border());
  term.group.hide();
  let mut clone_term = term.group.clone();
  let mut clone_group_content = group_content.clone();
  btn_term.set_callback(move |_|
  {
    if clone_term.visible()
    {
      clone_term.hide();
      clone_group_content.show();
    } // if
    else
    {
      clone_group_content.hide();
      clone_term.show();
    } // else
  });

} // }}}

// pub fn frame_footer() {{{
pub fn frame_footer()
{
  let mut frame = Frame::default()
    .with_id("footer_frame")
    .with_size(dimm::width_wizard(), dimm::height_footer())
    .with_pos(0, dimm::posy_footer());
  // frame.set_color(Color::Green);
  frame.set_type(PackType::Vertical);

  // Status bar
  let mut output_status = Output::default()
    .with_id("footer_status")
    .with_size(dimm::width_wizard(), dimm::height_status())
    .with_align(Align::Left)
    .with_pos(0, dimm::height_wizard() - dimm::height_status());
  output_status.set_text_size(dimm::height_text());

  // Continue
  let mut btn_next = shared::fltk::button::wide::default()
    .with_id("footer_next")
    .with_label("Next")
    .above_of(&output_status, dimm::border());
  btn_next.set_pos(dimm::width_wizard() - dimm::width_button_wide() - dimm::border(), btn_next.y());
  btn_next.set_color(Color::Blue);

  // Prev
  let mut btn_prev = shared::fltk::button::wide::default()
    .with_id("footer_prev")
    .with_label("Prev")
    .above_of(&output_status, dimm::border());
  btn_prev.set_pos(dimm::border(), btn_prev.y());

  // Separator
  let mut sep = shared::fltk::separator::horizontal(dimm::width_wizard() - dimm::border()*2)
    .above_of(&btn_next, dimm::border());
  sep.set_pos(dimm::border(), sep.y());
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
