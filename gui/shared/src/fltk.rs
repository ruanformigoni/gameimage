#![allow(special_module_name)]

use fltk::prelude::*;
use fltk::enums::*;
use fltk_theme;

use crate::dimm;

pub mod button;
pub mod dialog;
pub mod progress;
pub mod separator;
pub mod frame;
pub mod macros;

// pub fn theme() {{{
pub fn theme()
{

  if let Ok(font) = Font::load_font("/usr/share/fonts/noto/NotoSans-Regular.ttf")
  {
    Font::set_font(Font::Helvetica, &font);
    fltk::app::set_font(Font::Helvetica);
    fltk::app::set_font_size(12);
  } // if

  fltk_theme::ColorTheme::new(fltk_theme::color_themes::BLACK_THEME).apply();
  fltk::app::set_visible_focus(false);
  fltk::app::set_font_size(dimm::height_text());
  let set_color = |c: Color, hex: &str|
  {
    let r = Color::from_hex_str(hex).unwrap();
    let r = Color::darker(&r).to_rgb();
    fltk::app::set_color(c, r.0, r.1, r.2);
  };
  let str_black = "#35353A";
  let str_white = "#eeeeee";
  set_color(Color::White       , str_white);
  set_color(Color::Black       , str_black);
  set_color(Color::ForeGround  , str_white);
  set_color(Color::Foreground  , str_white);
  set_color(Color::BackGround  , str_black);
  set_color(Color::Background  , str_black);
  set_color(Color::BackGround2 , &Color::from_hex_str(str_black).unwrap().darker().to_hex_str());
  set_color(Color::Background2 , &Color::from_hex_str(str_black).unwrap().darker().to_hex_str());
  set_color(Color::Red         , "#F050A0");
  set_color(Color::Blue        , "#00A0F0");
  set_color(Color::Green       , "#00FF60");
  set_color(Color::Yellow      , "#F0F070");
  set_color(Color::Magenta     , "#D080F0");
  set_color(Color::Cyan        , "#70D0F0");
  set_color(Color::DarkRed     , &Color::darker(&Color::DarkRed).to_hex_str());
  set_color(Color::DarkBlue    , &Color::darker(&Color::DarkBlue).to_hex_str());
  set_color(Color::DarkGreen   , &Color::darker(&Color::DarkGreen).to_hex_str());
  set_color(Color::DarkYellow  , &Color::darker(&Color::DarkYellow).to_hex_str());
  set_color(Color::DarkMagenta , &Color::darker(&Color::DarkMagenta).to_hex_str());
  set_color(Color::DarkCyan    , &Color::darker(&Color::DarkCyan).to_hex_str());
  fltk::app::set_frame_color(Color::White);
  fltk::app::foreground(230,230,230);
  let color = Color::from_hex_str("#5294e2").unwrap().to_rgb();
  fltk::app::set_selection_color(color.0, color.1, color.2);
  fltk::app::set_frame_type(FrameType::BorderBox);
  fltk_theme::WidgetScheme::new(fltk_theme::SchemeType::Clean).apply();
} // pub fn theme() }}}

// pub trait WidgetExtExtra {{{
#[allow(warnings)]
pub trait WidgetExtExtra
{
  fn with_callback<F>(&mut self, callback : F) -> Self
    where F: FnMut(&mut Self) + 'static;
  fn with_frame(&mut self, frame : fltk::enums::FrameType) -> Self;
  fn with_svg(&mut self, data : &str) -> Self;
  fn with_shared_image(&mut self, path : std::path::PathBuf) -> Self;
  fn with_focus(&mut self, use_focus : bool) -> Self;
  fn with_color(&mut self, color : fltk::enums::Color) -> Self;
  fn with_color_selected(&mut self, color : fltk::enums::Color) -> Self;
  fn with_border(&mut self, x_border : i32, y_border : i32) -> Self;
  fn right_bottom_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn top_left_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn top_right_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn top_center_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn bottom_center_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn bottom_left_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn bottom_right_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn bottom_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn below_center_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn with_pos_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self;
  fn with_posx_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self;
  fn with_posy_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self;
  fn with_size_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self;
  fn with_width(&mut self, width : i32) -> Self;
  fn with_height(&mut self, height : i32) -> Self;
  fn set_width(&mut self, width : i32);
  fn set_height(&mut self, height : i32);
  fn with_width_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self;
  fn with_height_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self;
}

impl<T: WidgetExt + Clone> WidgetExtExtra for T
{
  fn with_callback<F>(&mut self, mut callback : F) -> Self
    where F: FnMut(&mut Self) + 'static
  {
    self.set_callback(move |e| callback(e));
    self.clone()
  }

  fn with_frame(&mut self, frame : fltk::enums::FrameType) -> Self
  {
    self.set_frame(frame);
    self.clone()
  }

  fn with_svg(&mut self, data : &str) -> Self
  {
    self.set_image(Some(fltk::image::SvgImage::from_data(data).unwrap()));
    self.clone()
  }

  fn with_shared_image(&mut self, path : std::path::PathBuf) -> Self
  {
    self.set_image_scaled(Some(fltk::image::SharedImage::load(path).unwrap()));
    self.clone()
  }

  fn with_focus(&mut self, use_focus : bool) -> Self
  {
    self.visible_focus(use_focus);
    self.clone()
  }

  fn with_color(&mut self, color : fltk::enums::Color) -> Self
  {
    self.set_color(color);
    self.clone()
  }

  fn with_color_selected(&mut self, color : fltk::enums::Color) -> Self
  {
    self.set_selection_color(color);
    self.clone()
  }

  fn with_border(&mut self, x_border : i32, y_border : i32) -> Self
  {
    self.set_pos(self.x() + x_border, self.y() + y_border);
    self.clone()
  }

  fn right_bottom_of<W: WidgetExt>(&mut self, other: &W, offset : i32) -> Self
  {
    self.set_pos(
        other.x() + other.w() - self.w() + offset
      , other.y() + other.h() - self.h() + offset
    );
    self.clone()
  }

  fn top_left_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self
  {
    self.set_pos(
        other.x()
      , other.y() + offset
    );
    self.clone()
  }

  fn top_right_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self
  {
    self.set_pos(
        other.x() + other.w() - self.w() + offset
      , other.y()
    );
    self.clone()
  }

  fn top_center_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self
  {
    self.set_pos(
        other.x() + (other.w() / 2) - (self.w() / 2)
      , other.y() + offset
    );
    self.clone()
  }

  fn bottom_center_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self
  {
    self.set_pos(
        other.x() + (other.w() / 2) - (self.w() / 2)
      , other.y() + other.h() - self.h() + offset
    );
    self.clone()
  }

  fn bottom_left_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self
  {
    self.set_pos(
        other.x() - offset
      , other.y() + other.h() - self.h() + offset
    );
    self.clone()
  }

  fn bottom_right_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self
  {
    self.set_pos(
        other.x() + other.w() - self.w() + offset
      , other.y() + other.h() - self.h() + offset
    );
    self.clone()
  }

  fn bottom_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self
  {
    self.set_pos(self.x(), other.y() + other.h() - self.h() + offset);
    self.clone()
  }

  fn below_center_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self
  {
    self.set_pos(
        other.x() + (other.w() / 2) - (self.w() / 2)
      , other.y() + other.h() + offset
    );
    self.clone()
  }

  fn with_pos_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self
  {
    self.set_pos(other.x(), other.y());
    self.clone()
  }

  fn with_posx_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self
  {
    self.set_pos(other.x(), self.y());
    self.clone()
  }

  fn with_posy_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self
  {
    self.set_pos(self.x(), other.y());
    self.clone()
  }

  fn with_size_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self
  {
    self.set_size(other.w(), other.h());
    self.clone()
  }

  fn with_width(&mut self, width : i32) -> Self
  {
    self.set_size(width, self.h());
    self.clone()
  }

  fn with_height(&mut self, height : i32) -> Self
  {
    self.set_size(self.w(), height);
    self.clone()
  }

  fn set_width(&mut self, width : i32)
  {
    self.set_size(width, self.h());
  }

  fn set_height(&mut self, height : i32)
  {
    self.set_size(self.w(), height);
  }

  fn with_width_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self
  {
    self.set_size(other.w(), self.h());
    self.clone()
  }

  fn with_height_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self
  {
    self.set_size(self.w(), other.h());
    self.clone()
  }
}
// }}}

// pub trait SenderExt {{{
pub trait SenderExt<T>
{
  fn send_awake(&self, value: T);
  fn send_activate(&self, value: T);
} // pub trait SenderExt

impl<T: 'static + Send + Sync> SenderExt<T> for fltk::app::Sender<T>
{
  fn send_awake(&self, value: T)
  {
    // Send
    self.send(value);
    // Awake app
    fltk::app::awake();
  } // send_awake

  fn send_activate(&self, value: T)
  {
    // Send
    self.send(value);
    // Activate
    for w in fltk::app::windows().unwrap_or_default()
    {
      ( 0..w.children() )
        .into_iter()
        .for_each(|e| { w.child(e).unwrap().clone().activate() });
    } // for
    fltk::app::redraw();
    fltk::app::awake();
  } // send_awake
} // impl }}}

// pub struct ScrollList {{{

#[derive(Clone)]
pub struct ScrollList
{
  border_x : i32,
  border_y : i32,
  scroll : fltk::group::Scroll,
  frame_type : fltk::frame::Frame,
  opt_current : Option<fltk::widget::Widget>,
} // ScrollList

impl ScrollList
{
  pub fn new(w : i32, h : i32, x : i32, y : i32) -> Self
  {
    let frame_type = fltk::frame::Frame::default()
      .with_size(w+2, h+2)
      .with_pos(x-1, y-1)
      .with_frame(fltk::enums::FrameType::BorderBox);

    let mut scroll = fltk::group::Scroll::default()
      .with_size(w, h)
      .with_pos(x, y);
    scroll.set_scrollbar_size(dimm::border());

    ScrollList{border_x: 0, border_y: 0, scroll: scroll.clone(), frame_type, opt_current: None}
  } // new()

  pub fn begin(&self)
  {
    self.scroll.begin();
  } // begin()

  pub fn end(&self)
  {
    self.scroll.end();
  } // end()

  pub fn widget_mut(&mut self) -> &mut fltk::group::Scroll
  {
    &mut self.scroll
  } // widget_mut()

  pub fn widget_ref(&self) -> &fltk::group::Scroll
  {
    &self.scroll
  } // widget_ref()

  pub fn add(&mut self, w : &mut fltk::widget::Widget)
  {
    if self.opt_current.is_some()
    {
      w.clone().below_of(&self.opt_current.clone().unwrap(), self.border_y);
    }
    else
    {
      // Create an empty widget to serve as a spacer for x
      let frame_spacer_x = fltk::frame::Frame::default()
        .with_size(self.border_x, dimm::border())
        .with_pos(self.scroll.x(), self.scroll.y());
        // .with_color(fltk::enums::Color::Red)
        // .with_frame(fltk::enums::FrameType::BorderBox);
      // Create an empty widget to serve as a spacer for y
      let frame_spacer_y = fltk::frame::Frame::default()
        .with_size(dimm::border(), self.border_y)
        .right_of(&frame_spacer_x, 0);
        // .with_color(fltk::enums::Color::Green)
        // .with_frame(fltk::enums::FrameType::BorderBox);
      w.clone().below_of(&frame_spacer_y, 0);
    } // else

    self.opt_current = Some(w.as_base_widget());
  } // add()

  pub fn set_border(&mut self, x : i32, y : i32)
  {
    self.border_x = x;
    self.border_y = y;
  } // widget_ref()

  pub fn set_frame(&mut self, frame_type : fltk::enums::FrameType)
  {
    self.frame_type.set_frame(frame_type);
  } // widget_mut()

} // impl ScrollList }}}

// pub fn search_column() {{{
pub fn search_column(x: i32, y: i32, w: i32, h: i32, label: &str) -> (fltk::group::Flex, fltk_evented::Listener<fltk::input::Input>)
{
  // Main column
  let mut col = fltk::group::Flex::default()
    .with_pos(x, y)
    .with_size(w, h);
  col.set_spacing(dimm::border());
  col.set_type(fltk::group::FlexType::Column);
  // Create label
  col.fixed(&fltk::frame::Frame::default()
    .with_size(col.w(), dimm::height_text())
    .with_align(Align::Inside | Align::Left)
    .with_label(label), dimm::height_text());
  // Input widget
  let mut input_query : fltk_evented::Listener<_> = fltk::input::Input::default()
    .with_size(col.w(), dimm::height_button_wide())
    .into();
  let _ = input_query.take_focus();
  col.fixed(&input_query.as_base_widget(), dimm::height_button_wide());
  (col, input_query)
} // pub fn search_column() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
