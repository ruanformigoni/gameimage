use std::env;

// Gui
use fltk::
{
  prelude::*,
  app::Sender,
  menu,
};

use shared::fltk::SenderExt;
use shared::fltk::WidgetExtExtra;
use shared::std::PathBufExt;

use crate::dimm;
use crate::gameimage;
use crate::frame;
use crate::common;
use crate::log;
use crate::log_alert;

// pub fn compress() {{{
pub fn compress(tx: Sender<common::Msg>
  , title: &str
  , msg_prev: common::Msg
  , _msg_curr: common::Msg
  , _msg_next: common::Msg)
{
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);

  // Set previous frame
  ui.btn_prev.clone().emit(tx.clone(), msg_prev);

  // Rename 'next' to 'compress'
  ui.btn_next.clone().set_label("Start");

  let mut term = frame::term::Term::new(dimm::border()
    , ui.group.w()
    , ui.group.h()
    , ui.group.x()
    , ui.group.y());

  // Open space for compress level button
  let mut widget_term = term.term.clone();
  widget_term.set_size(term.term.w()
    , term.term.h() - dimm::border() - dimm::height_button_wide()
  );
  let mut btn_level = menu::MenuButton::default()
    .with_size(dimm::width_button_wide(), dimm::height_button_wide())
    .with_focus(false)
    .bottom_left_of(&ui.group, 0)
    .with_callback(|e|
    {
      let str_level = e.choice().unwrap_or(String::from("7"));
      log!("Set compression level to {}", str_level);
      env::set_var("FIM_COMPRESSION_LEVEL", &str_level);
      e.set_value(e.value());
      e.set_label(&str_level);
    });
  let _ = fltk::frame::Frame::default()
    .with_size(widget_term.w() - dimm::width_button_wide(), dimm::height_text())
    .with_pos(btn_level.x() + btn_level.w() + dimm::border(), btn_level.y() + btn_level.h() / 4)
    .with_align(fltk::enums::Align::Inside | fltk::enums::Align::Center)
    .with_label("Select the compression level before clicking on start");
  for i in 0..11
  {
    btn_level.add_choice(&i.to_string());
  } // for
  env::set_var("FIM_COMPRESSION_LEVEL", "7");
  btn_level.set_value(8);
  btn_level.set_label("7");

  let clone_tx = tx.clone();
  ui.btn_next.clone().set_callback(move |_|
  {
    clone_tx.send_awake(common::Msg::WindDeactivate);
    let backend = match gameimage::gameimage::binary()
    {
      Ok(backend) => backend,
      Err(e) => { log_alert!("Error to execute backend: {}", e); return; }
    };
    let _ = term.dispatch(vec![&backend.string(), "compress"], move |code : i32|
    {
      clone_tx.send_awake(common::Msg::WindActivate);

      if code == 0
      {
        clone_tx.send_activate(common::Msg::DrawCreator);
      } // if
    });
  });
} // fn compress() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
