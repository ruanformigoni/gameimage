use std::env;
use std::sync::Mutex;

use fltk::prelude::*;
use fltk::{
  app::Sender,
  group::Group,
  button::Button,
  widget::Widget,
  group::PackType,
  enums::FrameType,
  frame::Frame,
  image::SharedImage,
};

use anyhow::anyhow as ah;

use shared::dimm;
use shared::svg;
use shared::std::PathBufExt;
use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;

use crate::db;
use crate::common;
use crate::games;
use common::Msg;

pub struct RetFrameCover
{
  pub frame : Frame,
  pub btn_left : Button,
  pub btn_right : Button,
} // Ret

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
fn new_menu_entries(frame: Frame) -> anyhow::Result<()>
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
    .with_frame(FrameType::BorderBox)
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
pub fn new(tx : Sender<Msg>, x : i32, y : i32) -> RetFrameCover
{
  let mut frame_base = Frame::default().with_size(dimm::width_launcher(), dimm::height_launcher());
  frame_base.set_type(PackType::Vertical);
  frame_base.set_frame(FrameType::FlatBox);

  let mut group_content = Group::default()
    .with_size(dimm::width_launcher() - dimm::border()*2, dimm::height_launcher() - (dimm::height_button_wide() + dimm::border() * 2) - dimm::border()*2)
    .with_pos(dimm::border(), dimm::border());
  group_content.set_type(PackType::Vertical);
  group_content.set_frame(FrameType::FlatBox);
  group_content.begin();

  let mut frame = Frame::default()
    .with_size(dimm::width_launcher(), dimm::height_launcher())
    .with_pos(x,y);
  frame.set_type(PackType::Vertical);
  frame.set_frame(FrameType::FlatBox);

  // Cover image
  if let Ok(env_image_launcher) = env::var("GIMG_LAUNCHER_IMG")
  && let Ok(shared_image) = SharedImage::load(env_image_launcher)
  {
    frame.set_image_scaled(Some(shared_image));
  } // if
  else
  {
    println!("Failed to set launcher image");
  } // else

  group_content.end();

  // Set bottom background
  let mut btn_bottom = Button::default()
    .with_size(frame_base.width(), dimm::bar())
    .with_focus(false)
    .center_x(&frame_base);
  btn_bottom.set_pos(btn_bottom.x(), frame_base.h() - dimm::bar());
  btn_bottom.set_frame(FrameType::NoBox);
  btn_bottom.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_background(1.0).as_str()).unwrap()));
  btn_bottom.deactivate();

  // Button left aligned
  let mut btn_left = shared::fltk::button::rect::list()
    .bottom_left_of(&frame, - dimm::border());
  btn_left.emit(tx, Msg::DrawMenu);

  // Button in the middle
  let mut btn_middle = shared::fltk::button::rect::switch()
    .bottom_center_of(&frame, - dimm::border());
  btn_middle.emit(tx, Msg::DrawSelector);
  btn_middle.hide();

  // Only show switch button in case there is more than one game
  if let Ok(vec_entry) = games::games() && vec_entry.len() > 1
  {
    btn_middle.show();
  } // if

  // Button right aligned
  let mut btn_right = shared::fltk::button::rect::play()
    .bottom_right_of(&frame_base, - dimm::border())
    .with_focus(false)
    .with_color(fltk::enums::Color::Green);
  let clone_tx = tx.clone();
  let mut clone_frame = frame.clone();
  btn_right.set_callback(move |_|
  {
    // Cover image black and white
    if let Ok(env_image_launcher) = env::var("GIMG_LAUNCHER_IMG_GRAYSCALE")
    && let Ok(shared_image) = SharedImage::load(env_image_launcher)
    {
      clone_frame.set_image_scaled(Some(shared_image));
    } // if
    else
    {
      println!("Failed to set launcher image");
    } // else
    fltk::app::redraw();
    // Reference to spawned process
    std::thread::spawn(move ||
    {
      // Deactivate window
      clone_tx.send_awake(common::Msg::WindDeactivate);
      // Launch game
      games::launch();
      // Redraw
      clone_tx.send_awake(Msg::WindActivate);
      clone_tx.send_awake(Msg::DrawCover);
    });
  });

  match new_menu_entries(frame.clone())
  {
    Ok(()) => (),
    Err(e) => eprintln!("{}", e),
  };

  RetFrameCover{ frame, btn_left, btn_right }
} // fn: new }}}

// fn: from {{{
#[allow(dead_code)]
pub fn from(tx : Sender<Msg>, w : Widget) -> RetFrameCover
{
  new(tx, w.x(), w.y())
} // fn: from }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
