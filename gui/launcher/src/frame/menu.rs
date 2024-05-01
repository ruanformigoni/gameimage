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
use crate::svg;
use common::Msg;

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
  scroll.widget_mut().set_frame(FrameType::BorderBox);
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
      .with_label(label);
    clone_scroll.add(&mut entry.as_base_widget());
    entry
  };

  let mut btn_env = f_make_entry("Environment");
  btn_env.emit(tx, Msg::DrawEnv);

  let mut btn_executables = f_make_entry("Executable Configuration");
  btn_executables.emit(tx, Msg::DrawExecutables);

  scroll.end();

  // Back to home
  let mut btn_home = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .with_align(Align::Inside | Align::Center)
    .center_x(&frame);
  btn_home.set_pos(btn_home.x(), frame.h() - dimm::bar());
  btn_home.set_frame(FrameType::BorderBox);
  btn_home.set_label_size(dimm::height_text()*2);
  btn_home.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_home(1.0).as_str()).unwrap()));
  btn_home.emit(tx, Msg::DrawCover);


  RetFrameSelector{ frame }
} // fn: new }}}

// fn: from {{{
#[allow(dead_code)]
pub fn from(tx : Sender<Msg>, w : Widget) -> RetFrameSelector
{
  new(tx, w.x(), w.y())
} // fn: from }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
