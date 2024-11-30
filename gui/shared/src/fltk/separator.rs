use fltk::
{
  draw,
  enums::Color,
  prelude::*,
  frame::Frame,
  enums::FrameType
};

use crate::dimm;
use crate::fltk::WidgetExtExtra;

pub fn vertical(height: i32) -> Frame
{
  let mut frame = Frame::default()
      .with_size(dimm::height_sep(), height)
      .with_frame(FrameType::FlatBox)
      .with_color(Color::White);

  frame.draw(move |f|
  {
    let (x,y,w,h) = (f.x(),f.y(),f.w(),f.h());
    draw::draw_rect_fill(x, y, w, h, f.color());
  });

  frame
}

pub fn horizontal(width: i32) -> Frame
{
  let mut frame = Frame::default()
      .with_size(width, dimm::height_sep())
      .with_frame(FrameType::FlatBox)
      .with_color(Color::White);

  frame.draw(move |f|
  {
    let (x,y,w,h) = (f.x(),f.y(),f.w(),f.h());
    draw::draw_rect_fill(x, y, w, h, f.color());
  });

  frame
}
// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :

