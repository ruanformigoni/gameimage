#![allow(warnings)]

use std::env;
use std::path::PathBuf;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  text::{TextBuffer,TextDisplay},
  menu::MenuButton,
  group::Group,
  image::SharedImage,
  input::FileInput,
  group::PackType,
  frame::Frame,
  dialog::dir_chooser,
  enums::{Align,FrameType,Color},
};

use crate::dimm;
use crate::frame;
use crate::common;
use crate::log;

// pub fn platform() {{{
pub fn platform(tx: Sender<common::Msg>, title: &str)
{
  let mut frame = Frame::default()
    .with_size(dimm::width(), dimm::height());
  frame.set_frame(FrameType::BorderBox);
  frame.set_type(PackType::Vertical);

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_header = ret_frame_header.frame.clone();
  let frame_content = ret_frame_header.frame_content.clone();
  let frame_footer = ret_frame_footer.frame.clone();

  if let Err(e) = common::common()
  {
    log!("Err: {}", e.to_string());
  } // if

  // Menu options to select platform
  let mut btn_menu = MenuButton::default()
    .with_size(dimm::width() - dimm::border()*2, dimm::height_button_wide())
    .above_of(&frame_footer, dimm::border());
  btn_menu.set_pos(btn_menu.x() + dimm::border(), btn_menu.y());

  // Create entries
  let mut clone_btn = btn_menu.clone();
  let mut f_add_entry = |platform : &str|
  {
    clone_btn.add_choice(platform);
  };
  f_add_entry("wine");
  f_add_entry("retroarch");
  f_add_entry("pcsx2");
  f_add_entry("rpcs3");
  f_add_entry("yuzu");

  // Create callback with descriptions
  let buffer = TextBuffer::default();
  let mut frame_text = TextDisplay::default()
    .with_size(dimm::width() - dimm::border()*2, frame_content.h() - btn_menu.h() - dimm::border()*2 - dimm::height_text())
    .with_align(Align::Top)
    .with_pos(frame_content.x() + dimm::border(), frame_content.y() + dimm::border());
  frame_text.set_buffer(buffer.clone());
  frame_text.set_frame(FrameType::BorderBox);
  frame_text.set_color(frame_content.color());
  frame_text.wrap_mode(fltk::text::WrapMode::AtBounds, 0);

  // Update buffer function
  let mut clone_buffer = buffer.clone();
  let mut f_update_buffer = move |str_platform : String|
  {
    clone_buffer.remove(0, clone_buffer.length());
    match str_platform.as_str()
    {
      "wine" => clone_buffer.insert(0, common::STR_DESC_WINE),
      "retroarch" => clone_buffer.insert(0, common::STR_DESC_RETR),
      "pcsx2" => clone_buffer.insert(0, common::STR_DESC_PCSX2),
      "rpcs3" => clone_buffer.insert(0, common::STR_DESC_RPCS3),
      "yuzu" => clone_buffer.insert(0, common::STR_DESC_YUZU),
      _ => ()
    }
  };

  // Check if variable is already set
  if let Some(env_platform) = env::var("GIMG_PLATFORM").ok()
  {
    match env_platform.as_str()
    {
      "wine" | "retroarch" | "pcsx2" | "rpcs3" | "yuzu" =>
      {
        btn_menu.set_label(env_platform.as_str());
        f_update_buffer(env_platform);
      },
      _ => (),
    } // match
  } // if

  // Set callback to dropdown menu selection
  let mut clone_update_buffer = f_update_buffer.clone();
  btn_menu.set_callback(move |e|
  {
    // Fetch choice
    let choice = e.choice().unwrap_or(String::from("None"));
    // Set as label
    e.set_label(choice.as_str());
    // Set as env var for later use
    env::set_var("GIMG_PLATFORM", choice.as_str());
    // Draw description of selection
    clone_update_buffer(choice);
  });

  // Set callback for prev
  let mut clone_btn_prev = ret_frame_footer.btn_prev.clone();
  let mut clone_tx = tx.clone();
  clone_btn_prev.set_callback(move |_|
  {
    tx.send(common::Msg::DrawWelcome);
  });


  // Set callback for next
  let mut clone_btn_next = ret_frame_footer.btn_next.clone();
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  let mut clone_tx = tx.clone();
  clone_btn_next.set_callback(move |_|
  {
    let env_platform = env::var("GIMG_PLATFORM");

    let mut str_platform = String::new();

    // Allow next if dropdown has valid value
    if let Some(platform) = env_platform.ok()
    {
      if platform != btn_menu.label()
      {
        clone_output_status.set_value("Please select a platform to proceed");
        return;
      } // if
      str_platform = platform;
    } // if
    else
    {
      clone_output_status.set_value("Please select a platform to proceed");
      return;
    } // else

    clone_output_status.set_value("Fetching list of files to download");

    // Disable window
    clone_tx.send(common::Msg::WindDeactivate);

    // Ask back-end for the files to download for the selected platform
    let rx_gameimage = if let Ok(rx_gameimage) = common::gameimage_cmd(vec![
          "fetch"
        , format!("--output-file={}.flatimage", str_platform).as_str()
        , format!("--platform={}", str_platform).as_str()
        , "--json=gameimage.fetch.json"
    ])
    {
      rx_gameimage
    }
    else
    {
      log!("Could not recover return code");
      clone_tx.send(common::Msg::WindActivate);
      return;
    }; // else

    std::thread::spawn(move ||
    {
      if let Ok(code) = rx_gameimage.recv() && code != 0
      {
        log!("Failed with code {}", code);
        return;
      } // if

      clone_tx.send(common::Msg::DrawFetch);
    });
  });
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
