use fltk::prelude::*;
use fltk::{
  app::Sender,
  enums::{Align,FrameType,Color},
  frame::Frame,
};

use clown::clown;
use common::Msg;
use shared::dimm;
use shared::fltk::WidgetExtExtra;
use shared::{fixed,hover_blink,hseparator_fixed,column,row,rescope};
use fltk::prelude::ButtonExt;

use crate::common;

pub mod enabler_executable;
pub mod environment;

// fn: new {{{
pub fn new(tx : Sender<Msg>)
{
  // Layout
  column!(col,
    col.set_margin(dimm::border_half());
    fixed!(col, frame_title, Frame::default(), dimm::height_text());
    hseparator_fixed!(col, col.w() - dimm::border()*2, dimm::border_half());
    column!(col_content, );
    col_content.set_spacing(0);
    hseparator_fixed!(col, col.w() - dimm::border()*2, dimm::border_half());
    column!(col_bottom,
      row!(row_bottom,
        fixed!(row_bottom, btn_back, shared::fltk::button::rect::back(), dimm::width_button_rec());
      );
      col_bottom.fixed(&row_bottom, dimm::height_button_rec());
    );
    col.fixed(&col_bottom, dimm::height_button_rec());
  );
  // Title
  let mut frame_title = frame_title.clone();
  frame_title.set_label("Menu");
  // Footer button
  let mut btn_back = btn_back.clone();
  btn_back.emit(tx, Msg::DrawCover);
  hover_blink!(btn_back);
  // Entries
  rescope!(col_content,
    let f_make_entry = #[clown] move |label : &str|
    {
      let entry = fltk::button::ToggleButton::default()
        .with_size(0, dimm::height_button_wide() + dimm::border_half())
        .with_frame(FrameType::FlatBox)
        .with_color(Color::BackGround)
        .with_color_selected(Color::BackGround.lighter())
        .with_align(Align::Left | Align::Inside)
        .with_label(&format!(" {}", label));
      hover_blink!(entry);
      honk!(col_content).clone().fixed(&mut entry.as_base_widget(), entry.h());
      entry
    };
    // Environment
    f_make_entry("Environment").emit(tx, Msg::DrawEnv);
    // Executables
    if let Ok(str_platform) = std::env::var("GIMG_PLATFORM")
    && let Ok(platform) = common::Platform::from_str(&str_platform)
    && platform == common::Platform::WINE
    {
      f_make_entry("Executable Configuration").emit(tx, Msg::DrawEnablerExecutable);
    }
  );
} // fn: new }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
