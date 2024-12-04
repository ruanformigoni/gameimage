// Gui
use fltk::prelude::*;
use fltk::{
  output::Output,
  frame::Frame,
  enums::{Align,Color},
};

use shared::fltk::WidgetExtExtra;
use shared::{hover_blink,hseparator_fixed,column,row,add,fixed};

use crate::dimm;
use crate::frame;

// pub fn layout() {{{
pub fn layout()
{
  column!(col,
    col.set_margin(dimm::border_half());
    col.set_spacing(dimm::border_half());
    row!(row,
      fixed!(row, btn_term, shared::fltk::button::rect::terminal().with_color(Color::Blue), dimm::width_button_rec());
      add!(row, frame_title, Frame::default().with_id("header_title").with_align(Align::Inside | Align::Center));
      fixed!(row, btn_resize, shared::fltk::button::rect::resize_down()
        .with_id("btn_resize")
        .with_color(Color::Blue), dimm::width_button_rec()
      );
    );
    col.fixed(&row, dimm::height_button_rec());
    hseparator_fixed!(col, dimm::width_wizard() - dimm::border()*2, dimm::border_half());
    column!(col_content_footer,
      column!(group_content, group_content.set_id("content"););
      col_content_footer.add(&group_content);
      hseparator_fixed!(col_content_footer, dimm::width_wizard() - dimm::border()*2, dimm::border_half());
      row!(footer,
        footer.set_id("footer");
        fixed!(footer, btn_prev, shared::fltk::button::wide::default()
          .with_id("footer_prev")
          .with_label("Prev"), dimm::width_button_wide());
        add!(footer, expand, Frame::default());
        fixed!(footer, btn_next, shared::fltk::button::wide::default()
          .with_id("footer_next")
          .with_label("Next")
          .with_color(Color::Blue), dimm::width_button_wide());
      );
      col_content_footer.fixed(&footer, dimm::height_button_wide());
      col_content_footer.fixed(&Output::default().with_id("footer_status"), 20);
    );
    col.add(&col_content_footer);
  );

  // Configure buttons
  hover_blink!(btn_term);
  hover_blink!(btn_prev);
  hover_blink!(btn_next);

  // Title font size
  frame_title.clone().set_label_size((dimm::height_text() as f32 * 1.5) as i32);

  let mut term = frame::term::Term::new_with_id("term_log"
    , 0
    , col_content_footer.w(), col_content_footer.h()
    , col_content_footer.x(), col_content_footer.y()
  );

  col_content_footer.resize_callback({
  let term = term.clone();
  move |_,x,y,w,h|
  {
    term.group.clone().resize(x,y,w,h);
  }});

  term.group.hide();
  btn_term.clone().set_callback({
  let mut term = term.group.clone();
  let mut col_content_footer = col_content_footer.clone();
  move |_|
  {
    if term.visible()
    {
      term.hide();
      col_content_footer.show();
      fltk::app::redraw();
    } // if
    else
    {
      col_content_footer.hide();
      term.show();
      fltk::app::redraw();
    } // else
  }});

} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
