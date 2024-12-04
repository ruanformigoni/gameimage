// Gui
use fltk::prelude::*;
use fltk::{
  output::Output,
  frame::Frame,
  enums::{Align,Color},
};

use shared::fltk::WidgetExtExtra;
use shared::{group,hover_blink,hseparator_fixed,column,row,add,fixed};

use crate::dimm;
use crate::frame;

// pub fn layout() {{{
pub fn layout()
{
  column!(outer,
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
      group!(group_content,
        group_content.set_id("content");
      );
      col.add(&group_content);
      hseparator_fixed!(col, dimm::width_wizard() - dimm::border()*2, dimm::border_half());
      row!(row,
        row.set_id("footer");
        fixed!(row, btn_prev, shared::fltk::button::wide::default()
          .with_id("footer_prev")
          .with_label("Prev"), dimm::width_button_wide());
        add!(row, expand, Frame::default());
        fixed!(row, btn_next, shared::fltk::button::wide::default()
          .with_id("footer_next")
          .with_label("Next")
          .with_color(Color::Blue), dimm::width_button_wide());
      );
      col.fixed(&row, dimm::height_button_wide());
    );
    outer.add(&col);
    outer.fixed(&Output::default().with_id("footer_status"), 20);
  );

  // Configure buttons
  hover_blink!(btn_term);
  hover_blink!(btn_prev);
  hover_blink!(btn_next);

  // Title font size
  frame_title.clone().set_label_size((dimm::height_text() as f32 * 1.5) as i32);

  let mut term = frame::term::Term::new_with_id("term_log"
    , 0
    , group_content.w(), group_content.h()
    , group_content.x(), group_content.y()
  );

  group_content.resize_callback({
  let term = term.clone();
  move |s,x,y,w,h|
  {
    term.group.clone().resize(x,y,w,h);
  }});

  term.group.hide();
  let mut clone_term = term.group.clone();
  let mut clone_group_content = group_content.clone();
  btn_term.clone().set_callback(move |_|
  {
    if clone_term.visible()
    {
      clone_term.hide();
      clone_group_content.show();
    } // if
    else
    {
      clone_group_content.hide();
      clone_term.show();
    } // else
  });

} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
