#![allow(warnings)]

use std::env;
use std::path::PathBuf;
use std::fs::File;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  browser::MultiBrowser,
  text::{TextBuffer,TextDisplay},
  menu::MenuButton,
  button::Button,
  group::Group,
  image::SharedImage,
  input::FileInput,
  group::PackType,
  frame::Frame,
  dialog,
  text,
  output,
  dialog::{dir_chooser,file_chooser},
  enums::{Align,FrameType,Color},
  misc::Progress,
};

use url as Url;
use anyhow;
use anyhow::anyhow as ah;

use crate::dimm;
use crate::frame;
use crate::wizard;
use crate::common;
use crate::common::PathBufExt;
use crate::common::OsStrExt;
use crate::common::WidgetExtExtra;
use crate::log;
use crate::db;
use crate::lib::download;
use crate::lib::svg;

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>, title: &str)
{
  wizard::name::name(tx.clone()
    , title
    , common::Msg::DrawCreator
    , common::Msg::DrawRetroarchIcon);
} // }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>, title: &str)
{
  wizard::icon::icon(tx.clone()
    , title
    , common::Msg::DrawRetroarchName
    , common::Msg::DrawRetroarchIcon
    , common::Msg::DrawRetroarchRom
  );
} // }}}

// pub fn rom() {{{
pub fn rom(tx: Sender<common::Msg>, title: &str)
{
  let install = wizard::install::install(tx.clone()
    , title
    , "rom"
    , common::Msg::DrawRetroarchIcon
    , common::Msg::DrawRetroarchRom
    , common::Msg::DrawRetroarchCore);

  let mut frame_list = install.frame_list.clone();
  let output_status = install.ret_frame_footer.output_status.clone();
  let btn_add = install.btn_add.clone();
  let mut btn_del = install.btn_del.clone();

  // Adjust to include the field below
  frame_list.set_size(frame_list.w(), frame_list.h() - dimm::border() - dimm::height_button_wide());

  // Set label for output field
  let mut output_default_label = text::TextDisplay::default()
    .with_width(frame_list.w() / 4)
    .with_height(dimm::height_button_wide())
    .below_of(&frame_list, dimm::border())
    .with_color(Color::BackGround)
    .with_frame(FrameType::NoBox);
  output_default_label.set_buffer(text::TextBuffer::default());
  output_default_label.insert("Default rom:");

  // Show default item below all items
  let mut output_default = text::TextDisplay::default()
    .with_width(frame_list.w() - (frame_list.w() / 4))
    .with_height(dimm::height_button_wide())
    .right_of(&output_default_label, 0)
    .with_color(Color::BackGround)
    .with_frame(FrameType::NoBox);
  output_default.set_buffer(text::TextBuffer::default());
  if let Ok(project) = db::project::current()
  && let Some(path_file_rom) = project.path_file_rom
  {
    output_default.insert(&path_file_rom.file_name_string());
  } // if

  // Update default rom
  let clone_frame_list = frame_list.clone();
  let mut clone_output_status = output_status.clone();
  let clone_tx = tx.clone();
  let btn_default = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .below_of(&btn_add, dimm::border())
    .with_frame(FrameType::RoundedFrame)
    .with_focus(false)
    .with_svg(svg::icon_check(1.0).as_str())
    .with_color(Color::Blue)
    .with_callback(move |_|
    {
      let vec_indices = clone_frame_list.selected_items();

      if vec_indices.len() == 0
      {
        clone_output_status.set_value("No item selected to set as default");
        return;
      } // if

      if vec_indices.len() != 1
      {
        clone_output_status.set_value("Only one item can be set as the default");
        return;
      } // if

      let selected = match clone_frame_list.text(*vec_indices.first().unwrap())
      {
        Some(item) => item,
        None => return,
      }; // match

      clone_tx.send(common::Msg::WindDeactivate);
      std::thread::spawn(move ||
      {
        if common::gameimage_sync(vec!["select", "rom", selected.as_str()]) != 0
        {
          log!("Could not select rom file '{}'", selected);
        }; // else

        clone_tx.send(common::Msg::DrawRetroarchRom);
      }); // std::thread
    }); // with_callback

    btn_del = btn_del.below_of(&btn_default, dimm::border());
} // }}}

// pub fn core() {{{
pub fn core(tx: Sender<common::Msg>, title: &str)
{
  let install = wizard::install::install(tx.clone()
    , title
    , "core"
    , common::Msg::DrawRetroarchRom
    , common::Msg::DrawRetroarchCore
    , common::Msg::DrawRetroarchBios);

  // Clone data from preset
  let mut frame_list_installed = install.frame_list.clone();
  let frame_content = install.ret_frame_header.frame_content.clone();
  let output_status = install.ret_frame_footer.output_status.clone();
  let mut btn_add = install.btn_add.clone();
  let mut btn_del = install.btn_del.clone();

  // Adjust size
  frame_list_installed.set_size(
      frame_content.width() - dimm::border()*3 - dimm::width_button_rec()
    , frame_content.height() / 2 - ( ( dimm::border()*3 + dimm::height_button_wide() ) / 2 )
  );

  // Set label for output field
  let mut output_default_label = text::TextDisplay::default()
    .with_width(frame_list_installed.w() / 4)
    .with_height(dimm::height_button_wide())
    .center_of(&frame_content)
    .with_posx_of(&frame_list_installed)
    .with_color(Color::BackGround)
    .with_frame(FrameType::NoBox);
  output_default_label.set_buffer(text::TextBuffer::default());
  output_default_label.insert("Default core:");

  // Show default item below all items
  let mut output_default = text::TextDisplay::default()
    .with_width(frame_list_installed.w() - (frame_list_installed.w() / 4))
    .with_height(dimm::height_button_wide())
    .right_of(&output_default_label, 0)
    .with_color(Color::BackGround)
    .with_frame(FrameType::NoBox);
  output_default.set_buffer(text::TextBuffer::default());
  if let Ok(project) = db::project::current()
  && let Some(path_file_core) = project.path_file_core
  {
    output_default.insert(&path_file_core.file_name_string());
  } // if


  // Add new item from file manager
  let clone_tx = tx.clone();
  btn_add.set_callback(move |_|
  {
    // Pick files to install
    let mut chooser = dialog::FileChooser::new("."
      , "*"
      , dialog::FileChooserType::Multi
      , "Pick one or multiple files");

    // Start dialog
    chooser.show();

    // Wait for choice(s)
    while chooser.shown() { fltk::app::wait(); } // while

    // Check if choice is valid
    if chooser.value(1).is_none()
    {
      log!("No file selected");
      return;
    } // if

    // Install files
    clone_tx.send(common::Msg::WindDeactivate);

    // Get items
    let mut vec_items = (1..chooser.count()+1).into_iter().map(|e| chooser.value(e).unwrap()).collect::<Vec<String>>();
    std::thread::spawn(move ||
    {
      // To &str collection
      let mut vec_str_items : Vec<&str> = vec_items.iter().map(|e| &**e).collect();
      // Prepend command
      let mut vec_cmd = vec!["install", "core"];
      vec_cmd.extend_from_slice(&vec_str_items);
      // Install cores
      if common::gameimage_sync(vec_cmd) != 0
      {
        log!("Failed to install one or more cores");
      } // else
      // Redraw window
      clone_tx.send(common::Msg::DrawRetroarchCore);
    });
  });

  // Update default core
  let clone_frame_list_installed = frame_list_installed.clone();
  let mut clone_output_status = output_status.clone();
  let btn_default = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .below_of(&btn_add, dimm::border())
    .with_frame(FrameType::RoundedFrame)
    .with_focus(false)
    .with_svg(svg::icon_check(1.0).as_str())
    .with_color(Color::Blue)
    .with_callback(move |_|
    {
      let vec_indices = clone_frame_list_installed.selected_items();

      if vec_indices.len() == 0
      {
        clone_output_status.set_value("No item selected to set as default");
        return;
      } // if

      if vec_indices.len() != 1
      {
        clone_output_status.set_value("Only one item can be set as the default");
        return;
      } // if

      let selected = match clone_frame_list_installed.text(*vec_indices.first().unwrap())
      {
        Some(item) => item,
        None => return,
      }; // match

      clone_tx.send(common::Msg::WindDeactivate);
      std::thread::spawn(move ||
      {
        if common::gameimage_sync(vec!["select", "core", selected.as_str()]) != 0
        {
          log!("Could not select core file '{}'", selected);
        }; // else

        clone_tx.send(common::Msg::DrawRetroarchCore);
      }); // std::thread
    }); // with_callback

  // Erase package
  let clone_frame_list_installed = frame_list_installed.clone();
  let mut clone_output_status = output_status.clone();
  btn_del
    .below_of(&btn_default, dimm::border())
    .with_callback(move |_|
  {
    let vec_indices = clone_frame_list_installed.selected_items();

    if vec_indices.len() == 0
    {
      clone_output_status.set_value("No item selected for deletion");
      clone_tx.send(common::Msg::DrawRetroarchCore);
      return;
    } // if

    // Get items
    let vec_items : Vec<String> = vec_indices.into_iter().map(|e|{ clone_frame_list_installed.text(e).unwrap() }).collect();

    // To &str collection
    let mut vec_str_items : Vec<&str> = vec_items.iter().map(|e| &**e).collect();

    // Prepend command
    let mut vec_cmd = vec!["install", "--remove", "core"];
    vec_cmd.extend_from_slice(&vec_str_items);

    // Run backend
    if common::gameimage_sync(vec_cmd) != 0
    {
      log!("Failed to delete one or more items");
    }; // else
    
    // Redraw
    clone_tx.send(common::Msg::DrawRetroarchCore);
  });

  // List of items to install
  let mut frame_list_remote = MultiBrowser::default()
    .with_size_of(&frame_list_installed)
    .with_posx_of(&output_default_label)
    .bottom_of(&frame_content, - dimm::border())
    .with_frame(FrameType::BorderBox);
  frame_list_remote.set_text_size(dimm::height_text());

  // Insert remote items in list
  let mut parent = frame_list_remote.as_base_widget();
  if let Ok(vec_items) = wizard::install::fetch_items(tx.clone(), "core".to_string(), true)
  {
    vec_items.iter().for_each(|item| frame_list_remote.add(item.as_str()) );
  } // if

  // Add new item from remote
  let clone_tx = tx.clone();
  let clone_frame_list_remote = frame_list_remote.clone();
  let _btn_install_from_remote = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .right_of(&frame_list_remote, dimm::border())
    .with_frame(FrameType::RoundedFrame)
    .with_focus(false)
    .with_svg(svg::icon_install(1.0).as_str())
    .with_color(Color::Green)
    .with_callback(move |_|
    {
      // Install files
      clone_tx.send(common::Msg::WindDeactivate);
      let clone_frame_list_remote = clone_frame_list_remote.clone();
      std::thread::spawn(move ||
      {
        let vec_indices = clone_frame_list_remote.selected_items();

        // Get items
        let vec_items : Vec<String> = vec_indices.into_iter().map(|e|{ clone_frame_list_remote.text(e).unwrap() }).collect();

        // To &str collection
        let mut vec_str_items : Vec<&str> = vec_items.iter().map(|e| &**e).collect();

        // Prepend command
        let mut vec_cmd = vec!["install", "--remote", "core"];
        vec_cmd.extend_from_slice(&vec_str_items);

        // Install with backend
        if common::gameimage_sync(vec_cmd) != 0
        {
          log!("Failed to execute backend to fetch remote cores");
        } // else

        clone_tx.send(common::Msg::DrawRetroarchCore);
      });
    });
} // }}}

// pub fn bios() {{{
pub fn bios(tx: Sender<common::Msg>, title: &str)
{
  wizard::install::install(tx.clone()
    , title
    , "bios"
    , common::Msg::DrawRetroarchCore
    , common::Msg::DrawRetroarchBios
    , common::Msg::DrawRetroarchTest);
} // }}}

// pub fn test() {{{
pub fn test(tx: Sender<common::Msg>, title: &str)
{
  wizard::test::test(tx.clone()
    , title
    , common::Msg::DrawRetroarchBios
    , common::Msg::DrawRetroarchTest
    , common::Msg::DrawRetroarchCompress);
} // }}}

// pub fn compress() {{{
pub fn compress(tx: Sender<common::Msg>, title: &str)
{
  wizard::compress::compress(tx.clone()
    , title
    , common::Msg::DrawRetroarchTest
    , common::Msg::DrawRetroarchCompress
    , common::Msg::DrawCreator);
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
