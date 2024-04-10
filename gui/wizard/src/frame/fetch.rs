use std::
{
  env,
  path::PathBuf,
  sync::{Arc,Mutex},
};

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

use crate::gameimage;
use crate::dimm;
use crate::frame;
use crate::lib;
use crate::common;
use crate::common::FltkSenderExt;
use crate::common::WidgetExtExtra;
use crate::common::PathBufExt;
use crate::log;

macro_rules! log_return
{
  ($($arg:tt)*) => { { log!($($arg)*); return; } }
} // log_return

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
  file_dest : PathBuf,
  prog      : Progress,
  btn_fetch : Button,
} // struct }}}

// fn verify_and_configure() {{{
fn verify_and_configure(mut output : Output) -> anyhow::Result<()>
{
  // Get platform
  let str_platform = env::var("GIMG_PLATFORM")?.to_lowercase();

  // Run backend to merge files
  output.set_value("Validating and extracting...");

  let arg_url_dwarfs;
  let mut args = vec![
      "fetch"
    , "--platform"
    , &str_platform
  ];

  if let Ok(url) = env::var("GIMG_FETCH_URL_DWARFS")
  {
    arg_url_dwarfs = format!("--url-dwarfs={}", url);
    args.push(&arg_url_dwarfs);
  } // if

  args.push("--sha");

  // Verify sha
  if common::gameimage_sync(args.clone()) != 0
  {
    log!("Failed to verify files with backend");
    return Err(ah!("Failed to verify files with backend"));
  } // if

  let _ = args.pop();

  // Process downloaded files
  if common::gameimage_sync(args.clone()) != 0
  {
    log!("Failed to configure downloaded files");
    return Err(ah!("Failed to configure downloaded files"));
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

  let vec_files = match gameimage::fetch::query_files()
  {
    Ok(vec_files) => vec_files,
    Err(e) => log_return!("Could not fetch url list from backend: {}", e),
  };

  let vec_urls = match gameimage::fetch::query_urls()
  {
    Ok(vec_urls) => vec_urls,
    Err(e) => log_return!("Could not fetch file list from backend: {}", e),
  };

  for (entry_path, entry_url) in std::iter::zip(vec_files, vec_urls)
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
      .with_label("Fetch")
      .with_focus(false);

    // Update base widget for positioning
    base = btn_fetch.as_base_widget();

    // Save in data to create callback afterwards
    vec_fetch.push(Data{file_dest, prog, btn_fetch});
  } // for

  // Function to fetch a file
  let clone_tx = tx.clone();
  let clone_output_status = ret_frame_footer.output_status.clone();
  let f_fetch = move |data : Data|
  {
    // Disable GUI
    clone_tx.send(common::Msg::WindDeactivate);

    #[derive(PartialEq)]
    enum StateBackend { RUN, END, }

    let state_backend = Arc::new(Mutex::new(StateBackend::RUN));

    // Spawn thread to read progress from the backend
    let clone_data = data.clone();
    let clone_state_backend = state_backend.clone();
    std::thread::spawn(move ||
    {
      let path_file_dst = clone_data.file_dest.clone();
      let f_wait = move ||
      {
        while ! path_file_dst.exists()
        {
          // Set guard
          match clone_state_backend.lock()
          {
            Ok(guard) => if *guard == StateBackend::END { break },
            Err(e) => { log!("Could not get state of backend: {}", e); break; },
          };
          std::thread::sleep(std::time::Duration::from_millis(100));
        } // while
      };

      let ipc = match lib::ipc::Ipc::new(clone_data.file_dest, f_wait)
      {
        Ok(ipc) => ipc,
        Err(e) => { log!("Could not create ipc instance: {}", e); return; },
      }; // match

      while let Ok(msg) = ipc.recv()
      {
        let progress = match msg.parse::<f64>()
        {
          Ok(progress) => progress,
          Err(e) => { log!("Could not convert progress to float: {}", e); return; },
        }; // match
        log!("Progress: {}", progress);
        clone_data.prog.clone().set_value(progress);
      } // while
    });

    // Start backend to download file
    let clone_data = data.clone();
    let clone_state_backend = state_backend.clone();
    let mut clone_output_status = clone_output_status.clone();
    std::thread::spawn(move ||
    {
      // Get platform
      let str_platform = match env::var("GIMG_PLATFORM")
      {
        Ok(var) => var.to_lowercase(),
        Err(e) =>
        {
          clone_tx.send(common::Msg::WindActivate);
          log!("Could not read variable GIMG_PLATFORM: {}", e);
          clone_output_status.set_value("Could not read variable GIMG_PLATFORM");
          return;
        },
      };

      let arg_only_file = format!("--only-file={}", clone_data.file_dest.string());
      let arg_url_dwarfs;
      let mut args = vec![
          "fetch"
        , "--platform"
        , &str_platform
        , &arg_only_file
      ];

      if let Ok(url) = env::var("GIMG_FETCH_URL_DWARFS")
      {
        arg_url_dwarfs = format!("--url-dwarfs={}", url);
        args.push(&arg_url_dwarfs);
      } // if

      log!("Args to gameimage: {:?}", args);

      if common::gameimage_sync(args) != 0
      {
        log!("Failed to fetch file list");
      } // if

      match clone_state_backend.lock()
      {
        Ok(mut guard) => *guard = StateBackend::END,
        Err(e) => log!("Could not lock state variable: {}", e),
      };

      clone_tx.send(common::Msg::WindActivate);
      clone_output_status.set_value("Download finished");
    });
  };


  for data in vec_fetch.clone()
  {
    let clone_data = data.clone();
    let clone_f_fetch = f_fetch.clone();
    clone_data.btn_fetch.clone().set_callback(move |_|
    {
      clone_f_fetch(clone_data.clone());
    });
  } // for

  // Set callback to btn next
  let clone_tx = tx.clone();
  ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    // Disable GUI
    clone_tx.send_awake(common::Msg::WindDeactivate);

    let mut clone_output_status = ret_frame_footer.output_status.clone();
    std::thread::spawn(move ||
    {
      // Draw package creator
      if verify_and_configure(clone_output_status.clone()).is_err()
      {
        log!("Failed to verify and configure downloaded files");
        clone_output_status.set_value("Download the required files to proceed");
        clone_tx.send_awake(common::Msg::WindActivate);
        return;
      } // if

      if let Err(e) = set_image_path()
      {
        log!("Could not set image path for GIMG_IMAGE with error {}", e);
      } // if

      clone_tx.send_awake(common::Msg::DrawCreator);
    });

    // Export name for expected image path
  });
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
