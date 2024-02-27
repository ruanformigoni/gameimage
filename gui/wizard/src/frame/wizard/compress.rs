#![allow(warnings)]

use std::env;
use std::path::PathBuf;
use std::fs::File;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  browser::HoldBrowser,
  text::{TextBuffer,TextDisplay},
  menu::MenuButton,
  button::Button,
  group::Group,
  image::SharedImage,
  input::FileInput,
  group::PackType,
  frame::Frame,
  dialog::{dir_chooser,file_chooser},
  enums::{Align,FrameType,Color},
  misc::Progress,
};

use url as Url;
use anyhow;
use anyhow::anyhow as ah;

use crate::dimm;
use crate::frame;
use crate::common;
use crate::db;
use crate::download;
use crate::svg;

// pub fn compress() {{{
pub fn compress(tx: Sender<common::Msg>
  , title: &str
  , msg_prev: common::Msg
  , msg_curr: common::Msg
  , msg_next: common::Msg)
{
  let mut frame = Frame::default()
    .with_size(dimm::width(), dimm::height());
  frame.set_frame(FrameType::BorderBox);
  frame.set_type(PackType::Vertical);

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_header = ret_frame_header.frame.clone();
  let frame_content = ret_frame_header.frame_content.clone();
  let frame_footer = ret_frame_footer.frame.clone();

  // Set previous frame
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), msg_prev);

  // Rename 'next' to 'compress'
  ret_frame_footer.btn_next.clone().set_label("Start");

  let mut term = frame::term::Term::new(dimm::border()
    , frame_content.w() - dimm::border()*2
    , frame_content.h() - dimm::border()*2
    , frame_content.x() + dimm::border()
    , frame_content.y() + dimm::border());

  let clone_tx = tx.clone();
  ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    clone_tx.send(common::Msg::WindDeactivate);
    term.dispatch(vec!["$GIMG_BACKEND compress"]
      , move |code : i32|
      {
        clone_tx.send(common::Msg::WindActivate);

        if code == 0
        {
          clone_tx.send(common::Msg::DrawCreator);
        } // if
      }
    );
  });
} // fn compress() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
