use fltk::prelude::*;
use fltk::{
  output::Output,
  app::Sender,
  widget::Widget,
  button::Button,
  group::PackType,
  enums::{Align,FrameType,Color},
  frame::Frame,
  group::Group,
};

use anyhow::anyhow as ah;

use shared::dimm;
use shared::std::PathBufExt;
use shared::std::OsStrExt;

use crate::svg;
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
  let path_file_db = match get_path_db_args()
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
  scroll.widget_mut().set_frame(FrameType::BorderBox);
  scroll.set_border(0, dimm::border());

  //
  // Create entries
  //
  let mut clone_scroll = scroll.clone();
  // let clone_tx = tx.clone();
  let mut f_make_entry = move |key : String|
  {
    let group = Group::default()
      .with_size(clone_scroll.widget_ref().w(),
        dimm::height_button_wide()*2
        + dimm::border()*3
        + dimm::height_text()*2
        + dimm::height_sep())
      .with_pos(clone_scroll.widget_ref().x(), clone_scroll.widget_ref().y());

    // Include in scroll list
    clone_scroll.add(&mut group.as_base_widget());

    let clone_widget = clone_scroll.widget_mut();

    group.begin();

    // Setup output for executable path
    let mut output_executable = Output::default()
      .with_size(clone_widget.width() - dimm::border()*3, dimm::height_button_wide())
      .with_pos(group.x() + dimm::border(), group.y() + dimm::border() + dimm::height_text())
      .with_align(Align::TopLeft)
      .with_label("Executable");
    let _ = output_executable.insert(key.as_str());
    output_executable.set_frame(FrameType::BorderBox);
    output_executable.set_text_size(dimm::height_text());

    // // Use button
    // let mut btn_use = fltk::button::ToggleButton::default()
    //   .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    //   .right_of(&output_executable, dimm::border())
    //   .with_align(Align::TopLeft)
    //   .with_label("Use")
    //   .with_focus(false);
    // btn_use.set_selection_color(Color::Blue);
    // btn_use.set_color(Color::BackGround);

    // Setup input for arguments
    let mut input_arguments = fltk::input::Input::default()
      .with_size(clone_widget.width() - dimm::border()*3, dimm::height_button_wide())
      .with_label("Arguments")
      .with_align(Align::TopLeft)
      .below_of(&output_executable, dimm::border() + dimm::height_text());
    input_arguments.set_frame(FrameType::BorderBox);
    input_arguments.set_text_size(dimm::height_text());
    if let Ok(db) = shared::db::kv::read(&path_file_db) && db.contains_key(&key)
    {
      let _ = input_arguments.insert(&db[&key]);
    } // if
    let clone_output_executable = output_executable.clone();
    let clone_path_file_db = path_file_db.clone();
    input_arguments.set_callback(move |e|
    {
      let _ = shared::db::kv::write(&clone_path_file_db, &clone_output_executable.value(), &e.value());
    });

    // Separator
    let mut sep = Frame::default()
      .below_of(&input_arguments, dimm::border())
      .with_size(clone_widget.width() - dimm::border()*3, dimm::height_sep());
    sep.set_frame(FrameType::FlatBox);
    sep.set_color(Color::Black);

    group.end();
  };

  // Get current database entries
  scroll.begin();
  if let Ok(paths) = find_executables()
  {
    for path in paths
    {
      f_make_entry(path.string());
    } // for
  } // if
  scroll.end();

  // Back to home
  let mut btn_home = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .with_align(Align::Inside | Align::Center)
    .center_x(&frame);
  btn_home.set_pos(btn_home.x(), frame.h() - dimm::bar());
  btn_home.set_frame(FrameType::BorderBox);
  btn_home.set_label_size(dimm::height_text()*2);
  btn_home.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_home(1.0).as_str()).unwrap()));
  btn_home.emit(tx, Msg::DrawCover);

  // Back to menu
  let mut btn_back = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .with_align(Align::Inside | Align::Center);
  btn_back.set_pos(dimm::border(), frame.h() - dimm::bar());
  btn_back.set_frame(FrameType::BorderBox);
  btn_back.set_label_size(dimm::height_text()*2);
  btn_back.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_back(1.0).as_str()).unwrap()));
  btn_back.emit(tx, Msg::DrawMenu);

  RetFrameExecutable{ frame }
} // fn: new }}}

// fn: from {{{
#[allow(dead_code)]
pub fn from(tx : Sender<Msg>, w : Widget) -> RetFrameExecutable
{
  new(tx, w.x(), w.y())
} // fn: from }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
