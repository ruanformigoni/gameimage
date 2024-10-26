use fltk::prelude::*;
use fltk::{
  app::Sender,
  widget::Widget,
  button::Button,
  group::PackType,
  enums::{Align,FrameType},
  frame::Frame,
};

use shared::dimm;
use shared::fltk::WidgetExtExtra;

use crate::common;
use common::Msg;

pub mod executables;
pub mod environment;

pub struct RetFrameSelector
{
  pub frame : Frame,
} // Ret

// fn: new {{{
pub fn new(tx : Sender<Msg>, x : i32, y : i32) -> RetFrameSelector
{
  //
  // Main
  //
  let mut frame = Frame::default()
    .with_size(dimm::width_launcher(), dimm::height_launcher())
    .with_pos(x, y);
  frame.set_type(PackType::Vertical);
  frame.set_frame(FrameType::FlatBox);

  let mut frame_title = Frame::default()
    .with_label("Menu")
    .with_size(frame.width() - dimm::border()*2, dimm::height_button_rec() / 2)
    .with_pos(dimm::border(), dimm::border());
  frame_title.set_frame(FrameType::FlatBox);
  frame_title.set_label_size(dimm::height_text());

  // Create scrollbar
  let mut scroll = shared::fltk::ScrollList::new(
    frame.w() - dimm::border()*2
    , frame.h() - dimm::bar() - frame_title.h() - dimm::border()*3
    , frame_title.x()
    , frame_title.y() + frame_title.h() + dimm::border()
  );
  scroll.set_frame(FrameType::BorderBox);
  // scroll.widget_mut().set_color(fltk::enums::Color::Blue);
  scroll.set_border(dimm::border(), dimm::border());

  //
  // Layout
  //
  let mut clone_scroll = scroll.clone();
  let mut f_make_entry = move |label : &str|
  {
    let entry = Button::default()
      .with_size(clone_scroll.widget_ref().width() - dimm::border()*2, dimm::height_button_wide())
      .with_frame(FrameType::BorderBox)
      .with_align(Align::Left | Align::Inside)
      .with_focus(false)
      .with_label(label);
      // .with_color(fltk::enums::Color::Yellow);
    clone_scroll.add(&mut entry.as_base_widget());
    entry
  };

  let mut btn_env = f_make_entry("Environment");
  btn_env.emit(tx, Msg::DrawEnv);

  // Enable executable list only for wine
  if let Ok(str_platform) = std::env::var("GIMG_PLATFORM")
  && let Ok(platform) = common::Platform::from_str(&str_platform)
  && platform == common::Platform::WINE
  {
    let mut btn_executables = f_make_entry("Executable Configuration");
    btn_executables.emit(tx, Msg::DrawExecutables);
  }

  scroll.end();

  // Back to home
  shared::fltk::button::rect::home()
    .bottom_center_of(&frame, - dimm::border())
    .emit(tx, Msg::DrawCover);

  RetFrameSelector{ frame }
} // fn: new }}}

// fn: from {{{
#[allow(dead_code)]
pub fn from(tx : Sender<Msg>, w : Widget) -> RetFrameSelector
{
  new(tx, w.x(), w.y())
} // fn: from }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
