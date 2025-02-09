use std::env;

// Gui
use fltk::
{
  prelude::*,
  app::Sender,
  menu,
};

use shared::fltk::SenderExt;
use shared::std::PathBufExt;

use crate::dimm;
use crate::gameimage;
use crate::frame;
use crate::common;
use crate::log;
use crate::log_alert;
use crate::log_err;
use shared::{column,row,fixed};

// fn compress_next() {{{
pub fn compress_next(tx: Sender<common::Msg>, term: frame::term::Term)
{
  tx.send_awake(common::Msg::WindDeactivate);
  let backend = match gameimage::gameimage::binary()
  {
    Ok(backend) => backend,
    Err(e) => { log_alert!("Error to execute backend: {}", e); return; }
  };
  let mut term = term.clone();
  std::thread::spawn(move ||
  {
    let handle = term.dispatch(vec![&backend.string(), r#"{ "op": "compress" }"#], |_| {});
    match handle
    {
      Ok(handle) => log_err!(handle.lock().unwrap().wait().map(|_|{})),
      Err(e) => log!("{}", e),
    };
    tx.send_activate(common::Msg::DrawCreator);
  });

} // fn compress_next() }}}

// pub fn compress() {{{
pub fn compress(tx: Sender<common::Msg>
  , title: &str
  , msg_prev: common::Msg
  , _msg_curr: common::Msg
  , _msg_next: common::Msg)
{
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);
  // Layout
  column!(col,
    let term = frame::term::Term::default();
    col.add(&term.group);
    row!(row,
      fixed!(row, btn_level, menu::MenuButton::default(), dimm::width_button_wide());
      row.add(&fltk::frame::Frame::default()
        .with_align(fltk::enums::Align::Inside | fltk::enums::Align::Center)
        .with_label("Select the compression level before clicking on start")
      );
    );
    col.fixed(&row, dimm::height_button_wide());
  );

  // Configure buttons
  ui.btn_prev.clone().emit(tx.clone(), msg_prev);
  let mut btn_next = ui.btn_next.clone();
  btn_next.set_label("Start");
  btn_next.set_callback(move |_| { compress_next(tx, term.clone()); });

  // Open space for compress level button
  let mut btn_level = btn_level.clone();
  btn_level.set_callback(|e|
  {
    let str_level = e.choice().unwrap_or(String::from("7"));
    log!("Set compression level to {}", str_level);
    env::set_var("FIM_COMPRESSION_LEVEL", &str_level);
    e.set_value(e.value());
    e.set_label(&str_level);
  });
  for i in 0..11
  {
    btn_level.add_choice(&i.to_string());
  } // for
  env::set_var("FIM_COMPRESSION_LEVEL", "7");
  btn_level.set_value(8);
  btn_level.set_label("7");
} // fn compress() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
