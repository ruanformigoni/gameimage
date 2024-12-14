use fltk::
{
  app::Sender,
  enums::{
    FrameType,
    Color,
  },
  browser::MultiBrowser,
  dialog,
  output,
  prelude::*,
};

use shared::{hover_blink,column,row,add,fixed};
use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;
use shared::dimm;

use crate::common;
use shared::std::PathBufExt;
use crate::log_status;
use crate::frame;
use crate::gameimage;
use crate::wizard;

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>, title: &str)
{
  wizard::name::name(tx.clone()
    , title
    , common::Msg::DrawPlatform
    , common::Msg::DrawRpcs3Icon);
} // }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>, title: &str)
{
  frame::icon::project(tx.clone()
    , title
    , common::Msg::DrawRpcs3Name
    , common::Msg::DrawRpcs3Icon
    , common::Msg::DrawRpcs3Rom
  );
} // }}}

// pub fn rom() {{{
pub fn rom(tx: Sender<common::Msg>, title: &str)
{
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);
  // Layout
  row!(row,
    add!(row, frame_list, MultiBrowser::default());
    column!(col,
      fixed!(col, btn_add, shared::fltk::button::rect::add(), dimm::height_button_rec());
      fixed!(col, btn_del, shared::fltk::button::rect::del(), dimm::height_button_rec());
    );
    row.fixed(&col, dimm::width_button_rec());
  );
  // Configure buttons
  ui.btn_prev.clone().emit(tx.clone(), common::Msg::DrawRpcs3Icon);
  ui.btn_next.clone().emit(tx.clone(), common::Msg::DrawRpcs3Bios);
  // List of the currently installed items
  let mut frame_list = frame_list.clone();
  frame_list.set_frame(FrameType::BorderBox);
  frame_list.set_text_size(dimm::height_text());
  // Insert items in list of currently installed items
  let result_vec_items = gameimage::search::search_local("rom");
  if let Ok(vec_items) = result_vec_items
  {
    for item in vec_items { frame_list.add(&item.string()); } // for
  } // if
  // Add new item
  let mut btn_add = btn_add.clone()
    .with_color(Color::Green);
  hover_blink!(btn_add);
  let clone_tx = tx.clone();
  btn_add.set_callback(move |_|
  {
    // Pick files to install
    let mut chooser = dialog::FileChooser::new("."
      , "*"
      , dialog::FileChooserType::Directory
      , "Pick a directory with the .SFB file");
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
    // Deactivate window
    clone_tx.send_awake(common::Msg::WindDeactivate);
    // Fetch choice
    let str_choice = chooser.value(1).unwrap();
    // Install
    let clone_tx = clone_tx.clone();
    std::thread::spawn(move ||
    {
      // Install directory with backend
      match gameimage::install::install("rom", vec![str_choice])
      {
        Ok(_) => log_status!("Successfully installed rom"),
        Err(e) => log_status!("Failed to install rom: {}", e),
      } // match
      clone_tx.send_activate(common::Msg::DrawRpcs3Rom);
    });
  });
  // Erase package
  let mut btn_del = btn_del.clone()
    .with_color(Color::Red);
  hover_blink!(btn_del);
  btn_del.set_callback(move |_|
  {
    let vec_indices = frame_list.selected_items();
    // Check if any item was selected
    if vec_indices.len() == 0
    {
      log_status!("No item selected for deletion");
      return;
    } // if
    // Get items
    let vec_items : Vec<String> = vec_indices.into_iter().map(|e|{ frame_list.text(e).unwrap() }).collect();
    // Run backend
    match gameimage::install::remove("rom", vec_items)
    {
      Ok(_) => log_status!("Removed rom(s) successfully"),
      Err(e) => log_status!("Could not remove rom(s): '{}'", e),
    } // match
    clone_tx.send_activate(common::Msg::DrawRpcs3Rom);
  });
} // }}}

// pub fn bios() {{{
pub fn bios(tx: Sender<common::Msg>, title: &str)
{
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);
  // Layout
  column!(col,
    add!(col, frame_text, output::MultilineOutput::default());
    row!(row,
      row.add(&fltk::frame::Frame::default());
      fixed!(row, btn_launch, shared::fltk::button::wide::default(), dimm::width_button_wide());
      row.add(&fltk::frame::Frame::default());
    );
    col.fixed(&row, dimm::height_button_wide());
  );
  // Set button callbacks
  ui.btn_prev.clone().emit(tx.clone(), common::Msg::DrawRpcs3Rom);
  ui.btn_next.clone().emit(tx.clone(), common::Msg::DrawRpcs3Test);
  // Box with explanation text
  let mut frame_text = frame_text.clone();
  frame_text.set_color(Color::BackGround);
  frame_text.set_frame(FrameType::BorderBox);
  frame_text.set_text_size(dimm::height_text());
  let _ = frame_text.append("Here you can install the firmware and the .pkg, .rap and .edat files\n");
  let _ = frame_text.append("Clicking on 'Open' will open RPCS3\n");
  let _ = frame_text.append("Go to 'File -> Install Packages/Raps/Edats' for DLC\n");
  let _ = frame_text.append("Go to 'File -> Install Firmware' for the BIOS\n");
  // Button to launch rpcs3 and install files
  let mut btn_launch = btn_launch;
  btn_launch.set_color(Color::Green);
  btn_launch.set_label("Open");
  btn_launch.set_callback(move |_|
  {
    tx.send_awake(common::Msg::WindDeactivate);
    std::thread::spawn(move ||
    {
      match gameimage::install::gui()
      {
        Ok(_) => log_status!("Gui exited successfully"),
        Err(e) => log_status!("Install gui exited with error: {}", e),
      }; // match
      tx.send_awake(common::Msg::WindActivate);
    });
  });
  hover_blink!(btn_launch);
} // }}}

// pub fn test() {{{
pub fn test(tx: Sender<common::Msg>, title: &str)
{
  wizard::test::test(tx.clone()
    , title
    , common::Msg::DrawRpcs3Bios
    , common::Msg::DrawRpcs3Test
    , common::Msg::DrawRpcs3Compress);
} // }}}

// pub fn compress() {{{
pub fn compress(tx: Sender<common::Msg>, title: &str)
{
  wizard::compress::compress(tx.clone()
    , title
    , common::Msg::DrawRpcs3Test
    , common::Msg::DrawRpcs3Compress
    , common::Msg::DrawCreator);
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
