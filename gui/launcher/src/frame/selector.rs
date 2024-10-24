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
use shared::fltk::WidgetExtExtra;

use crate::games;
use crate::common;
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
    .with_label("Switch Game")
    .with_size(frame.width() - dimm::border()*2, dimm::height_button_rec() / 2)
    .with_pos(dimm::border(), dimm::border());
  frame_title.set_frame(FrameType::FlatBox);
  frame_title.set_label_size(dimm::height_text());

  // Create scrollbar
  let mut scroll = Scroll::default()
    .with_size(frame.w() - dimm::border()*2, frame.h() - dimm::bar() - frame_title.h() - dimm::border() * 3)
    .with_pos(frame_title.x(), frame_title.y() + frame_title.h() + dimm::border());
  scroll.set_frame(FrameType::BorderBox);


  //
  // Layout
  //
  let mut parent = scroll.as_base_widget();
  let clone_frame_selector = scroll.clone();
  let mut f_make_entry = move |label : &str|
  {
    let mut entry = Button::default()
      .with_size(parent.width(), dimm::height_button_wide())
      .with_focus(false);
    entry.set_type(PackType::Vertical);
    entry.set_frame(FrameType::BorderBox);
    entry.set_label_size(dimm::height_text());
    entry.set_align(Align::Left | Align::Inside);
    entry.set_label(label);
    if parent.is_same(&clone_frame_selector.as_base_widget())
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

  // Create entries
  if let Ok(vec_games) = games::games()
  {
    for game in vec_games
    {
      // Set entry name
      let osstr_name_file = game.path_root.file_name().unwrap_or_default();
      let str_name_file = osstr_name_file.to_str().unwrap_or_default();
      let mut entry = f_make_entry(str_name_file);
      entry.set_callback(move |_|
      {
        games::select(&game);
        tx.send(Msg::DrawCover);
      });
    } // for
  } // if

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
