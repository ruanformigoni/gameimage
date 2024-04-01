use std::sync::{Arc,Mutex};
use std::path::PathBuf;
use std::fs;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  button,
  button::{Button,CheckButton},
  group,
  frame::Frame,
  output,
  dialog,
  enums::{FrameType,Color},
};

use crate::dimm;
use crate::frame;
use crate::common;
use crate::common::PathBufExt;
use crate::common::WidgetExtExtra;
use crate::log;
use crate::db;
use crate::lib::svg;

// fn create_entry() {{{
fn create_entry(project : db::project::Entry
  , scroll : &mut common::ScrollList
  , width: i32
  , height: i32) -> anyhow::Result<(button::CheckButton, PathBuf)>
{
  let frame_pack = group::Pack::default()
    .with_size(width, height)
    .with_type(group::PackType::Horizontal);

  // Include in scroll list
  scroll.add(&mut frame_pack.as_base_widget(), dimm::border());

  //
  // Icon
  //
  let width_icon = (height as f32 * 0.66666666) as i32;
  let mut frame_icon = Frame::default()
    .with_focus(false)
    .with_size(width_icon, height);

  if let Ok(path_file_icon) = project.get_path_absolute(db::project::EntryName::PathFileIcon)
  {
    log!("Path to icon: {}", path_file_icon.string());
    let path_file_resized = PathBuf::from(path_file_icon.clone())
      .parent()
      .unwrap()
      .join("icon.creator.resized.png");
    if let Err(e) = common::image_resize(path_file_resized.clone()
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
  f_add_field("Project: ", &project.get_project().ok(), true);
  f_add_field("Platform: ", &project.get_platform().ok(), true);
  f_add_field("Default rom: ", &project.get_path_relative(db::project::EntryName::PathFileRom).ok().map(|e| e.string()), true);
  f_add_field("Default core: ", &project.get_path_relative(db::project::EntryName::PathFileCore).ok().map(|e| e.string()), true);
  f_add_field("Default bios: ", &project.get_path_relative(db::project::EntryName::PathFileBios).ok().map(|e| e.string()), false);
  let _ = frame_info.set_position(0);

  //
  // CheckButton
  //
  let btn_checkbox = CheckButton::default()
    .with_size(dimm::width_checkbutton(), height)
    .with_focus(false)
    .with_frame(FrameType::BorderBox);

  frame_pack.end();

  Ok((btn_checkbox , project.get_dir_self()?))
} // }}}

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
      clone_tx.send(common::Msg::DrawFetch);
    } // if
  });

  // Finish package creation on click next
  ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    if dialog::choice2_default("Finish image creation?", "No", "Yes", "") == Some(1)
    {
      clone_tx.send(common::Msg::DrawDesktop);
    } // if
  });

  let mut scroll = common::ScrollList::new(
    frame_content.width() - dimm::border()*3 - dimm::width_button_rec()
    , frame_content.height() - dimm::border()*2
    , frame_content.x() + dimm::border()
    , frame_content.y() + dimm::border()
  );

  scroll.begin();

  // Fetch entries
  let projects = match db::project::list()
  {
    Ok(projects) => projects,
    Err(e) => { log!("Could not get project list: {}", e); vec![] },
  };

  // Process entries if any
  let vec_btn = Arc::new(Mutex::new(Vec::<(button::CheckButton,PathBuf)>::new()));
  for project in &projects
  {
    let width_entry = scroll.widget_ref().w() - scroll.widget_ref().scrollbar_size() - dimm::border();
    let height_entry = dimm::height_button_rec()*4;
    let (button, path_dir_project) = match create_entry(project.clone(), &mut scroll, width_entry, height_entry)
    {
      Ok(ret) => ret,
      Err(e) => { log!("Could not create entry for project with error: {}", e); continue; },
    }; // match

    match vec_btn.lock()
    {
      Ok(mut lock) => lock.push((button, path_dir_project)),
      Err(e) => log!("Could not lock checkbox buttons with error: {}", e),
    }
  } // for

  scroll.end();

  // Add new package
  let mut btn_add = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .right_of(scroll.widget_mut(), dimm::border());
  btn_add.set_frame(FrameType::RoundedFrame);
  btn_add.visible_focus(false);
  btn_add.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_add(1.0).as_str()).unwrap()));
  btn_add.set_color(Color::Green);
  btn_add.emit(tx, common::wizard_by_platform().unwrap_or(common::Msg::DrawCreator));

  // Erase package
  let mut btn_del = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .below_of(&btn_add, dimm::border());
  btn_del.set_frame(FrameType::RoundedFrame);
  btn_del.visible_focus(false);
  btn_del.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_del(1.0).as_str()).unwrap()));
  btn_del.set_color(Color::Red);
  let clone_vec_checkbutton = vec_btn.clone();
  let clone_tx = tx.clone();
  btn_del.set_callback(move |_|
  {
    if dialog::choice2_default("Erase the selected projects?", "No", "Yes", "") != Some(1)
    {
      return;
    } // if

    let lock = match clone_vec_checkbutton.lock()
    {
      Ok(lock) => lock,
      Err(e) => { log!("Could not acquire lock for checkbutton: {}", e); return; },
    };

    // Remove all currently selected projects
    for (checkbutton, path_dir_project) in lock.iter()
    {
      if checkbutton.is_checked()
      {
        let _ = fs::remove_file(path_dir_project.with_extension("dwarfs"));
        let _ = fs::remove_dir_all(path_dir_project);
      }
    } // for

    // Refresh
    clone_tx.send(common::Msg::DrawCreator);
  });

  // Include inside image
  let mut btn_insert = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .below_of(&btn_del, dimm::border());
  btn_insert.set_color(Color::Blue);
  btn_insert.set_frame(FrameType::RoundedFrame);
  btn_insert.visible_focus(false);
  btn_insert.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_box_heart(1.0).as_str()).unwrap()));
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  let clone_tx = tx.clone();
  btn_insert.set_callback(move |_|
  {
    if dialog::choice2_default("Include selected projects in the image?", "No", "Yes", "") != Some(1)
    {
      return;
    } // if

    // Set status
    clone_output_status.set_value("Inserting projects in the image");

    // Disable window
    clone_tx.send(common::Msg::WindDeactivate);

    // Include files in new thread
    let clone_vec_checkbutton = vec_btn.clone();
    std::thread::spawn(move ||
    {
      // Get lock to vector of checkbuttons
      let lock = match clone_vec_checkbutton.lock()
      {
        Ok(lock) => lock,
        Err(e) => { log!("Failed to lock checkbutton vector with: {}", e); return; }
      }; // match

      // Remove all currently selected projects
      for (checkbutton, path_dir_project) in lock.iter()
      {
        if ! checkbutton.is_checked() { continue; } // if

        let path_file_dwarfs = path_dir_project.with_extension("dwarfs");

        log!("File: {}", path_file_dwarfs.string());

        // Wait for message & check return value
        if common::gameimage_sync(vec!["package", &path_file_dwarfs.string()]) != 0
        {
          log!("Could not include {} into the image", path_file_dwarfs.string());
        } // if
      } // for

      // Refresh
      clone_tx.send(common::Msg::DrawCreator);
    });
  });

}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
