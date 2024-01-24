use std::env;

use fltk::prelude::*;
use fltk::{
  app::Sender,
  widget::Widget,
  button::Button,
  group::{PackType,Scroll},
  enums::{Align,FrameType},
  frame::Frame,
};

use crate::dimm;
use crate::mounts;
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
      .with_size(parent.width(), dimm::height_button_wide());
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

  // Populate entries
  if let Ok(vec_pairs) = mounts::mounts()
  {
    for (path_boot, path_root, path_icon, path_icon_grayscale) in vec_pairs
    {
      // Set entry name
      let osstr_name_file = path_root.file_name().unwrap_or(std::ffi::OsStr::new(""));
      let str_name_file = osstr_name_file.to_str().unwrap_or("");
      let mut entry = f_make_entry(str_name_file);
      entry.set_callback(move |_|
      {
        env::set_var("GIMG_LAUNCHER_BOOT", path_boot.to_str().unwrap_or(""));
        env::set_var("GIMG_LAUNCHER_ROOT", path_root.to_str().unwrap_or(""));
        env::set_var("GIMG_LAUNCHER_IMG", path_icon.to_str().unwrap_or(""));
        env::set_var("GIMG_LAUNCHER_IMG_GRAYSCALE", path_icon_grayscale.to_str().unwrap_or(""));
        tx.send(Msg::DrawCover);
      });
    } // for
  } // if

  scroll.end();

  // Back to home
  let mut btn_home = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .with_align(Align::Inside | Align::Center)
    .center_x(&frame);
  btn_home.set_pos(btn_home.x(), frame.h() - dimm::bar());
  btn_home.set_frame(FrameType::BorderBox);
  btn_home.set_label_size(dimm::height_text()*2);
  btn_home.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_home().as_str()).unwrap()));
  btn_home.emit(tx, Msg::DrawCover);

  // Back to menu
  let mut btn_back = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .with_align(Align::Inside | Align::Center);
  btn_back.set_pos(dimm::border(), frame.h() - dimm::bar());
  btn_back.set_frame(FrameType::BorderBox);
  btn_back.set_label_size(dimm::height_text()*2);
  btn_back.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_back().as_str()).unwrap()));
  btn_back.emit(tx, Msg::DrawMenu);


  RetFrameSelector{ frame }
} // fn: new }}}

// fn: from {{{
#[allow(dead_code)]
pub fn from(tx : Sender<Msg>, w : Widget) -> RetFrameSelector
{
  new(tx, w.x(), w.y())
} // fn: from }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
