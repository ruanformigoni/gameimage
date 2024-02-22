#![allow(warnings)]

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::ffi::OsStr;

// Gui
use fltk::prelude::*;
use fltk::{
  widget::Widget,
  app::Sender,
  text::{TextBuffer,TextDisplay},
  menu::MenuButton,
  button::Button,
  group::{Group, Scroll},
  image::SharedImage,
  input::FileInput,
  group::PackType,
  frame::Frame,
  output::Output,
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

// fn create_entries() {{{
fn create_entries(entry : db::project::Entry
  , parent_base : Widget
  , parent_curr : Widget) -> Widget
{
  let mut frame_icon = Frame::default()
    .with_size(dimm::height_button_rec()*2, dimm::height_button_rec()*3)
    .below_of(&parent_curr, dimm::border());
  frame_icon.set_frame(FrameType::BorderBox);

  // Adjust position if parent is base
  if parent_curr.is_same(&parent_base.as_base_widget())
  {
    frame_icon.set_pos(parent_base.x() + dimm::border(), parent_base.y() + dimm::border());
  } // if

  //
  // Set icon
  //
  if let Some(path_file_icon) = entry.path_file_icon.clone()
  {
    let path_file_resized = PathBuf::from(path_file_icon.clone())
      .parent()
      .unwrap()
      .join("icon.resized.png");
    common::image_resize(path_file_resized.clone(), path_file_icon, frame_icon.w() as u32, frame_icon.h() as u32);
    if let Ok(mut image) = fltk::image::SharedImage::load(path_file_resized)
    {
      image.scale(frame_icon.w(), frame_icon.h(), true, true);
      frame_icon.set_image_scaled(Some(image));
    } // if
  } // if

  //
  // Set Info
  //
  let buffer = TextBuffer::default();
  let mut frame_info = TextDisplay::default()
    .with_size(parent_base.w() - dimm::border()*3 - dimm::height_button_rec()*2, dimm::height_button_rec()*3)
    .right_of(&frame_icon, dimm::border());
  frame_info.set_frame(FrameType::BorderBox);
  frame_info.set_buffer(buffer.clone());

  let f_add_entry = |title: &str, entry : Option<String>, push_newline: bool|
  {
    if let Some(value) = entry
    {
      frame_info.insert(title);
      frame_info.insert(value.as_str());
      if push_newline { frame_info.insert("\n"); }
    }
  }; // f_add_entry

  f_add_entry("Platform: "
    , entry.platform
    , true
  );
  f_add_entry("Default rom: "
    , entry.path_file_rom.clone().map(|e|{ common::osstr_to_str(e.file_name()) })
    , true
  );
  f_add_entry("Default core: "
    , entry.path_file_core.clone().map(|e|{ common::osstr_to_str(Some(e.as_os_str())) })
    , true
  );
  f_add_entry("Default bios: "
    , entry.path_file_bios.clone().map(|e|{ common::osstr_to_str(Some(e.as_os_str())) })
    , false
  );

  frame_icon.as_base_widget()
} // }}}

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

  // Set previous frame
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), common::Msg::DrawFetch);

  // List of currently built packages
  let mut frame_list = Frame::default()
    .with_size(frame_content.width() - dimm::border()*3 - dimm::width_button_rec()
      , frame_content.height() - dimm::border()*2)

    .with_pos(frame_content.x() + dimm::border(), frame_content.y() + dimm::border());
  frame_list.set_frame(FrameType::BorderBox);

  let mut scroll = Scroll::default()
    .with_size(frame_list.w(), frame_list.h())
    .with_pos(frame_list.x(), frame_list.y());
  scroll.set_frame(FrameType::BorderBox);
  scroll.begin();

  // Populate entries
  if let Ok(results) = db::project::get()
  {
    let mut parent = scroll.as_base_widget();

    for result in results
    {
      parent = create_entries(result.clone(), scroll.clone().as_base_widget(), parent.clone());
    } // if
  } // if

  scroll.end();

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
  ret_frame_footer.btn_next.clone().hide();
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
