use std::collections::HashMap;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  enums::{Align,FrameType,Color},
  group::PackType,
};

use lazy_static::lazy_static;

use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;

use crate::dimm;
use crate::frame;
use crate::common;
use crate::log;
use crate::gameimage;

lazy_static!
{
  pub static ref HASH_PLATFORM_MSG: HashMap<common::Platform, common::Msg> =
  {
    let mut m = HashMap::new();
    m.insert(common::Platform::Wine, common::Msg::DrawWineName);
    m.insert(common::Platform::Linux, common::Msg::DrawLinuxName);
    m.insert(common::Platform::Retroarch, common::Msg::DrawRetroarchName);
    m.insert(common::Platform::Pcsx2, common::Msg::DrawPcsx2Name);
    m.insert(common::Platform::Rcps3, common::Msg::DrawRpcs3Name);
    m
  };
}

// fn platform_add() {{{
fn platform_add(tx: Sender<common::Msg>
  , widget: &fltk::widget::Widget,platform: common::Platform
  , is_installed: bool) -> fltk::group::Flex
{
  let mut row = fltk::group::Flex::new(widget.x() + dimm::border()
    , widget.y() + dimm::border()
    , widget.w() - dimm::border()*2
    , dimm::height_button_wide()
    , ""
  );
  row.set_type(PackType::Horizontal);
  row.set_spacing(dimm::border());
  // Create progress bar
  let _ = fltk::frame::Frame::default()
    .with_label(frame::fetch::HASH_PLATFORM_DESCR.get(platform.as_str()).unwrap_or(&""))
    .with_align(Align::Left | Align::Inside)
    .with_frame(FrameType::BorderBox)
    .with_color(Color::BackGround);
  // Create start button
  let mut btn_start = shared::fltk::button::rect::arrow_forward()
    .with_color(if is_installed { Color::Green } else { Color::BackGround })
    .with_focus(false);
  let clone_tx = tx.clone();
  btn_start.set_callback(move |_|
  {
    std::env::set_var("GIMG_PLATFORM", platform.as_str());
    clone_tx.send_awake(*HASH_PLATFORM_MSG.get(&platform).unwrap());
  });
  if ! is_installed { btn_start.deactivate(); }
  row.fixed(&btn_start, dimm::width_button_rec());
  row.end();
  row
} // fn platform_add() }}}

// fn platform_list() {{{
fn platform_list(tx: Sender<common::Msg>, widget: &fltk::widget::Widget) -> anyhow::Result<()>
{
  let vec_platforms = gameimage::fetch::installed()?;
  let mut col = fltk::group::Flex::new(widget.x() + dimm::border()
    , widget.y() + dimm::border()
    , widget.w() - dimm::border()*2
    , widget.h() - dimm::border()*2
    , ""
  );
  col.set_type(PackType::Vertical);
  col.set_frame(FrameType::BorderBox);
  col.set_spacing(dimm::border());
  col.set_margin(dimm::border());
  let row_linux     = platform_add(tx, &col.as_base_widget(), common::Platform::Linux, vec_platforms.contains(&common::Platform::Linux));
  let row_rpcs3     = platform_add(tx, &col.as_base_widget(), common::Platform::Rcps3, vec_platforms.contains(&common::Platform::Rcps3));
  let row_retroarch = platform_add(tx, &col.as_base_widget(), common::Platform::Retroarch, vec_platforms.contains(&common::Platform::Retroarch));
  let row_pcsx2     = platform_add(tx, &col.as_base_widget(), common::Platform::Pcsx2, vec_platforms.contains(&common::Platform::Pcsx2));
  let row_wine      = platform_add(tx, &col.as_base_widget(), common::Platform::Wine, vec_platforms.contains(&common::Platform::Wine));
  col.fixed(&row_linux, dimm::height_button_wide());
  col.fixed(&row_rpcs3, dimm::height_button_wide());
  col.fixed(&row_retroarch, dimm::height_button_wide());
  col.fixed(&row_pcsx2, dimm::height_button_wide());
  col.fixed(&row_wine, dimm::height_button_wide());
  col.end();

  Ok(())
} // fn platform_list() }}}

// pub fn platform() {{{
pub fn platform(tx: Sender<common::Msg>, title: &str)
{
  // Enter the build directory
  if let Err(e) = common::dir_build() { log!("{}", e); }
  // Create frame from template
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();
  // Configure buttons
  ret_frame_footer.btn_prev.clone().emit(tx, common::Msg::DrawCreator);
  ret_frame_footer.btn_next.clone().hide();
  // Clone content frame
  let frame_content = ret_frame_header.frame_content.clone();
  // List platforms to fetch
  if let Err(e) = platform_list(tx, &frame_content.as_base_widget()) { log!("{}", e); };
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
