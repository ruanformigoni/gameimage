pub mod rect
{

use fltk::prelude::*;
use fltk::enums::{Align, FrameType};
use crate::dimm;
use crate::svg;
use crate::fltk::WidgetExtExtra;

pub fn button() -> fltk::button::Button
{
  fltk::button::Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .with_align(Align::Inside | Align::Center)
    .with_focus(false)
    .with_frame(FrameType::BorderBox)
}

pub fn home() -> fltk::button::Button
{
  button().with_svg(svg::icon_home(1.0).as_str())
}

pub fn back() -> fltk::button::Button
{
  button().with_svg(svg::icon_back(1.0).as_str())
}

pub fn list() -> fltk::button::Button
{
  button().with_svg(svg::icon_list(1.0).as_str())
}

pub fn switch() -> fltk::button::Button
{
  button().with_svg(svg::icon_switch(1.0).as_str())
}

pub fn play() -> fltk::button::Button
{
  button().with_svg(svg::icon_play(1.0).as_str())
}

pub fn add() -> fltk::button::Button
{
  button().with_svg(svg::icon_add(1.0).as_str())
}

pub fn toggle(value : bool) -> fltk::button::ToggleButton
{
  let mut btn = fltk::button::ToggleButton::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .with_align(Align::Inside | Align::Center)
    .with_focus(false)
    .with_frame(FrameType::BorderBox);

  let mut listener : fltk_evented::Listener<_> = btn.clone().into();

  btn.set_value(value);

  // Initial image
  if value
  {
    btn.with_svg(&svg::with_size::icon_box_selected(dimm::width_button_rec(), dimm::height_button_rec()));
  } // if
  else
  {
    btn.with_svg(&svg::with_size::icon_box_deselected(dimm::width_button_rec(), dimm::height_button_rec()));
  } // else

  // Update image
  listener.on_click(move |e|
  {
    if e.value()
    {
      e.with_svg(&svg::with_size::icon_box_selected(dimm::width_button_rec(), dimm::height_button_rec()));
    } // if
    else
    {
      e.with_svg(&svg::with_size::icon_box_deselected(dimm::width_button_rec(), dimm::height_button_rec()));
    } // else
  });

  btn
}

}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
