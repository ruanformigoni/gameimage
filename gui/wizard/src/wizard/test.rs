// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  enums::Color,
};

use shared::fltk::SenderExt;
use shared::fltk::WidgetExtExtra;
use shared::std::PathBufExt;
use shared::{column,row,fixed,hover_blink};

use crate::gameimage;
use crate::dimm;
use crate::frame;
use crate::common;
use crate::log_alert;

// pub fn test() {{{
pub fn test(tx: Sender<common::Msg>
  , title: &str
  , msg_prev: common::Msg
  , _msg_curr: common::Msg
  , msg_next: common::Msg)
{
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);
  // Layout
  column!(col,
    let term = frame::term::Term::default();
    row!(row,
      row.add(&fltk::frame::Frame::default());
      fixed!(row, btn_test, shared::fltk::button::wide::default(), dimm::width_button_wide());
      row.add(&fltk::frame::Frame::default());
    );
    col.fixed(&row, dimm::height_button_wide());
  );
  // Cofigure buttons
  hover_blink!(btn_test);
  ui.btn_prev.clone().emit(tx.clone(), msg_prev);
  ui.btn_next.clone().emit(tx.clone(), msg_next);

  // Add a 'test' button
  let clone_tx = tx.clone();
  let mut term = term.clone();
  let mut btn_test = btn_test.clone()
    .with_label("Test")
    .with_color(Color::Green);
  btn_test.set_callback(move |_|
  {
    clone_tx.send_awake(common::Msg::WindDeactivate);
    let backend = match gameimage::gameimage::binary()
    {
      Ok(backend) => backend,
      Err(e) => { log_alert!("Error to execute backend: {}", e); return; }
    };
    let _ = term.dispatch(vec![&backend.string(), "test"], move |_|
    {
      clone_tx.send_awake(common::Msg::WindActivate);
    });
  });
} // fn test() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
