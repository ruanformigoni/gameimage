pub mod rect
{

use fltk::prelude::*;
use fltk::enums::Align;
use crate::dimm;
use crate::svg;
use crate::fltk::WidgetExtExtra;


pub fn button<T>() -> T
  where T : Clone + Send + Sync + std::default::Default + fltk::prelude::WidgetExt
{
  T::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .with_align(Align::Inside | Align::Center)
    .with_focus(false)
    .with_frame(fltk::enums::FrameType::BorderBox)
}

pub fn filter() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_filter(1.0).as_str()) }

pub fn install() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_install(1.0).as_str()) }

pub fn home() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_home(1.0).as_str()) }

pub fn back() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_back(1.0).as_str()) }

pub fn configure() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_configure(1.0).as_str()) }

pub fn list() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_list(1.0).as_str()) }

pub fn switch() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_switch(1.0).as_str()) }

pub fn play() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_play(1.0).as_str()) }

pub fn add() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_add(1.0).as_str()) }

pub fn del() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_del(1.0).as_str()) }

pub fn folder() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_folder(1.0).as_str()) }

pub fn save() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_save(1.0).as_str()) }

pub fn check() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_check(1.0).as_str()) }

pub fn cloud() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_cloud(1.25).as_str()) }

pub fn refresh() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_refresh(1.0).as_str()) }

pub fn joystick() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_joystick(1.0).as_str()) }

pub fn arrow_forward() -> fltk::button::Button { button::<fltk::button::Button>().with_svg(svg::icon_arrow_forward(2.0).as_str()) }

pub fn toggle(value : bool) -> fltk::button::ToggleButton
{
  let mut btn  = button::<fltk::button::ToggleButton>()
    .with_frame(fltk::enums::FrameType::FlatBox);

  // Set image
  btn.draw(move |e|
  {
    fltk::draw::draw_rect_fill(e.x()+2, e.y()+2, e.w()-4, e.h()-4, fltk::enums::Color::from_hex_str("#ffffff").unwrap());
    // -1 on w and h because if is drawn with the same size as the button it leaves a weird border when updated
    match e.value()
    {
      true =>
      {
        fltk::image::SvgImage::from_data(&svg::with_size::icon_box_selected(dimm::width_button_rec()-1, dimm::height_button_rec()-1))
          .unwrap()
          .draw(e.x(), e.y(), e.w(), e.h());
      },
      false =>
      {
        fltk::image::SvgImage::from_data(&svg::with_size::icon_box_deselected(dimm::width_button_rec()-1, dimm::height_button_rec()-1))
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
  let mut btn  = button::<fltk::button::RadioButton>()
    .with_frame(fltk::enums::FrameType::FlatBox);

  // Set image
  btn.draw(move |e|
  {
    fltk::draw::draw_rect_fill(e.x()+2, e.y()+2, e.w()-4, e.h()-4, fltk::enums::Color::from_hex_str("#ffffff").unwrap());
    match e.value()
    {
      true =>
      {
        fltk::image::SvgImage::from_data(&svg::with_size::icon_box_selected(dimm::width_button_rec()-1, dimm::height_button_rec()-1))
          .unwrap()
          .draw(e.x(), e.y(), e.w(), e.h());
      },
      false =>
      {
        fltk::image::SvgImage::from_data(&svg::with_size::icon_box_deselected(dimm::width_button_rec()-1, dimm::height_button_rec()-1))
          .unwrap()
          .draw(e.x(), e.y(), e.w(), e.h());
      },
    } // match
  });

  btn
} // radio()

} // pub mod rect

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
