use std::env;

use fltk::prelude::*;
use fltk::{
  app::Sender,
  group::Group,
  button::Button,
  widget::Widget,
  group::PackType,
  prelude::WidgetExt,
  enums::FrameType,
  frame::Frame,
  image::SharedImage,
};

use crate::svg;
use crate::dimm;
use crate::common;
use common::Msg;

pub struct RetFrameCover
{
  pub frame : Frame,
  pub btn_left : Button,
  pub btn_right : Button,
} // Ret

// fn: new {{{
pub fn new(tx : Sender<Msg>, x : i32, y : i32) -> RetFrameCover
{
  let mut frame_base = Frame::default().with_size(dimm::width(), dimm::height());
  frame_base.set_type(PackType::Vertical);
  frame_base.set_frame(FrameType::FlatBox);

  let mut group_content = Group::default()
    .with_size(dimm::width() - dimm::border()*2, dimm::height() - (dimm::height_button_wide() + dimm::border() * 2) - dimm::border()*2)
    .with_pos(dimm::border(), dimm::border());
  group_content.set_type(PackType::Vertical);
  group_content.set_frame(FrameType::FlatBox);
  group_content.begin();

  let mut frame = Frame::default()
    .with_size(dimm::width(), dimm::height())
    .with_pos(x,y);
  frame.set_type(PackType::Vertical);
  frame.set_frame(FrameType::FlatBox);

  // Cover image
  if let Some(env_image_launcher) = env::var("GIMG_LAUNCHER_IMG").ok()
  {
    if let Some(mut shared_image) = SharedImage::load(env_image_launcher).ok()
    {
      let img_height = frame.h();
      let img_width  = frame.w();
      let mut frame_image = Frame::default().with_size(img_width, img_height)
        .with_pos(frame.x(), frame.y())
        .with_type(PackType::Vertical);
      frame_image.set_frame(FrameType::FlatBox);
      frame_image.draw(move |f| {
        shared_image.scale(f.w(), f.h(), true, true);
        shared_image.draw(f.x() + (img_width - shared_image.width())/2, f.y(), f.w(), f.h());
      });
    } // if
    else
    {
      println!("Failed to load launcher image");
    } // else
  } // if
  else
  {
    println!("Failed to fetch environment variable GIMG_LAUNCHER_IMG")
  } // else

  group_content.end();

  // Set bottom background
  let mut btn_bottom = Button::default()
    .with_size(frame_base.width(), dimm::bar())
    .center_x(&frame_base);
  btn_bottom.set_pos(btn_bottom.x(), frame_base.h() - dimm::bar());
  btn_bottom.set_frame(FrameType::NoBox);
  btn_bottom.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_background().as_str()).unwrap()));
  btn_bottom.deactivate();

  // Button left aligned
  let mut btn_left = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .below_of(&frame_base, -dimm::height_button_rec());
  btn_left.set_pos(btn_left.x() + dimm::border(), btn_left.y() - dimm::border());
  btn_left.set_frame(FrameType::BorderBox);
  btn_left.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_list().as_str()).unwrap()));
  btn_left.emit(tx, Msg::DrawMenu);

  // Button right aligned
  let mut btn_right = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .below_of(&frame_base, -dimm::height_button_rec());
  btn_right.set_pos(frame_base.w() - dimm::border() - btn_right.w(), btn_right.y() - dimm::border());
  btn_right.set_frame(FrameType::BorderBox);
  btn_right.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_play().as_str()).unwrap()));
  btn_right.emit(tx, Msg::Launch);

  RetFrameCover{ frame, btn_left, btn_right }
} // fn: new }}}

// fn: from {{{
#[allow(dead_code)]
pub fn from(tx : Sender<Msg>, w : Widget) -> RetFrameCover
{
  new(tx, w.x(), w.y())
} // fn: from }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
