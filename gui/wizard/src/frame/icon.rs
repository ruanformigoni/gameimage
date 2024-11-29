use std::env;
use std::path::PathBuf;
use std::sync::{Arc,Mutex};
use std::borrow::BorrowMut;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  menu,
  button,
  input,
  input::FileInput,
  frame::Frame,
  dialog::file_chooser,
  enums::{Align,FrameType,Color},
};

use shared::fltk::WidgetExtExtra;

use anyhow::anyhow as ah;

use shared::fltk::SenderExt;
use shared::std::PathBufExt;

use crate::dimm;
use crate::db;
use crate::frame;
use crate::common;
use crate::log;
use crate::log_return_void;
use crate::log_err;
use crate::gameimage;

// resize_draw_image() {{{
fn resize_draw_image(mut frame : Frame, path_file_icon : PathBuf) -> anyhow::Result<()>
{
  // Get path to project directory
  let db_global = match db::global::read()
  {
    Ok(db_global) => db_global,
    Err(e) => { return Err(ah!("Error to open global directory: {}", e)); },
  };
  let path_dir_project = match db_global.get_project_dir(&db_global.project)
  {
    Ok(path_dir_project) => path_dir_project,
    Err(e) => { return Err(ah!("Error to open project directory: {}", e)); },
  }; // match
  // Create path to resized icon
  let path_icon_resized_parent = path_dir_project.join("icon");
  let path_icon_resized = path_icon_resized_parent.join("icon.wizard.resized.png");
  match std::fs::create_dir_all(&path_icon_resized_parent)
  {
    Ok(()) => log!("Created directory {}", path_icon_resized_parent.string()),
    Err(e) => { return Err(ah!("Failure to create directories: {}", e));  },
  }; // match
  // Resize icon
  match shared::image::resize(path_icon_resized.clone(), path_file_icon, frame.w() as u32, frame.h() as u32)
  {
    Ok(()) => log!("Resized image '{}'", path_icon_resized.string()),
    Err(e) => return Err(ah!("Failed to resize image to '{}', with err '{}'", path_icon_resized.string(), e)),
  }; // if
  // Load image
  match fltk::image::PngImage::load(path_icon_resized)
  {
    Ok(png_image) =>
    {
      frame.set_image_scaled(Some(png_image));
      fltk::app::redraw();
      fltk::app::awake();
    },
    Err(e) => return Err(ah!("Could not load png icon: {}", e)),
  } // if

  Ok(())
} // resize_draw_image() }}}

// enum IconFrame {{{
#[derive(PartialEq)]
enum IconFrame
{
  Web,
  Local,
} // enum }}}

// impl IconFrame {{{
impl IconFrame
{
  fn as_str(&self) -> &'static str
  {
    match self
    {
      IconFrame::Local => "Local File",
      IconFrame::Web => "Web Search",
    } // match
  } // as_str

  fn from_str(&self, src : &str) -> IconFrame
  {
    match src
    {
      "Local File" => IconFrame::Local,
      _ => IconFrame::Web,
    } // match
  } // as_str
} // impl IconFrame }}}

// pub struct Icon {{{
#[derive(Clone)]
pub struct Icon
{
  pub ret_frame_header   : crate::frame::common::RetFrameHeader,
  pub ret_frame_footer   : crate::frame::common::RetFrameFooter,
  pub arc_path_file_icon : Arc<Mutex<Option<PathBuf>>>,
  pub opt_frame_icon     : Option<Frame>,
  pub opt_input_icon     : Option<FileInput>
} // Icon }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>
  , title: &str
  , msg_prev : common::Msg
  , msg_curr : common::Msg) -> Icon
{
  // Save previously selected icon path
  static OPTION_PATH_FILE_ICON : once_cell::sync::Lazy<Arc<Mutex<Option<PathBuf>>>> = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let mut ret = Icon
  {   ret_frame_header: ret_frame_header.clone()
    , ret_frame_footer: ret_frame_footer.clone()
    , arc_path_file_icon: OPTION_PATH_FILE_ICON.clone()
    , opt_frame_icon: None
    , opt_input_icon: None
  };

  let frame_content = ret_frame_header.frame_content.clone();

  let mut col = fltk::group::Flex::default()
    .column()
    .with_size_of(&frame_content)
    .with_pos_of(&frame_content);

  // Footer callbacks
  ret_frame_footer.btn_prev.clone().emit(tx, msg_prev);

  // Spacer
  col.add(&Frame::default());

  // Create icon box
  let mut row = fltk::group::Flex::default().row();
  row.add(&Frame::default());
  let frame_icon = Frame::default()
    .with_size(150, 225)
    .with_frame(FrameType::BorderBox);
  ret.opt_frame_icon = Some(frame_icon.clone());
  row.fixed(&frame_icon,150);
  row.add(&Frame::default());
  row.end();
  col.fixed(&row, 225);

  // Spacer
  col.add(&Frame::default());

  // Icon
  let mut row = fltk::group::Flex::default().row();
  let mut input_icon = FileInput::default();
  input_icon.set_readonly(true);
  input_icon.deactivate();
  ret.opt_input_icon = Some(input_icon.clone());
  row.add(&input_icon);
  let mut btn_search = shared::fltk::button::rect::search()
    .with_color(Color::Green);
  row.fixed(&btn_search, dimm::width_button_rec());
  row.end();
  col.fixed(&row, dimm::height_button_wide() + dimm::border()/2);

  // Check path cache
  if let Some(path_file_icon) = OPTION_PATH_FILE_ICON.lock().unwrap().clone()
  {
    input_icon.set_value(&path_file_icon.string());
    log_err!(resize_draw_image(frame_icon.clone(), path_file_icon.clone()));
  } // if

  // // Set input_icon callback
  let mut clone_input_icon = input_icon.clone();
  btn_search.set_callback(move |_|
  {
    let str_choice = match file_chooser("Select the icon", "*.{jpg,png}", ".", false)
    {
      Some(str_choice) => str_choice,
      None => { log!("No file selected"); return; }
    }; // match
    // Update static icon
    *OPTION_PATH_FILE_ICON.lock().unwrap() = Some(PathBuf::from(&str_choice));
    // Show file path on selector
    clone_input_icon.set_value(str_choice.as_str());
    // Set preview image
    match resize_draw_image(frame_icon.clone(), str_choice.into())
    {
      Ok(_) => log!("Set preview image"),
      Err(_) => log!("Failed to load icon image into preview"),
    } // match
  });

  col.end();

  ret
} // }}}

// pub fn project() {{{
pub fn project(tx: Sender<common::Msg>
  , title: &str
  , msg_prev : common::Msg
  , msg_curr : common::Msg
  , msg_next : common::Msg)
{
  let ret = icon(tx, title, msg_prev, msg_curr);
  let mut btn_next = ret.ret_frame_footer.btn_next.clone();

  // Callback to install the selected icon with the backend
  let clone_tx = tx.clone();
  btn_next.set_callback(move |_|
  {
    let arc_path_file_icon = ret.arc_path_file_icon.clone();
    let mut output_status = ret.ret_frame_footer.output_status.clone();
    clone_tx.send_awake(common::Msg::WindDeactivate);

    // Check if an icon was selected
    let path_file_icon = if let Ok(option_path_file_icon) = arc_path_file_icon.lock()
    && let Some(path_file_icon) = option_path_file_icon.as_ref()
    {
      path_file_icon.clone()
    }
    else
    {
      output_status.set_value("No icon selected");
      clone_tx.send_awake(msg_curr);
      log_return_void!("No Icon selected");
    };

    // Set selected icon as icon
    let clone_tx = clone_tx.clone();
    std::thread::spawn(move ||
    {
      // Try to install icon
      log!("Installing icon...");

      match gameimage::install::icon(&path_file_icon)
      {
        Ok(_) => log!("Successfully installed icon"),
        Err(e) => { clone_tx.send_awake(msg_curr); log_return_void!("Could not install icon with error: {}", e); },
      } // match

      clone_tx.send_awake(msg_next);
    });
  });
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
