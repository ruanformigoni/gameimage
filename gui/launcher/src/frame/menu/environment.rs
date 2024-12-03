use fltk::prelude::*;
use fltk::{
  output::Output,
  app::Sender,
  enums::Color,
  frame::Frame,
  group,
};

use shared::dimm;
use shared::{add,fixed,row,column,hpack,scroll,hover_blink,hseparator,hseparator_fixed,rescope};

use crate::common::Msg;

// get_path_db_env() {{{
fn get_path_db_env() -> anyhow::Result<std::path::PathBuf>
{
  let mut path_db : std::path::PathBuf = std::env::var("GIMG_LAUNCHER_ROOT")?.into();
  path_db.push("gameimage.env.json");

  Ok(path_db)
} // get_path_db_env() }}}

// fn: new_entry {{{
fn new_entry(tx: Sender<crate::common::Msg>, mut col: group::Pack, key: &str, val: &str)
{
  add!(col, output_key, Output::default().with_size(0, dimm::height_button_wide()));
  output_key.clone().insert(key).map_err(|e| eprintln!("{}", e)).ok();
  row!(row,
    add!(row, output_val, Output::default());
    output_val.clone().insert(val).map_err(|e| eprintln!("{}", e)).ok();
    fixed!(row, btn_del, shared::fltk::button::rect::del(), dimm::width_button_rec());
    row.resize(row.x(), row.y(), row.w(), dimm::height_button_wide());
  );
  col.add(&row);
  hseparator!(col, row.w());
  hover_blink!(btn_del);
  let clone_key: String = key.into();
  let mut btn_del = btn_del.clone();
  btn_del.set_color(Color::Red);
  btn_del.set_callback(move |_|
  {
    let path_file_db = match get_path_db_env()
    {
      Ok(e) => e,
      Err(e) => { eprintln!("Could not retrieve path to db file: {}", e); std::path::PathBuf::default() }
    }; // match
    match shared::db::kv::erase(&path_file_db, clone_key.clone())
    {
      Ok(_) => println!("Erased key '{}'", clone_key),
      Err(e) => println!("Failed to erase key '{}' with error '{}'", clone_key, e.to_string()),
    } // if
    tx.send(Msg::DrawEnv);
  });
} // new_entry }}}

// fn: new_dialog {{{
fn new_dialog(tx: Sender<crate::common::Msg>)
{
  let path_file_db = match get_path_db_env()
  {
    Ok(e) => e,
    Err(e) => { eprintln!("Could not retrieve path to db file: {}", e); std::path::PathBuf::default() }
  }; // match
  let dialog = shared::fltk::dialog::key_value();
  let clone_dialog = dialog.clone();
  let clone_tx = tx.clone();
  let clone_path_file_db = path_file_db.clone();
  dialog.btn_ok.clone().set_callback(move |_|
  {
    clone_dialog.wind.clone().hide();
    let key = clone_dialog.input_key.value();
    let value = clone_dialog.input_value.value();
    if key.is_empty() { return; }
    match shared::db::kv::write(&clone_path_file_db, &key.clone(), &value.clone())
    {
      Ok(_) => println!("Set key '{}' with value '{}'", key.clone(), value.clone()),
      Err(e) => println!("Failed to set key '{}' with error '{}'", key, e.to_string()),
    } // if
    clone_tx.send(Msg::DrawEnv);
  });
  dialog.wind.clone().show();
} // new_dialog }}}

// fn: new {{{
pub fn new(tx : Sender<Msg>)
{
  // Layout
  column!(col,
    col.set_margin(dimm::border_half());
    fixed!(col, frame_title, Frame::default(), dimm::height_text());
    hseparator_fixed!(col, col.w() - dimm::border()*2, dimm::border_half());
    // Content
    scroll!(scroll,
      hpack!(col_scroll,);
      col_scroll.set_spacing(dimm::border());
      col_scroll.set_size(0,0);
    );
    hseparator_fixed!(col, col.w() - dimm::border()*2, dimm::border_half());
    column!(col_bottom,
      row!(row_bottom,
        fixed!(row_bottom, btn_back, &shared::fltk::button::rect::back(), dimm::width_button_rec());
        row_bottom.add(&Frame::default());
        fixed!(row_bottom, btn_home, &shared::fltk::button::rect::home(), dimm::width_button_rec());
        row_bottom.add(&Frame::default());
        fixed!(row_bottom, btn_add, &shared::fltk::button::rect::add(), dimm::width_button_rec());
      );
      col_bottom.fixed(&row_bottom, dimm::height_button_rec());
    );
    col.fixed(&col_bottom, dimm::height_button_rec());
  );

  // Title
  let mut frame_title = frame_title.clone();
  frame_title.set_label("Environment Variables");

  // Scroll resize callback
  scroll.resize_callback({let mut c = col_scroll.clone(); move |_,_,_,w,_|
  {
    c.resize(c.x(),c.y(),w-dimm::border_half()*3,c.h());
  }});
  scroll.set_type(group::ScrollType::VerticalAlways);

  // Configure buttons
  let mut btn_back = btn_back.clone();
  btn_back.emit(tx, Msg::DrawMenu);
  hover_blink!(btn_back);
  let mut btn_home = btn_home.clone();
  btn_home.set_color(Color::Blue);
  btn_home.emit(tx, Msg::DrawCover);
  hover_blink!(btn_home);
  let mut btn_add = btn_add.clone();
  btn_add.set_color(Color::Green);
  btn_add.set_callback(move |_| new_dialog(tx));
  hover_blink!(btn_add);

  let path_file_db = match get_path_db_env()
  {
    Ok(e) => e,
    Err(e) => { eprintln!("Could not retrieve path to db file: {}", e); std::path::PathBuf::default() }
  }; // match

  rescope!(col_scroll,
    if let Some(entries) = shared::db::kv::read(&path_file_db).map_err(|e| eprintln!("{}", e)).ok()
    {
      entries.iter().for_each(|(k,v)| { new_entry(tx, col_scroll.clone(), &k, &v); });
    } // if
  );
} // fn: new }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
