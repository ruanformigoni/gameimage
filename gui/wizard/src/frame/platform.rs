use std::env;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  text::{TextBuffer,TextDisplay},
  menu::MenuButton,
  enums::{Align,FrameType},
};

use crate::dimm;
use crate::frame;
use crate::common;
use crate::common::WidgetExtExtra;
use crate::common::FltkSenderExt;
use crate::log;

// pub fn platform() {{{
pub fn platform(tx: Sender<common::Msg>, title: &str)
{
  // Enter the build directory
  if let Err(e) = common::dir_build()
  {
    log!("Err: {}", e.to_string());
  } // if

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

  // Menu options to select platform
  let mut btn_menu = MenuButton::default()
    .with_size(dimm::width() - dimm::border()*2, dimm::height_button_wide())
    .bottom_of(&frame_content, - dimm::border())
    .center_x(&frame_content)
    .with_focus(false);
  btn_menu.add_choice("linux|wine|retroarch|pcsx2|rpcs3|ryujinx");

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
      "linux" => clone_buffer.insert(0, common::STR_DESC_LINUX),
      "wine" => clone_buffer.insert(0, common::STR_DESC_WINE),
      "retroarch" => clone_buffer.insert(0, common::STR_DESC_RETR),
      "pcsx2" => clone_buffer.insert(0, common::STR_DESC_PCSX2),
      "rpcs3" => clone_buffer.insert(0, common::STR_DESC_RPCS3),
      // "ryujinx" => clone_buffer.insert(0, common::STR_DESC_RYUJINX),
      _ => ()
    }
  };

  // Check if variable is already set
  if let Some(env_platform) = env::var("GIMG_PLATFORM").ok()
  {
    if env_platform.as_str() == "wine"
    {
      btn_menu.set_width(frame_text.w() - dimm::border() - dimm::width_button_wide()*2);
      let mut btn_wine_dist = MenuButton::default()
        .with_size(dimm::width_button_wide()*2, dimm::height_button_wide())
        .with_focus(false)
        .right_of(&btn_menu, dimm::border())
        .with_callback(|e|
        {
          let choice = e.choice().unwrap_or(String::from("None"));
          std::env::set_var("GIMG_WINE_DIST", choice.as_str());
          e.set_label(&choice);
        });
      // Set dist options
      btn_wine_dist.add_choice("caffe|default|osu-tkg|soda|staging|tkg|vaniglia");
      // Init default value for btn_wine_dist
      if let Ok(var) = std::env::var("GIMG_WINE_DIST")
      {
        btn_wine_dist.set_label(&var);
      } // if
      else
      {
        std::env::set_var("GIMG_WINE_DIST", "default");
        btn_wine_dist.set_label("default");
      } // else
    }
    // Update menu
    btn_menu.set_label(env_platform.as_str());
    // Update description box
    f_update_buffer(env_platform);
  } // if

  // Set callback to dropdown menu selection
  let mut clone_update_buffer = f_update_buffer.clone();
  let clone_tx = tx.clone();
  btn_menu.set_callback(move |e|
  {
    // Fetch choice
    let choice = e.choice().unwrap_or(String::from("None"));
    // Set as label
    e.set_label(&choice);
    // Set as env var for later use
    env::set_var("GIMG_PLATFORM", &choice);
    // Draw description of selection
    clone_update_buffer(choice.clone());
    // Redraw
    clone_tx.send_awake(common::Msg::DrawPlatform);
  });

  // Set callback for prev
  ret_frame_footer.btn_prev.clone().emit(tx, common::Msg::DrawWelcome);

  // Set callback for next
  let mut clone_btn_next = ret_frame_footer.btn_next.clone();
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  let clone_tx = tx.clone();
  clone_btn_next.set_callback(move |_|
  {
    let str_platform = if let Ok(str_platform) = env::var("GIMG_PLATFORM")
      && btn_menu.label() == str_platform
    {
      str_platform
    } // if
    else
    {
      clone_output_status.set_value("Please select a platform to proceed");
      return;
    }; // else

    // Fetch files
    clone_output_status.set_value("Fetching list of files to download");

    // Disable window
    clone_tx.send_awake(common::Msg::WindDeactivate);

    // Fetch files
    std::thread::spawn(move ||
    {
      // Ask back-end for the files to download for the selected platform
      if common::gameimage_sync(vec![
            "fetch"
          , format!("--output-file={}.flatimage", str_platform).as_str()
          , format!("--platform={}", str_platform).as_str()
          , "--json=gameimage.fetch.json"
      ]) != 0
      {
        log!("Failed to fetch");
        clone_tx.send_awake(common::Msg::WindActivate);
        return;
      } // if

      // Draw next frame
      clone_tx.send_awake(common::Msg::DrawFetch);
    });
  });
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
