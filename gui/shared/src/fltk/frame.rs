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

pub fn bordered() -> Frame
{
  let mut frame = Frame::default()
    .with_frame(FrameType::FlatBox)
    .with_color(Color::White);

  frame.draw(move |f|
  {
    let (x,y,w,h) = (f.x(),f.y(),f.w(),f.h());
    draw::draw_rect_fill(x, y, w, h, f.color());
    draw::draw_rect_fill(x + dimm::height_sep()
      , y + dimm::height_sep()
      , w - dimm::height_sep()*2
      , h - dimm::height_sep()*2
      , Color::BackGround
    );
    // Draw Image
    if let Some(mut image) = f.image()
    {
      let img_w = image.width();
      let img_h = image.height();
      let img_x = x + (w - img_w) / 2; // Center horizontally
      let img_y = y + (h - img_h) / 2; // Center vertically
      image.draw(img_x, img_y, img_w, img_h);
    }
    // Draw label
    draw::set_draw_color(Color::Foreground);
    draw::draw_text2(&f.label(), f.x(), f.y(), f.w(), f.h(), f.align());
  });

  frame
}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
