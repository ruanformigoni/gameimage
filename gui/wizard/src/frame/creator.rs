use std::env;
use std::path::PathBuf;
use std::fs::File;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  text::{TextBuffer,TextDisplay},
  menu::MenuButton,
  button::Button,
  group::Group,
  image::SharedImage,
  input::FileInput,
  group::PackType,
  frame::Frame,
  dialog::dir_chooser,
  enums::{Align,FrameType,Color},
  misc::Progress,
};

use url as Url;
use anyhow;
use anyhow::anyhow as ah;

use crate::dimm;
use crate::frame;
use crate::common;
use crate::db;
use crate::download;
use crate::svg;

// pub fn creator() {{{
pub fn creator(tx: Sender<common::Msg>, title: &str)
{
  let mut frame = Frame::default()
    .with_size(dimm::width(), dimm::height());
  frame.set_frame(FrameType::BorderBox);
  frame.set_type(PackType::Vertical);

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_header = ret_frame_header.frame.clone();
  let frame_content = ret_frame_header.frame_content.clone();
  let frame_footer = ret_frame_footer.frame.clone();

  // List of currently built packages
  let mut frame_list = Frame::default()
    .with_size(frame_content.width() - dimm::border()*3 - dimm::width_button_rec()
      , frame_content.height() - dimm::border()*2)

    .with_pos(frame_content.x() + dimm::border(), frame_content.y() + dimm::border());
  frame_list.set_frame(FrameType::BorderBox);

  // Add new package
  let mut btn_add = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .right_of(&frame_list, dimm::border());
  btn_add.set_frame(FrameType::RoundedFrame);
  btn_add.visible_focus(false);
  btn_add.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_add(1.0).as_str()).unwrap()));
  btn_add.set_color(Color::Green);
  btn_add.emit(tx, common::Msg::DrawRetroarchName);

  // Erase package
  let mut btn_del = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .below_of(&btn_add, dimm::border());
  btn_del.set_frame(FrameType::RoundedFrame);
  btn_del.visible_focus(false);
  btn_del.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_del(1.0).as_str()).unwrap()));
  btn_del.set_color(Color::Red);

  // Configure bottom buttons
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), common::Msg::DrawFetch);
  ret_frame_footer.btn_next.clone().hide();
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
