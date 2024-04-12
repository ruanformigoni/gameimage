use fltk::prelude::*;
use fltk::{
  app::Sender,
  widget::Widget,
  button::Button,
  group::{PackType,Scroll},
  enums::{Align,FrameType},
  frame::Frame,
};

use shared::dimm;

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
    .with_size(dimm::width(), dimm::height())
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
  let mut scroll = Scroll::default()
    .below_of(&frame_title, dimm::border())
    .with_size(frame.w() - dimm::border()*2, frame.h() - dimm::bar() - frame_title.h() - dimm::border()*3);
  scroll.set_scrollbar_size(dimm::width_button_rec() / 4);
  scroll.set_frame(FrameType::BorderBox);

  //
  // Layout
  //
  let mut parent = scroll.as_base_widget();
  let clone_scroll = scroll.clone();
  let mut f_make_entry = move |label : &str|
  {
    let mut entry = Button::default()
      .with_size(parent.width(), dimm::height_button_wide());
    entry.set_type(PackType::Vertical);
    entry.set_frame(FrameType::BorderBox);
    entry.set_label_size(dimm::height_text());
    entry.set_align(Align::Left | Align::Inside);
    entry.set_label(label);
    if parent.is_same(&clone_scroll.as_base_widget())
    {
      entry.clone().above_of(&parent, -entry.h() - dimm::border());
      entry.set_pos(entry.x() + dimm::border(), entry.y());
      entry.set_size(entry.w() - dimm::border()*2, entry.h());
    } // if
    else
    {
      entry.clone().above_of(&parent, -entry.h() * 2 - dimm::border());
    } // else
    parent = entry.as_base_widget().clone();
    entry
  };

  let mut btn_env = f_make_entry("Environment");
  btn_env.emit(tx, Msg::DrawEnv);

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
