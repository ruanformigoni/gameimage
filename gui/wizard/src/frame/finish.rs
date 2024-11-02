// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  output,
  text,
};

use anyhow::anyhow as ah;

use shared::fltk::WidgetExtExtra;

use crate::db;
use crate::dimm;
use crate::frame;
use crate::log;
use crate::common;
use shared::std::OsStrExt;
use shared::std::PathBufExt;

// fn: finish_file_location() {{{
fn finish_file_location(output: &mut fltk::output::Output) -> anyhow::Result<String>
{
  let db_global = db::global::read()?;
  let _ = output.insert(&db_global.path_file_output.string());
  Ok(format!(".{}.config",  &db_global.path_file_output.file_name().ok_or(ah!("Could not get file stem"))?.string()))
} // fn: finish_file_location() }}}

// pub fn finish() {{{
pub fn finish(tx: Sender<common::Msg>, title: &str)
{
  // Enter the build directory
  if let Err(e) = common::dir_build()
  {
    log!("Err: {}", e.to_string());
  } // if

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

  // Hide prev button
  ret_frame_footer.btn_prev.clone().hide();

  // Set next button to start over
  ret_frame_footer.btn_next.clone().emit(tx, common::Msg::DrawWelcome);
  ret_frame_footer.btn_next.clone().set_label("Finish");

  // Show where the package was saved into
  let mut output_saved_location = output::Output::default()
    .with_width(frame_content.w() - dimm::border()*2)
    .with_height(dimm::height_button_wide())
    .with_align(fltk::enums::Align::Top | fltk::enums::Align::Left)
    .top_center_of(&frame_content, dimm::border()*3)
    .with_label("Your package was saved in this location")
    .with_focus(false);

  // Retrieve output image file location
  let str_package_basename = finish_file_location(&mut output_saved_location).unwrap_or_default();

  let mut output_info = text::TextDisplay::default()
    .with_width_of(&output_saved_location)
    .with_height(dimm::height_button_wide()*8)
    .below_of(&output_saved_location, dimm::border())
    .with_color(fltk::enums::Color::BackGround)
    .with_frame(fltk::enums::FrameType::NoBox);
  output_info.wrap_mode(text::WrapMode::AtColumn, 0);
  output_info.set_buffer(text::TextBuffer::default());
  output_info.insert("You can now move the package to your games folder,");
  output_info.insert(" other Linux computer or an external hard drive.");
  output_info.insert(" To start using your application, simply click to launch.");
  output_info.insert("\n\n");
  output_info.insert("Regardless of where you store your package,");
  output_info.insert(" launching it for the first time will generate");
  output_info.insert(&format!(" a directory called '{}'", str_package_basename));
  output_info.insert(", this directory contains application data such as save games.");
  output_info.insert("\n\n");
  output_info.insert("If you encounter any issues or have suggestions for new features,");
  output_info.insert(" I encourage you to create an issue on GitHub or GitLab.");
  output_info.insert(" Your feedback is invaluable to help project improve.");
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
