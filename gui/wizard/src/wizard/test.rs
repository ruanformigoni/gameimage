// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  enums::Color,
};

use shared::fltk::SenderExt;
use shared::fltk::WidgetExtExtra;
use shared::std::PathBufExt;

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

  // Set previous/next frame
  ui.btn_prev.clone().emit(tx.clone(), msg_prev);
  ui.btn_next.clone().emit(tx.clone(), msg_next);

  let mut term = frame::term::Term::new(dimm::border()
    , ui.group.w()
    , ui.group.h() - dimm::border() - dimm::height_button_wide()
    , ui.group.x()
    , ui.group.y());

  // Add a 'test' button
  let mut btn_test = shared::fltk::button::wide::default()
    .below_center_of(&term.group, dimm::border())
    .with_label("Test")
    .with_color(Color::Green);

  let clone_tx = tx.clone();
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
