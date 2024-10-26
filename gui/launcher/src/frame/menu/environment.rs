use fltk::prelude::*;
use fltk::{
  output::Output,
  app::Sender,
  widget::Widget,
  button::Button,
  group::{PackType,Scroll},
  enums::{Align,FrameType,Color},
  frame::Frame,
  group::Group,
};

use shared::dimm;
use shared::fltk::WidgetExtExtra;

use crate::svg;
use crate::common::Msg;

pub struct RetFrameEnv
{
  pub frame : Frame,
} // Ret

// get_path_db_env() {{{
fn get_path_db_env() -> anyhow::Result<std::path::PathBuf>
{
  let mut path_db : std::path::PathBuf = std::env::var("GIMG_LAUNCHER_ROOT")?.into();
  path_db.push("gameimage.env.json");

  Ok(path_db)
} // get_path_db_env() }}}

// fn: new {{{
pub fn new(tx : Sender<Msg>, x : i32, y : i32) -> RetFrameEnv
{
  let path_file_db = match get_path_db_env()
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
    .with_label("Environment Variables")
    .with_size(frame.width() - dimm::border()*2, dimm::height_button_rec() / 2)
    .with_pos(dimm::border(), dimm::border());
  frame_title.set_frame(FrameType::FlatBox);
  frame_title.set_label_size(dimm::height_text());

  // Create scrollbar
  let mut scroll = Scroll::default()
    .below_of(&frame_title, dimm::border())
    .with_size(frame.w() - dimm::border()*2, frame.h() - dimm::bar() - frame_title.h() - dimm::border() * 3);
  scroll.set_frame(FrameType::BorderBox);
  scroll.set_scrollbar_size(dimm::width_button_rec() / 4);


  //
  // Create entries
  //
  let mut parent = scroll.as_base_widget();
  let clone_scroll = scroll.clone();
  let clone_tx = tx.clone();
  let clone_path_file_db = path_file_db.clone();
  let mut f_make_entry = move |key : String, val : String|
  {
    let mut group = Group::default()
      .with_size(clone_scroll.w(), dimm::height_button_wide()*2 + dimm::border() * 3)
      .with_pos(clone_scroll.x(), clone_scroll.y());
    // Position below parent
    if parent.is_same(&clone_scroll.as_base_widget())
    {
      group.clone().above_of(&parent, -group.h());
      group.set_pos(group.x(), group.y());
    } // if
    else
    {
      group.clone().above_of(&parent, -group.h() * 2 - dimm::border());
    } // else

    group.begin();

    // Setup key widget
    let mut btn_key = Output::default()
      .with_size(clone_scroll.width() - dimm::width_button_rec() - dimm::border()*3, dimm::height_button_wide())
      .with_align(Align::Left | Align::Inside)
      .with_pos(group.x() + dimm::border(), group.y() + dimm::border());
    btn_key.set_value(key.as_str());
    btn_key.set_frame(FrameType::BorderBox);
    btn_key.set_text_size(dimm::height_text());
    // Setup val widget
    let mut btn_val = Output::default()
      .with_size(clone_scroll.width() - dimm::border()*2, dimm::height_button_wide())
      .with_align(Align::Left | Align::Inside)
      .below_of(&btn_key, dimm::border());
    btn_val.set_value(val.as_str());
    btn_val.set_frame(FrameType::BorderBox);
    btn_val.set_text_size(dimm::height_text());
    // Erase button
    let mut btn_del = Button::default()
      .with_size(dimm::width_button_rec(), dimm::height_button_rec())
      .with_focus(false)
      .right_of(&btn_key, dimm::border());
    btn_del.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_del(1.0).as_str()).unwrap()));
    btn_del.set_color(Color::Red);
    let clone_key = key.clone();
    let clone_tx = clone_tx.clone();
    let clone_path_file_db = clone_path_file_db.clone();
    btn_del.set_callback(move |_|
    {
      match shared::db::kv::erase(&clone_path_file_db, clone_key.clone())
      {
        Ok(_) => println!("Erased key '{}'", clone_key),
        Err(e) => println!("Failed to erase key '{}' with error '{}'", clone_key, e.to_string()),
      } // if
      clone_tx.send(Msg::DrawEnv);
    });
    // Separator
    let mut sep = Frame::default()
      .below_of(&btn_val, dimm::border())
      .with_size(clone_scroll.width() - dimm::border()*2, 2);
    sep.set_frame(FrameType::FlatBox);
    sep.set_color(Color::Black);

    group.end();

    // Update parent
    parent = group.as_base_widget().clone();
  };

  // Get current database entries
  if let Ok(entries) = shared::db::kv::read(&path_file_db)
  {
    for (key, val) in entries
    {
      f_make_entry(key, val);
    } // for
  } // if

  scroll.end();

  // Add var button
  let mut btn_add = shared::fltk::button::rect::add()
    .with_focus(false)
    .with_color(Color::Green)
    .right_bottom_of(&frame, - dimm::border());
  let clone_tx = tx.clone();
  btn_add.set_callback(move |_|
  {
    let dialog = shared::fltk::dialog::key_value();
    let clone_dialog = dialog.clone();
    let clone_tx = clone_tx.clone();
    let clone_path_file_db = path_file_db.clone();
    dialog.btn_ok.clone().set_callback(move |_|
    {
      clone_dialog.wind.clone().hide();
      let key = clone_dialog.input_key.value();
      let value = clone_dialog.input_value.value();
      if key.is_empty() { return; }
      match shared::db::kv::write(&clone_path_file_db, &key.clone(), &value.clone())
      {
        Ok(_) => println!("Set key '{}' with value '{}'", key.clone(), value.clone()),
        Err(e) => println!("Failed to set key '{}' with error '{}'", key, e.to_string()),
      } // if
      clone_tx.send(Msg::DrawEnv);
    });
    dialog.wind.clone().show();
  });

  // Back to home
  shared::fltk::button::rect::home()
    .bottom_center_of(&frame, - dimm::border())
    .emit(tx, Msg::DrawCover);

  // Back to menu
  shared::fltk::button::rect::back()
    .bottom_left_of(&frame, - dimm::border())
    .emit(tx, Msg::DrawMenu);

  RetFrameEnv{ frame }
} // fn: new }}}

// fn: from {{{
#[allow(dead_code)]
pub fn from(tx : Sender<Msg>, w : Widget) -> RetFrameEnv
{
  new(tx, w.x(), w.y())
} // fn: from }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
