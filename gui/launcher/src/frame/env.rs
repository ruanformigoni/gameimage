use fltk::prelude::*;
use fltk::{
  input::Input,
  output::Output,
  window::Window,
  app::Sender,
  widget::Widget,
  button::Button,
  group::{PackType,Scroll},
  enums::{Align,FrameType,Color},
  frame::Frame,
  group::Group,
};

use crate::dimm;
use crate::svg;
use crate::db;
use crate::common::Msg;

pub struct RetFrameEnv
{
  pub frame : Frame,
} // Ret


// fn: new {{{
pub fn new(tx : Sender<Msg>, x : i32, y : i32) -> RetFrameEnv
{
  //
  // Main
  //
  let mut frame = Frame::default()
    .with_size(dimm::width(), dimm::height())
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
      .right_of(&btn_key, dimm::border());
    btn_del.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_del().as_str()).unwrap()));
    btn_del.set_color(Color::Red);
    let clone_key = key.clone();
    let clone_tx = clone_tx.clone();
    btn_del.set_callback(move |_|
    {
      match db::env::del(clone_key.clone())
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
  if let Ok(entries) = db::env::get()
  {
    for db::env::Var{ key, val } in entries.env
    {
      f_make_entry(key, val);
    } // for
  } // if

  scroll.end();

  // Add var button
  let mut btn_add = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .with_align(Align::Inside | Align::Center)
    .with_label("+");
  let clone_tx = tx.clone();
  btn_add.set_pos(frame.w() - btn_add.w() - dimm::border(), frame.h() - dimm::bar());
  btn_add.set_frame(FrameType::BorderBox);
  btn_add.set_label_size(dimm::height_text()*2);
  btn_add.set_color(Color::Green);
  btn_add.set_callback(move |_|
  {
    let mut wind = Window::default()
      .with_size(
          dimm::width_button_wide() * 4 + dimm::border() * 3
        , dimm::height_button_wide() * 3 + dimm::border() * 4
      );
    wind.begin();
    let input_key = Input::default()
      .with_pos(wind.w() - dimm::width_button_wide()*3 - dimm::border(), dimm::border())
      .with_size(dimm::width_button_wide()*3, dimm::height_button_wide())
      .with_align(Align::Left);
    let _label_key = Frame::default()
      .with_size(dimm::width_button_wide(), dimm::height_button_wide())
      .left_of(&input_key, dimm::border())
      .with_align(Align::Inside | Align::Left)
      .with_label("Key");
    let input_value = Input::default()
      .below_of(&input_key, dimm::border())
      .with_size(input_key.w(), input_key.h())
      .with_align(input_key.align());
    let label_value = Frame::default()
      .with_size(dimm::width_button_wide(), dimm::height_button_wide())
      .left_of(&input_value, dimm::border())
      .with_align(Align::Inside | Align::Left)
      .with_label("Value");
    let mut btn_ok = Button::default()
      .with_size(dimm::width_button_wide(), dimm::height_button_wide())
      .below_of(&label_value, dimm::border())
      .with_label("OK");
    btn_ok.set_pos(wind.w() / 2 - btn_ok.w() / 2, btn_ok.y());
    btn_ok.set_color(Color::Green);
    let mut clone_wind = wind.clone();
    let clone_input_key = input_key.clone();
    let clone_input_value = input_value.clone();
    let clone_tx = clone_tx.clone();
    btn_ok.set_callback(move |_|
    {
      clone_wind.hide();
      let key = clone_input_key.value();
      let value = clone_input_value.value();
      if key.is_empty() { return; }
      match db::env::set(key.clone(), value.clone())
      {
        Ok(_) => println!("Set key '{}' with value '{}'", key.clone(), value.clone()),
        Err(e) => println!("Failed to set key '{}' with error '{}'", key, e.to_string()),
      } // if
      clone_tx.send(Msg::DrawEnv);
    });
    wind.end();
    wind.show();
  });

  // Back to home
  let mut btn_home = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .with_align(Align::Inside | Align::Center)
    .center_x(&frame);
  btn_home.set_pos(btn_home.x(), frame.h() - dimm::bar());
  btn_home.set_frame(FrameType::BorderBox);
  btn_home.set_label_size(dimm::height_text()*2);
  btn_home.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_home().as_str()).unwrap()));
  btn_home.emit(tx, Msg::DrawCover);

  // Back to menu
  let mut btn_back = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .with_align(Align::Inside | Align::Center);
  btn_back.set_pos(dimm::border(), frame.h() - dimm::bar());
  btn_back.set_frame(FrameType::BorderBox);
  btn_back.set_label_size(dimm::height_text()*2);
  btn_back.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_back().as_str()).unwrap()));
  btn_back.emit(tx, Msg::DrawMenu);

  RetFrameEnv{ frame }
} // fn: new }}}

// fn: from {{{
#[allow(dead_code)]
pub fn from(tx : Sender<Msg>, w : Widget) -> RetFrameEnv
{
  new(tx, w.x(), w.y())
} // fn: from }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
