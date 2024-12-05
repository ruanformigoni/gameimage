#[macro_export]
macro_rules! rescope
{
  ($row_name:ident, $($body:tt)*) =>
  {
    $row_name.begin();
    $($body)*
    $row_name.end();
  };
}

#[macro_export]
macro_rules! group
{
  ($col_name:ident, $($body:tt)*) =>
  {
    let mut $col_name = fltk::group::Group::default_fill();
    $($body)*
    $col_name.end();
  };
}

#[macro_export]
macro_rules! tabs
{
  ($col_name:ident, $($body:tt)*) =>
  {
    let mut $col_name = fltk::group::Tabs::default_fill();
    $($body)*
    $col_name.end();
  };
}

#[macro_export]
macro_rules! hpack
{
  ($col_name:ident, $($body:tt)*) =>
  {
    let mut $col_name = fltk::group::Pack::default_fill();
    $col_name.set_type(fltk::group::PackType::Vertical);
    $($body)*
    $col_name.end();
  };
}

#[macro_export]
macro_rules! column
{
  ($col_name:ident, $($body:tt)*) =>
  {
    let mut $col_name = fltk::group::Flex::default_fill().column();
    $($body)*
    $col_name.end();
  };
}

#[macro_export]
macro_rules! row
{
  ($row_name:ident, $($body:tt)*) =>
  {
    let mut $row_name = fltk::group::Flex::default_fill().row();
    $($body)*
    $row_name.end();
  };
}

#[macro_export]
macro_rules! scroll
{
  ($scroll_name:ident, $($body:tt)*) =>
  {
    let mut $scroll_name = fltk::group::Scroll::default_fill();
    $scroll_name.set_scrollbar_size(dimm::border());
    $($body)*
    $scroll_name.end();
  };
}

#[macro_export]
macro_rules! hseparator_fixed
{
  ($col_name:ident, $width:expr, $height: expr) =>
  {{
    column!(col_inner,
      col_inner.add(&Frame::default());
      let sep = shared::fltk::separator::horizontal($width);
      col_inner.fixed(&sep, sep.h());
      col_inner.add(&Frame::default());
    );
    $col_name.fixed(&col_inner, $height);
  }}
}

#[macro_export]
macro_rules! hseparator
{
  ($col_name:ident, $width:expr) =>
  {{
    column!(col_inner,
      col_inner.add(&Frame::default());
      let sep = shared::fltk::separator::horizontal($width);
      col_inner.fixed(&sep, sep.h());
      col_inner.add(&Frame::default());
    );
    $col_name.add(&col_inner);
  }}
}

#[macro_export]
macro_rules! fixed
{
  ($col_name:ident, $name:ident, $widget:expr, $size:expr) =>
  {
    let $name = $widget.clone();
    $col_name.fixed(&$name, $size);
  }
}

#[macro_export]
macro_rules! add
{
  ($col_name:ident, $name:ident, $widget:expr) =>
  {
    let $name = $widget.clone();
    $col_name.add(&$name);
  }
}

#[macro_export]
macro_rules! col_center
{
  ($widget:expr) =>
  {{
    let mut col = fltk::group::Flex::default_fill().column();
    col.add(&fltk::frame::Frame::default());
    let widget = $widget.clone();
    col.fixed(&widget, widget.w());
    col.add(&fltk::frame::Frame::default());
    col.end();
    (col,widget)
  }}
}

#[macro_export]
macro_rules! row_center
{
  ($widget:expr) =>
  {{
    let mut row = fltk::group::Flex::default_fill().row();
    row.add(&fltk::frame::Frame::default());
    let widget = $widget.clone();
    row.fixed(&widget, widget.w());
    row.add(&fltk::frame::Frame::default());
    row.end();
    (row,widget)
  }}
}

#[macro_export]
macro_rules! hover_blink
{
  ($button:ident) =>
  {
    $button.clone().handle(|b,ev| match ev {
      fltk::enums::Event::Enter => { b.set_value(true); true},
      fltk::enums::Event::Leave => { b.set_value(false); true},
      _ => false,
    });
  };
}
// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
