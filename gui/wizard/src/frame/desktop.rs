use std::{
  path::PathBuf,
  sync::{Mutex,Arc,atomic::{AtomicBool, Ordering}},
};

use fltk::
{
  prelude::*,
  app::Sender,
  frame::Frame,
  group::Flex,
  enums::{Color,Align},
  input::FileInput,
  dialog::file_chooser,
};
use anyhow::anyhow as ah;

use shared::fltk::SenderExt;
use shared::fltk::WidgetExtExtra;
use shared::std::PathBufExt;

use crate::common;
use crate::dimm;
use crate::log;
use crate::log_err;
use crate::log_status;
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
    .ok_or(ah!("No icon selected"))?;
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
  // Save previously selected icon path
  static OPTION_PATH_FILE_ICON : once_cell::sync::Lazy<Arc<Mutex<Option<PathBuf>>>>
    = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

  let mut ui = crate::GUI.lock().unwrap().ui.clone()(title);

  let mut col = Flex::default()
    .column()
    .with_size_of(&ui.group)
    .with_pos_of(&ui.group);

  // Spacer
  col.add(&Frame::default());

  // Create icon box
  let mut row = Flex::default().row();
  row.set_spacing(dimm::border());
  let frame_icon = shared::fltk::frame::bordered()
    .with_size(150, 225);
  row.fixed(&frame_icon,150);

  let mut col_options = Flex::default().column();

  // Select application name with an input field
  // l
  col_options.fixed(
      &Frame::default().with_label("Select the application name").with_align(Align::Inside | Align::Left)
    , dimm::height_text()
  );
  let input_name = fltk::input::Input::default();
  col_options.fixed(&input_name, dimm::height_button_wide());

  // Button to enable desktop entry
  let f_create_atomic_option = move |label: &str| -> (Arc<AtomicBool>, fltk::button::CheckButton)
  {
    let atomic_option = Arc::new(AtomicBool::new(true));
    let clone_atomic_option = atomic_option.clone();
    let mut btn_check = shared::fltk::button::rect::checkbutton()
      .with_align(Align::Inside | Align::Left)
      .with_color(Color::BackGround)
      .with_label(label);
    btn_check.set_checked(true);
    let f_clone_atomic_option = move |val: bool| { clone_atomic_option.store(val, Ordering::SeqCst); };
    btn_check.set_callback(move |e| f_clone_atomic_option(e.is_checked()));
    (atomic_option.clone(), btn_check.clone())
  };

  // Get integration items
  let (is_integrate_entry, btn_integrate_entry) = f_create_atomic_option("Show icon in the start menu?");
  let (is_integrate_icon, btn_show) = f_create_atomic_option("Show icon file manager?");
  col_options.fixed(&btn_integrate_entry, dimm::width_checkbutton());
  col_options.fixed(&btn_show, dimm::width_checkbutton());
  col_options.end();
  row.add(&col_options);
  row.end();
  col.fixed(&row, 225);

  // Spacer
  col.add(&Frame::default());

  // Icon
  let mut row = Flex::default().row();
  let mut input_icon = FileInput::default();
  input_icon.set_readonly(true);
  input_icon.deactivate();
  row.add(&input_icon);
  let mut btn_search = shared::fltk::button::rect::search()
    .with_color(Color::Green);
  row.fixed(&btn_search, dimm::width_button_rec());
  row.end();
  col.fixed(&row, dimm::height_button_wide() + dimm::border()/2);

  // Check path cache
  if let Some(path_file_icon) = OPTION_PATH_FILE_ICON.lock().unwrap().clone()
  {
    input_icon.set_value(&path_file_icon.string());
    log_err!(crate::frame::icon::resize_draw_image(frame_icon.clone(), path_file_icon.clone()));
  } // if

  // // Set input_icon callback
  let mut clone_input_icon = input_icon.clone();
  btn_search.set_callback(move |_|
  {
    let str_choice = match file_chooser("Select the icon", "*.{jpg,png}", ".", false)
    {
      Some(str_choice) => str_choice,
      None => { log_status!("No file selected"); return; }
    }; // match
    // Update static icon
    *OPTION_PATH_FILE_ICON.lock().unwrap() = Some(PathBuf::from(&str_choice));
    // Show file path on selector
    clone_input_icon.set_value(str_choice.as_str());
    // Set preview image
    match crate::frame::icon::resize_draw_image(frame_icon.clone(), str_choice.into())
    {
      Ok(_) => log_status!("Set preview image"),
      Err(_) => log_status!("Failed to load icon image into preview"),
    } // match
  });

  // Callback to install projects and configure desktop integration
  let arc_path_file_icon = OPTION_PATH_FILE_ICON.clone();
  ui.btn_prev.emit(tx, common::Msg::DrawCreator);
  ui.btn_next.set_callback(#[clown] move |_|
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
        Err(e) => { log_status!("{}", e); tx.send_awake(common::Msg::WindActivate); return; },
      }; // match
      tx.send_activate(common::Msg::DrawFinish);
    });
  });

  col.end();
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
