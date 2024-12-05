// Gui
use fltk::prelude::*;
use fltk::{
  output::Output,
  frame::Frame,
  enums::{Align,Color},
};

use shared::fltk::WidgetExtExtra;
use shared::{tabs,hover_blink,hseparator_fixed,column,row,add,fixed};

use crate::dimm;
use crate::frame;

// pub fn layout() {{{
pub fn layout()
{
  column!(col,
    col.set_margin(dimm::border_half());
    col.set_spacing(dimm::border_half());
    row!(row_header,
      fixed!(row_header, btn_term, shared::fltk::button::rect::terminal().with_color(Color::Blue), dimm::width_button_rec());
      add!(row_header, frame_title, Frame::default().with_id("header_title").with_align(Align::Inside | Align::Center));
      fixed!(row_header, btn_resize, shared::fltk::button::rect::resize_down()
        .with_id("btn_resize")
        .with_color(Color::Blue), dimm::width_button_rec()
      );
    );
    col.fixed(&row_header, dimm::height_button_rec());
    hseparator_fixed!(col, dimm::width_wizard() - dimm::border()*2, dimm::border_half());
    tabs!(tab_content,
      column!(col_content_footer,
        col_content_footer.set_frame(fltk::enums::FrameType::FlatBox);
        col_content_footer.set_color(Color::BackGround);
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
      column!(col_content_term,
        col_content_term.set_frame(fltk::enums::FrameType::FlatBox);
        col_content_term.set_color(Color::BackGround);
        let term = frame::term::Term::default();
        term.term.clone().set_id("term_log");
      );
    );
    col.add(&tab_content);
  );

  // Configure buttons
  hover_blink!(btn_term);
  hover_blink!(btn_resize);
  hover_blink!(btn_prev);
  hover_blink!(btn_next);

  // Title font size
  frame_title.clone().set_label_size((dimm::height_text() as f32 * 1.5) as i32);

  // Switch between tabs
  btn_term.clone().set_callback({
    let col_content_term = col_content_term.clone();
    let col_content_footer = col_content_footer.clone();
    move |_|
    {
      if tab_content.value().unwrap().is_same(&col_content_footer.as_group().unwrap())
      {
        let _ = tab_content.set_value(&col_content_term.as_group().unwrap());
      }
      else
      {
        let _ = tab_content.set_value(&col_content_footer.as_group().unwrap());
      } // else
    }
  });

} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
