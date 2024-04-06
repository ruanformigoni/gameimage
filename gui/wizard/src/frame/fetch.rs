use std::env;
use std::path::PathBuf;

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  button::Button,
  output::Output,
  enums::{FrameType,Color},
  misc::Progress,
};

use url as Url;
use anyhow::anyhow as ah;

use crate::dimm;
use crate::frame;
use crate::common;
use crate::common::FltkSenderExt;
use crate::log;
use crate::db;
use crate::lib::download;

// fn url_basename() {{{
fn url_basename(url : Url::Url) -> anyhow::Result<String>
{
  Ok(url.clone()
    .path_segments().ok_or(ah!("Could not split url into segments"))?
    .last().ok_or(ah!("Could not get last segment of url"))?
    .to_string())
} // fn: url_basename }}}

// fn set_image_path() {{{
fn set_image_path() -> anyhow::Result<()>
{
  let str_platform = env::var("GIMG_PLATFORM")?.to_lowercase();
  env::set_var("GIMG_IMAGE", format!("{}.flatimage", str_platform));
  Ok(())
} // }}}

// struct Data {{{
#[derive(Clone)]
struct Data
{
  some_url  : Option<Url::Url>,
  file_dest : PathBuf,
  prog      : Progress,
  btn_fetch : Button,
} // struct }}}

// fn fetch_files() {{{
fn fetch_files(vec_data : Vec<Data>
  , mut output : Output) -> anyhow::Result<()>
{
  // Get platform
  let str_platform = env::var("GIMG_PLATFORM")?.to_lowercase();

  // Verify SHA for each file
  for data in vec_data.clone()
  {
    // Create path to SHA file
    let mut path_file_sha : String = data.file_dest
      .to_str()
      .ok_or(ah!("Failed to convert file_dest to string"))?
      .into();
    path_file_sha.push_str(".sha256sum");
    // Use download modules to verify SHA
    if let Err(e) = download::sha(PathBuf::from(path_file_sha), data.file_dest)
    {
      output.set_value("SHA verify failed, download the files before proceeding");
      return Err(anyhow::anyhow!(e.to_string()));
    } // if
  } // for

  // Run backend to merge files
  output.set_value("Validating and extracting...");

  if common::gameimage_sync(vec![
      "fetch"
    , "--platform"
    , &str_platform
    , "--output-file"
    , &format!("{}.flatimage", str_platform)
  ]) != 0
  {
    log!("Failed to fetch file list");
    return Err(ah!("Failed to fetch file list"));
  } // if

  Ok(())
}
// }}}

// pub fn fetch() {{{
pub fn fetch(tx: Sender<common::Msg>, title: &str)
{
  // Enter the build directory
  if let Err(e) = common::dir_build()
  {
    log!("Err: {}", e.to_string());
  } // if

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

  // Set callback to btn prev
  ret_frame_footer.btn_prev.clone().emit(tx, common::Msg::DrawPlatform);

  // Switch to build dir
  if   let Ok(str_path) = env::var("GIMG_DIR")
    && let Ok(_) = env::set_current_dir(&str_path)
  {
    ret_frame_footer
      .output_status
      .clone()
      .set_value(format!("Switched dir to {}", str_path).as_str());
  } // if
  else
  {
    ret_frame_footer
      .output_status
      .clone()
      .set_value(format!("Could not switch dir").as_str());
  } // else

  // Save the data here to configure the callback between the button press
  // , download and progress bar
  let mut vec_fetch : Vec<Data> = vec![];

  // Populate 'vec_fetch' with links and download paths
  let mut base = frame_content.as_base_widget();

  let result_pairs_values = db::fetch::get();

  if result_pairs_values.is_err()
  {
    log!("Could not fetch file list, '{}'", result_pairs_values.unwrap_err().to_string());
    return;
  } // if

  for (entry_path, entry_url) in result_pairs_values.unwrap()
  {
    // Get full path to save the file into
    let file_dest = std::path::Path::new(&entry_path).to_path_buf();

    // Parse url
    let url = if let Ok(value) = Url::Url::parse(&entry_url)
    {
      value
    }
    else
    {
      log!("Could not create url '{}'", entry_url);
      return;
    }; // if

    // Get basename
    let url_basename = if let Ok(value) = url_basename(url.clone())
    {
      value
    }
    else
    {
      log!("Could not get url basename");
      return;
    }; // if

    // Create progress bar
    let mut prog = Progress::default()
      .above_of(&base, - dimm::border())
      .with_size(frame_content.w() - dimm::width_button_wide() - dimm::border()*3, dimm::height_button_wide())
      .with_label(url_basename.as_str());
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
    vec_fetch.push(Data{some_url: Some(url), file_dest, prog, btn_fetch});
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
      log!("Download result: {:?}", result);
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
    clone_btn_fetch.set_callback(move |_|
    {
      f_fetch(clone_data.clone());
    });
  } // for


  // Set callback to btn next
  let clone_tx = tx.clone();
  ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    // Disable GUI
    clone_tx.send_awake(common::Msg::WindDeactivate);

    let clone_vec_fetch = vec_fetch.clone();
    let clone_output_status = ret_frame_footer.output_status.clone();
    std::thread::spawn(move ||
    {
      if fetch_files(clone_vec_fetch.clone(), clone_output_status.clone()).is_ok()
      {
        if let Err(e) = set_image_path()
        {
          log!("Could not set image path for GIMG_IMAGE with error {}", e);
        } // if
        // Draw package creator
        clone_tx.send_awake(common::Msg::DrawCreator);
      } // if

      // Re-enable GUI
      clone_tx.send_awake(common::Msg::WindActivate);
    });

    // Export name for expected image path
  });
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
