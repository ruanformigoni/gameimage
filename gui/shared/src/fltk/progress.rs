use fltk::
{
  draw,
  enums::Color,
  misc::Progress,
  prelude::*,
};

pub fn progress() -> Progress
{
  let mut frame = Progress::default();

  frame.draw(move |f|
  {
    let (x,y,w,h) = (f.x(),f.y(),f.w(),f.h());
    let cs = f.selection_color();
    let cf = f.color();
    let value = f.value().clamp(f.minimum(), f.maximum()) as f32 / 100.0;
    let filled_width = (w as f32 * value).round() as i32;

    draw::draw_rect_fill(x, y, w, h, cf);
    draw::draw_rect_fill(x, y, filled_width, h, cs);
    draw::draw_rect_with_color(x, y, w, h, Color::BackGround.lighter());
    draw::set_draw_color(Color::ForeGround);
    draw::draw_text2(&f.label() , x , y , w , h , f.align());
  });

  frame
}
// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
