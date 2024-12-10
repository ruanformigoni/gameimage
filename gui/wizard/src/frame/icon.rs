use std::path::PathBuf;
use std::sync::{Arc,Mutex};

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  input::FileInput,
  frame::Frame,
  dialog::file_chooser,
  enums::Color,
};

use shared::fltk::WidgetExtExtra;

use anyhow::anyhow as ah;

use shared::hover_blink;
use shared::fltk::SenderExt;
use shared::std::PathBufExt;

use crate::dimm;
use crate::db;
use crate::common;
use crate::log;
use crate::log_status;
use crate::log_return_void;
use crate::log_err;
use crate::gameimage;

// resize_draw_image() {{{
pub fn resize_draw_image(mut frame : Frame, path_file_icon : PathBuf) -> anyhow::Result<()>
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

// pub struct Icon {{{
#[derive(Clone)]
pub struct Icon
{
  pub arc_path_file_icon : Arc<Mutex<Option<PathBuf>>>,
  pub opt_frame_icon     : Option<Frame>,
  pub opt_input_icon     : Option<FileInput>
} // Icon }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>
  , title: &str
  , msg_prev : common::Msg
  , _msg_curr : common::Msg) -> (crate::Ui, Icon)
{
  // Save previously selected icon path
  static OPTION_PATH_FILE_ICON : once_cell::sync::Lazy<Arc<Mutex<Option<PathBuf>>>> = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

  let ui = crate::GUI.lock().unwrap().ui.clone()(title);

  let mut ret = Icon
  {
      arc_path_file_icon: OPTION_PATH_FILE_ICON.clone()
    , opt_frame_icon: None
    , opt_input_icon: None
  };

  let mut col = fltk::group::Flex::default()
    .column()
    .with_size_of(&ui.group)
    .with_pos_of(&ui.group);

  // Footer callbacks
  ui.btn_prev.clone().emit(tx, msg_prev);

  // Spacer
  col.add(&Frame::default());

  // Create icon box
  let mut row = fltk::group::Flex::default().row();
  row.add(&Frame::default());
  let frame_icon = shared::fltk::frame::bordered()
    .with_size(150, 225);
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
  hover_blink!(btn_search);
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
      None => { log_status!("No file selected"); return; }
    }; // match
    // Update static icon
    *OPTION_PATH_FILE_ICON.lock().unwrap() = Some(PathBuf::from(&str_choice));
    // Show file path on selector
    clone_input_icon.set_value(str_choice.as_str());
    // Set preview image
    match resize_draw_image(frame_icon.clone(), str_choice.into())
    {
      Ok(_) => log_status!("Set preview image"),
      Err(_) => log_status!("Failed to load icon image into preview"),
    } // match
  });

  col.end();

  (ui, ret)
} // }}}

// pub fn project() {{{
pub fn project(tx: Sender<common::Msg>
  , title: &str
  , msg_prev : common::Msg
  , msg_curr : common::Msg
  , msg_next : common::Msg)
{
  let (ui,ret) = icon(tx, title, msg_prev, msg_curr);
  let mut btn_next = ui.btn_next.clone();

  // Callback to install the selected icon with the backend
  let clone_tx = tx.clone();
  btn_next.set_callback(move |_|
  {
    let arc_path_file_icon = ret.arc_path_file_icon.clone();

    // Check if an icon was selected
    let path_file_icon = if let Some(path_file_icon) = arc_path_file_icon.lock().unwrap().as_ref()
    {
      path_file_icon.clone()
    }
    else
    {
      log_status!("No icon selected");
      clone_tx.send_activate(msg_curr);
      return;
    };

    // Set selected icon as icon
    clone_tx.send_awake(common::Msg::WindDeactivate);
    let clone_tx = clone_tx.clone();
    std::thread::spawn(move ||
    {
      // Try to install icon
      log_status!("Installing icon...");

      match gameimage::install::icon(&path_file_icon)
      {
        Ok(_) => log_status!("Successfully installed icon"),
        Err(e) => { clone_tx.send_activate(msg_curr); log_return_void!("Could not install icon with error: {}", e); },
      } // match

      clone_tx.send_activate(msg_next);
    });
  });
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
