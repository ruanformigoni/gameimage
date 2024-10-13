use std::env;
use std::sync::Mutex;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  text::{TextBuffer,TextDisplay},
  menu::MenuButton,
  enums::{Align,FrameType},
};

use shared::fltk::WidgetExtExtra;

use shared::fltk::SenderExt;

use crate::dimm;
use crate::frame;
use crate::common;
use crate::log;
use crate::gameimage;

// enum Platform {{{
#[derive(PartialEq, Clone)]
enum Platform
{
  Linux,
  Wine,
  WineUrl,
  Retroarch,
  Pcsx2,
  Rcps3,
} // }}}

// impl Platform {{{
impl Platform
{
  fn as_str(&self) -> &'static str
  {
    match self
    {
      Platform::Linux                    => "linux",
      Platform::Wine | Platform::WineUrl => "wine",
      Platform::Retroarch                => "retroarch",
      Platform::Pcsx2                    => "pcsx2",
      Platform::Rcps3                    => "rpcs3",
    } // match
  } // as_str

  fn from_str(src : &str) -> Option<Platform>
  {
    match src
    {
      "linux"     => Some(Platform::Linux),
      "wine"      => Some(Platform::Wine),
      "wine_url"  => Some(Platform::WineUrl),
      "retroarch" => Some(Platform::Retroarch),
      "pcsx2"     => Some(Platform::Pcsx2),
      "rpcs3"     => Some(Platform::Rcps3),
      _           => None,
    } // match
  } // as_str
} // impl IconFrame }}}

// pub fn platform() {{{
pub fn platform(tx: Sender<common::Msg>, title: &str)
{
  // Keep track of which frame to draw (search web or local)
  static PLATFORM : once_cell::sync::Lazy<Mutex<Option<Platform>>> = once_cell::sync::Lazy::new(|| Mutex::new(None));

  // Remember custom url field
  static URL : once_cell::sync::Lazy<Mutex<Option<String>>> = once_cell::sync::Lazy::new(|| Mutex::new(None));

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
    .with_size(dimm::width_wizard() - dimm::border()*2, dimm::height_button_wide())
    .bottom_of(&frame_content, - dimm::border())
    .center_x(&frame_content)
    .with_focus(false);
  btn_menu.add_choice("linux|wine|retroarch|pcsx2|rpcs3");

  // Create callback with descriptions
  let buffer = TextBuffer::default();
  let mut frame_text = TextDisplay::default()
    .with_size(dimm::width_wizard() - dimm::border()*2, frame_content.h() - btn_menu.h() - dimm::border()*2 - dimm::height_text())
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
  if let Ok(lock) = PLATFORM.lock()
  {
    if *lock == Some(Platform::Wine)
    {
      btn_menu.set_width(frame_text.w() - dimm::border() - (dimm::width_button_wide() as f32 * 2.5) as i32);
      let mut btn_wine_dist = MenuButton::default()
        .with_size((dimm::width_button_wide() as f32 * 2.5) as i32, dimm::height_button_wide())
        .with_focus(false)
        .right_of(&btn_menu, dimm::border())
        .with_callback(|e|
        {
          let choice = e.choice().unwrap_or(String::from("None"));
          std::env::set_var("GIMG_WINE_DIST", choice.as_str());
          e.set_label(&choice);
        });
      // Set dist options
      btn_wine_dist.add_choice("caffe|umu-pronton-ge|osu-tkg|soda|staging|tkg|vaniglia");
      // Init default value for btn_wine_dist
      if let Ok(var) = std::env::var("GIMG_WINE_DIST")
      {
        btn_wine_dist.set_label(&var);
      } // if
      else
      {
        std::env::set_var("GIMG_WINE_DIST", "umu-proton-ge");
        btn_wine_dist.set_label("umu-proton-ge");
      } // else
    } // if
    else if *lock == Some(Platform::WineUrl)
    {
      frame_text.set_size(frame_text.w(), frame_text.h() - dimm::height_button_wide() - dimm::height_text() - dimm::border());
      let _btn_wine_dist = fltk::input::Input::default()
        .with_size_of(&btn_menu)
        .above_of(&btn_menu, dimm::border())
        .with_label("Insert the url for the custom wine tarball")
        .with_align(Align::Top | Align::Left)
        .with_callback(|e|
        {
          match URL.lock()
          {
            Ok(mut guard) => *guard = Some(e.value()),
            Err(e) => log!("Could not lock input url field: {}", e),
          };
        });
    } // else if

    // Reset URL
    if *lock != Some(Platform::WineUrl) && let Ok(mut guard) = URL.lock()
    {
      *guard = None;
      if let Err(e) = gameimage::fetch::url_clear()
      {
        log!("Could not clear custom url: {}", e);
      } // if
    } // if

    if let Some(platform) = lock.clone()
    {
      // Update menu
      btn_menu.set_label(platform.as_str());
      // Update description box
      f_update_buffer(platform.as_str().into());
    } // if
  }

  // Set callback to dropdown menu selection
  let mut clone_update_buffer = f_update_buffer.clone();
  let clone_tx = tx.clone();
  btn_menu.set_callback(move |e|
  {
    // Fetch choice
    let choice = e.choice().unwrap_or(String::from("None"));
    // Set as label
    e.set_label(&choice);
    // Update platform
    if let Ok(mut guard) = PLATFORM.lock() && let Some(platform) = Platform::from_str(&choice)
    {
      *guard = Some(platform.clone());
      env::set_var("GIMG_PLATFORM", platform.as_str());
    } // if
    else
    {
      log!("Could not update GIMG_PLATFORM");
    } // else
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
    if let Ok(guard) = PLATFORM.lock() && *guard == None
    {
      log!("No platform was selected");
      return;
    } // if

    // Fetch files
    clone_output_status.set_value("Fetching list of files to download");

    // Disable window
    clone_tx.send_awake(common::Msg::WindDeactivate);

    // Set custom url if it was passed
    if let Ok(guard) = URL.lock() && let Some(url) = guard.clone()
    {
      if let Err(e) = gameimage::fetch::set_url_layer(&url)
      {
        log!("Exit backend with error: {}", e);
      } // if
    } // match
    else
    {
      log!("Could not set custom url");
    } // else

    // Disable window
    clone_tx.send_awake(common::Msg::DrawFetch);
  });
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
