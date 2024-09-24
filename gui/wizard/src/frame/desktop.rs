use std::sync::{Arc,atomic::{AtomicBool, Ordering}};
use fltk::
{
  prelude::*,
  app::Sender,
};

use shared::fltk::SenderExt;
use shared::fltk::WidgetExtExtra;

use crate::common;
use crate::dimm;
use crate::log;
use crate::log_return_void;
use crate::gameimage;

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
  let frame_sep = ret.ret_frame_footer.sep.clone();

  // Move icon frame to the left
  let mut frame_icon = match ret.opt_frame_icon
  {
    Some(frame_icon) => frame_icon,
    None => { log!("Failed to retrieve icon frame"); return; }
  }; // match

  frame_icon.set_pos(frame_sep.x(), frame_icon.y());

  // Select application name with an input field
  let input_name = fltk::input::Input::default()
    .with_size(frame_sep.w() - frame_icon.w() - dimm::border(), dimm::height_button_wide())
    .with_align(fltk::enums::Align::Top | fltk::enums::Align::Left)
    .with_pos(frame_icon.x() + frame_icon.w() + dimm::border(), frame_icon.y() + dimm::height_text())
    .with_label("Select the application name");

  // Button to enable desktop entry
  let is_integrate_entry = Arc::new(AtomicBool::new(true));
  let clone_is_integrate_entry = is_integrate_entry.clone();
  let f_set_integrate_entry = move |val: bool| { clone_is_integrate_entry.store(val, Ordering::SeqCst); };
  let mut btn_check_icon = fltk::button::CheckButton::default()
    .with_size(dimm::width_checkbutton(), dimm::width_checkbutton())
    .below_of(&input_name, dimm::border())
    .with_align(fltk::enums::Align::Right)
    .with_focus(false)
    .with_label("Show icon in the start menu?");
  btn_check_icon.set_checked(true);
  let clone_f_set_integrate_entry = f_set_integrate_entry.clone();
  btn_check_icon.set_callback(move |e| clone_f_set_integrate_entry(e.is_checked()));

  // Callback to install icon
  let clone_tx = tx.clone();
  let clone_is_integrate_entry = is_integrate_entry.clone();
  let clone_input_name = input_name.clone();
  ret.ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    let arc_path_file_icon = ret.arc_path_file_icon.clone();
    let mut output_status = ret.ret_frame_footer.output_status.clone();

    clone_tx.send_awake(common::Msg::WindDeactivate);

    // Check if name field is valid
    let str_name = clone_input_name.value();
    if str_name.is_empty()
    {
      output_status.set_value("No application name was selected");
      clone_tx.send_awake(common::Msg::WindActivate);
      log!("No application name was selected");
      return;
    } // if

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

    let clone_is_integrate_entry = clone_is_integrate_entry.clone();
    std::thread::spawn(move ||
    {
      // Set as desktop entry icon for image
      // Wait for message & check return value
      let integration_items = match clone_is_integrate_entry.load(Ordering::SeqCst)
      {
        true => "mimetype,icon,entry",
        false => "mimetype,icon",
      };
      match gameimage::desktop::desktop(&str_name, &path_file_icon, integration_items)
      {
        Ok(()) => log!("Finished desktop configuration"),
        Err(e) => { clone_tx.send_awake(common::Msg::WindActivate); log_return_void!("{}", e); }
      } // match

      clone_tx.send_awake(common::Msg::DrawName);
      fltk::app::awake();
    });

  });
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
