// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  browser::MultiBrowser,
  button::Button,
  dialog,
  enums::{FrameType,Color},
};

use crate::dimm;
use crate::frame;
use crate::common;
use crate::common::PathBufExt;
use crate::common::WidgetExtExtra;
use crate::common::FltkSenderExt;
use crate::common::VecExt;
use crate::log;
use crate::gameimage;
use crate::lib::svg;

// struct Install {{{
pub struct Install
{
  pub ret_frame_header : frame::common::RetFrameHeader,
  pub ret_frame_footer : frame::common::RetFrameFooter,
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
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

  // Set previous frame
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), msg_prev);
  ret_frame_footer.btn_next.clone().emit(tx.clone(), msg_next);

  // List of the currently installed items
  let mut frame_list = MultiBrowser::default()
    .with_size(frame_content.width() - dimm::border()*3 - dimm::width_button_rec()
      , frame_content.height() - dimm::border()*2)
    .with_pos(frame_content.x() + dimm::border(), frame_content.y() + dimm::border());
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
  let btn_add = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .right_of(&frame_list, dimm::border())
    .with_frame(FrameType::RoundedFrame)
    .with_focus(false)
    .with_svg(svg::icon_add(1.0).as_str())
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
        for str_choice in vec_entries
        {
          // Install with backend
          if common::gameimage_sync(vec!["install", &clone_label, &str_choice]) != 0
          {
            log!("Failed to execute backend");
          } // if
        } // for
        clone_tx.send_awake(msg_curr);
      });
    });

  // Erase package
  let mut btn_del = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .below_of(&btn_add, dimm::border());
  btn_del.set_frame(FrameType::RoundedFrame);
  btn_del.visible_focus(false);
  btn_del.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_del(1.0).as_str()).unwrap()));
  btn_del.set_color(Color::Red);
  let mut clone_output_status = ret_frame_footer.output_status.clone();
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
      // Prepend command
      if common::gameimage_sync(vec!["install", "--remove", &clone_label].append_strings(vec_items).as_str_slice()) != 0
      {
        log!("Failed to delete items");
      } // if
      // Redraw GUI
      clone_tx.send(msg_curr);
    }); // std::thread
  }); // set_callback

  Install{ ret_frame_header, ret_frame_footer, frame_list, btn_add, btn_del }
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
