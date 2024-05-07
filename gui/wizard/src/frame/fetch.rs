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
use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;

use crate::gameimage;
use crate::dimm;
use crate::frame;
use crate::lib;
use crate::common;
use shared::std::PathBufExt;
use crate::log;
use crate::log_return_void;

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
  clicked   : Arc<Mutex<bool>>,
} // struct }}}

// fn backend_validate_and_configure() {{{
fn backend_validate_and_configure(mut output : Output) -> anyhow::Result<()>
{
  output.set_value("Validating and extracting...");

  // Run backend to merge files
  gameimage::fetch::validate()?;

  // Use backend to configure downloaded files
  gameimage::fetch::configure()?;

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

  // Create scroll list
  let mut scroll = shared::fltk::ScrollList::new(
    frame_content.width() - dimm::border()*2
    , frame_content.height() - dimm::border()*2
    , frame_content.x() + dimm::border()
    , frame_content.y() + dimm::border()
  );
  scroll.set_frame(FrameType::BorderBox);
  scroll.set_border(dimm::border(), dimm::border());

  let vec_files = match gameimage::fetch::query_files()
  {
    Ok(vec_files) => vec_files,
    Err(e) => log_return_void!("Could not fetch url list from backend: {}", e),
  };

  let vec_urls = match gameimage::fetch::query_urls()
  {
    Ok(vec_urls) => vec_urls,
    Err(e) => log_return_void!("Could not fetch file list from backend: {}", e),
  };

  scroll.begin();
  for (entry_path, entry_url) in std::iter::zip(vec_files, vec_urls)
  {
    // Get full path to save the file into
    let file_dest = std::path::Path::new(&entry_path).to_path_buf();

    // Parse url
    let url = match Url::Url::parse(&entry_url)
    {
      Ok(value) => value,
      Err(e) => { log!("Could not create url '{}': {}", entry_url, e); return; },
    }; // match

    // Get basename
    let url_basename = match url_basename(url.clone())
    {
      Ok(value) => value,
      Err(e) => { log!("Could not get url basename: {}", e); return; },
    }; // match

    // Create progress bar
    let prog = Progress::default()
      .with_size(scroll.widget_ref().w() - dimm::width_button_wide() - dimm::border()*3, dimm::height_button_wide())
      .with_label(url_basename.as_str())
      .with_frame(FrameType::BorderBox)
      .with_color(Color::BackGround)
      .with_color_selected(Color::Blue);
    scroll.add(&mut prog.as_base_widget());

    // Create start button
    let btn_fetch = Button::default()
      .right_of(&prog, dimm::border())
      .with_size(dimm::width_button_wide(), dimm::height_button_wide())
      .with_label("Fetch")
      .with_focus(false);

    // Save in data to create callback afterwards
    vec_fetch.push(Data{file_dest, prog, btn_fetch, clicked: Arc::new(Mutex::new(false))});
  } // for
  scroll.end();

  // Function to fetch a file
  let clone_tx = tx.clone();
  let clone_output_status = ret_frame_footer.output_status.clone();
  let f_fetch = move |data : Data|
  {
    // Disable GUI
    clone_tx.send_awake(common::Msg::WindDeactivate);

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
        clone_data.prog.clone().set_value(progress);
      } // while
    });

    // Start backend to download file
    let clone_data = data.clone();
    let clone_state_backend = state_backend.clone();
    let mut clone_output_status = clone_output_status.clone();
    std::thread::spawn(move ||
    {
      let mut clone_btn_fetch = clone_data.btn_fetch.clone();

      // Change fetch button
      clone_btn_fetch.set_color(Color::Green);
      clone_btn_fetch.set_label("...");

      let path_file_dst = clone_data.file_dest.clone();
      let mut is_success = true;
      match gameimage::fetch::fetch(Some(path_file_dst.clone()))
      {
        Ok(_) => log!("Successfully fetched file {}", path_file_dst.string()),
        Err(e) => { log!("Failed to fetch file '{}' with error '{}'", path_file_dst.string(), e); is_success = false; },
      }; // match

      match clone_state_backend.lock()
      {
        Ok(mut guard) => *guard = StateBackend::END,
        Err(e) => log!("Could not lock state variable: {}", e),
      };

      match is_success
      {
        true =>
        {
          // Set download as completed
          clone_btn_fetch.set_label("Done");
          match clone_data.clicked.lock()
          {
            Ok(mut guard) => *guard = true,
            Err(e) => log!("Could not lock data to mark as completed: {}", e),
          } // match
        },
        false => { clone_btn_fetch.set_label("Failure"); clone_btn_fetch.set_color(Color::Red); },
      }; // match

      clone_tx.send_awake(common::Msg::WindActivate);
      clone_output_status.set_value("Operation finished");
    }); // std::thread
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
  let mut clone_btn_next = ret_frame_footer.btn_next.clone();
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  let clone_vec_fetch = vec_fetch.clone();
  clone_btn_next.set_callback(move |_|
  {
    // Allow proceeding if all files were successfully downloaded
    if ! clone_vec_fetch.iter().all(|e| { return match e.clicked.lock() { Ok(guard) => *guard, Err(_) => false, }; })
    {
      clone_output_status.set_value("Download all files before proceeding");
      return;
    } // if

    // Disable GUI
    clone_tx.send_awake(common::Msg::WindDeactivate);

    let mut clone_output_status = ret_frame_footer.output_status.clone();
    std::thread::spawn(move ||
    {
      // Draw package creator
      if backend_validate_and_configure(clone_output_status.clone()).is_err()
      {
        clone_tx.send_awake(common::Msg::WindActivate);
        clone_output_status.set_value("Download the required files to proceed");
        log_return_void!("Failed to verify and configure downloaded files");
      } // if

      if let Err(e) = set_image_path()
      {
        log!("Could not set image path for GIMG_IMAGE with error {}", e);
      } // if

      clone_tx.send_awake(common::Msg::DrawCreator);
    });
  });
}
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
