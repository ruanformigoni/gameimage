use fltk::{
  draw,
  enums::{Align,Color},
};

use crate::fltk::WidgetExtExtra;

fn button<T>() -> T
  where T : Clone
  + Send
  + Sync
  + std::default::Default
  + fltk::prelude::WidgetExt
  + fltk::prelude::WidgetBase
  + fltk::prelude::ButtonExt
{
  let mut btn = T::default()
    .with_align(Align::Inside | Align::Center)
    .with_color(Color::BackGround.lighter())
    .with_color_selected(Color::BackGround.darker())
    .with_focus(false)
    .with_frame(fltk::enums::FrameType::NoBox);

  btn.draw(move |b|
  {
    let (x,y,w,h) = (b.x(),b.y(),b.w(),b.h());
    let c = if ! b.active() { b.color().darker() } else if b.is_set() { b.color().lighter() } else { b.color() };
    draw::set_draw_color(c);
    draw::draw_rounded_rectf(x, y, w, h, 3);
    if let Some(mut image) = b.image() {
      let img_w = image.width();
      let img_h = image.height();
      let img_x = x + (w - img_w) / 2; // Center horizontally
      let img_y = y + (h - img_h) / 2; // Center vertically
      image.draw(img_x, img_y, img_w, img_h);
    }
    // Label (change color if pressed)
    draw::set_draw_color(b.label_color());
    draw::draw_text2(&b.label(), x, y, w, h, b.align());
  });

  btn
}

pub mod wide
{

use fltk::prelude::*;

pub fn default() -> fltk::button::Button
{
  crate::fltk::button::button::<fltk::button::Button>()
    .with_size(crate::dimm::width_button_wide(), crate::dimm::height_button_wide())
}

} // pub mod rect

pub mod rect
{

use fltk::{
  draw,
  prelude::*,
  enums::Color,
};

use crate::dimm;
use crate::svg;
use crate::svg::*;
use crate::fltk::WidgetExtExtra;
use crate::fltk::button;

macro_rules! create_buttons
{
  ($($name:ident),*) =>
  {
    $(
      pub fn $name() -> fltk::button::Button
      {
        button::button::<fltk::button::Button>()
            .with_svg((concat_idents!(icon_,$name)(1.0).as_str()))
            .with_size(dimm::width_button_rec(), dimm::height_button_rec())
      }
    )*
  };
}

create_buttons!(search, terminal, filter, install , home, back, configure, list, switch, add, del,
  folder, save, check, cloud, refresh, joystick, arrow_forward, play
);

pub fn checkbutton() -> fltk::button::CheckButton
{
  let mut btn  = button::button::<fltk::button::CheckButton>()
    .with_frame(fltk::enums::FrameType::NoBox);

  // Set image
  btn.draw(move |e|
  {
    fltk::draw::draw_rect_fill(e.x(), e.y(), e.w(), e.h(), e.color());
    // -1 on w and h because if is drawn with the same size as the button it leaves a weird border when updated
    let w = 18;
    let h = 18;
    let x = e.x();
    let y = e.y() + (e.h() / 2) - (h / 2);
    match e.is_checked()
    {
      true =>
      {
        fltk::draw::set_draw_color(Color::Blue);
        fltk::draw::draw_rounded_rectf(x, y, w, h, 4);
        fltk::draw::draw_rounded_rect(x, y, w, h, 4);
      },
      false =>
      {
        fltk::draw::set_draw_color(Color::BackGround2);
        fltk::draw::draw_rounded_rectf(x, y, w, h, 4);
        fltk::draw::draw_rounded_rect(x, y, w, h, 4);
      },
    } // match
    // Draw label
    draw::set_draw_color(Color::White);
    draw::draw_text2(&format!(" {}", e.label()), e.x() + w, e.y(), e.w(), e.h(), e.align());
  });

  btn
} // toggle()

pub fn toggle(value : bool) -> fltk::button::ToggleButton
{
  let mut btn  = button::button::<fltk::button::ToggleButton>()
    .with_frame(fltk::enums::FrameType::FlatBox);

  // Set image
  btn.draw(move |e|
  {
    fltk::draw::draw_rect_fill(e.x(), e.y(), e.w(), e.h(), Color::BackGround);
    match e.value()
    {
      true =>
      {
        fltk::image::SvgImage::from_data(&svg::with_size::icon_box_selected(dimm::width_button_rec(), dimm::height_button_rec()))
          .unwrap()
          .draw(e.x(), e.y(), e.w(), e.h());
      },
      false =>
      {
        fltk::image::SvgImage::from_data(&svg::with_size::icon_box_deselected(dimm::width_button_rec(), dimm::height_button_rec()))
          .unwrap()
          .draw(e.x(), e.y(), e.w(), e.h());
      },
    } // match
  });

  btn.set_value(value);

  btn
} // toggle()

pub fn radio() -> fltk::button::RadioButton
{
  let mut btn  = button::button::<fltk::button::RadioButton>()
    .with_frame(fltk::enums::FrameType::FlatBox);

  // Set image
  btn.draw(move |e|
  {
    fltk::draw::draw_rect_fill(e.x(), e.y(), e.w(), e.h(), Color::BackGround);
    match e.value()
    {
      true =>
      {
        fltk::draw::draw_rect_fill(e.x(), e.y(), e.w(), e.h(), Color::Green);
        fltk::image::SvgImage::from_data(&svg::with_size::icon_box_selected(dimm::width_button_rec(), dimm::height_button_rec()))
          .unwrap()
          .draw(e.x(), e.y(), e.w(), e.h());
      },
      false =>
      {
        fltk::draw::draw_rect_fill(e.x(), e.y(), e.w(), e.h(), Color::BackGround.lighter());
        fltk::image::SvgImage::from_data(&svg::with_size::icon_box_deselected(dimm::width_button_rec(), dimm::height_button_rec()))
          .unwrap()
          .draw(e.x(), e.y(), e.w(), e.h());
      },
    } // match
  });

  btn
} // radio()

} // pub mod rect

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
