use fltk::prelude::*;
use fltk::{
  output::Output,
  app::Sender,
  widget::Widget,
  group::PackType,
  enums::{Align,FrameType,Color},
  frame::Frame,
};

use anyhow::anyhow as ah;

use shared::dimm;
use shared::std::PathBufExt;
use shared::std::OsStrExt;
use shared::fltk::WidgetExtExtra;

use crate::common::Msg;

pub struct RetFrameExecutable
{
  pub frame : Frame,
} // Ret

// fn find_executables() {{{
fn find_executables() -> anyhow::Result<Vec<std::path::PathBuf>>
{
  let mut ret = vec![];

  let path_dir_boot = std::path::PathBuf::from(std::env::var("GIMG_LAUNCHER_BOOT")?)
    .parent()
    .ok_or(ah!("Could not fetch parent path for boot directory"))?
    .to_owned();

  let mut path_dir_wine = path_dir_boot.clone();
  path_dir_wine.push("wine");

  for entry in walkdir::WalkDir::new(&path_dir_wine)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    let path = entry.into_path();
    // Skip if is not a regular file
    if ! path.is_file() { continue; }
    // Skip windows folder
    if path.components().any(|e| e.as_os_str().string() == "windows") { continue; }
    // Check if is an executable file
    if ! path.file_name_string().to_lowercase().ends_with(".exe")
    && ! path.file_name_string().to_lowercase().ends_with(".msi")
    {
      continue;
    } // if
    // Make path relative
    match path.strip_prefix(path_dir_boot.clone())
    {
      Ok(e) => ret.push(e.to_path_buf()),
      Err(_) => (),
    }
  } // for

  Ok(ret)
} // find_executables() }}}

// get_path_db_executable() {{{
fn get_path_db_executable() -> anyhow::Result<std::path::PathBuf>
{
  let mut path_db : std::path::PathBuf = std::env::var("GIMG_LAUNCHER_ROOT")?.into();
  path_db.push("gameimage.wine.executable.json");

  Ok(path_db)
} // get_path_db_executable() }}}

// get_path_db_args() {{{
fn get_path_db_args() -> anyhow::Result<std::path::PathBuf>
{
  let mut path_db : std::path::PathBuf = std::env::var("GIMG_LAUNCHER_ROOT")?.into();
  path_db.push("gameimage.wine.args.json");

  Ok(path_db)
} // get_path_db_args() }}}

// fn: new {{{
pub fn new(tx : Sender<Msg>, x : i32, y : i32) -> RetFrameExecutable
{
  let path_file_db_args = match get_path_db_args()
  {
    Ok(e) => e,
    Err(e) => { eprintln!("Could not retrieve path to db file: {}", e); std::path::PathBuf::default() }
  }; // match

  let path_file_db_executable = match get_path_db_executable()
  {
    Ok(e) => e,
    Err(e) => { eprintln!("Could not retrieve path to db file: {}", e); std::path::PathBuf::default() }
  }; // match

  //
  // Main
  //
  let mut frame = Frame::default()
    .with_size(dimm::width_launcher(), dimm::height_launcher())
    .with_pos(x, y);
  frame.set_type(PackType::Vertical);
  frame.set_frame(FrameType::FlatBox);

  let mut frame_title = Frame::default()
    .with_label("Executable Configuration")
    .with_size(frame.width() - dimm::border()*2, dimm::height_button_rec() / 2)
    .with_pos(dimm::border(), dimm::border());
  frame_title.set_frame(FrameType::FlatBox);
  frame_title.set_label_size(dimm::height_text());

  // Create scrollbar
  let mut scroll = shared::fltk::ScrollList::new(
    frame.w() - dimm::border()*2
    , frame.h() - dimm::bar() - frame_title.h() - dimm::border()*3
    , frame_title.x()
    , frame_title.y() + frame_title.h() + dimm::border()
  );
  scroll.set_frame(FrameType::BorderBox);
  scroll.set_border(dimm::border(), dimm::border() + dimm::height_text());

  //
  // Create entries
  //
  let mut clone_scroll = scroll.clone();
  // let clone_tx = tx.clone();
  let clone_path_file_db_executable = path_file_db_executable.clone();
  let db_executables = shared::db::kv::read(&clone_path_file_db_executable).unwrap_or_default();
  let mut f_make_entry = move |key : String|
  {
    // Setup output for executable path
    let mut output_executable = Output::default()
      .with_size(clone_scroll.widget_ref().w() - dimm::border()*4 - dimm::width_button_rec(), dimm::height_button_wide())
      .with_align(Align::TopLeft)
      .with_label("Executable");
    let _ = output_executable.insert(key.as_str());
    output_executable.set_frame(FrameType::BorderBox);
    output_executable.set_text_size(dimm::height_text());
    clone_scroll.add(&mut output_executable.as_base_widget());

    // Use button
    let clone_path_file_db_executable = clone_path_file_db_executable.clone();
    let mut btn_use = shared::fltk::button::rect::toggle(db_executables.contains_key(&output_executable.value()))
      .right_of(&output_executable, dimm::border());

    // Label for use button
    let _ = fltk::frame::Frame::default()
      .with_size(btn_use.w(), dimm::height_text())
      .above_of(&btn_use, 2)
      .with_align(Align::Inside)
      .with_label("Use")
      .with_color(Color::BackGround)
      .with_frame(FrameType::NoBox);


    // Setup 'use button' callback
    let clone_output_executable = output_executable.clone();
    btn_use.set_callback(move |e|
    {
      if e.value()
      {
        if let Err(e) = shared::db::kv::write(&clone_path_file_db_executable, &clone_output_executable.value(), &"1".to_string())
        {
          eprintln!("Could not insert key '{}' in db: {}", clone_output_executable.value(), e);
        } // if
      }
      else
      {
        if let Err(e) = shared::db::kv::erase(&clone_path_file_db_executable, clone_output_executable.value())
        {
          eprintln!("Could not remove key '{}' from db: {}", clone_output_executable.value(), e);
        } // if
      }
    });


    // Setup input for arguments
    let mut input_arguments : fltk_evented::Listener<_> = fltk::input::Input::default()
      .with_size(clone_scroll.widget_ref().w() - dimm::border()*3, dimm::height_button_wide())
      .with_label("Arguments")
      .with_align(Align::TopLeft)
      .below_of(&output_executable, dimm::border() + dimm::height_text())
      .into();
    input_arguments.set_frame(FrameType::BorderBox);
    input_arguments.set_text_size(dimm::height_text());
    if let Ok(db) = shared::db::kv::read(&path_file_db_args) && db.contains_key(&key)
    {
      let _ = input_arguments.insert(&db[&key]);
    } // if
    let clone_output_executable = output_executable.clone();
    let clone_path_file_db = path_file_db_args.clone();
    input_arguments.on_keyup(move |e|
    {
      if e.value().is_empty()
      {
        let _ = shared::db::kv::erase(&clone_path_file_db, clone_output_executable.value());
        return;
      }
      let _ = shared::db::kv::write(&clone_path_file_db, &clone_output_executable.value(), &e.value());
    });
    clone_scroll.add(&mut input_arguments.as_base_widget());

    // Separator
    let mut sep = Frame::default()
      .below_of(&input_arguments.as_base_widget(), dimm::border())
      .with_size(clone_scroll.widget_ref().w() - dimm::border()*3, dimm::height_sep());
    sep.set_frame(FrameType::FlatBox);
    sep.set_color(Color::Black);
    clone_scroll.set_border(dimm::border(), dimm::border());
    clone_scroll.add(&mut sep.as_base_widget());
    clone_scroll.set_border(dimm::border(), dimm::border() + dimm::height_text());
  };

  // Get current database entries
  scroll.begin();
  for path in find_executables().unwrap_or_default()
  {
    f_make_entry(path.string());
  } // for
  scroll.end();

  // Back to home
  shared::fltk::button::rect::home()
    .bottom_center_of(&frame, - dimm::border())
    .emit(tx, Msg::DrawCover);

  // Back to menu
  shared::fltk::button::rect::back()
    .bottom_left_of(&frame, - dimm::border())
    .emit(tx, Msg::DrawMenu);

  RetFrameExecutable{ frame }
} // fn: new }}}

// fn: from {{{
#[allow(dead_code)]
pub fn from(tx : Sender<Msg>, w : Widget) -> RetFrameExecutable
{
  new(tx, w.x(), w.y())
} // fn: from }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
