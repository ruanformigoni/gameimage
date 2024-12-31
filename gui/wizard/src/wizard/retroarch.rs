// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  browser::MultiBrowser,
  dialog,
  output,
  enums::{Color,Align},
};

use clown::clown;

use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;
use shared::{hover_blink,column,row,add,fixed};

use crate::dimm;
use crate::wizard;
use crate::frame;
use crate::common;
use shared::std::PathBufExt;
use crate::log;
use crate::log_status;
use crate::db;
use crate::gameimage;

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>, title: &str)
{
  wizard::name::name(tx.clone()
    , title
    , common::Msg::DrawPlatform
    , common::Msg::DrawRetroarchIcon);
} // }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>, title: &str)
{
  frame::icon::project(tx.clone()
    , title
    , common::Msg::DrawRetroarchName
    , common::Msg::DrawRetroarchIcon
    , common::Msg::DrawRetroarchRom
  );
} // }}}

// fn rom_callback_add() {{{
fn rom_callback_add(tx: Sender<common::Msg>, label: String)
{
  // Pick files to install
  let mut chooser = dialog::FileChooser::new("."
    , "*"
    , dialog::FileChooserType::Multi
    , "Pick one or multiple files");
  // Start dialog
  chooser.show();
  // Wait for choice(s)
  while chooser.shown() { fltk::app::wait(); } // while
  // Check if choice is valid
  if chooser.value(1).is_none()
  {
    log_status!("No file selected");
    return;
  } // if
  // Install files
  tx.send_awake(common::Msg::WindDeactivate);
  let mut vec_entries : Vec<String> = vec![];
  (1..chooser.count()+1).into_iter().for_each(|idx| { vec_entries.push(chooser.value(idx).unwrap()); });
  let clone_tx = tx.clone();
  std::thread::spawn(move ||
  {
    match gameimage::install::install(&label, vec_entries.clone())
    {
      Ok(_) => log_status!("Installed selected files"),
      Err(e) => log_status!("Failed to install files: {}", e),
    }; // match
    clone_tx.send_activate(common::Msg::DrawRetroarchRom);
  });
} // fn rom_callback_add() }}}

// fn rom_callback_del() {{{
fn rom_callback_del(tx: Sender<common::Msg>, label: String, list: MultiBrowser)
{
  tx.send_awake(common::Msg::WindDeactivate);
  // Get selected items
  let vec_indices = list.selected_items();
  // Check number of selected items
  if vec_indices.len() == 0
  {
    log_status!("No item selected for deletion");
    tx.send_awake(common::Msg::WindActivate);
    return;
  } // if
  // Remove selected items
  let clone_tx = tx.clone();
  let clone_frame_list = list.clone();
  std::thread::spawn(move ||
  {
    // Get items
    let vec_items : Vec<String> = vec_indices.into_iter().map(|e|{ clone_frame_list.text(e).unwrap() }).collect();
    // Run backend
    match gameimage::install::remove(&label, vec_items.clone())
    {
      Ok(_) => log_status!("Successfully removed files"),
      Err(e) => log_status!("Failed to remove files: {}", e),
    }; // match
    // Redraw GUI
    clone_tx.send_activate(common::Msg::DrawRetroarchRom);
  }); // std::thread
} // fn rom_callback_del() }}}

// fn rom_callback_default() {{{
fn rom_callback_default(tx: Sender<common::Msg>, label: String, list: MultiBrowser)
{
  let vec_indices = list.selected_items();
  // Check number of selected items
  if vec_indices.len() == 0
  {
    log_status!("No item selected to set as default");
    return;
  } // if
  // Only one item should be the default rom
  if vec_indices.len() != 1
  {
    log_status!("Only one item can be set as the default");
    return;
  } // if
  // Get selected item
  let selected = match list.text(*vec_indices.first().unwrap())
  {
    Some(item) => std::path::PathBuf::from(item),
    None => return,
  }; // match
  // Select rom
  tx.send_awake(common::Msg::WindDeactivate);
  std::thread::spawn(#[clown] move ||
  {
    match gameimage::select::select(&label, &selected)
    {
      Ok(_) => log_status!("Changed default rom to '{}'", selected.string()),
      Err(e) => log_status!("Could not select rom file '{}': '{}'", selected.string(), e),
    } // match
    tx.send_activate(common::Msg::DrawRetroarchRom);
  }); // std::thread
} // fn rom_callback_default() }}}

// pub fn rom() {{{
pub fn rom(tx: Sender<common::Msg>, title: &str)
{
  static LABEL: &str = "rom";

  let ui = crate::GUI.lock().unwrap().ui.clone()(title);

  // Set previous frame
  ui.btn_prev.clone().emit(tx.clone(), common::Msg::DrawRetroarchIcon);
  ui.btn_next.clone().emit(tx.clone(), common::Msg::DrawRetroarchCore);
  // Layout
  column!(col,
    row!(row,
      add!(row, list, MultiBrowser::default());
      column!(col_buttons,
        fixed!(col_buttons, btn_add, shared::fltk::button::rect::add(), dimm::height_button_rec());
        fixed!(col_buttons, btn_default, shared::fltk::button::rect::check(), dimm::height_button_rec());
        fixed!(col_buttons, btn_del, shared::fltk::button::rect::del(), dimm::height_button_rec());
        col_buttons.add(&fltk::frame::Frame::default());
      );
      row.fixed(&col_buttons, dimm::width_button_rec());
    );
    col.fixed(&fltk::frame::Frame::default()
        .with_align(Align::Inside | Align::Left)
        .with_label("Default rom:")
      , dimm::height_text()
    );
    fixed!(col, output_default, output::Output::default(), dimm::height_button_wide());
  );
  // Buttons
  hover_blink!(btn_add);
  hover_blink!(btn_del);
  hover_blink!(btn_default);
  // List of the currently installed items
  let mut list = list.clone();
  // Insert items in list of currently installed items
  match gameimage::search::search_local(LABEL)
  {
    Ok(vec_items) => for item in vec_items { list.add(&item.string()); },
    Err(e) => log_status!("Could not get items to insert: {}", e),
  }; // match
  // Add new item
  btn_add.clone()
    .with_color(Color::Green)
    .with_callback(#[clown] move |_| { rom_callback_add(honk!(tx), LABEL.into()); });
  // Erase package
  btn_del.clone()
    .with_color(Color::Red)
    .with_callback(#[clown] move |_| { rom_callback_del(honk!(tx), LABEL.into(), honk!(list).clone()); });
  // Show default item below all items
  let mut output_default = output_default.clone();
  output_default.deactivate();
  if let Ok(project) = db::project::current()
  && let Ok(path_file_rom) = project.get_path_relative(db::project::EntryName::PathFileRom)
  {
    let _ = output_default.insert(&path_file_rom.file_name_string());
  } // if
  // Update default rom
  btn_default.clone()
    .with_color(Color::Blue)
    .with_callback(#[clown] move |_| { rom_callback_default(honk!(tx), LABEL.into(), honk!(list).clone()) });
} // }}}

// fn core_callback_add() {{{
fn core_callback_add(tx: Sender<common::Msg>, label: String)
{
  // Pick files to install
  let mut chooser = dialog::FileChooser::new("."
    , "*"
    , dialog::FileChooserType::Multi
    , "Pick one or multiple files");
  // Start dialog
  chooser.show();
  // Wait for choice(s)
  while chooser.shown() { fltk::app::wait(); } // while
  // Check if choice is valid
  if chooser.value(1).is_none()
  {
    log!("No file selected");
    return;
  } // if
  // Get items
  tx.send_awake(common::Msg::WindDeactivate);
  let vec_items = (1..chooser.count()+1).into_iter().map(|e| chooser.value(e).unwrap()).collect::<Vec<String>>();
  // Install cores
  std::thread::spawn(move ||
  {
    match gameimage::install::install(&label, vec_items)
    {
      Ok(_) => log!("Successfully installed cores"),
      Err(e) => log!("Failed to install one or more cores: {}", e),
    }; // match
    // Redraw window
    tx.send_activate(common::Msg::DrawRetroarchCore);
  });
} // fn core_callback_add() }}}

// fn core_callback_default() {{{
fn core_callback_default(tx: Sender<common::Msg>, label: String, list_installed: MultiBrowser)
{
  let vec_indices = list_installed.selected_items();
  // Check for selected item
  if vec_indices.len() == 0
  {
    log_status!("No item selected to set as default");
    return;
  } // if
  // Only one core can be the default
  if vec_indices.len() != 1
  {
    log_status!("Only one item can be set as the default");
    return;
  } // if
  // Get selected item
  let selected = match list_installed.text(*vec_indices.first().unwrap())
  {
    Some(item) => std::path::PathBuf::from(item),
    None => return,
  }; // match
  // Set as default
  tx.send_awake(common::Msg::WindDeactivate);
  std::thread::spawn(move ||
  {
    match gameimage::select::select(&label, &selected)
    {
      Ok(_) => log!("Selected core successfully"),
      Err(e) => log!("Could not select core file '{}': '{}'", selected.string(), e),
    } // match

    tx.send_activate(common::Msg::DrawRetroarchCore);
  }); // std::thread
} // fn core_callback_default() }}}

// fn core_callback_del() {{{
fn core_callback_del(tx: Sender<common::Msg>, label: String, list_installed: MultiBrowser)
{
  let vec_indices = list_installed.selected_items();
  // Get number of items to delete
  if vec_indices.len() == 0 { log_status!("No item selected for deletion"); return; }
  // Get as items
  let vec_items : Vec<String> = vec_indices.into_iter().map(|e|{ list_installed.text(e).unwrap() }).collect();
  // Run backend
  match gameimage::install::remove(&label, vec_items)
  {
    Ok(_) => log!("Removed core successfully"),
    Err(e) => log!("Could not remove core file(s) '{}'", e),
  } // match
  // Redraw
  tx.send_awake(common::Msg::DrawRetroarchCore);
} // fn core_callback_del() }}}

// fn core_callback_remote() {{{
fn core_callback_remote(tx: Sender<common::Msg>, label: String, list_remote: MultiBrowser)
{
  // Install files
  tx.send_awake(common::Msg::WindDeactivate);
  let clone_frame_list_remote = list_remote.clone();
  std::thread::spawn(move ||
  {
    // Get indices
    let vec_indices = clone_frame_list_remote.selected_items();
    // Get text
    let vec_items : Vec<String> = vec_indices.into_iter().map(|e|{ clone_frame_list_remote.text(e).unwrap() }).collect();
    // Install with backend
    match gameimage::install::remote(&label, vec_items)
    {
      Ok(_) => log!("Remote cores installed successfully"),
      Err(e) => log!("Failed to install remote cores: {}", e),
    } // match
    tx.send_activate(common::Msg::DrawRetroarchCore);
  });

} // fn core_callback_remote() }}}

// pub fn core() {{{
pub fn core(tx: Sender<common::Msg>, title: &str)
{
  static LABEL: &str = "core";
  // Refresh GUI
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);
  // Set previous frame
  ui.btn_prev.clone().emit(tx.clone(), common::Msg::DrawRetroarchRom);
  ui.btn_next.clone().emit(tx.clone(), common::Msg::DrawRetroarchBios);
  // Layout
  column!(col,
    row!(row,
      add!(row, list_installed, MultiBrowser::default());
      column!(col_buttons,
        fixed!(col_buttons, btn_add, shared::fltk::button::rect::add(), dimm::height_button_rec());
        fixed!(col_buttons, btn_default, shared::fltk::button::rect::check(), dimm::height_button_rec());
        fixed!(col_buttons, btn_del, shared::fltk::button::rect::del(), dimm::height_button_rec());
        col_buttons.add(&fltk::frame::Frame::default());
      );
      row.fixed(&col_buttons, dimm::width_button_rec());
    );
    col.fixed(&fltk::frame::Frame::default()
        .with_align(Align::Inside | Align::Left)
        .with_label("Default core:")
      , dimm::height_text()
    );
    row!(row_bottom,
      column!(col_bottom,
        fixed!(col_bottom, output_default, output::Output::default(), dimm::height_button_wide());
        add!(col_bottom, list_remote, MultiBrowser::default());
      );
      column!(col_bottom_buttons,
        fixed!(col_bottom_buttons, btn_cloud, shared::fltk::button::rect::cloud(), dimm::height_button_rec());
        col_bottom_buttons.add(&fltk::frame::Frame::default());
      );
      row_bottom.fixed(&col_bottom_buttons, dimm::width_button_rec());
    );
  );
  // Buttons
  hover_blink!(btn_add);
  hover_blink!(btn_del);
  hover_blink!(btn_cloud);
  hover_blink!(btn_default);
  // Insert items in list of currently installed items
  match gameimage::search::search_local(LABEL)
  {
    Ok(vec_items) => for item in vec_items { list_installed.clone().add(&item.string()); },
    Err(e) => log_status!("Could not get items to insert: {}", e),
  }; // match
  // Show default item below all items
  let mut output_default = output_default.clone();
  output_default.deactivate();
  if let Ok(project) = db::project::current()
  && let Ok(path_file_core) = project.get_path_relative(db::project::EntryName::PathFileCore)
  {
    let _ = output_default.insert(&path_file_core.file_name_string());
  } // if
  // Add new item from file manager
  btn_add.clone()
    .with_color(Color::Green)
    .set_callback(move |_| { core_callback_add(tx, LABEL.into()); });
  // Update default core
  btn_default.clone()
    .with_color(Color::Blue)
    .with_callback(#[clown] move |_| { core_callback_default(tx, LABEL.into(), honk!(list_installed).clone()); });
  // Add new item from remote
  btn_cloud.clone()
    .with_color(Color::Green)
    .with_callback(#[clown] move |_| { core_callback_remote(tx, LABEL.into(), honk!(list_remote).clone()) });
  // Erase package
  btn_del.clone()
    .with_color(Color::Red)
    .with_callback(#[clown] move |_| { core_callback_del(tx, LABEL.into(), honk!(list_installed).clone()); });
  // List of items to install
  let mut list_remote = list_remote.clone();
  list_remote.set_text_size(dimm::height_text());
  // Insert remote items in list
  if let Ok(vec_items) = gameimage::search::search_remote("core")
  {
    vec_items.iter().for_each(|item| list_remote.add(&item.string()) );
  } // if
} // }}}

// pub fn bios() {{{
pub fn bios(tx: Sender<common::Msg>, title: &str)
{
  wizard::install::install(tx.clone()
    , title
    , "bios"
    , common::Msg::DrawRetroarchCore
    , common::Msg::DrawRetroarchBios
    , common::Msg::DrawRetroarchTest);
} // }}}

// pub fn test() {{{
pub fn test(tx: Sender<common::Msg>, title: &str)
{
  wizard::test::test(tx.clone()
    , title
    , common::Msg::DrawRetroarchBios
    , common::Msg::DrawRetroarchTest
    , common::Msg::DrawRetroarchCompress);
} // }}}

// pub fn compress() {{{
pub fn compress(tx: Sender<common::Msg>, title: &str)
{
  wizard::compress::compress(tx.clone()
    , title
    , common::Msg::DrawRetroarchTest
    , common::Msg::DrawRetroarchCompress
    , common::Msg::DrawCreator);
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
