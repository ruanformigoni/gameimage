use fltk::prelude::*;
use fltk::{
  output,
  app::Sender,
  group,
  button,
  enums::{Align,Color},
  frame::Frame,
};

use anyhow::anyhow as ah;

use shared::dimm;
use shared::std::PathBufExt;
use shared::std::OsStrExt;
use shared::{fixed,row,column,hpack,scroll,hover_blink,rescope};

use crate::common::Msg;

// fn find_executables() {{{
fn find_executables() -> anyhow::Result<Vec<std::path::PathBuf>>
{
  let mut ret = vec![];

  let path_dir_boot = std::path::PathBuf::from(std::env::var("GIMG_LAUNCHER_BOOT")?)
    .parent()
    .ok_or(ah!("Could not fetch parent path for boot directory"))?
    .to_owned();

  let mut path_dir_wine = path_dir_boot.clone();
  path_dir_wine.push("wine");

  for entry in walkdir::WalkDir::new(&path_dir_wine)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    let path = entry.into_path();
    // Skip if is not a regular file
    if ! path.is_file() { continue; }
    // Skip windows folder
    if path.components().any(|e| e.as_os_str().string() == "windows") { continue; }
    // Check if is an executable file
    if ! path.file_name_string().to_lowercase().ends_with(".exe")
    && ! path.file_name_string().to_lowercase().ends_with(".msi")
    {
      continue;
    } // if
    // Make path relative
    match path.strip_prefix(path_dir_boot.clone())
    {
      Ok(e) => ret.push(e.to_path_buf()),
      Err(_) => (),
    }
  } // for

  Ok(ret)
} // find_executables() }}}

// get_path_db_executable() {{{
fn get_path_db_executable() -> anyhow::Result<std::path::PathBuf>
{
  let mut path_db : std::path::PathBuf = std::env::var("GIMG_LAUNCHER_ROOT")?.into();
  path_db.push("gameimage.wine.executable.json");
  Ok(path_db)
} // get_path_db_executable() }}}

// get_path_db_args() {{{
fn get_path_db_args() -> anyhow::Result<std::path::PathBuf>
{
  let mut path_db : std::path::PathBuf = std::env::var("GIMG_LAUNCHER_ROOT")?.into();
  path_db.push("gameimage.wine.args.json");
  Ok(path_db)
} // get_path_db_args() }}}

// fn: new_layout_entry {{{
fn new_layout_entry(str_output: &str, is_use: bool)
  -> (output::Output,button::CheckButton,fltk_evented::Listener<fltk::input::Input>)
{
  column!(col,
    col.set_spacing(dimm::border_half());
    // 'Executable' / 'Use' row
    row!(row,
      // Executable label and output field
      column!(col_output,
        col_output.set_spacing(dimm::border_half());
        fixed!(col_output
          , label
          , Frame::default().with_label("Executable").with_align(Align::Inside | Align::Left)
          , dimm::height_text()
        );
        fixed!(col_output, output_executable, output::Output::default(), dimm::height_button_rec());
        let _ = output_executable.clone().insert(str_output);
      );
      row.add(&col_output);
      // 'Use' label and button
      column!(col_btn,
        col_btn.set_spacing(dimm::border_half());
        fixed!(col_btn
          , label
          , Frame::default().with_label("Use").with_align(Align::Inside | Align::Left)
          , dimm::height_text()
        );
        fixed!(col_btn, btn_use, shared::fltk::button::rect::checkmark::<fltk::button::CheckButton>(), dimm::height_button_rec());
        btn_use.clone().set_value(is_use);
      );
      row.fixed(&col_btn, dimm::width_button_rec());
    );
    col.fixed(&row, dimm::height_button_rec() + dimm::height_text() + dimm::border_half());
    // 'Arguments' label and input field
    column!(col_args,
      col_args.fixed(&Frame::default().with_label("Arguments").with_align(Align::Inside | Align::Left) , dimm::height_text());
      let input_arguments : fltk_evented::Listener<_> = fltk::input::Input::default().into();
      col_args.fixed(&input_arguments.as_base_widget(), dimm::height_button_wide());
    );
    col.fixed(&col_args.clone(), shared::fit_to_children_height!(col_args));
    col.fixed(&shared::fltk::separator::horizontal(col.w()), dimm::height_sep());
    col.fixed(&Frame::default(), 0);
  );
  shared::fit_to_children_height!(col);
  (output_executable, btn_use, input_arguments)
} // fn: new_layout_entry }}}

// fn: new_callback {{{
fn new_callback(output: output::Output
  , mut btn_use: button::CheckButton
  , mut input: fltk_evented::Listener<fltk::input::Input>
  , key: String
  , path_file_db_executable: std::path::PathBuf)
{
  // Setup 'use button' callback
  let clone_output_executable = output.clone();
  btn_use.set_callback({
    let output = output.clone();
    let path_file_db_executable = path_file_db_executable.clone();
    move |e|
    {
      if e.is_checked()
      {
        shared::db::kv::write(&path_file_db_executable, &output.value(), &"1".to_string())
          .map_err(|e| eprintln!("Could not insert key '{}' in db: {}", output.value(), e)).ok();
      } // if
      else
      {
        shared::db::kv::erase(&path_file_db_executable, output.value())
          .map_err(|e| eprintln!("Could not remove key '{}' from db: {}", output.value(), e)).ok();
      } // else
    } // fn
  });

  let path_file_db_args = match get_path_db_args()
  {
    Ok(e) => e,
    Err(e) => { eprintln!("Could not retrieve path to db file: {}", e); std::path::PathBuf::default() }
  }; // match

  if let Ok(db) = shared::db::kv::read(&path_file_db_args) && db.contains_key(&key)
  {
    let _ = input.insert(&db[&key]);
  } // if
  input.on_keyup(move |e|
  {
    if e.value().is_empty()
    {
      shared::db::kv::erase(&path_file_db_args, clone_output_executable.value())
        .map_err(|e| eprintln!("{}",e)).ok();
      return;
    } // if
    shared::db::kv::write(&path_file_db_args, &clone_output_executable.value(), &e.value())
      .map_err(|e| eprintln!("{}",e)).ok();
  });

} // fn: new_callback }}}

// fn: new {{{
pub fn new(tx : Sender<Msg>)
{
  let path_file_db_executable = match get_path_db_executable()
  {
    Ok(e) => e,
    Err(e) => { eprintln!("Could not retrieve path to db file: {}", e); std::path::PathBuf::default() }
  }; // match

  // Layout
  column!(col,
    col.set_margin(dimm::border_half());
    fixed!(col, frame_title, Frame::default(), dimm::height_text());
    col.fixed(&shared::fltk::separator::horizontal(col.w()), dimm::height_sep());
    scroll!(scroll,
      hpack!(col_scroll,);
      col_scroll.set_spacing(0);
    );
    col.fixed(&shared::fltk::separator::horizontal(col.w()), dimm::height_sep());
    column!(col_bottom,
      row!(row_bottom,
        fixed!(row_bottom, btn_back, &shared::fltk::button::rect::back(), dimm::width_button_rec());
        row_bottom.add(&Frame::default());
        fixed!(row_bottom, btn_home, &shared::fltk::button::rect::home(), dimm::width_button_rec());
        row_bottom.add(&Frame::default());
        row_bottom.fixed(&Frame::default(), dimm::width_button_rec());
      );
      col_bottom.fixed(&row_bottom, dimm::height_button_rec());
    );
    col.fixed(&col_bottom, dimm::height_button_rec());
  );
  // Title
  let mut frame_title = frame_title.clone();
  frame_title.set_label("Executable Configuration");
  // Auto resize column to scroll width
  scroll.resize_callback({let mut c = col_scroll.clone(); move |_,_,_,w,_|
  {
    c.resize(c.x(),c.y(),w-dimm::border_half()*3,c.h());
  }});
  scroll.set_type(group::ScrollType::VerticalAlways);
  // Create entries
  let f_make_entry =
  {
    let path_file_db_executable = path_file_db_executable.clone();
    let db_executables = shared::db::kv::read(&path_file_db_executable).unwrap_or_default();
    move |key : String|
    {
      let (output,btn,input) = new_layout_entry(key.as_str()
        , db_executables.contains_key(key.as_str())
      );
      new_callback(output, btn, input, key, path_file_db_executable);
    }
  };
  rescope!(col_scroll,
    let mut vec_executables = find_executables().unwrap_or_default();
    vec_executables.sort();
    vec_executables.iter().for_each(|e| f_make_entry.clone()(e.string()));
  );
  // Configure buttons
  let mut btn_home = btn_home.clone();
  btn_home.set_color(Color::Blue);
  btn_home.emit(tx, Msg::DrawCover);
  hover_blink!(btn_home);
  let mut btn_back = btn_back.clone();
  btn_back.emit(tx, Msg::DrawMenu);
  hover_blink!(btn_back);
} // fn: new }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
