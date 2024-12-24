use fltk::prelude::*;
use fltk::{
  group,
  enums,
  app::Sender,
  button::Button,
  frame::Frame,
};

use shared::dimm;
use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;

use crate::games;
use crate::common;
use common::Msg;
use shared::{fixed,row,column,hpack,scroll,hover_blink,hseparator_fixed,rescope};

// fn: new {{{
pub fn new(tx : Sender<Msg>)
{
  // Layout
  column!(col,
    col.set_margin(dimm::border_half());
    fixed!(col, frame_title, Frame::default(), dimm::height_text());
    hseparator_fixed!(col, col.w() - dimm::border()*2, dimm::border_half());
    // Content
    scroll!(scroll,
      hpack!(col_scroll,);
      col_scroll.set_spacing(0);
      col_scroll.set_size(0,0);
    );
    hseparator_fixed!(col, col.w() - dimm::border()*2, dimm::border_half());
    column!(col_bottom,
      row!(row_bottom,
        row_bottom.add(&Frame::default());
        fixed!(row_bottom, btn_home, &shared::fltk::button::rect::home(), dimm::width_button_rec());
        row_bottom.add(&Frame::default());
      );
      col_bottom.fixed(&row_bottom, dimm::height_button_rec());
    );
    col.fixed(&col_bottom, dimm::height_button_rec());
  );

  // Title
  let mut frame_title = frame_title.clone();
  frame_title.set_label("Switch Game");
  // Scroll resize callback
  scroll.resize_callback({let mut c = col_scroll.clone(); move |_,_,_,w,_|
  {
    c.resize(c.x(),c.y(),w-dimm::border_half()*3,c.h());
  }});
  scroll.set_type(group::ScrollType::VerticalAlways);
  // Configure buttons
  let mut btn_home = btn_home.clone();
  btn_home.set_color(enums::Color::Blue);
  btn_home.emit(tx, Msg::DrawCover);
  hover_blink!(btn_home);

  // Create entries
  rescope!(col_scroll,
    for game in games::games().unwrap_or_default()
    {
      // Set entry name
      let osstr_name_file = game.path_root.file_name().unwrap_or_default();
      let str_name_file = osstr_name_file.to_str().unwrap_or_default();
      let entry = Button::default()
        .with_size(0, dimm::height_button_wide() + dimm::border_half())
        .with_label(&str_name_file)
        .with_frame(enums::FrameType::FlatBox)
        .with_color(enums::Color::BackGround)
        .with_color_selected(enums::Color::BackGround.lighter())
        .with_align(enums::Align::Left | enums::Align::Inside)
        .with_callback(move |_| { games::select(&game); tx.send_awake(Msg::DrawCover); });
      hover_blink!(entry);
      col_scroll.add(&entry);
    } // for
  );

} // fn: new }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
