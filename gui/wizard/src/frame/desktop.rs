use fltk::prelude::*;
use fltk::app::Sender;

use shared::fltk::SenderExt;

use crate::common;
use shared::std::PathBufExt;
use crate::log;

// pub fn desktop() {{{
pub fn desktop(tx: Sender<common::Msg>, title: &str)
{
  let ret = crate::frame::icon::icon(tx
    , title
    , common::Msg::DrawDesktop
    , common::Msg::DrawDesktop
  );

  // Hide prev button
  ret.ret_frame_footer.btn_prev.clone().hide();

  // Callback to install icon
  let clone_tx = tx.clone();
  ret.ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    let arc_path_file_icon = ret.arc_path_file_icon.clone();
    let mut output_status = ret.ret_frame_footer.output_status.clone();

    clone_tx.send_awake(common::Msg::WindDeactivate);

    // Check if an icon was selected
    let path_file_icon = if let Ok(option_path_file_icon) = arc_path_file_icon.lock()
    && let Some(path_file_icon) = option_path_file_icon.as_ref()
    {
      path_file_icon.clone()
    }
    else
    {
      output_status.set_value("No icon selected");
      clone_tx.send_awake(common::Msg::WindActivate);
      log!("No Icon selected");
      return;
    };

    std::thread::spawn(move ||
    {
      // Set as desktop entry icon for image
      // Wait for message & check return value
      if common::gameimage_sync(vec!["desktop", &path_file_icon.string()]) != 0
      {
        log!("Failed to execute desktop command on backend");
        clone_tx.send_awake(common::Msg::WindActivate);
        return;
      } // if

      clone_tx.send_awake(common::Msg::DrawName);
      fltk::app::awake();
    });

  });
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
