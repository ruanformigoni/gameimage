use std::env;
use std::path::PathBuf;
use std::fs::File;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  text::{TextBuffer,TextDisplay},
  menu::MenuButton,
  button::Button,
  group::Group,
  image::SharedImage,
  input::FileInput,
  group::PackType,
  frame::Frame,
  dialog::dir_chooser,
  enums::{Align,FrameType,Color},
  misc::Progress,
};

use url as Url;
use anyhow;
use anyhow::anyhow as ah;

use crate::dimm;
use crate::frame;
use crate::common;
use crate::db;
use crate::download;

// fn url_basename() {{{
fn url_basename(url : Url::Url) -> anyhow::Result<String>
{
  Ok(url.clone()
    .path_segments().ok_or(ah!("Could not split url into segments"))?
    .last().ok_or(ah!("Could not get last segment of url"))?
    .to_string())
} // fn: url_basename }}}

// pub fn fetch() {{{
pub fn fetch(tx: Sender<common::Msg>, title: &str)
{
  let mut frame = Frame::default()
    .with_size(dimm::width(), dimm::height());
  frame.set_frame(FrameType::BorderBox);
  frame.set_type(PackType::Vertical);

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_header = ret_frame_header.frame.clone();
  let frame_footer = ret_frame_footer.frame.clone();

  let mut frame_content = Frame::default()
    .with_size(dimm::width(), dimm::height() - dimm::height_header() - dimm::height_footer())
    .below_of(&frame_header, 0);
  frame_content.set_type(PackType::Vertical);


  // Callback Data
  #[derive(Clone)]
  struct Data
  {
    some_url  : Option<Url::Url>,
    file_dest : PathBuf,
    prog      : Progress,
    btn_fetch : Button,
  } // struct

  // Save the data here to configure the callback between the button press
  // , download and progress bar
  let mut vec_fetch : Vec<Data> = vec![];

  // Populate 'vec_fetch' with links and download paths
  let mut base = frame_content.as_base_widget();

  let result_pairs_values = db::fetch::get();

  if result_pairs_values.is_err()
  {
    println!("Could not fetch file list, '{}'", result_pairs_values.unwrap_err().to_string());
    return;
  } // if

  for (entry_path, entry_url) in result_pairs_values.unwrap()
  {
    // Get full path to save the file into
    let file_dest = std::path::Path::new(&entry_path).to_path_buf();

    // Parse url
    let some_url = Url::Url::parse(&entry_url).ok();

    // Check
    if some_url.is_none()
    {
      println!("Could not create url '{}'", entry_url);
      return;
    } // if

    // Get basename
    let mut result_url_basename = url_basename(some_url.clone().unwrap());

    // Check
    if result_url_basename.is_err()
    {
      println!("Could not get url basename: '{}'", result_url_basename.unwrap_err().to_string());
      return;
    } // if

    // Create progress bar
    let mut prog = Progress::default()
      .above_of(&base, - dimm::border())
      .with_size(frame_content.w() - dimm::width_button_wide() - dimm::border()*3, dimm::height_button_wide())
      .with_label(result_url_basename.unwrap().as_str());
    prog.set_pos(dimm::border(), base.y() + dimm::border());
    prog.set_frame(FrameType::FlatBox);
    prog.set_color(Color::Background2);
    prog.set_selection_color(Color::Blue);
    if base != frame_content.as_base_widget()
    {
      prog.set_pos(prog.x(), prog.y() + dimm::height_button_wide());
    } // if

    // Create start button
    let btn_fetch = Button::default()
      .right_of(&prog, dimm::border())
      .with_size(dimm::width_button_wide(), dimm::height_button_wide())
      .with_label("Fetch");

    // Update base widget for positioning
    base = btn_fetch.as_base_widget();

    // Save in data to create callback afterwards
    vec_fetch.push(Data{some_url, file_dest, prog, btn_fetch});
  } // for

  // Function to fetch a file
  let f_fetch = move |data : Data|
  {
    // Disable button while download is active
    let mut btn = data.btn_fetch.clone();
    btn.deactivate();
    // Clone data into new thread, that keeps downloading after callback ends
    let clone_data = data.clone();
    let mut clone_e_beg = btn.clone();
    let mut clone_e_prog = btn.clone();
    let mut clone_e_ok = btn.clone();
    let mut clone_e = btn.clone();
    let mut clone_prog_prog = clone_data.prog.clone();
    let mut clone_prog_finish = clone_data.prog.clone();
    std::thread::spawn(move ||
    {
      let vec_animation_chars = vec!["=", "==", "===", "===="];
      let result = download::download(clone_data.some_url
        , clone_data.file_dest
        // on_start
        , move ||
        {
          clone_e_beg.deactivate();
          clone_e_beg.set_color(Color::DarkGreen);
          clone_e_beg.set_label("=");
        }
        // on_progress
        , move |f64_progress|
        {
          let len = clone_e_prog.label().chars().count()%vec_animation_chars.len();
          clone_e_prog.set_label( vec_animation_chars[len] );
          clone_prog_prog.set_value(f64_progress);
          fltk::app::awake();
        }
        // on_finish
        , move ||
        {
          clone_e_ok.set_label("Done");
          clone_e_ok.set_color(Color::DarkGreen);
        }
      );
      println!("Download result: {:?}", result);
      if result.is_err()
      {
        clone_e.set_label("Retry");
        clone_e.set_color(Color::DarkRed);
        clone_e.activate();
      } // if
      else
      {
        clone_e.deactivate();
        clone_prog_finish.set_value(100.0);
      } // else
      fltk::app::awake();
    });    
  };


  for data in vec_fetch.clone()
  {
    let clone_data = data.clone();
    let mut clone_btn_fetch = clone_data.btn_fetch.clone();
    clone_btn_fetch.set_callback(move |e|
    {
      f_fetch(clone_data.clone());
    });
  } // for


  // Set callback to btn prev
  ret_frame_footer.btn_prev.clone().emit(tx, common::Msg::DrawPlatform);

  // Set callback to btn next
  let clone_vec_fetch = vec_fetch.clone();
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  let clone_tx = tx.clone();
  ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    for data in clone_vec_fetch.clone()
    {
      let mut str_file_sha = data.file_dest.to_str().unwrap_or("").to_owned();
      str_file_sha.push_str(".sha256sum");
      if download::sha(PathBuf::from(str_file_sha), data.file_dest).is_err()
      {
        clone_output_status.set_value("SHA verify failed, download the files before proceeding");
        return;
      } // if
    } // for
    clone_tx.send(common::Msg::DrawFetch);
  });
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
