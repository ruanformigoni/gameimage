use std::sync::{Mutex,Arc,atomic::{AtomicBool, Ordering}};
use fltk::
{
  prelude::*,
  app::Sender,
};
use anyhow::anyhow as ah;

use shared::fltk::SenderExt;
use shared::fltk::WidgetExtExtra;

use crate::common;
use crate::dimm;
use crate::log;
use crate::gameimage;
use crate::frame;
use clown::clown;

// fn desktop_next() {{{
fn desktop_next(tx: Sender<common::Msg>
  , str_name: String
  , arc_path_file_icon: Arc<Mutex<Option<std::path::PathBuf>>>
  , arc_is_integrate_entry: Arc<AtomicBool>
  , arc_is_integrate_icon: Arc<AtomicBool>) -> anyhow::Result<()>
{
  // Check if name field is valid
  if str_name.is_empty() { return Err(ah!("No application name was selected")); } // if
  // Check if an icon was selected
  let path_file_icon = arc_path_file_icon.clone().lock()
    .map_err(|_| ah!("Could not lock path_file_icon"))
    .map(|mut e| e.take())?
    .ok_or(ah!("No file selected as the icon"))?;
  // Get projects to include
  let str_name_projects = match frame::creator::PROJECTS.lock()
  {
    Ok(guard) => guard,
    Err(e) => { return Err(ah!("Could not lock PROJECTS: {}", e)); }
  }; // match
  // Package projects
  log!("Projects to include in the image: {}", str_name_projects);
  // Wait for message & check return value
  if let Err(e) = gameimage::package::package(&str_name, &str_name_projects)
  {
    return Err(ah!("Could not include projects into the image: {}", e));
  } // match
  // Desktop integration
  let mut vec_integration_items = Vec::<String>::new();
  if arc_is_integrate_entry.load(Ordering::SeqCst)
  {
    vec_integration_items.append(&mut vec!["entry".into(), "icon".into()]);
  } // if
  if arc_is_integrate_icon.load(Ordering::SeqCst)
  {
    vec_integration_items.push("mimetype".into());
  } // if
  // Setup integration
  let integration_items = vec_integration_items.join(",");
  if ! integration_items.is_empty()
  {
    match gameimage::desktop::icon(&path_file_icon)
    {
      Ok(()) => log!("Finished icon configuration"),
      Err(e) => { tx.send_awake(common::Msg::WindActivate); return Err(ah!("{}", e)) }
    } // match
    match gameimage::desktop::desktop(&str_name, &integration_items)
    {
      Ok(()) => log!("Finished desktop configuration"),
      Err(e) => { tx.send_awake(common::Msg::WindActivate); return Err(ah!("{}", e)) }
    } // match
  } // if
  Ok(())
} // fn desktop_next() }}}

// pub fn desktop() {{{
pub fn desktop(tx: Sender<common::Msg>, title: &str)
{
  let ret_frame_icon = crate::frame::icon::icon(tx
    , title
    , common::Msg::DrawDesktop
    , common::Msg::DrawDesktop
  );

  // Hide prev button
  ret_frame_icon.ret_frame_footer.btn_prev.clone().hide();
  let frame_sep = ret_frame_icon.ret_frame_footer.sep.clone();

  // Move icon frame to the left
  let mut frame_icon = match ret_frame_icon.opt_frame_icon.clone()
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
  let f_create_atomic_option = move |label: &str, widget_parent: &fltk::widget::Widget|
    -> (Arc<AtomicBool>, fltk::button::CheckButton)
  {
    let atomic_option = Arc::new(AtomicBool::new(true));
    let clone_atomic_option = atomic_option.clone();
    let mut btn_check = fltk::button::CheckButton::default()
      .with_size(dimm::width_checkbutton(), dimm::width_checkbutton())
      .below_of(widget_parent, dimm::border())
      .with_align(fltk::enums::Align::Right)
      .with_focus(false)
      .with_label(label);
    btn_check.set_checked(true);
    let f_clone_atomic_option = move |val: bool| { clone_atomic_option.store(val, Ordering::SeqCst); };
    btn_check.set_callback(move |e| f_clone_atomic_option(e.is_checked()));
    (atomic_option.clone(), btn_check.clone())
  };

  // Get integration items
  let (is_integrate_entry, btn_integrate_entry) = f_create_atomic_option("Show icon in the start menu?", &input_name.as_base_widget());
  let (is_integrate_icon, _) = f_create_atomic_option("Show icon file manager?", &btn_integrate_entry.as_base_widget());

  // Callback to install projects and configure desktop integration
  let mut clone_btn_next = ret_frame_icon.ret_frame_footer.btn_next.clone();
  let arc_path_file_icon = ret_frame_icon.arc_path_file_icon.clone();
  clone_btn_next.set_callback(#[clown] move |_|
  {
    tx.send_awake(common::Msg::WindDeactivate);
    std::thread::spawn(#[clown] move ||
    {
      match desktop_next(tx, honk!(input_name).value()
        , honk!(arc_path_file_icon).clone()
        , honk!(is_integrate_entry).clone()
        , honk!(is_integrate_icon).clone())
      {
        Ok(()) => (),
        Err(e) => { log!("{}", e); tx.send_awake(common::Msg::WindActivate); return; },
      }; // match
      tx.send_awake(common::Msg::DrawFinish);
    });
  });
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
