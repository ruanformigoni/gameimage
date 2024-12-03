use std::env;
use std::sync::Mutex;

use fltk::prelude::*;
use fltk::{
  app::Sender,
  group::Flex,
  enums,
  frame::Frame,
  image::SharedImage,
};

use anyhow::anyhow as ah;

use shared::dimm;
use shared::std::PathBufExt;
use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;
use shared::{hover_blink,column,row,add,fixed};

use crate::db;
use crate::common;
use crate::games;
use common::Msg;

// fn: get_path_db_executable() {{{
fn get_path_db_executable() -> anyhow::Result<std::path::PathBuf>
{
  let mut path_db : std::path::PathBuf = std::env::var("GIMG_LAUNCHER_ROOT")?.into();
  path_db.push("gameimage.wine.executable.json");

  Ok(path_db)
} // fn: get_path_db_executable() }}}

// fn: get_default_executable() {{{
fn get_default_executable() -> anyhow::Result<std::path::PathBuf>
{
  let path_file_db = std::path::PathBuf::from(std::env::var("GIMG_LAUNCHER_ROOT")? + "/gameimage.json");
  let db_project = db::project::read(&path_file_db)?;
  Ok(db_project.path_file_rom.ok_or(ah!("Could not read path_file_rom"))?.into())
} // fn: get_default_executable()}}}

// fn: new_menu_entries() {{{
fn new_menu_entries(frame: Flex) -> anyhow::Result<()>
{
  // Keep track of the currently selected item
  static PATH_CURRENT_EXECUTABLE : once_cell::sync::Lazy<Mutex<Option<std::path::PathBuf>>>
    = once_cell::sync::Lazy::new(|| Mutex::new(None));
  // Open database
  let path_file_db_executable = get_path_db_executable()?;
  // Executable selection menu
  let db_executables = shared::db::kv::read(&path_file_db_executable).unwrap_or_default();
  // Return if there are no executables selected
  if db_executables.len() == 0 { return Ok(()); } // if
  // Get the default executable
  let path_default_executable = get_default_executable()?.string();
  // Add menu entries for executable selection
  let mut btn_executable = fltk::menu::Choice::default()
    .with_size(frame.w(), dimm::height_button_rec())
    .top_left_of(&frame, 0)
    .with_frame(enums::FrameType::BorderBox)
    .with_focus(false);
  for executable in db_executables.keys().into_iter().chain(vec![path_default_executable.clone()].iter())
  {
    if executable.is_empty() { continue; }
    btn_executable.add(&executable
    , fltk::enums::Shortcut::None
    , fltk::menu::MenuFlag::Normal
    , |e|
    {
      let path_file_executable : std::path::PathBuf = e.item_pathname(None).unwrap_or(String::new()).into();
      std::env::set_var("GIMG_LAUNCHER_EXECUTABLE", &path_file_executable);
      match PATH_CURRENT_EXECUTABLE.lock()
      {
        Ok(mut guard) => *guard = Some(path_file_executable),
        Err(e) => eprintln!("Could not lock PATH_CURRENT_EXECUTABLE: {}", e),
      } // if
    });
  } // for
  // Set default entry or use existing
  match PATH_CURRENT_EXECUTABLE.lock()
  {
    Ok(guard) => match guard.clone()
    {
      Some(value) => { btn_executable.set_item(&btn_executable.find_item(&value.string()).unwrap()); },
      None => { btn_executable.set_item(&btn_executable.find_item(&path_default_executable).unwrap()); },
    },
    Err(e) => { eprintln!("{}", e); },
  } // match

  Ok(())
} // fn: new_menu_entries() }}}

// fn: new {{{
pub fn new(tx : Sender<Msg>)
{
  // Layout
  let mut frame_background = Frame::default_fill();
  if let Ok(env_image_launcher) = env::var("GIMG_LAUNCHER_IMG")
  && let Ok(shared_image) = SharedImage::load(env_image_launcher)
  {
    frame_background.set_image_scaled(Some(shared_image.clone()));
    frame_background.resize_callback(move |s,_,_,_,_| { s.set_image_scaled(Some(shared_image.clone())); });
  } // if
  else
  {
    println!("Failed to set launcher image");
  } // else
  // Buttons
  column!(col,
    col.add(&Frame::default());
    row!(row,
      row.set_margin(dimm::border_half());
      fixed!(row, btn_menu, shared::fltk::button::rect::list(), dimm::width_button_rec());
      add!(row, _spacer, Frame::default());
      fixed!(row, btn_switch, shared::fltk::button::rect::switch(), dimm::width_button_rec());
      add!(row, _spacer, Frame::default());
      fixed!(row, btn_play, shared::fltk::button::rect::play().with_color(enums::Color::Blue), dimm::width_button_rec());
    );
    col.fixed(&row, dimm::height_button_rec() + dimm::border());
  );

  let mut fb: Vec<u8> = vec![0u8; (dimm::width_launcher() * dimm::height_launcher() * 4) as usize];
  // Fill with required color
  for (_, pixel) in fb.chunks_exact_mut(4).enumerate() {
    pixel.copy_from_slice(&[0, 0, 0, 96]);
  }
  let image = fltk::image::RgbImage::new(&fb, dimm::width_launcher(), dimm::height_button_rec() + dimm::border(), enums::ColorDepth::Rgba8).unwrap();
  // Bottom background
  let mut row = row.clone();
  row.set_align(enums::Align::Inside | enums::Align::Center);
  row.set_frame(enums::FrameType::NoBox);
  row.set_image(Some(image));
  row.resize_callback(move |s,_,_,w,_|
  {
    let image = fltk::image::RgbImage::new(&fb, w, dimm::height_button_rec() + dimm::border(), enums::ColorDepth::Rgba8).unwrap();
    s.set_image(Some(image));
  });

  // Button left aligned
  btn_menu.clone().emit(tx, Msg::DrawMenu);
  btn_switch.clone().emit(tx, Msg::DrawSelector);
  btn_switch.clone().hide();
  hover_blink!(btn_menu);
  hover_blink!(btn_switch);
  hover_blink!(btn_play);

  // Only show switch button in case there is more than one game
  if let Ok(vec_entry) = games::games() && vec_entry.len() > 1
  {
    btn_switch.clone().show();
  } // if

  // Button right aligned
  let clone_tx = tx.clone();
  let mut clone_frame_background = frame_background.clone();
  btn_play.clone().set_callback(move |_|
  {
    // Cover image black and white
    if let Ok(env_image_launcher) = env::var("GIMG_LAUNCHER_IMG_GRAYSCALE")
    && let Ok(shared_image) = SharedImage::load(env_image_launcher)
    {
      clone_frame_background.set_image_scaled(Some(shared_image));
    } // if
    else
    {
      println!("Failed to set launcher image");
    } // else
    fltk::app::redraw();
    // Deactivate window
    clone_tx.send_awake(common::Msg::WindDeactivate);
    // Reference to spawned process
    std::thread::spawn(move ||
    {
      // Launch game
      games::launch();
      // Redraw
      clone_tx.send_activate(Msg::DrawCover);
    });
  });

  match new_menu_entries(col.clone())
  {
    Ok(()) => (),
    Err(e) => eprintln!("{}", e),
  };
} // fn: new }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
