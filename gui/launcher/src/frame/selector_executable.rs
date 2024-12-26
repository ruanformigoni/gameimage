use std::collections::HashMap;

use fltk::prelude::*;
use fltk::{
  app::Sender,
  frame::Frame,
  enums,
  group,
};

use anyhow::anyhow as ah;

use shared::dimm;
use shared::fltk::WidgetExtExtra;
use shared::std::PathBufExt;
use shared::{fixed,hover_blink,hseparator_fixed,column,hpack,scroll,row,rescope};

use common::Msg;

use crate::common;
use crate::db;

// fn: get_path_db_executable() {{{
fn get_path_db_executable() -> anyhow::Result<std::path::PathBuf>
{
  let mut path_db : std::path::PathBuf = std::env::var("GIMG_LAUNCHER_ROOT")?.into();
  path_db.push("gameimage.executable.json");

  Ok(path_db)
} // fn: get_path_db_executable() }}}

// fn: get_path_db_alias() {{{
fn get_path_db_alias() -> anyhow::Result<std::path::PathBuf>
{
  let mut path_db : std::path::PathBuf = std::env::var("GIMG_LAUNCHER_ROOT")?.into();
  path_db.push("gameimage.alias.json");
  Ok(path_db)
} // fn: get_path_db_alias() }}}

// fn: get_default_executable() {{{
fn get_default_executable() -> anyhow::Result<std::path::PathBuf>
{
  let path_file_db = std::path::PathBuf::from(std::env::var("GIMG_LAUNCHER_ROOT")? + "/gameimage.json");
  let db_project = db::project::read(&path_file_db)?;
  Ok(db_project.path_file_rom.ok_or(ah!("Could not read path_file_rom"))?.into())
} // fn: get_default_executable()}}}

// fn: get_menu_entries() {{{
pub fn get_menu_entries() -> anyhow::Result<(Vec<String>,HashMap<String,String>)>
{
  // Read executables from database
  let db_executables = shared::db::kv::read(&get_path_db_executable()?).unwrap_or_default();
  // Read executables from database
  let db_alias = shared::db::kv::read(&get_path_db_alias()?).unwrap_or_default();
  // Gather executables in a vector
  let mut executables: Vec<String> = db_executables.keys().cloned().into_iter().collect();
  // Avoid duplicate of default executable in the list
  let default_executable = get_default_executable()?.string();
  if ! executables.contains(&default_executable)
  {
    executables.push(default_executable);
  } // if
  executables.sort_by_key(|e| db_alias.get(&e.clone()).unwrap_or(&e.clone()).clone());
  Ok((executables, db_alias))
} // fn: new_menu_entries() }}}

// fn: new {{{
pub fn new(tx : Sender<Msg>)
{
  // Layout
  column!(col,
    col.set_margin(dimm::border_half());
    fixed!(col, frame_title, Frame::default(), dimm::height_text());
    hseparator_fixed!(col, col.w() - dimm::border()*2, dimm::border_half());
    scroll!(scroll_content,
      hpack!(col_content, );
      col_content.set_spacing(dimm::border_half());
    );
    hseparator_fixed!(col, col.w() - dimm::border()*2, dimm::border_half());
    column!(col_bottom,
      row!(row_bottom,
        fixed!(row_bottom, btn_back, shared::fltk::button::rect::back(), dimm::width_button_rec());
      );
      col_bottom.fixed(&row_bottom, dimm::height_button_rec());
    );
    col.fixed(&col_bottom, dimm::height_button_rec());
  );
  // Title
  let mut frame_title = frame_title.clone();
  frame_title.set_label("Executable Selection");
  // Auto resize column to scroll width
  scroll_content.resize_callback({let mut c = col_content.clone(); move |_,_,_,w,_|
  {
    c.resize(c.x(),c.y(),w-dimm::border_half()*3,c.h());
  }});
  scroll_content.set_type(group::ScrollType::VerticalAlways);
  // Footer button
  let mut btn_back = btn_back.clone();
  btn_back.emit(tx, Msg::DrawCover);
  hover_blink!(btn_back);
  // Entries
  rescope!(col_content,
    let (executables, aliases) = get_menu_entries().unwrap_or_default();
    for entry in executables
    {
      row!(row_entry,
        let mut btn = fltk::button::ToggleButton::default()
          .with_size(0, dimm::height_button_wide() + dimm::border_half())
          .with_frame(enums::FrameType::FlatBox)
          .with_color(enums::Color::BackGround)
          .with_color_selected(enums::Color::BackGround.lighter())
          .with_align(enums::Align::Left | enums::Align::Inside);
        hover_blink!(btn);
        // Define the trimming logic
        let clone_aliases = aliases.clone();
        let clone_entry = entry.clone();
        let trim_label = move |button: &mut fltk::button::ToggleButton|
        {
          let btn_width = button.width();
          let label = clone_aliases.get(&clone_entry).unwrap_or(&clone_entry).clone();
          let mut label_trimmed = label.clone();
          while fltk::draw::measure(&label_trimmed, false).0 > btn_width as i32
          {
            if label_trimmed.len() > 0 { label_trimmed.remove(0); } // if
          } // while
          if label_trimmed != label && label_trimmed.len() > 3
          {
            label_trimmed.replace_range(0..3, "...");
          } // if
          button.set_label(&label_trimmed);
        };
        // Apply the resize callback for label
        btn.resize_callback(move |btn, _, _, _, _| { trim_label(btn); });
        // Set selection callback
        btn.set_callback({
          let tx = tx.clone();
          move |_|
          {
            std::env::set_var("GIMG_LAUNCHER_EXECUTABLE", &entry);
            tx.send(common::Msg::DrawCover);
          }
        });
        hover_blink!(btn);
        row_entry.add(&btn);
      );
      row_entry.resize(row_entry.x(), row_entry.y(), row_entry.w(), dimm::height_button_wide() + dimm::border_half());
    } // for
  );
} // fn: new }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
