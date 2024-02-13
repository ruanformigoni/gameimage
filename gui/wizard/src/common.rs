// Imports {{{
// Constants
use crate::dimm;

// Concurrency
use std::sync::Arc;
use std::sync::Mutex;

// Urls
use url as Url;

// Paths
use std::path::PathBuf;
use std::ffi::OsStr;

// Search
use rust_search::{FileSize, FilterExt, SearchBuilder};

// Rust std
use std::env;
use std::path::Path;

// Downloads
use crate::download;

// Gui
use fltk::prelude::*;
use fltk::{
  widget::Widget,
  image::SharedImage,
  app::App,
  input::{Input,FileInput},
  dialog::{dir_chooser, file_chooser},
  text::{TextDisplay,TextBuffer},
  output::Output,
  group::{Group, PackType, Scroll},
  window::Window,
  menu::MenuButton,
  button::Button,
  frame::Frame,
  enums::{Align,FrameType,Color},
  misc::Progress,
};

// Error
use anyhow::anyhow as ah;
// }}}

// Constants {{{
const STR_DESC_WINE : &str = "Wine is a program which allows running Microsoft Windows programs (including DOS, Windows 3.x, Win32, and Win64 executables) on Unix. It consists of a program loader which loads and executes a Microsoft Windows binary, and a library (called Winelib) that implements Windows API calls using their Unix, X11 or Mac equivalents.  The library may also be used for porting Windows code into native Unix executables. Wine is free software, released under the GNU LGPL.";

const STR_DESC_RETR : &str = "RetroArch is a frontend for emulators, game engines and media players. It enables you to run classic games on a wide range of computers and consoles through its slick graphical interface. Settings are also unified so configuration is done once and for all. In addition to this, you are able to run original game discs (CDs) from RetroArch. RetroArch has advanced features like shaders, netplay, rewinding, next-frame response times, runahead, machine translation, blind accessibility features, and more!";

const STR_DESC_PCSX2 : &str = "Being almost as old as the console it is emulating, PCSX2 not only has a lot of history behind it, but a continually evolving future. PCSX2 is a free and open-source PlayStation 2 (PS2) emulator. Its purpose is to emulate the PS2's hardware, using a combination of MIPS CPU Interpreters, Recompilers and a Virtual Machine which manages hardware states and PS2 system memory. The project has been running for almost 20 years. Past versions could only run a few public domain game demos, but newer versions can run most games at full speed, including popular titles such as Final Fantasy X and Devil May Cry 3. A significant majority of the official PS2 library is considered playable or perfect, with the remainder at least making it to the menus. PCSX2 allows you to play PS2 games on your PC, with many additional features and benefits. A few of those benefits include: custom resolutions and upscaling, virtual and sharable memory cards, save-states, patching system, internal recorder to achieve lossless quality at full speed.";

const STR_DESC_RPCS3 : &str = "RPCS3 is a multi-platform open-source Sony PlayStation 3 emulator and debugger written in C++ for Windows, Linux, macOS and FreeBSD. The purpose of the project is to completely and accurately emulate the Sony PlayStation 3 Computer Entertainment System in its entirety with the power of open-source community and reverse engineering. Our goal is to preserve the legacy of the PlayStation 3 hardware and its vast library by bringing it and its exclusives to the PC platform. We want to achieve this by targeting and supporting multiple operating systems as well as being compatible with a wide range of computer hardware with realistic requirements.";

const STR_DESC_YUZU : &str = "Yuzu is an experimental open-source emulator for the Nintendo Switch from the creators of Citra. It is written in C++ with portability in mind, with builds actively maintained for Windows, Linux and Android.";
// }}}

// pub struct DataFrameDefault {{{
#[derive(Clone)]
pub struct DataFrameDefault
{
  pub app            : App,
  pub wind           : Window,
  pub group          : Group,
  pub sep_top        : Frame,
  pub frame          : Frame,
  pub header         : Frame,
  pub group_content  : Group,
  pub text_status    : Output,
  pub sep_bottom     : Frame,
  pub btn_prev       : Button,
  pub btn_next       : Button,
}
// }}}

// fn dispatch_gameimage_cmd() {{{
fn dispatch_gameimage_cmd(stage : &str, env : Vec<(&str,&str)>) -> anyhow::Result<i32>
{
  let mut str_cmd : String = String::new();
  str_cmd.push_str("$GIMG_SCRIPT_DIR/main.sh");
  str_cmd.push_str(" --dir=$GIMG_DIR");
  str_cmd.push_str(" --name=$GIMG_NAME");
  str_cmd.push_str(" --platform=$GIMG_PLATFORM");
  let mut vec_env : Vec<(&str,&str)> = vec![
      ("GIMG_STAGE", stage)
    , ("GIMG_FETCH_DRY","1")
    , ("GIMG_INTERACTIVE","0")
  ];
  vec_env.extend(env);
  // vec_env
  let some_handle = std::process::Command::new("/tmp/gameimage/bin/bash")
    .envs(vec_env)
    .stderr(std::process::Stdio::inherit())
    .stdout(std::process::Stdio::inherit())
    .args(["-c", str_cmd.as_str()])
    .spawn();


  // Wait for back-end to execute
  let mut some_status : Option<std::process::ExitStatus> = None;

  if let Some(mut handle) = some_handle.ok()
  {
    some_status = handle.wait().ok();
  } // if
  else
  {
    return Err(ah!("Failed to dispatch gameimage command"));
  } // else

  // Check return status
  if let Some(status) = some_status
  {
    return Ok(status.code().ok_or(ah!("Failed to extract status code"))?);
  }

  Err(ah!("Failure to fetch gameimage execution status"))
} // fn: dispatch_gameimage_cmd }}}

// fn frame_content_reset() {{{
fn frame_default_reset<F>(data_frame_default : DataFrameDefault, callback : F)
where
  F: FnMut(DataFrameDefault) + Send + Sync + 'static + Clone
{
  // Reset status
  let mut text_status = data_frame_default.text_status.clone();
  text_status.set_value("");

  // Reset content
  let mut group_content = data_frame_default.group_content.clone();
  group_content.clear();

  // Redraw window
  let mut wind = data_frame_default.wind.clone();
  wind.begin();
  callback.clone()(data_frame_default.clone());
  wind.end();
  wind.redraw();

  // Update app
  fltk::app::flush();
} // fn: frame_default_reset }}}

// pub fn frame_default() {{{
pub fn frame_default(app : App, wind : Window, title : &str) -> DataFrameDefault
{
  let mut group = Group::default().with_size(dimm::WIDTH, dimm::HEIGHT);
  group.set_frame(FrameType::FlatBox);

  let mut frame = Frame::default().with_size(dimm::WIDTH, dimm::HEIGHT);
  frame.set_frame(FrameType::NoBox);
  frame.set_type(PackType::Vertical);

  // Header
  let mut header = Frame::new(dimm::BORDER
    , dimm::BORDER
    , dimm::WIDTH-dimm::BORDER*2
    , dimm::HEIGHT_BUTTON_REC
    , title);
  header.set_frame(FrameType::NoBox);
  header.set_label_size((dimm::HEIGHT_TEXT as f32 * 1.5) as i32);

  // Separator
  let mut sep_top = Frame::default()
    .below_of(&header, dimm::BORDER)
    .with_size(dimm::WIDTH - dimm::BORDER*2, 2);
  sep_top.set_frame(FrameType::BorderBox);

  // Main content
  let mut group_content = Group::new(dimm::BORDER
    , dimm::BORDER
    , dimm::WIDTH - dimm::BORDER*2
    , dimm::HEIGHT_BUTTON_WIDE*12 - dimm::BORDER
    , "")
  .below_of(&sep_top, dimm::BORDER);
  group_content.set_frame(FrameType::NoBox);
  group_content.begin();
  group_content.end();

  // Status bar
  let mut text_status = Output::default()
    .with_size(dimm::WIDTH_STATUS, dimm::HEIGHT_STATUS)
    .with_align(Align::Left)
    .below_of(&frame, -dimm::HEIGHT_STATUS);
  text_status.deactivate();

  // Continue
  let mut btn_next = Button::default()
    .with_size(dimm::WIDTH_BUTTON_WIDE, dimm::HEIGHT_BUTTON_WIDE)
    .with_label("Next")
    .above_of(&text_status, dimm::BORDER);
  btn_next.set_pos(frame.w() - dimm::WIDTH_BUTTON_WIDE - dimm::BORDER, btn_next.y());
  btn_next.set_color(Color::Blue);

  // Prev
  let mut btn_prev = Button::default()
    .with_size(dimm::WIDTH_BUTTON_WIDE, dimm::HEIGHT_BUTTON_WIDE)
    .with_label("Prev")
    .above_of(&text_status, dimm::BORDER);
  btn_prev.set_pos(frame.x() + dimm::BORDER, btn_prev.y());
  btn_prev.set_color(Color::Background);

  // Separator
  let mut sep_bottom = Frame::default()
    .above_of(&btn_next, dimm::BORDER)
    .with_size(dimm::WIDTH - dimm::BORDER*2, 2);
  sep_bottom.set_frame(FrameType::BorderBox);
  sep_bottom.set_pos(dimm::BORDER, sep_bottom.y());

  group.end();

  DataFrameDefault { app, wind, group, frame, sep_top, header, group_content, text_status, sep_bottom, btn_prev, btn_next }
}
// }}}

// pub fn frame_welcome() {{{
pub fn frame_welcome(data_frame_default : DataFrameDefault)
{
  let group_content = data_frame_default.group_content.clone();

  let header = data_frame_default.header.clone().with_label("Welcome to GameImage!");

  // Start content
  group_content.begin();

  // Project Logo
  let mut frame_image = Frame::default()
    .with_size(group_content.w(), dimm::HEIGHT_BUTTON_WIDE*4)
    .below_of(&group_content, 0);
  frame_image.set_align(Align::Inside | Align::Bottom);
  frame_image.set_pos(frame_image.x(), frame_image.y() - frame_image.h());
  frame_image.set_pos(frame_image.x(), frame_image.y()
    - group_content.h()/2 + frame_image.h()/2 - dimm::HEIGHT_BUTTON_WIDE);
  let mut clone_frame_image = frame_image.clone();
  let image = SharedImage::load("/tmp/gameimage/gameimage.svg")
    .and_then(move |mut img|
    {
      img.scale(clone_frame_image.w(), clone_frame_image.h(), true, true);
      clone_frame_image.set_image(Some(img.clone()));
      Ok(img)
    });
  frame_image.redraw();

  // Determine temporary build directory
  let sep_bottom = data_frame_default.sep_bottom.clone();
  let mut input_dir = FileInput::new(dimm::BORDER
    , dimm::BORDER
    , dimm::WIDTH - dimm::BORDER*2
    , dimm::HEIGHT_BUTTON_WIDE + dimm::HEIGHT_TEXT
    , "Select The Directory for GameImage's Temporary Files")
    .above_of(&sep_bottom, dimm::BORDER)
    .with_align(Align::Top | Align::Left);
  input_dir.set_readonly(true);

  // // Check if GIMG_DIR exists
  if let Some(env_dir_build) = env::var("GIMG_DIR").ok()
  {
    input_dir.set_value(&env_dir_build);
  } // if

  // // Set input_dir callback
  input_dir.set_callback(|e|
  {
    let choice = dir_chooser("Select the build directory", "", false);
    let str_choice = choice.unwrap_or(String::from(""));
    e.set_value(str_choice.as_str());
    env::set_var("GIMG_DIR", str_choice.as_str());
  });

  // Set callback for next
  let clone_data_frame_default = data_frame_default.clone();
  let mut clone_wind = data_frame_default.wind.clone();
  let mut clone_group_content = data_frame_default.group_content.clone();
  let mut clone_btn_next = data_frame_default.btn_next.clone();
  let mut clone_text_status = data_frame_default.text_status;
  clone_btn_next.set_callback(move |_|
  {
    if let Some(var) = env::var("GIMG_DIR").ok()
    {
      if Path::new(&var).exists()
      {
        frame_default_reset(clone_data_frame_default.clone()
          , move |e| { frame_select_platform(e.clone()); });
        return;
      } // if
    } // if

    clone_text_status.set_value("Invalid temporary files directory");
  });
  
  // Finish content
  group_content.end();
} // fn: frame_welcome }}}

// pub fn frame_select_platform() {{{
pub fn frame_select_platform(data_frame_default : DataFrameDefault)
{
  let group_content = data_frame_default.group_content.clone();

  let header = data_frame_default.header.clone().with_label("Select the Target Platform");

  group_content.begin();

  // Menu options to select platform
  let mut btn_menu = MenuButton::new(dimm::BORDER
    , dimm::BORDER
    , dimm::WIDTH - dimm::BORDER*2
    , dimm::HEIGHT_BUTTON_WIDE
    , "")
    .below_of(&group_content, -group_content.h());

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

  // Let description empty
  let mut group_text = Group::new(dimm::BORDER
    , dimm::BORDER
    , dimm::WIDTH - dimm::BORDER*2
    , dimm::HEIGHT_BUTTON_WIDE*9 - dimm::BORDER
    , "")
  .below_of(&btn_menu, dimm::BORDER);
  group_text.set_frame(FrameType::BorderBox);

  // Create callback with descriptions
  let buffer = TextBuffer::default();
  group_text.clear();
  group_text.begin();
  let mut frame = TextDisplay::default()
    .with_align(Align::Top)
    .below_of(&btn_menu, dimm::BORDER)
    .with_size(dimm::WIDTH - dimm::BORDER*2, group_text.h());
  frame.set_buffer(buffer.clone());
  frame.set_frame(FrameType::BorderBox);
  frame.set_color(group_text.color());
  frame.wrap_mode(fltk::text::WrapMode::AtBounds, 0);
  group_text.end();

  // Update buffer function
  let mut clone_buffer = buffer.clone();
  let mut f_update_buffer = move |str_platform : String|
  {
    clone_buffer.remove(0, clone_buffer.length());
    match str_platform.as_str()
    {
      "wine" => clone_buffer.insert(0, STR_DESC_WINE),
      "retroarch" => clone_buffer.insert(0, STR_DESC_RETR),
      "pcsx2" => clone_buffer.insert(0, STR_DESC_PCSX2),
      "rpcs3" => clone_buffer.insert(0, STR_DESC_RPCS3),
      "yuzu" => clone_buffer.insert(0, STR_DESC_YUZU),
      _ => ()
    }
  };

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

  // Set callback for prev
  let clone_data_frame_default = data_frame_default.clone();
  let mut clone_btn_prev = data_frame_default.btn_prev.clone();
  let mut clone_group_content = data_frame_default.group_content.clone();
  let mut clone_wind = data_frame_default.wind.clone();
  let mut clone_text_status = data_frame_default.text_status.clone();
  clone_btn_prev.set_callback(move |_|
  {
    frame_default_reset(clone_data_frame_default.clone()
      ,|e| { frame_welcome(e.clone()); } );
  });

  // Set callback for next
  let clone_data_frame_default = data_frame_default.clone();
  let mut clone_btn_next = data_frame_default.btn_next.clone();
  let mut clone_group_content = data_frame_default.group_content.clone();
  let mut clone_wind = data_frame_default.wind.clone();
  let mut clone_text_status = data_frame_default.text_status.clone();
  clone_btn_next.set_callback(move |_|
  {
    let env_platform = env::var("GIMG_PLATFORM");

    // Allow next if dropdown has valid value
    if let Some(platform) = env_platform.ok()
    {
      if platform != btn_menu.label()
      {
        clone_text_status.set_value("Please select a platform to proceed");
        return;
      } // if
    } // if
    else
    {
      clone_text_status.set_value("Please select a platform to proceed");
      return;
    } // else

    clone_text_status.set_value("Fetching list of files to download");

    // For update status
    fltk::app::flush();

    // Ask back-end for the files to download for the selected platform
    let cmd_result = dispatch_gameimage_cmd("fetch", vec![]);

    if cmd_result.is_err()
    {
      clone_text_status.set_value(&cmd_result.unwrap_err().to_string());
      return;
    } // if

    // Go to next frame
    frame_default_reset(clone_data_frame_default.clone()
      ,|e| { frame_fetch_tools(e.clone()); } );
  });

  // Finish group
  group_content.end();
}
// }}}

// pub fn frame_fetch_tools() {{{
pub fn frame_fetch_tools(data_frame_default : DataFrameDefault) -> DataFrameDefault
{
  let group_content = data_frame_default.group_content.clone();

  let header = data_frame_default.header.clone().with_label("Fetch the Required Tools");

  group_content.begin();

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
  if let (Some(keys), Some(vals)) =
    (std::fs::read_to_string("/tmp/gameimage/logs/_fetch_keys.log").ok()
    , std::fs::read_to_string("/tmp/gameimage/logs/_fetch_vals.log").ok())
  {
    let mut base = group_content.as_base_widget();
    for line in std::iter::zip(keys.lines(), vals.lines())
    {
      // Get full path to save the file into
      let file_dest = std::path::Path::new(line.0).to_path_buf();

      // Parse url
      let some_url = Url::Url::parse(line.1).ok();

      // Get basename
      let mut url_basename = String::new();
      if let Some(url) = some_url.clone()
      {
        if let Some(url_segments) = url.path_segments()
        {
          if let Some(url_segment) = url_segments.last()
          {
            url_basename = url_segment.to_string();
          } // if
        } // if
      } // if
        
      // Create progress bar
      let mut prog = Progress::default()
        .above_of(&base, - dimm::BORDER)
        .with_size(group_content.w() - dimm::WIDTH_BUTTON_WIDE - dimm::BORDER, dimm::HEIGHT_BUTTON_WIDE)
        .with_label(url_basename.as_str());
      prog.set_pos(dimm::BORDER, base.y() + dimm::BORDER);
      prog.set_frame(FrameType::FlatBox);
      prog.set_color(Color::Background2);
      prog.set_selection_color(Color::Blue);
      if base != group_content.as_base_widget()
      {
        prog.set_pos(prog.x(), prog.y() + dimm::HEIGHT_BUTTON_WIDE);
      } // if

      // Create start button
      let btn_fetch = Button::default()
        .right_of(&prog, dimm::BORDER)
        .with_size(dimm::WIDTH_BUTTON_WIDE, dimm::HEIGHT_BUTTON_WIDE)
        .with_label("Fetch");

      // Update base widget for positioning
      base = btn_fetch.as_base_widget();

      // Save in data to create callback afterwards
      vec_fetch.push(Data{some_url, file_dest, prog, btn_fetch});
    } // for
  } // if

  // Function to fetch a file
  let f_fetch = move |data_frame_default : DataFrameDefault, data : Data|
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
        // on_error
        , move || {}
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
    let clone_data_frame_default = data_frame_default.clone();
    let mut clone_btn_fetch = clone_data.btn_fetch.clone();
    clone_btn_fetch.set_callback(move |e|
    {
      f_fetch(clone_data_frame_default.clone(), clone_data.clone());
    });
  } // for

  // Set callback to btn prev
  let clone_data_frame_default = data_frame_default.clone();
  data_frame_default.btn_prev.clone().set_callback(move |_|
  {
    frame_default_reset(clone_data_frame_default.clone(), |e|{ frame_select_platform(e); });
  });

  // Set callback to btn next
  let clone_data_frame_default = data_frame_default.clone();
  let clone_vec_fetch = vec_fetch.clone();
  clone_data_frame_default.btn_next.clone().set_callback(move |_|
  {
    let mut text_status = clone_data_frame_default.text_status.clone();
    for data in clone_vec_fetch.clone()
    {
      let mut str_file_sha = data.file_dest.to_str().unwrap_or("").to_owned();
      str_file_sha.push_str(".sha256sum");
      if download::sha(PathBuf::from(str_file_sha), data.file_dest).is_err()
      {
        text_status.set_value("SHA verify failed, download the files before proceeding");
        return;
      } // if
    } // for
    frame_default_reset(clone_data_frame_default.clone(), |e|{ frame_info(e); });
  });

  group_content.end();

  data_frame_default
}
// }}}

// pub fn frame_info() {{{
pub fn frame_info(data_frame_default : DataFrameDefault)
{
  let group_content = data_frame_default.group_content.clone();

  let header = data_frame_default.header.clone().with_label("Enter the Game Details");

  group_content.begin();

  // Game name
  let mut input_name = Input::new(dimm::BORDER
    , dimm::BORDER
    , dimm::WIDTH - dimm::BORDER*2
    , dimm::HEIGHT_BUTTON_WIDE
    , "Application Name")
    .below_of(&group_content, - group_content.h() + dimm::HEIGHT_TEXT + dimm::BORDER)
    .with_align(Align::Top | Align::Left);

  // // Check if GIMG_NAME exists
  if let Some(env_name) = env::var("GIMG_NAME").ok()
  {
    input_name.set_value(&env_name);
  } // if

  // // Set input_name callback
  input_name.set_callback(|e|
  {
    env::set_var("GIMG_NAME", e.value());
  });

  // Icon
  let mut input_icon = FileInput::new(dimm::BORDER
    , dimm::BORDER
    , dimm::WIDTH - dimm::BORDER*2
    , dimm::HEIGHT_BUTTON_WIDE + dimm::HEIGHT_TEXT
    , "Application Icon")
    .below_of(&input_name, dimm::BORDER + dimm::HEIGHT_TEXT)
    .with_align(Align::Top | Align::Left);
  input_icon.set_readonly(true);

  // // Check if GIMG_ICON exists
  if let Some(env_icon) = env::var("GIMG_ICON").ok()
  {
    input_icon.set_value(&env_icon);
  } // if

  // // Set input_icon callback
  input_icon.set_callback(|e|
  {
    let choice = file_chooser("Select the icon", "*.jpg|*.png|*.svg", ".", false);
    let str_choice = choice.unwrap_or(String::from(""));
    e.set_value(str_choice.as_str());
    env::set_var("GIMG_ICON", str_choice.as_str());
  });
  
  // Set callback for previous frame
  let clone_data_frame_default = data_frame_default.clone();
  data_frame_default.btn_prev.clone().set_callback(move |_|
  {
    frame_default_reset(clone_data_frame_default.clone(), |e| { frame_fetch_tools(e); });
  });

  // Set callback for next frame
  let clone_data_frame_default = data_frame_default.clone();
  let clone_name = input_name.clone();
  let clone_input_icon = input_icon.clone();
  clone_data_frame_default.btn_next.clone().set_callback(move |_|
  {
    let mut clone_text_status = data_frame_default.text_status.clone();
    // Check name field
    if clone_name.value().is_empty()
    {
      clone_text_status.set_value("Name field is empty");
      return;
    } // if
    // Check icon field
    if clone_input_icon.value().is_empty()
    {
      clone_text_status.set_value("No icon was selected");
      return;
    } // if
    frame_default_reset(clone_data_frame_default.clone(), |e| { frame_configure(e); });
  });

  // Finish group
  group_content.end();
}
// }}}

// pub fn frame_configure() {{{
fn frame_configure(data_frame_default : DataFrameDefault)
{
  let mut group_content = data_frame_default.group_content.clone();

  let header = data_frame_default.header.clone().with_label("Configure and Install");

  // Save widgets to disable GUI on button click
  let arc_widgets : Arc<Mutex<Vec<Widget>>> = Arc::new(Mutex::new(vec![]));

  let clone_arc_widgets = arc_widgets.clone();
  let f_register_widgets = move |vec_w : Vec<Widget>|
  {
    // Include button & labels in arc
    if let Some(mut lock) = clone_arc_widgets.lock().ok()
    {
      for w in vec_w { lock.push(w.clone()); } // for
    } // if
  };

  // Register prev & next buttons
  f_register_widgets(vec![
    data_frame_default.btn_prev.clone().as_base_widget(),
    data_frame_default.btn_next.clone().as_base_widget()]
  );

  group_content.begin();

  let clone_arc_widgets = arc_widgets.clone();
  let clone_f_register_widgets = f_register_widgets.clone();
  let mut f_install_tool = move |mut parent: Widget
    , stage: &str
    , name: &str
    , desc : &str
    , env: Vec<(&str,&str)>|
  {
    // Label
    let mut label = Frame::new(dimm::BORDER
      , dimm::BORDER
      , dimm::WIDTH_BUTTON_WIDE
      , dimm::HEIGHT_BUTTON_WIDE
      , desc)
      .below_of(&parent, dimm::BORDER)
      .with_align(Align::Inside | Align::Center);
    label.set_pos(parent.x(), label.y());
    label.set_frame(FrameType::BorderBox);
    label.set_size(dimm::WIDTH - dimm::BORDER*3 - dimm::WIDTH_BUTTON_WIDE, dimm::HEIGHT_BUTTON_WIDE);

    // Button
    let mut btn = Button::new(dimm::BORDER
      , dimm::BORDER
      , dimm::WIDTH_BUTTON_WIDE
      , dimm::HEIGHT_BUTTON_WIDE
      , name)
      .right_of(&label, dimm::BORDER);
    btn.set_color(Color::Blue);

    // Register widgets to disable when required
    clone_f_register_widgets(vec![btn.clone().as_base_widget(), label.clone().as_base_widget()]);

    // Callback
    let clone_env: Vec<_> = env.into_iter().map(|(k,v)| (k.to_string(), v.to_string()) ).collect();
    let clone_stage = stage.to_owned();
    let clone_arc_widgets = clone_arc_widgets.clone();
    btn.set_callback(move |e|
    {
      let mut clone_e = e.clone();
      let clone_env = clone_env.clone();
      let clone_stage = clone_stage.clone();
      let clone_arc_widgets = clone_arc_widgets.clone();
      std::thread::spawn(move ||
      {
        if let Some(vec) = clone_arc_widgets.lock().ok()
        {
          for mut w in vec.clone().into_iter()
          {
            w.deactivate();
          } // for
        } // if
        clone_e.deactivate();
        let _ = dispatch_gameimage_cmd(clone_stage.as_str(), clone_env
          .iter()
          .map(|(k, v)| (k.as_str(), v.as_str()))
          .collect());
        clone_e.activate();
        fltk::app::awake();
        if let Some(vec) = clone_arc_widgets.lock().ok()
        {
          for mut w in vec.clone().into_iter()
          {
            w.activate();
          } // for
        } // if
      });
    });

    parent = label.as_base_widget();

    (btn.clone(), label.clone())
  };

  let (btn, label) = f_install_tool(data_frame_default.sep_top.clone().as_base_widget()
    , "configure"
    , "DXVK"
    , "Install DXVK for directx 9/10/11"
    , vec![("GIMG_INSTALL_DXVK", "1"), ("GIMG_WINETRICKS_CUSTOM", ""), ("GIMG_ARCH", "win64")]);

  let (btn, label) = f_install_tool(label.clone().as_base_widget()
    , "configure"
    , "VKD3D"
    , "Install VKD3D for directx 12"
    , vec![("GIMG_INSTALL_VKD3D", "1"), ("GIMG_WINETRICKS_CUSTOM", ""), ("GIMG_ARCH", "win64")]);

  let (mut btn, label) = f_install_tool(label.clone().as_base_widget()
    , "configure"
    , "Tricks"
    , "Run a custom winetricks command"
    , vec![("GIMG_ARCH", "win64")]);

  // Push it down to move label above
  btn.set_pos(btn.x(), btn.y() + dimm::HEIGHT_TEXT + dimm::BORDER);

  let mut input_custom_cmd = Input::new(dimm::BORDER
    , dimm::BORDER
    , label.w()
    , dimm::HEIGHT_BUTTON_WIDE
    , "")
    .left_of(&btn, dimm::BORDER);

  // Check existing value
  if let Some(env_var) = env::var("GIMG_WINETRICKS_CUSTOM").ok()
  {
    input_custom_cmd.set_value(&env_var);
  } // if

  // Register callback
  input_custom_cmd.set_callback(|e|
  {
    env::set_var("GIMG_WINETRICKS_CUSTOM", e.value());
  });

  // Register in widgets vec
  f_register_widgets(vec![input_custom_cmd.clone().as_base_widget()]);

  let (mut btn, mut label) = f_install_tool(input_custom_cmd.clone().as_base_widget()
    , "install"
    , "Wine"
    , "Install a program with wine"
    , vec![("GIMG_ARCH", "win64")]);

  // Push it down to move label above
  btn.set_pos(btn.x(), btn.y() + dimm::HEIGHT_TEXT + dimm::BORDER);

  let mut input_custom_cmd = Input::new(dimm::BORDER
    , dimm::BORDER
    , label.w()
    , dimm::HEIGHT_BUTTON_WIDE
    , "")
    .left_of(&btn, dimm::BORDER);

  // Check existing value
  if let Some(env_var) = env::var("GIMG_WINE_INSTALL_CUSTOM").ok()
  {
    input_custom_cmd.set_value(&env_var);
  } // if

  // Register callback
  input_custom_cmd.set_callback(|e|
  {
    let some_choice = file_chooser("Select the executable", "*.exe|*.msi", ".", false);
    if let Some(choice) = some_choice
    {
      e.set_value(&choice);
      env::set_var("GIMG_WINE_INSTALL_CUSTOM", e.value());
    } // if
  });
  input_custom_cmd.set_readonly(true);

  // Register in widgets vec
  f_register_widgets(vec![input_custom_cmd.clone().as_base_widget()]);

  // Set callback for previous frame
  let clone_data_frame_default = data_frame_default.clone();
  data_frame_default.btn_prev.clone().set_callback(move |_|
  {
    frame_default_reset(clone_data_frame_default.clone(), |e| { frame_info(e); });
  });

  // Set callback for next frame
  let clone_data_frame_default = data_frame_default.clone();
  data_frame_default.btn_next.clone().set_callback(move |_|
  {
    frame_default_reset(clone_data_frame_default.clone(), |e| { frame_test(e); });
  });

  group_content.end();
  
} // fn: frame_configure }}}

// pub fn frame_test() {{{
fn frame_test(data_frame_default : DataFrameDefault)
{
  let mut group_content = data_frame_default.group_content.clone();

  let header = data_frame_default.header.clone().with_label("Test the Installed Application");

  // Save widgets to disable GUI on button click
  let arc_widgets : Arc<Mutex<Vec<Widget>>> = Arc::new(Mutex::new(vec![]));

  let clone_arc_widgets = arc_widgets.clone();
  let f_register_widgets = move |vec_w : Vec<Widget>|
  {
    // Include button & labels in arc
    if let Some(mut lock) = clone_arc_widgets.lock().ok()
    {
      for w in vec_w { lock.push(w.clone()); } // for
    } // if
  };

  // Register prev & next buttons
  f_register_widgets(vec![
    data_frame_default.btn_prev.clone().as_base_widget(),
    data_frame_default.btn_next.clone().as_base_widget()]
  );

  group_content.begin();

  let clone_arc_widgets = arc_widgets.clone();
  let clone_f_register_widgets = f_register_widgets.clone();
  let mut f_test_binary = move |mut parent: Widget
    , stage: &str
    , name: &str
    , desc : &str
    , env: Vec<(&str,&str)>|
  {
    // Label
    let mut label = TextDisplay::new(dimm::BORDER
      , dimm::BORDER
      , dimm::WIDTH_BUTTON_WIDE
      , dimm::HEIGHT_BUTTON_WIDE
      , "")
      .below_of(&parent, dimm::BORDER)
      .with_align(Align::Inside | Align::Center);

    let height_btn = ( dimm::HEIGHT_BUTTON_WIDE as f64 *1.2 ) as i32;

    let mut buf = fltk::text::TextBuffer::default();
    label.set_buffer(buf);
    label.set_pos(parent.x(), label.y());
    label.insert(desc);
    label.set_frame(FrameType::BorderBox);
    label.set_size(dimm::WIDTH - dimm::BORDER*5 - dimm::WIDTH_BUTTON_WIDE, height_btn);
    label.set_scrollbar_size(dimm::HEIGHT_TEXT/2);
    label.show_insert_position();

    // Button
    let mut btn = Button::new(dimm::BORDER
      , dimm::BORDER
      , dimm::WIDTH_BUTTON_WIDE
      , height_btn
      , name)
      .right_of(&label, dimm::BORDER);
    btn.set_color(Color::Blue);

    // Register widgets to disable when required
    clone_f_register_widgets(vec![btn.clone().as_base_widget(), label.clone().as_base_widget()]);

    // Callback
    let clone_env: Vec<_> = env.into_iter().map(|(k,v)| (k.to_string(), v.to_string()) ).collect();
    let clone_stage = stage.to_owned();
    let clone_arc_widgets = clone_arc_widgets.clone();
    let clone_label = label.clone();
    btn.set_callback(move |e|
    {
      let mut clone_e = e.clone();
      let clone_env = clone_env.clone();
      let clone_stage = clone_stage.clone();
      let clone_arc_widgets = clone_arc_widgets.clone();
      let clone_label = clone_label.clone();
      std::thread::spawn(move ||
      {
        if let Some(vec) = clone_arc_widgets.lock().ok()
        {
          for mut w in vec.clone().into_iter()
          {
            w.deactivate();
          } // for
        } // if

        // Set test var for test phase
        env::set_var("GIMG_WINE_TEST_CUSTOM", clone_label.buffer().unwrap().text().as_str());

        clone_e.deactivate();
        let _ = dispatch_gameimage_cmd(clone_stage.as_str(), clone_env
          .iter()
          .map(|(k, v)| (k.as_str(), v.as_str()))
          .collect());
        clone_e.activate();
        fltk::app::awake();
        if let Some(vec) = clone_arc_widgets.lock().ok()
        {
          for mut w in vec.clone().into_iter()
          {
            w.activate();
          } // for
        } // if
      });
    });

    parent = label.as_base_widget();

    (btn.clone(), label.clone())
  };

  // Create scrollbar
  let scrollbar = Scroll::default()
    .with_pos(group_content.x(), group_content.y())
    .with_size(group_content.w(), group_content.h());

  // Search for files in drive_c
  if let Some(mut str_prefix) = env::var("GIMG_DIR").ok()
  {
    str_prefix.to_string().push_str("/build/AppDir/app/wine/drive_c");

      // .custom_filter(|dir| dir.metadata().unwrap().is_file() )
    let search: Vec<String> = SearchBuilder::default()
      .location(str_prefix)
      // Exclude windows/
      .custom_filter(|e|
      {
        // Requires re-fetch of env var, because this library does not support FnMut on 2.1.0
        let mut str_prefix = env::var("GIMG_DIR").unwrap();
        let str_windows = format!("{}/drive_c/windows/", str_prefix);
        e.path().file_name() != Some(OsStr::new("windows"))
      })
      // Exclude files not ending with .exe or .msi
      .custom_filter(|e|
      {
        if e.path().is_file()
        {
          return e.path().extension() == Some(OsStr::new("exe"))
            || e.path().extension() == Some(OsStr::new("msi"))
        }
        return true;
      })
      .build()
      .filter(|e| Path::new(e).is_file() )
      .collect();

    let mut parent = data_frame_default.sep_top.clone().as_base_widget();
    for entry in search
    {
      // println!("Result: {}", entry);
      let (mut btn, label) = f_test_binary(parent
        , "test"
        , "Run"
        , &entry
        , vec![("GIMG_ARCH", "win64")]);
      parent = label.clone().as_base_widget();
    } // for
  } // if

  scrollbar.end();

  group_content.end();
  
} // fn: frame_test }}}
 
// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
