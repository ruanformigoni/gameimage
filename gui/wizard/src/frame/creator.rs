#![allow(warnings)]

use std::sync::{Arc,Mutex};
use std::env;
use std::path::PathBuf;
use std::fs;
use std::ffi::OsStr;

// Gui
use fltk::prelude::*;
use fltk::{
  widget::Widget,
  app::Sender,
  text::{TextBuffer,TextDisplay},
  menu::MenuButton,
  button::{Button,CheckButton},
  group::{Group, Scroll},
  image::SharedImage,
  input::FileInput,
  output::MultilineOutput,
  group::PackType,
  frame::Frame,
  output::Output,
  dialog,
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
use crate::common::PathBufExt;
use crate::common::OsStrExt;
use crate::common::WidgetExtExtra;
use crate::log;
use crate::db;
use crate::lib::download;
use crate::lib::svg;

// fn create_entry() {{{
fn create_entry(project : db::project::Entry
  , parent_base : Widget
  , parent_curr : Widget) -> anyhow::Result<(Widget, CheckButton, PathBuf)>
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
  if let Some(path_file_icon) = project.path_file_icon.clone()
  {
    let path_file_resized = PathBuf::from(path_file_icon.clone())
      .parent()
      .unwrap()
      .join("icon.creator.resized.png");
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
  // let buffer = TextBuffer::default();
  let buffer = TextBuffer::default();
  let mut frame_info = TextDisplay::default()
    .with_size(parent_base.w() - dimm::border()*3 - dimm::height_button_rec()*2, dimm::height_button_rec()*3)
    .right_of(&frame_icon, dimm::border());
  frame_info.set_frame(FrameType::BorderBox);
  frame_info.set_buffer(buffer.clone());
  frame_info.set_color(Color::Background);

  let mut f_add_entry = |title: &str, field : Option<String>, push_newline: bool|
  {
    if let Some(value) = field
    {
      frame_info.insert(title);
      frame_info.insert(value.as_str());
      if push_newline { frame_info.insert("\n"); }
    }
  }; // f_add_entry

  f_add_entry("Platform: "
    , project.platform
    , true
  );
  f_add_entry("Default rom: "
    , project.path_file_rom.clone().and_then(|e|
      {
        e.file_name().map(|e|{ e.string() })
      })
    , true
  );
  f_add_entry("Default core: "
    , project.path_file_core.clone().and_then(|e|
      {
        e.file_name().map(|e|{ e.string() })
      })
    , true
  );
  f_add_entry("Default bios: "
    , project.path_file_bios.clone().and_then(|e|
      {
        e.file_name().map(|e|{ e.string() })
      })
    , false
  );

  //
  // Set checkbox
  //
  let mut btn_checkbox = CheckButton::default()
    .with_size(dimm::width_button_rec() / 2, dimm::height_button_rec() / 2)
    .right_bottom_of(&frame_info, - dimm::border() / 2);
  btn_checkbox.visible_focus(false);

  Ok((
      frame_icon.as_base_widget()
    , btn_checkbox
    , project.path_dir_self.ok_or(ah!("Could not read project directory"))?
  ))
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

  if let Err(e) = common::common()
  {
    log!("Err: {}", e.to_string());
  } // if

  // Configure bottom buttons
  let clone_tx = tx.clone();
  ret_frame_footer.btn_prev.clone().set_callback(move |_|
  {
    if dialog::choice_default("This will reset the image, are you sure?", "No", "Yes", "") == 1
    {
      clone_tx.send(common::Msg::DrawFetch);
    } // if
  });

  // Finish package creation on click next
  ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    if dialog::choice_default("Finish image creation?", "No", "Yes", "") == 1
    {
      clone_tx.send(common::Msg::DrawDesktop);
    } // if
  });

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
  let vec_btn_checkbox = Arc::new(Mutex::new(Vec::<(CheckButton,PathBuf)>::new()));
  if let Ok(projects) = db::project::list()
  {
    let mut parent = scroll.as_base_widget();

    for project in projects
    {
      if   let Ok(( parent_new,  checkbutton , path_dir_project )) =
              create_entry(project.clone(), scroll.clone().as_base_widget(), parent.clone())
        && let Ok(mut lock) = vec_btn_checkbox.lock()
      {
        lock.push((checkbutton, path_dir_project));
        parent = parent_new;
      }
    } // for
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
  btn_add.emit(tx, common::wizard_by_platform().unwrap_or(common::Msg::DrawCreator));

  // Erase package
  let mut btn_del = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .below_of(&btn_add, dimm::border());
  btn_del.set_frame(FrameType::RoundedFrame);
  btn_del.visible_focus(false);
  btn_del.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_del(1.0).as_str()).unwrap()));
  btn_del.set_color(Color::Red);
  let clone_vec_checkbutton = vec_btn_checkbox.clone();
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  let clone_tx = tx.clone();
  btn_del.set_callback(move |_|
  {
    if dialog::choice_default("Erase the selected projects?", "No", "Yes", "") != 1
    {
      return;
    } // if

    let lock = clone_vec_checkbutton.lock();

    if lock.is_err()
    {
      clone_output_status.set_value("Could not acquire lock for checkbutton");
    } // if

    // Remove all currently selected projects
    for (checkbutton, path_dir_project) in lock.unwrap().iter()
    {
      if checkbutton.is_checked()
      {
        fs::remove_file(path_dir_project.with_extension("dwarfs"));
        fs::remove_dir_all(path_dir_project);
      }
    } // for

    // Refresh
    clone_tx.send(common::Msg::DrawCreator);
  });

  // Include inside image
  let mut btn_insert = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .below_of(&btn_del, dimm::border());
  btn_insert.set_color(Color::Blue);
  btn_insert.set_frame(FrameType::RoundedFrame);
  btn_insert.visible_focus(false);
  btn_insert.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_box_heart(1.0).as_str()).unwrap()));
  let clone_vec_checkbutton = vec_btn_checkbox.clone();
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  let clone_tx = tx.clone();
  btn_insert.set_callback(move |_|
  {
    if dialog::choice_default("Include selected projects in the image?", "No", "Yes", "") != 1
    {
      return;
    } // if

    // Set status
    clone_output_status.set_value("Inserting projects in the image");

    // Disable window
    clone_tx.send(common::Msg::WindDeactivate);

    // Update GUI
    fltk::app::flush();
    fltk::app::awake();

    // Include files in new thread
    let clone_vec_checkbutton = vec_btn_checkbox.clone();
    let mut clone_output_status = ret_frame_footer.output_status.clone();
    std::thread::spawn(move ||
    {
      let lock = clone_vec_checkbutton.lock();

      if lock.is_err()
      {
        clone_output_status.set_value("Could not acquire lock for checkbutton");
      } // if

      // Remove all currently selected projects
      for (checkbutton, path_dir_project) in lock.unwrap().iter()
      {
        if ! checkbutton.is_checked()
        {
          continue;
        } // if

        let path_file_dwarfs = path_dir_project.with_extension("dwarfs");
        log!("File: {}", path_file_dwarfs.string());

        // Wait for message & check return value
        if common::gameimage_sync(vec!["package", &path_file_dwarfs.string()]) != 0
        {
          log!("Could not include {} into the image", path_file_dwarfs.string());
        } // if
      } // for

      // Refresh
      clone_tx.send(common::Msg::WindActivate);
      clone_tx.send(common::Msg::DrawCreator);
    });
  });

}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
