use fltk::
{
  frame::Frame,
  window::Window,
  prelude::*,
  enums::{Align,Color},
};

use crate::dimm;
use crate::svg;
use crate::hover_blink;

#[derive(Clone)]
pub struct KeyValue
{
  pub wind : Window,
  pub input_key : fltk::input::Input,
  pub input_value : fltk::input::Input,
  pub btn_ok : fltk::button::Button,
}

pub fn key_value() -> KeyValue
{
  let mut wind = Window::default().with_size(
      dimm::width_button_wide() * 4 + dimm::border() * 3
    , dimm::height_button_wide() * 3 + dimm::border() * 4
  );
  // Window should be de-attached from other windows
  if let Some(mut parent) = wind.parent()
  {
    parent.remove(&wind);
  } // if
  // Window icon
  if let Some(image) = fltk::image::SvgImage::from_data(svg::ICON_GAMEIMAGE).ok()
  {
    wind.set_icon(Some(image.clone()));
  } // if
  wind.begin();
  let input_key = fltk::input::Input::default()
    .with_pos(wind.w() - dimm::width_button_wide()*3 - dimm::border(), dimm::border())
    .with_size(dimm::width_button_wide()*3, dimm::height_button_wide())
    .with_align(Align::Left);
  let _label_key = Frame::default()
    .with_size(dimm::width_button_wide(), dimm::height_button_wide())
    .left_of(&input_key, dimm::border())
    .with_align(Align::Inside | Align::Left)
    .with_label("Key");
  let input_value = fltk::input::Input::default()
    .below_of(&input_key, dimm::border())
    .with_size(input_key.w(), input_key.h())
    .with_align(input_key.align());
  let label_value = Frame::default()
    .with_size(dimm::width_button_wide(), dimm::height_button_wide())
    .left_of(&input_value, dimm::border())
    .with_align(Align::Inside | Align::Left)
    .with_label("Value");
  let mut btn_ok = crate::fltk::button::wide::default()
    .with_size(dimm::width_button_wide(), dimm::height_button_wide())
    .below_of(&label_value, dimm::border())
    .with_label("OK");
  hover_blink!(btn_ok);
  btn_ok.set_pos(wind.w() / 2 - btn_ok.w() / 2, btn_ok.y());
  btn_ok.set_color(Color::Green);
  wind.end();

  KeyValue{ wind, input_key, input_value, btn_ok }
}
// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
