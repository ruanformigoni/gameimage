// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  button::Button,
  enums::Color,
};

use shared::fltk::SenderExt;

use crate::gameimage;
use crate::dimm;
use crate::frame;
use crate::common;
use crate::log_alert;
use shared::std::PathBufExt;

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

  // Add a 'test' button
  let mut btn_test = Button::default()
    .with_size(dimm::width_button_wide(), dimm::height_button_wide())
    .with_label("Test");
  btn_test.set_label_size(dimm::height_text());
  btn_test.clone().center_x(&ui.group);
  btn_test.set_pos(btn_test.x(), ui.btn_next.y());
  btn_test.set_color(Color::Green);

  let mut term = frame::term::Term::new(dimm::border()
    , ui.group.w()
    , ui.group.h()
    , ui.group.x()
    , ui.group.y());

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
