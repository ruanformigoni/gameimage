use std::env;
use std::path::PathBuf;

// Gui
use fltk::prelude::*;
use fltk::app::Sender;

use shared::fltk::SenderExt;

use crate::dimm;
use crate::frame;
use crate::common;
use shared::std::PathBufExt;
use crate::log;

// pub fn compress() {{{
pub fn compress(tx: Sender<common::Msg>
  , title: &str
  , msg_prev: common::Msg
  , _msg_curr: common::Msg
  , _msg_next: common::Msg)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

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
    clone_tx.send_awake(common::Msg::WindDeactivate);

    let path_gimg_backend = if let Ok(var) = env::var("GIMG_BACKEND")
    {
      PathBuf::from(var)
    } // if
    else
    {
      log!("Could not fetch GIMG_BACKEND var");
      return;
    }; // else

    let _ = term.dispatch(vec![&path_gimg_backend.string(), "compress"]
      , move |code : i32|
      {
        clone_tx.send_awake(common::Msg::WindActivate);

        if code == 0
        {
          clone_tx.send_awake(common::Msg::DrawCreator);
        } // if
      }
    );
  });
} // fn compress() }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
