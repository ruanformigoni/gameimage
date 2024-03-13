#![allow(warnings)]

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::sync::{Arc,Mutex};
use std::borrow::BorrowMut;

// Gui
use fltk::prelude::*;
use fltk_grid;
use fltk_evented;
use fltk::{
  app::{Sender,Receiver},
  window::Window,
  text::{TextBuffer,TextDisplay},
  menu,
  menu::MenuButton,
  button,
  button::Button,
  group::Group,
  image::SharedImage,
  input,
  input::{Input,FileInput},
  group::PackType,
  frame::Frame,
  dialog::{file_chooser,dir_chooser},
  enums::{Align,FrameType,Color},
  misc,
  misc::Progress,
};

use url as Url;
use anyhow;
use anyhow::anyhow as ah;
use once_cell;
use image_search;

use crate::dimm;
use crate::frame;
use crate::common;
use crate::common::WidgetExtExtra;
use crate::common::PathBufExt;
use crate::db;
use crate::download;
use crate::svg;
use crate::log;
use crate::scaling;


// get_icon() {{{
fn get_icon() -> anyhow::Result<PathBuf>
{
  Ok(db::project::dir()?.join("icon/icon.png"))
} // fn: get_icon }}}

// set_image() {{{
fn set_image(mut frame : Frame) -> anyhow::Result<()>
{
  // Get image
  let path_icon = get_icon()?;

  // Resize
  let path_icon_resized = PathBuf::from(path_icon.clone())
    .parent()
    .unwrap()
    .join("icon.wizard.resized.png");
  common::image_resize(path_icon_resized.clone(), path_icon, frame.w() as u32, frame.h() as u32);

  match fltk::image::PngImage::load(path_icon_resized)
  {
    Ok(png_image) =>
    {
      frame.set_image_scaled(Some(png_image));
      frame.redraw();
      return Ok(())
    },
    Err(e) =>
    {
      log!("Could not load png icon: {}", e);
    },
  } // if

  Err(ah!("Could not set cover frame image"))
} // set_image() }}}

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

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>
  , title: &str
  , msg_prev : common::Msg
  , msg_curr : common::Msg
  , msg_next : common::Msg)
{
  // Keep track of which frame to draw (search web or local)
  static icon_frame : once_cell::sync::Lazy<Mutex<IconFrame>>
    = once_cell::sync::Lazy::new(|| Mutex::new(IconFrame::Web));

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_header = ret_frame_header.frame.clone();
  let frame_content = ret_frame_header.frame_content.clone();
  let frame_footer = ret_frame_footer.frame.clone();

  // Footer callbacks
  ret_frame_footer.btn_prev.clone().emit(tx, msg_prev);
  let clone_tx = tx.clone();
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    if let Ok(path_file_icon) = env::var("GIMG_ICON")
    {
      if ! PathBuf::from(path_file_icon).is_file()
      {
        log!("Icon file is invalid");
        clone_output_status.set_value("Icon file is invalid");
        return;
      } // if
    } // if
    else
    {
      log!("Icon is not set");
      clone_output_status.set_value("Icon is not set");
      return;
    } // else

    clone_tx.send(msg_next);
  });

  // Callback to install the selected icon with the backend
  let clone_tx = tx.clone();
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  let f_install_icon = move |path_file_icon : PathBuf|
  {
    // Try to install icon
    clone_output_status.set_value("Installing icon...");
    if let Ok(rx_gameimage) = common::gameimage_cmd(vec!["install", "icon", &path_file_icon.string()])
    {
      clone_tx.send(common::Msg::WindDeactivate);
      let _ = rx_gameimage.recv();
      clone_tx.send(common::Msg::WindActivate);
    } // if
    else
    {
      clone_output_status.set_value("Could not install icon, use .jpg or .png");
      return;
    } // else

    // Set environment variable
    match get_icon()
    {
      Ok(path_icon) => env::set_var("GIMG_ICON", path_icon),
      Err(e) =>
      {
        log!("Could not get icon path: {}", e);
        clone_output_status.set_value(format!("Could not get icon path: {}", e).as_str());
      }
    } // if
  };

  // Select source
  let clone_tx = tx.clone();
  let mut menu_source = menu::MenuButton::default()
    .with_width(frame_content.w() - dimm::border()*2)
    .with_height(dimm::height_button_wide())
    .with_pos_of(&frame_content)
    .with_focus(false)
    .with_border(dimm::border(), dimm::border())
    .with_callback(move |e|
    {
      let label = e.text(e.value()).unwrap_or(String::new());
      e.set_label(label.as_str());
      clone_tx.send(msg_curr);
      if let Ok(mut lock)  = icon_frame.lock()
      {
        *lock = lock.from_str(label.as_str());
      } // if
    });
    menu_source.add_choice(IconFrame::Web.as_str());
    menu_source.add_choice(IconFrame::Local.as_str());
    menu_source.set_label(icon_frame.lock().unwrap().as_str());

  // Scale icon image size
  let f_scale = |val: i32| -> i32
  {
    (val as f32 * scaling::factor().unwrap_or(1.0)) as i32
  };

  if let Ok(lock) = icon_frame.lock() && *lock == IconFrame::Local
  {
    // Create icon box
    let frame_icon = Frame::default()
      .with_size(f_scale(150), f_scale(225))
      .center_of(&frame_content)
      .with_frame(FrameType::BorderBox);

    // Icon
    let mut input_icon = FileInput::default()
      .with_size(frame_content.w() - dimm::border()*2, dimm::height_button_wide() + dimm::border())
      .below_of(&frame_content, 0)
      .with_align(Align::Top | Align::Left);
    input_icon.set_pos(frame_content.x() + dimm::border()
      , input_icon.y() - input_icon.h() - dimm::border());
    input_icon.set_readonly(true);

    // Check if GIMG_ICON exists
    if let Some(env_icon) = env::var("GIMG_ICON").ok()
    {
      // Set value of select field
      input_icon.set_value(&env_icon);
      // Update preview
      if let Err(e) = set_image(frame_icon.clone())
      {
        ret_frame_footer.output_status
          .clone()
          .set_value(format!("Failed to load preview: {}", e.to_string()).as_str());
      } // if
    } // if

    // // Set input_icon callback
    let mut clone_f_install_icon = f_install_icon.clone();
    let mut clone_output_status = ret_frame_footer.output_status.clone();
    input_icon.set_callback(move |e|
    {
      let choice = file_chooser("Select the icon", "*.{jpg,png}", ".", false);

      if choice.is_none()
      {
        return;
      } // if

      let str_choice = choice.unwrap();

      // Show file path on selector
      e.set_value(str_choice.as_str());

      // Install icon
      clone_f_install_icon(str_choice.into());

      // Set preview image
      if let Err(e) = set_image(frame_icon.clone())
      {
        clone_output_status.set_value("Failed to load icon image into preview");
      } // if
      else
      {
        clone_output_status.set_value("Set preview image");
      } // else
      });
  } // if
  else
  {
    // search input
    // Keep track of the last text the user entered
    static static_str_input : once_cell::sync::Lazy<Mutex<String>> = once_cell::sync::Lazy::new(|| Mutex::new(String::new()));
    let mut input_search = input::Input::default()
      .with_size_of(&menu_source)
      .with_width(menu_source.w() - dimm::border() - dimm::width_button_wide())
      .below_of(&menu_source, dimm::border());
    input_search.set_value((static_str_input.lock().unwrap().as_str()));

    // search button
    let mut btn_search = button::Button::default()
      .with_size(dimm::width_button_wide(), dimm::height_button_wide())
      .right_of(&input_search, dimm::border())
      .with_label("Search")
      .with_focus(false)
      .with_color(Color::Green);

    // Temporary images path
    let path_dir_images = if let Ok(mut str_dir_projects) = env::var("GIMG_DIR")
    {
      let path_dir_images = PathBuf::from(str_dir_projects).join("thumbnails");
      let _  = std::fs::create_dir_all(&path_dir_images);
      path_dir_images
    } // if
    else
    {
      log!("Could not determine directory to download temporary images into");
      return;
    }; // else

    let mut scroll = fltk::group::Scroll::default()
      .with_width_of(&menu_source)
      .with_height(frame_content.h() - dimm::height_button_wide()*2 - dimm::border()*4)
      .below_of(&input_search, dimm::border())
      .with_frame(FrameType::BorderBox);
    scroll.set_scrollbar_size(dimm::border());

    scroll.begin();

    let mut count_row : i32 = 5;
    let mut count_col : i32 = 4;

    let mut grid = fltk_grid::Grid::default()
      .with_width(menu_source.w() - dimm::border())
      .with_height((( menu_source.w() - dimm::border() ) as f32 * 1.5) as i32)
      .below_of(&input_search, dimm::border())
      .with_frame(FrameType::BorderBox);
    // grid.show_grid(true);
    grid.set_layout(count_row, count_col);

    let mut iter_dir = if let Ok(iter_dir) = std::fs::read_dir(&path_dir_images)
    {
      iter_dir
    } // if
    else
    {
      log!("Failed to grab directory iterator to {}", path_dir_images.string());
      return;
    }; // else

    let width_tile = grid.w() / count_col;
    let height_tile = grid.h() / count_row;
    let mut curr_row : i32 = 0;
    let mut curr_col : i32 = 0;

    let arc_vec_radio : Arc<Mutex<Vec<button::RadioButton>>> = Arc::new(Mutex::new(Vec::new()));

    for result_path_file_image in iter_dir
      .borrow_mut()
      .filter(|e| { e.as_ref().is_ok_and(|e|{ e.file_type().is_ok_and(|e| e.is_file()) }) })
      .filter(|e| { e.as_ref().is_ok_and(|e|
      {
        e.path().string().ends_with(".jpg") || e.path().string().ends_with(".png")
      }) })
    {
      let path_file_image = if let Ok(path_file_image) = result_path_file_image
      {
        path_file_image.path()
      } // if
      else
      {
        return;
      }; // else

      let clone_arc_vec_radio = arc_vec_radio.clone();
      let clone_f_install_icon = f_install_icon.clone();
      let clone_tx = tx.clone();
      let arc_path_file_image = Arc::new(Mutex::new(path_file_image.clone()));
      let mut btn_image = button::RadioButton::default()
        .with_width(width_tile - dimm::border())
        .with_height(height_tile - dimm::border())
        .with_focus(false)
        .with_shared_image(path_file_image)
        .with_pos_of(&grid)
        .with_callback(move |e|
        {
          clone_tx.send(common::Msg::WindDeactivate);
          let clone_tx = clone_tx.clone();
          let clone_btn = e.clone();
          let mut clone_f_install_icon = clone_f_install_icon.clone();
          let clone_arc_path_file_image = arc_path_file_image.clone();
          std::thread::spawn(move ||
          {
            if let Ok(lock) = clone_arc_path_file_image.lock()
            {
              clone_f_install_icon(lock.clone());
            } // if
            clone_tx.send(common::Msg::WindActivate);
          });
        });
      btn_image.set_selection_color(Color::Blue);

      if let Ok(mut lock) = arc_vec_radio.lock()
      {
        lock.push(btn_image.clone());
      } // if

      grid.set_widget(&mut btn_image, curr_row as usize, curr_col as usize);

      if ( curr_row == count_row-1 && curr_col == count_col-1 ) { break; }
      else if ( curr_col == count_col-1 ) { curr_row += 1; curr_col = 0; }
      else { curr_col += 1; }
    } // for

    scroll.end();

    let clone_input_search = input_search.clone();
    let clone_path_dir_images = path_dir_images.clone();
    let clone_tx = tx.clone();
    let mut clone_output_status = ret_frame_footer.output_status.clone();

    let mut f_callback_search = move ||
    {
      let args = image_search::Arguments::new(clone_input_search.value().as_str(), 15)
        .directory(&clone_path_dir_images);

      if let Ok(mut lock) = static_str_input.lock()
      {
        *lock = clone_input_search.value();
      } // if

      tx.send(common::Msg::WindDeactivate);

      clone_output_status.set_value("Downloading images from the provided query...");

      let clone_args = args.clone();
      let clone_path_dir_images = clone_path_dir_images.clone();
      let mut clone_output_status = clone_output_status.clone();
      std::thread::spawn(move ||
      {
        let _  = std::fs::remove_dir_all(&clone_path_dir_images);
        let _  = std::fs::create_dir_all(&clone_path_dir_images);
        // Returns the urls of the search results
        if let Ok(_image_urls) = image_search::blocking::urls(clone_args.clone())
        // Returns the search results as Image structs
        && let Ok(_images) = image_search::blocking::search(clone_args.clone())
        // Downloads the search results and returns  the paths to the files
        && let Ok(_paths) = image_search::blocking::download(clone_args)
        {
          clone_output_status.set_value("Download successful");
        } // if
        else
        {
          clone_output_status.set_value("Download failed, try again");
        } // else

        tx.send(common::Msg::WindActivate);
        tx.send(msg_curr);
      });
    };

    // Set search callbacks
    let mut input_search_listener : fltk_evented::Listener<_> = input_search.clone().into();
    let mut clone_f_callback_search = f_callback_search.clone();
    input_search_listener.on_keydown(move |_|
    {
      if fltk::app::event_key() == fltk::enums::Key::Enter
      {
        clone_f_callback_search();
      }
    });

    let mut clone_f_callback_search = f_callback_search.clone();
    btn_search.set_callback(move |_| { clone_f_callback_search() });


  } // else

} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
