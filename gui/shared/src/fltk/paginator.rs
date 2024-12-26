use fltk::{
  prelude::*,
  frame,
  output,
};

use crate::{dimm,column,row,hover_blink};

// pub fn paginator() {{{
pub fn paginator<F,G,H>(get_page: F, set_page: G, count_pages: H) -> (fltk::group::Flex, fltk_evented::Listener<fltk::input::Input>)
  where F: Clone + FnMut() -> usize,
        G: 'static + Clone + FnMut(usize),
        H: 'static + Clone + FnMut() -> usize
{
  column!(col,
    row!(row,
      let mut btn_prev = crate::fltk::button::rect::arrow_backward();
      row.fixed(&btn_prev, dimm::width_button_rec());
      row.add(&frame::Frame::default());
      row!(row_pages,
        let mut input_page : fltk_evented::Listener<_> = fltk::input::Input::default().into();
        row_pages.fixed(&input_page.as_base_widget(), dimm::width_button_rec());
        row_pages.fixed(&frame::Frame::default().with_label("/"), dimm::border());
        let mut output_pages = output::Output::default(); 
        row_pages.fixed(&output_pages, dimm::width_button_rec());
        let _ = output_pages.insert(&count_pages.clone()().to_string());
      );
      row.fixed(&row_pages, dimm::width_button_rec()*2 + dimm::border());
      row.add(&frame::Frame::default());
      let mut btn_next = crate::fltk::button::rect::arrow_forward();
      row.fixed(&btn_next, dimm::width_button_rec());
    );
    col.fixed(&row, dimm::height_button_wide());
  );
  // Adjust size
  col.resize(col.x(), col.y(), col.w(), dimm::height_button_wide());
  // Configure hover
  hover_blink!(btn_prev);
  hover_blink!(btn_next);
  // Configure callback
  input_page.set_value(&get_page.clone()().to_string());
  input_page.on_keydown({
    let mut set_page = set_page.clone();
    move |e|
    {
      // Filter non-digit values
      e.set_value(&e.value().chars().filter(|c| "1234567890".contains(*c)).collect::<String>());
      // Make sure it does not stay empty
      if e.value().is_empty() { e.set_value("0"); } // if
      // Remove leading zeroes
      while e.value().starts_with("0") && e.value().len() > 1 { e.set_value(&e.value()[1..]); }
      // Set page
      if fltk::app::event_key() == fltk::enums::Key::Enter
      {
        set_page(e.value().parse::<usize>().unwrap_or_default());
      } // if
    }
  });
  btn_prev.set_callback({
    let input_page = input_page.clone();
    let mut set_page = set_page.clone();
    move |_|
    {
      let value = input_page.value().parse::<usize>().unwrap_or_default();
      set_page(if value == 0 { 0 } else { value-1 });
    }
  });
  btn_next.set_callback({
    let input_page = input_page.clone();
    let mut set_page = set_page.clone();
    move |_|
    {
      let value = input_page.value().parse::<usize>().unwrap_or_default();
      set_page((value + 1).min(count_pages.clone()()));
    }
  });
  (col, input_page)
} // pub fn paginator() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
