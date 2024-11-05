use std::sync::{Arc,Mutex};
use std::path::PathBuf;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  button,
  button::CheckButton,
  frame::Frame,
  output,
  dialog,
  enums::{FrameType,Color},
};

use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;
use shared::std::PathBufExt;

use crate::gameimage;
use crate::dimm;
use crate::frame;
use crate::common;
use crate::log;
use crate::db;
use lazy_static::lazy_static;

lazy_static!
{
  pub static ref PROJECTS: Mutex<String> = Mutex::new(String::new());
}

// fn create_entry() {{{
fn create_entry(project : db::project::Entry
  , scroll : &mut shared::fltk::ScrollList
  , width: i32
  , height: i32) -> anyhow::Result<(button::CheckButton, db::project::Entry)>
{
  //
  // Icon
  //
  let width_icon = (height as f32 * 0.66666666) as i32;
  let mut frame_icon = Frame::default()
    .with_focus(false)
    .with_size(width_icon, height);

  // Include in scroll list
  scroll.add(&mut frame_icon.as_base_widget());

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
  let mut frame_info = output::MultilineOutput::default()
    .with_size(width - width_icon - dimm::width_checkbutton(), height)
    .right_of(&frame_icon, 0)
    .with_frame(FrameType::BorderBox)
    .with_color(Color::Background);
  frame_info.set_text_size(dimm::height_text());

  // Include fields in entry
  let mut f_add_field = |title: &str, field : &Option<String>, push_newline: bool|
  {
    if let Some(value) = field
    {
      let _ = frame_info.insert(title);
      let _ = frame_info.insert(value.as_str());
      if push_newline { let _ = frame_info.insert("\n"); }
    }
  }; // f_add_field
  f_add_field("Project: ", &Some(project.get_project()), true);
  f_add_field("Platform: ", &Some(project.get_platform()), true);
  f_add_field("Default rom: ", &project.get_path_relative(db::project::EntryName::PathFileRom).ok().map(|e| e.string()), true);
  f_add_field("Default core: ", &project.get_path_relative(db::project::EntryName::PathFileCore).ok().map(|e| e.string()), true);
  f_add_field("Default bios: ", &project.get_path_relative(db::project::EntryName::PathFileBios).ok().map(|e| e.string()), false);
  let _ = frame_info.set_position(0);

  //
  // CheckButton
  //
  let btn_checkbox = CheckButton::default()
    .with_size(dimm::width_checkbutton(), height)
    .right_of(&frame_info, 0)
    .with_focus(false)
    .with_frame(FrameType::BorderBox);

  Ok((btn_checkbox , project))
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
  if let Err(e) = common::dir_build()
  {
    log!("Err: {}", e.to_string());
  } // if

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();


  // Configure bottom buttons
  let clone_tx = tx.clone();
  ret_frame_footer.btn_prev.clone().set_callback(move |_|
  {
    if dialog::choice2_default("This will reset the image, are you sure?", "No", "Yes", "") == Some(1)
    {
      clone_tx.send_awake(common::Msg::DrawWelcome);
    } // if
  });

  let mut scroll = shared::fltk::ScrollList::new(
    frame_content.width() - dimm::border()*3 - dimm::width_button_rec()
    , frame_content.height() - dimm::border()*2
    , frame_content.x() + dimm::border()
    , frame_content.y() + dimm::border()
  );
  scroll.set_frame(FrameType::BorderBox);
  scroll.set_border(dimm::border(), dimm::border());

  scroll.begin();

  // Fetch entries
  let projects = match db::project::list()
  {
    Ok(projects) => projects,
    Err(e) => { log!("Could not get project list: {}", e); vec![] },
  };

  // Process entries if any
  let vec_btn = Arc::new(Mutex::new(Vec::<(button::CheckButton,db::project::Entry)>::new()));
  for project in &projects
  {
    let width_entry = scroll.widget_ref().w() - dimm::border()*2;
    let height_entry = dimm::height_button_rec()*4;
    let (button, project) = match create_entry(project.clone(), &mut scroll, width_entry, height_entry)
    {
      Ok(ret) => ret,
      Err(e) => { log!("Could not create entry for project with error: {}", e); continue; },
    }; // match

    match vec_btn.lock()
    {
      Ok(mut lock) => lock.push((button, project)),
      Err(e) => log!("Could not lock checkbox buttons with error: {}", e),
    }
  } // for

  scroll.end();

  // Add new package
  let mut btn_add = shared::fltk::button::rect::add()
    .right_of(scroll.widget_mut(), dimm::border())
    .with_color(Color::Green);
  btn_add.emit(tx, common::Msg::DrawPlatform);

  // Erase package
  let mut btn_del = shared::fltk::button::rect::del()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .below_of(&btn_add, dimm::border())
    .with_color(Color::Red);
  let clone_vec_checkbutton = vec_btn.clone();
  let clone_tx = tx.clone();
  btn_del.set_callback(move |_|
  {
    tx.send_awake(common::Msg::WindDeactivate);
    creator_del(clone_vec_checkbutton.lock().unwrap().iter().filter(|e| e.0.is_checked()).map(|e| e.1.clone()).collect());
    tx.send_awake(common::Msg::DrawCreator);
  });

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


}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
