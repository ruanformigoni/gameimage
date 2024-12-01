// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  browser::MultiBrowser,
  button::Button,
  dialog,
  enums::{FrameType,Color},
};

use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;

use crate::dimm;
use crate::common;
use shared::std::PathBufExt;
use crate::log;
use crate::gameimage;

// struct Install {{{
pub struct Install
{
  pub frame_list : MultiBrowser,
  pub btn_add : Button,
  pub btn_del : Button,
} // struct Install }}}

// pub fn install() {{{
pub fn install(tx: Sender<common::Msg>
  , title: &str
  , label: &str
  , msg_prev: common::Msg
  , msg_curr: common::Msg
  , msg_next: common::Msg) -> Install
{
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);

  // Set previous frame
  ui.btn_prev.clone().emit(tx.clone(), msg_prev);
  ui.btn_next.clone().emit(tx.clone(), msg_next);

  // List of the currently installed items
  let mut frame_list = MultiBrowser::default()
    .with_size(ui.group.width() - dimm::border() - dimm::width_button_rec(), ui.group.height())
    .with_pos_of(&ui.group);
  frame_list.set_frame(FrameType::BorderBox);
  frame_list.set_text_size(dimm::height_text());

  // Insert items in list of currently installed items
  match gameimage::search::search_local(label)
  {
    Ok(vec_items) => for item in vec_items { frame_list.add(&item.string()); },
    Err(e) => log!("Could not get items to insert: {}", e),
  }; // match

  // Add new item
  let clone_tx = tx.clone();
  let clone_label : String = label.to_string();
  let btn_add = shared::fltk::button::rect::add()
    .right_of(&frame_list, dimm::border())
    .with_color(Color::Green)
    .with_callback(move |_|
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

      // Install files
      clone_tx.send_awake(common::Msg::WindDeactivate);

      let count = chooser.count()+1;
      let clone_label = clone_label.clone();
      let mut vec_entries : Vec<String> = vec![];
      (1..count).into_iter().for_each(|idx| { vec_entries.push(chooser.value(idx).unwrap()); });
      std::thread::spawn(move ||
      {
        match gameimage::install::install(&clone_label, vec_entries.clone())
        {
          Ok(_) => log!("Files installed"),
          Err(e) => log!("Failed to install files: {}", e),
        }; // match
        clone_tx.send_activate(msg_curr);
      });
    });

  // Erase package
  let mut btn_del = shared::fltk::button::rect::del()
    .below_of(&btn_add, dimm::border())
    .with_color(Color::Red);
  let mut clone_output_status = ui.status.clone();
  let clone_label = label.to_string();
  let clone_frame_list = frame_list.clone();
  let clone_tx = tx.clone();
  btn_del.set_callback(move |_|
  {
    clone_tx.send_awake(common::Msg::WindDeactivate);

    let vec_indices = clone_frame_list.selected_items();

    if vec_indices.len() == 0
    {
      clone_output_status.set_value("No item selected for deletion");
      clone_tx.send_awake(common::Msg::WindActivate);
      return;
    } // if

    // Remove
    let clone_tx = clone_tx.clone();
    let clone_label = clone_label.clone();
    let clone_frame_list = clone_frame_list.clone();
    std::thread::spawn(move ||
    {
      // Get items
      let vec_items : Vec<String> = vec_indices.into_iter().map(|e|{ clone_frame_list.text(e).unwrap() }).collect();
      // Run backend
      match gameimage::install::remove(&clone_label, vec_items.clone())
      {
        Ok(_) => log!("Successfully removed files"),
        Err(e) => log!("Failed to remove files: {}", e),
      }; // match
      // Redraw GUI
      clone_tx.send_activate(msg_curr);
    }); // std::thread
  }); // set_callback

  Install{ frame_list, btn_add, btn_del }
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
