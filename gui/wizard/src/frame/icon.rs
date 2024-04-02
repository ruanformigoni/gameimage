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

use anyhow::anyhow as ah;

use crate::dimm;
use crate::frame;
use crate::common;
use crate::common::WidgetExtExtra;
use crate::common::PathBufExt;
use crate::log;
use crate::lib::scaling;

// resize_draw_image() {{{
fn resize_draw_image(mut frame : Frame, path_file_icon : PathBuf) -> anyhow::Result<()>
{
  // Resize
  let path_icon_resized = PathBuf::from(path_file_icon.clone())
    .parent()
    .unwrap()
    .join("icon.wizard.resized.png");

  if let Err(e) = common::image_resize(path_icon_resized.clone(), path_file_icon, frame.w() as u32, frame.h() as u32)
  {
    log!("Failed to resize image to '{}', with err '{}'", path_icon_resized.string(), e);
  } // if

  match fltk::image::PngImage::load(path_icon_resized)
  {
    Ok(png_image) =>
    {
      frame.set_image_scaled(Some(png_image));
      fltk::app::redraw();
      fltk::app::awake();
      return Ok(())
    },
    Err(e) =>
    {
      log!("Could not load png icon: {}", e);
    },
  } // if

  Err(ah!("Could not set cover frame image"))
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
} // Icon }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>
  , title: &str
  , msg_prev : common::Msg
  , msg_curr : common::Msg) -> Icon
{
  // Keep track of which frame to draw (search web or local)
  static ICON_FRAME : once_cell::sync::Lazy<Mutex<IconFrame>> = once_cell::sync::Lazy::new(|| Mutex::new(IconFrame::Web));

  // Save previously selected icon path
  static OPTION_PATH_FILE_ICON : once_cell::sync::Lazy<Arc<Mutex<Option<PathBuf>>>> = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let ret = Icon{ ret_frame_header: ret_frame_header.clone()
    , ret_frame_footer: ret_frame_footer.clone()
    , arc_path_file_icon: OPTION_PATH_FILE_ICON.clone()
  };

  let frame_content = ret_frame_header.frame_content.clone();

  // Footer callbacks
  ret_frame_footer.btn_prev.clone().emit(tx, msg_prev);

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
      if let Ok(mut lock)  = ICON_FRAME.lock()
      {
        *lock = lock.from_str(label.as_str());
      } // if
    });
    // menu_source.add_choice(IconFrame::Web.as_str());
    menu_source.add_choice(IconFrame::Local.as_str());
    menu_source.set_label(ICON_FRAME.lock().unwrap().as_str());

  // Scale icon image size
  let f_scale = |val: i32| -> i32
  {
    (val as f32 * scaling::factor().unwrap_or(1.0)) as i32
  };

  if let Ok(lock) = ICON_FRAME.lock() && *lock == IconFrame::Local
  {

    // Create icon box
    let frame_icon = Frame::default()
      .with_size(f_scale(150), f_scale(225))
      .center_of(&frame_content)
      .with_frame(FrameType::BorderBox);

    // Icon
    let mut input_icon = FileInput::default()
      .with_size(frame_content.w() - dimm::border()*2, dimm::height_button_wide() + dimm::border())
      .bottom_of(&frame_content, - dimm::border())
      .with_posx_of(&menu_source)
      .with_align(Align::Top | Align::Left);
    input_icon.set_readonly(true);

    if let Ok(option_path_file_icon) = OPTION_PATH_FILE_ICON.lock()
    && let Some(path_file_icon) = option_path_file_icon.as_ref()
    {
      // Set value of select field
      input_icon.set_value(&path_file_icon.string());
      // Update preview
      if let Err(e) = resize_draw_image(frame_icon.clone(), path_file_icon.clone())
      {
        ret_frame_footer.output_status
          .clone()
          .set_value(format!("Failed to load preview: {}", e.to_string()).as_str());
      } // if
    } // if

    // // Set input_icon callback
    let mut clone_output_status = ret_frame_footer.output_status.clone();
    input_icon.set_callback(move |e|
    {
      let str_choice = if let Some(str_choice) = file_chooser("Select the icon", "*.{jpg,png}", ".", false)
      {
        str_choice
      } // if
      else
      {
        clone_output_status.set_value("No file selected");
        log!("No file selected");
        return;
      }; // else

      // Update static icon
      match OPTION_PATH_FILE_ICON.lock()
      {
        Ok(mut guard) => *guard = Some(PathBuf::from(&str_choice)),
        Err(e) => log!("Failed to set static with error: {}", e),
      } // match

      // Show file path on selector
      e.set_value(str_choice.as_str());

      // Set preview image
      match resize_draw_image(frame_icon.clone(), str_choice.into())
      {
        Ok(_) => clone_output_status.set_value("Set preview image"),
        Err(_) => clone_output_status.set_value("Failed to load icon image into preview"),
      } // match
    });
  } // if
  else
  {
    // search input
    // Keep track of the last text the user entered
    static STR_INPUT : once_cell::sync::Lazy<Mutex<String>> = once_cell::sync::Lazy::new(|| Mutex::new(String::new()));
    let mut input_search = input::Input::default()
      .with_size_of(&menu_source)
      .with_width(menu_source.w() - dimm::border() - dimm::width_button_wide())
      .below_of(&menu_source, dimm::border());
    input_search.set_value(STR_INPUT.lock().unwrap().as_str());

    // search button
    let mut btn_search = button::Button::default()
      .with_size(dimm::width_button_wide(), dimm::height_button_wide())
      .right_of(&input_search, dimm::border())
      .with_label("Search")
      .with_focus(false)
      .with_color(Color::Green);

    // Temporary images path
    let path_dir_images = if let Ok(str_dir_projects) = env::var("GIMG_DIR")
    {
      let path_dir_images = PathBuf::from(str_dir_projects).join("thumbnails");
      let _  = std::fs::create_dir_all(&path_dir_images);
      path_dir_images
    } // if
    else
    {
      log!("Could not determine directory to download temporary images into");
      return ret;
    }; // else

    let mut scroll = fltk::group::Scroll::default()
      .with_width_of(&menu_source)
      .with_height(frame_content.h() - dimm::height_button_wide()*2 - dimm::border()*4)
      .below_of(&input_search, dimm::border())
      .with_frame(FrameType::BorderBox);
    scroll.set_scrollbar_size(dimm::border());

    scroll.begin();

    let count_row : i32 = 5;
    let count_col : i32 = 4;

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
      return ret;
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
        return ret;
      }; // else

      let arc_path_file_image = Arc::new(Mutex::new(path_file_image.clone()));
      let mut btn_image = button::RadioButton::default()
        .with_width(width_tile - dimm::border())
        .with_height(height_tile - dimm::border())
        .with_focus(false)
        .with_shared_image(path_file_image)
        .with_pos_of(&grid)
        .with_callback(move |_|
        {
          let clone_arc_path_file_image = arc_path_file_image.clone();
          std::thread::spawn(move ||
          {
            if let Ok(lock) = clone_arc_path_file_image.lock()
            {
              // Update last selected icon
              match OPTION_PATH_FILE_ICON.lock()
              {
                Ok(mut guard) => *guard = Some(PathBuf::from(lock.clone())),
                Err(e) => log!("Failed to set static with error: {}", e),
              } // match
            } // if
            else
            {
              log!("Could not install selected image");
            } // else
          });
        });
      btn_image.set_selection_color(Color::Blue);

      if let Ok(mut lock) = arc_vec_radio.lock()
      {
        lock.push(btn_image.clone());
      } // if

      let _ = grid.set_widget(&mut btn_image, curr_row as usize, curr_col as usize);

      if curr_row == count_row-1 && curr_col == count_col-1 { break; }
      else if curr_col == count_col-1 { curr_row += 1; curr_col = 0; }
      else { curr_col += 1; }
    } // for

    scroll.end();

    let clone_input_search = input_search.clone();
    let clone_path_dir_images = path_dir_images.clone();
    let mut clone_output_status = ret_frame_footer.output_status.clone();

    let f_callback_search = move ||
    {
      let args = image_search::Arguments::new(clone_input_search.value().as_str(), 15)
        .directory(&clone_path_dir_images);

      if let Ok(mut lock) = STR_INPUT.lock()
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
        // Try this 10 times, sometimes it doesnt work, idk why
        for _ in 1..10
        {
          // Returns the urls of the search results
          let _image_urls = match image_search::blocking::urls(clone_args.clone())
          {
            Ok(e) => e,
            Err(e) => { log!("Could not fetch image url with error: {}", e); continue; }
          }; // match

          // Returns the search results as Image structs
          let _images = match image_search::blocking::search(clone_args.clone())
          {
            Ok(e) => e,
            Err(e) => { log!("Could not search images with error: {}", e); continue; }
          }; // match

          // Downloads the search results and returns  the paths to the files
          let _paths = match image_search::blocking::download(clone_args.clone())
          {
            Ok(e) => { clone_output_status.set_value("Download successful"); e }
            Err(e) => { log!("Failure to download with error {}", e); continue }
          }; // if
        } // for

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
        fltk::app::awake();
      }
    });

    let mut clone_f_callback_search = f_callback_search.clone();
    btn_search.set_callback(move |_| { clone_f_callback_search(); fltk::app::awake(); });


  } // else

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
    clone_tx.send(common::Msg::WindDeactivate);

    // Check if an icon was selected
    let path_file_icon = if let Ok(option_path_file_icon) = arc_path_file_icon.lock()
    && let Some(path_file_icon) = option_path_file_icon.as_ref()
    {
      path_file_icon.clone()
    }
    else
    {
      output_status.set_value("No icon selected");
      clone_tx.send(msg_curr);
      log!("No Icon selected");
      return;
    };

    // Set selected icon as icon
    let clone_tx = clone_tx.clone();
    std::thread::spawn(move ||
    {
      // Try to install icon
      log!("Installing icon...");

      if common::gameimage_sync(vec!["install", "icon", &path_file_icon.string()]) != 0
      {
        log!("Could not install icon, use .jpg or .png");
        clone_tx.send(msg_curr);
        return;
      } // if

      clone_tx.send(msg_next);
    });
  });
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
