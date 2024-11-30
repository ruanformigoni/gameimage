use std::sync::{Arc,Mutex};
use std::path::PathBuf;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  button,
  group,
  button::CheckButton,
  frame::Frame,
  output,
  dialog,
  enums::{FrameType,Color,Align},
};

use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;
use shared::std::PathBufExt;

use crate::gameimage;
use crate::dimm;
use crate::frame;
use crate::common;
use crate::log;
use crate::log_err;
use crate::db;
use lazy_static::lazy_static;

lazy_static!
{
  pub static ref PROJECTS: Mutex<String> = Mutex::new(String::new());
}

// fn create_entry() {{{
fn create_entry(project : db::project::Entry, height: i32)
  -> anyhow::Result<(group::Flex, button::CheckButton, db::project::Entry)>
{
  let mut row = fltk::group::Flex::default()
    .with_type(fltk::group::FlexType::Row)
    .with_size(0, height);

  //
  // Icon
  //
  let width_icon = (height as f32 * 0.66666666) as i32;
  let mut frame_icon = Frame::default()
    .with_focus(false)
    .with_size(width_icon, height);
  row.fixed(&frame_icon, width_icon);
  if let Ok(path_file_icon) = project.get_path_absolute(db::project::EntryName::PathFileIcon)
  {
    let path_file_resized = PathBuf::from(path_file_icon.clone())
      .parent()
      .unwrap()
      .join("icon.creator.resized.png");
    if let Err(e) = shared::image::resize(path_file_resized.clone()
      , path_file_icon.clone()
      , frame_icon.w() as u32, frame_icon.h() as u32)
    {
      log!("Failed to resize image {}, with error {}", path_file_icon.string(), e);
    }
    if let Ok(mut image) = fltk::image::SharedImage::load(path_file_resized)
    {
      image.scale(frame_icon.w(), frame_icon.h(), true, true);
      frame_icon.set_image_scaled(Some(image));
    } // if
  } // if
  else
  {
    log!("Could not read icon directory for project");
  } // else

  //
  // Info
  //
  let mut frame_info = fltk::group::Flex::default()
    .column()
    .with_frame(FrameType::BorderBox)
    .with_size(0, height);
  frame_info.set_margins(dimm::border()/2, dimm::border()/2, dimm::border()/2, 0);
  let mut f_add_field = |title: &str, field : &str|
  {
    let mut frame_entry = fltk::group::Flex::default().column();
    frame_entry.set_spacing(dimm::border()/2);
    let label = Frame::default()
      .with_label(title)
      .with_size(0, dimm::height_text())
      .with_align(Align::Inside | Align::Left);
    frame_entry.fixed(&label, dimm::height_text());
    let mut output = output::Output::default();
    output.set_value(field);
    frame_entry.fixed(&output, dimm::height_button_wide());
    frame_entry.end();
    frame_info.add(&frame_entry);
  }; // f_add_field
  f_add_field("PROJECT", &project.get_project());
  f_add_field("PLATFORM", &project.get_platform());
  frame_info.end();
  row.add(&frame_info);

  //
  // CheckButton
  //
  let btn_checkbox = CheckButton::default()
    .with_size(dimm::width_checkbutton(), height)
    .with_focus(false)
    .with_frame(FrameType::BorderBox);
  row.fixed(&btn_checkbox, btn_checkbox.w());

  row.end();

  Ok((row, btn_checkbox , project))
} // }}}

// creator_del() {{{
fn creator_del(vec_project: Vec<db::project::Entry>)
{
  if dialog::choice2_default("Erase the selected projects?", "No", "Yes", "") != Some(1)
  {
    return;
  } // if

  // Remove all currently selected projects
  for str_name in vec_project.iter().map(|e| e.get_project())
  {
    if let Err(e) = gameimage::project::del(&str_name)
    {
      log!("Could not erase project '{}': {}", str_name, e)
    }
  } // for
} // creator_del() }}}

// pub fn creator() {{{
pub fn creator(tx: Sender<common::Msg>, title: &str)
{
  // Enter the build directory
  log_err!(common::dir_build());

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();
  let frame_content = ret_frame_header.frame_content.clone();

  // Configure bottom buttons
  ret_frame_footer.btn_prev.clone().emit(tx, common::Msg::DrawWelcome);

  let mut row_content = fltk::group::Flex::default()
    .with_type(fltk::group::FlexType::Row)
    .with_size(frame_content.w(), frame_content.h())
    .with_pos(frame_content.x(), frame_content.y());
  row_content.set_spacing(dimm::border() / 2);

  let mut scroll = fltk::group::Scroll::default()
    .with_size(row_content.w() - dimm::border()*2 - dimm::width_button_rec(), row_content.h());
  scroll.set_type(fltk::group::ScrollType::VerticalAlways);
  scroll.set_scrollbar_size(dimm::border());

  // Fetch entries
  let projects = match db::project::list()
  {
    Ok(projects) => projects,
    Err(e) => { log!("Could not get project list: {}", e); vec![] },
  };

  let mut col_projects = fltk::group::Pack::default_fill()
    .with_type(fltk::group::PackType::Vertical);
  col_projects.set_spacing(dimm::border());

  // Process entries if any
  let vec_btn = Arc::new(Mutex::new(Vec::<(button::CheckButton,db::project::Entry)>::new()));
  for project in &projects
  {
    let (row_project, button, project) = match create_entry(project.clone(), dimm::height_button_rec()*4)
    {
      Ok(ret) => ret,
      Err(e) => { log!("Could not create entry for project with error: {}", e); continue; },
    }; // match
    col_projects.add(&row_project);

    match vec_btn.lock()
    {
      Ok(mut lock) => lock.push((button, project)),
      Err(e) => log!("Could not lock checkbox buttons with error: {}", e),
    }
  } // for

  col_projects.end();
  scroll.add(&col_projects);
  scroll.end();
  row_content.add(&scroll);

  let mut col_buttons = fltk::group::Flex::default_fill()
    .with_type(fltk::group::FlexType::Column)
    .with_size(dimm::width_button_rec(), row_content.h());
  col_buttons.set_spacing(dimm::border());

  // Add new package
  let mut btn_add = shared::fltk::button::rect::add().with_color(Color::Green);
  btn_add.emit(tx, common::Msg::DrawPlatform);
  col_buttons.fixed(&btn_add, dimm::height_button_rec());

  // Erase package
  let mut btn_del = shared::fltk::button::rect::del().with_color(Color::Red);
  let clone_vec_checkbutton = vec_btn.clone();
  let clone_tx = tx.clone();
  btn_del.set_callback(move |_|
  {
    tx.send_awake(common::Msg::WindDeactivate);
    creator_del(clone_vec_checkbutton.lock().unwrap().iter().filter(|e| e.0.is_checked()).map(|e| e.1.clone()).collect());
    tx.send_awake(common::Msg::DrawCreator);
  });
  col_buttons.fixed(&btn_del, dimm::height_button_rec());
  row_content.fixed(&col_buttons, dimm::width_button_rec());

  // Finish package creation on click next
  let clone_vec_btn = vec_btn.clone();
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    if dialog::choice2_default("Include selected projects in the image?", "No", "Yes", "") != Some(1)
    {
      return;
    } // if

    match clone_vec_btn.lock()
    {
      Ok(e) => if e.is_empty()
      {
        log!("No project to include");
        clone_output_status.set_value("No project to include");
        return;
      } // if
      else if ! e.iter().any(|e| e.0.is_set())
      {
        log!("No project was selected");
        clone_output_status.set_value("No project was selected");
        return;
      } // else if
      Err(e) => { log!("Could not lock projects vector: {}", e); return; }
    }

    // Set status
    clone_output_status.set_value("Inserting projects in the image");

    // Disable window
    clone_tx.send_awake(common::Msg::WindDeactivate);

    // Include files in new thread
    let clone_vec_checkbutton = clone_vec_btn.clone();
    std::thread::spawn(move ||
    {
      // Get lock to vector of checkbuttons
      let lock = match clone_vec_checkbutton.lock()
      {
        Ok(lock) => lock,
        Err(e) =>
        {
          clone_tx.send_awake(common::Msg::WindActivate);
          log!("Failed to lock checkbutton vector with: {}", e); return;
        }
      }; // match
      // Transform projects in a ':' separated string to send to the backend
      let str_name_projects = lock.iter()
        .filter(|e| e.0.is_checked())
        .map(|e| e.1.get_project())
        .collect::<Vec<String>>()
        .join(":");
      // Update projects list
      match PROJECTS.lock()
      {
        Ok(mut guard) => *guard = str_name_projects.clone(),
        Err(e) => log!("Could not lock PROJECTS: {}", e),
      }; // match
      // Refresh
      clone_tx.send_awake(common::Msg::WindActivate);
      clone_tx.send_awake(common::Msg::DrawDesktop);
    });
  });
  row_content.end();
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
