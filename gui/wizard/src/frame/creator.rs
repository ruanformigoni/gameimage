use std::sync::{Arc,Mutex};
use std::path::PathBuf;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  button,
  group,
  frame::Frame,
  output,
  dialog,
  enums::{FrameType,Color,Align},
};

use clown::clown;
use lazy_static::lazy_static;

use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;
use shared::std::PathBufExt;
use shared::{rescope,hover_blink,column,row,fixed,scroll,hpack};

use crate::gameimage;
use crate::dimm;
use crate::common;
use crate::log;
use crate::log_err;
use crate::log_status;
use crate::db;

lazy_static!
{
  pub static ref PROJECTS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

// fn create_entry() {{{
fn create_entry(project : db::project::Entry, height: i32)
  -> anyhow::Result<(group::Flex, button::CheckButton, db::project::Entry)>
{
  let mut row = fltk::group::Flex::default()
    .row()
    .with_color(Color::lighter(&Color::BackGround))
    .with_frame(FrameType::FlatBox)
    .with_size(0, height);
  row.set_spacing(0);
  row.set_margins(0, 0, dimm::border_half(), 0);

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
  let btn_checkbox = shared::fltk::button::rect::checkbutton()
    .with_size(dimm::width_checkbutton(), height)
    .with_focus(false)
    .with_frame(FrameType::NoBox);
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
      log_status!("Could not erase project '{}': {}", str_name, e)
    }
  } // for
} // creator_del() }}}

// pub fn creator() {{{
pub fn creator(tx: Sender<common::Msg>, title: &str)
{
  // Enter the build directory
  log_err!(common::dir_build());

  let ui = crate::GUI.lock().unwrap().ui.clone()(title);

  row!(row,
    scroll!(scroll,
      hpack!(col_projects,
        col_projects.set_spacing(dimm::border_half());
      );
    );
    row.add(&scroll);
    column!(col_buttons,
      fixed!(col_buttons, btn_add, shared::fltk::button::rect::add().with_color(Color::Green), dimm::height_button_rec());
      fixed!(col_buttons, btn_del, shared::fltk::button::rect::del().with_color(Color::Red), dimm::height_button_rec());
      fixed!(col_buttons, btn_sel_all, shared::fltk::button::rect::check_all().with_color(Color::Blue), dimm::height_button_rec());
      col_buttons.add(&Frame::default_fill());
    );
    row.fixed(&col_buttons, dimm::width_button_rec());
  );

  // Configure row
  row.set_spacing(dimm::border_half());
  // Configure scroll
  scroll.set_type(fltk::group::ScrollType::VerticalAlways);
  scroll.set_scrollbar_size(dimm::border());
  // Configure bottom buttons
  ui.btn_prev.clone().emit(tx, common::Msg::DrawWelcome);
  // Add scroll entries
  let projects = match db::project::list()
  {
    Ok(mut projects) => { projects.sort_by_key(|e| e.get_project()); projects},
    Err(e) => { log_status!("Could not get project list: {}", e); vec![] },
  };
  // Configure col resize
  scroll.resize_callback(#[clown] move |_s,x,y,w,_h|
  {
    let mut col = honk!(col_projects).clone();
    col.resize(x,y,w-dimm::border_half()*3,col.h());
  });

  rescope!(col_projects,
    // Process entries if any
    let vec_btn = Arc::new(Mutex::new(Vec::<(button::CheckButton,db::project::Entry)>::new()));
    // Select all button
    btn_sel_all.clone().set_callback(#[clown] move |e|
    {
      e.set(!e.is_set());
      honk!(vec_btn).lock().unwrap().iter().for_each(|f| f.0.set_checked(e.is_set()))
    });
    // Include select all button and projects in the column
    for project in &projects
    {
      let (row_project, button, project) = match create_entry(project.clone(), dimm::height_button_rec()*4)
      {
        Ok(ret) => ret,
        Err(e) => { log_status!("Could not create entry for project with error: {}", e); continue; },
      }; // match
      col_projects.add(&row_project);

      match vec_btn.lock()
      {
        Ok(mut lock) => lock.push((button, project)),
        Err(e) => log_status!("Could not lock checkbox buttons with error: {}", e),
      }
    } // for
  );

  // Add new package
  let mut btn_add = btn_add.clone();
  hover_blink!(btn_add);
  btn_add.emit(tx, common::Msg::DrawPlatform);

  // Erase package
  let mut btn_del = btn_del.clone();
  hover_blink!(btn_del);
  let clone_vec_checkbutton = vec_btn.clone();
  let clone_tx = tx.clone();
  btn_del.set_callback(move |_|
  {
    tx.send_awake(common::Msg::WindDeactivate);
    let clone_tx = tx.clone();
    let clone_vec_checkbutton = clone_vec_checkbutton.clone();
    std::thread::spawn(move ||
    {
      creator_del(clone_vec_checkbutton.lock().unwrap().iter().filter(|e| e.0.is_checked()).map(|e| e.1.clone()).collect());
      clone_tx.send_activate(common::Msg::DrawCreator);
    });
  });

  // Finish package creation on click next
  let clone_vec_btn = vec_btn.clone();
  ui.btn_next.clone().set_callback(move |_|
  {
    if dialog::choice2_default("Include selected projects in the image?", "No", "Yes", "") != Some(1)
    {
      return;
    } // if

    match clone_vec_btn.lock()
    {
      Ok(e) => if e.is_empty()
      {
        log_status!("No project to include");
        return;
      } // if
      else if ! e.iter().any(|e| e.0.is_set())
      {
        log_status!("No project was selected");
        return;
      } // else if
      Err(e) => { log_status!("Could not lock projects vector: {}", e); return; }
    }

    // Disable window
    clone_tx.send_awake(common::Msg::WindDeactivate);

    // Include files in new thread
    let clone_vec_checkbutton = clone_vec_btn.clone();
    std::thread::spawn(move ||
    {
      // Vector of checkbuttons
      let vec_btn = clone_vec_checkbutton.lock().unwrap();
      // Save projects in current state
      *PROJECTS.lock().unwrap() = vec_btn.iter()
        .filter(|e| e.0.is_checked())
        .map(|e| e.1.get_project())
        .collect::<Vec<String>>();
      // Refresh
      clone_tx.send_activate(common::Msg::DrawDesktop);
    });
  });
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
