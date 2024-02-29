use std::
{
  io::Read,
  sync::{Arc,Mutex,mpsc,OnceLock},
  path::PathBuf,
  ffi::OsStr,
  env,
  path,
};

use fltk::
{
  app,
  widget::Widget,
  prelude::WidgetExt,
};

use image;
use anyhow::anyhow as ah;

use crate::frame;
use crate::dimm;

#[derive(Debug, Clone, Copy)]
pub enum Msg
{
  DrawWelcome,
  DrawPlatform,
  DrawFetch,
  DrawCreator,
  DrawDesktop,
  DrawName,

  DrawWineName,
  DrawWineIcon,
  DrawWineConfigure,
  DrawWineRom,
  DrawWineDefault,
  DrawWineCompress,

  DrawRetroarchName,
  DrawRetroarchIcon,
  DrawRetroarchRom,
  DrawRetroarchCore,
  DrawRetroarchBios,
  DrawRetroarchTest,
  DrawRetroarchCompress,

  DrawPcsx2Name,
  DrawPcsx2Icon,
  DrawPcsx2Rom,
  DrawPcsx2Bios,
  DrawPcsx2Test,
  DrawPcsx2Compress,

  WindActivate,
  WindDeactivate,

  Quit,
}

pub fn impl_log(value : &str)
{
  static TX : OnceLock<Mutex<frame::term::Term>> = OnceLock::new();

  match TX.get_or_init(|| Mutex::new(frame::term::Term::new(dimm::border()
    , dimm::width() - dimm::border()*2
    , dimm::height() - dimm::border()*2
    , dimm::border()
    , dimm::border()
  ))).lock()
  {
    Ok(term) => term.append(value),
    Err(_) => (),
  } // match
}

#[macro_export]
macro_rules! log
{
  ($($arg:tt)*) =>
  {
    let mut output = format!($($arg)*);
    output.push('\n');
    common::impl_log(output.as_str());
  }
}


// pub fn wizard_by_platform {{{
pub fn wizard_by_platform() -> anyhow::Result<Msg>
{

  match env::var("GIMG_PLATFORM")?.to_lowercase().as_str()
  {
    "wine"      => Ok(Msg::DrawWineName),
    "retroarch" => Ok(Msg::DrawRetroarchName),
    "pcsx2"     => Ok(Msg::DrawPcsx2Name),
    _           => Err(ah!("Unrecognized platform")),
  } // match

} // fn: wizard_by_platform }}}

// Constants {{{
pub const STR_DESC_WINE : &str = "Wine is a program which allows running Microsoft Windows programs (including DOS, Windows 3.x, Win32, and Win64 executables) on Unix. It consists of a program loader which loads and executes a Microsoft Windows binary, and a library (called Winelib) that implements Windows API calls using their Unix, X11 or Mac equivalents.  The library may also be used for porting Windows code into native Unix executables. Wine is free software, released under the GNU LGPL.";

pub const STR_DESC_RETR : &str = "RetroArch is a frontend for emulators, game engines and media players. It enables you to run classic games on a wide range of computers and consoles through its slick graphical interface. Settings are also unified so configuration is done once and for all. In addition to this, you are able to run original game discs (CDs) from RetroArch. RetroArch has advanced features like shaders, netplay, rewinding, next-frame response times, runahead, machine translation, blind accessibility features, and more!";

pub const STR_DESC_PCSX2 : &str = "Being almost as old as the console it is emulating, PCSX2 not only has a lot of history behind it, but a continually evolving future. PCSX2 is a free and open-source PlayStation 2 (PS2) emulator. Its purpose is to emulate the PS2's hardware, using a combination of MIPS CPU Interpreters, Recompilers and a Virtual Machine which manages hardware states and PS2 system memory. The project has been running for almost 20 years. Past versions could only run a few public domain game demos, but newer versions can run most games at full speed, including popular titles such as Final Fantasy X and Devil May Cry 3. A significant majority of the official PS2 library is considered playable or perfect, with the remainder at least making it to the menus. PCSX2 allows you to play PS2 games on your PC, with many additional features and benefits. A few of those benefits include: custom resolutions and upscaling, virtual and sharable memory cards, save-states, patching system, internal recorder to achieve lossless quality at full speed.";

pub const STR_DESC_RPCS3 : &str = "RPCS3 is a multi-platform open-source Sony PlayStation 3 emulator and debugger written in C++ for Windows, Linux, macOS and FreeBSD. The purpose of the project is to completely and accurately emulate the Sony PlayStation 3 Computer Entertainment System in its entirety with the power of open-source community and reverse engineering. Our goal is to preserve the legacy of the PlayStation 3 hardware and its vast library by bringing it and its exclusives to the PC platform. We want to achieve this by targeting and supporting multiple operating systems as well as being compatible with a wide range of computer hardware with realistic requirements.";

pub const STR_DESC_YUZU : &str = "Yuzu is an experimental open-source emulator for the Nintendo Switch from the creators of Citra. It is written in C++ with portability in mind, with builds actively maintained for Windows, Linux and Android.";
// }}}

// pub fn dir_build() {{{
pub fn dir_build() -> anyhow::Result<()>
{
  let path_dir_gimg = path::PathBuf::from(env::var("GIMG_DIR")?);

  Ok(env::set_current_dir(path_dir_gimg)?)
} // fn: dir_build }}}

// pub fn gameimage_cmd() {{{
pub fn gameimage_cmd(args : Vec<String>) -> anyhow::Result<i32>
{
  dir_build()?;

  let path_binary_gameimage = path::PathBuf::from(env::var("GIMG_BACKEND")?);

  let handle = std::process::Command::new(path_binary_gameimage)
    .env_remove("LD_PRELOAD")
    .env("FIM_FIFO", "0")
    .stderr(std::process::Stdio::piped())
    .stdout(std::process::Stdio::piped())
    .args(args)
    .spawn()?;

  // Create arc reader for stdout
  let arc_handle = Arc::new(Mutex::new(handle));

  let clone_arc_handle = arc_handle.clone();
  std::thread::spawn(move ||
  {
    // Acquire stdout
    let mut lock =
      if let Ok(lock) = clone_arc_handle.lock() && lock.stdout.is_some()
      {
        lock
      }
      else
      {
        return; 
      }; // else

    // Create buf
    let mut buf = vec![0; 4096];

    // Write buf to stdout
    loop
    {
      std::thread::sleep(std::time::Duration::from_millis(50));

      let bytes_read = match lock.stdout.as_mut().unwrap().read(&mut buf)
      {
        Ok(bytes_read) => bytes_read,
        Err(_) => break,
      };

      if bytes_read == 0 { break; }
      let output = String::from_utf8_lossy(&buf[..bytes_read]);
      impl_log(&output);

      let bytes_read = match lock.stderr.as_mut().unwrap().read(&mut buf)
      {
        Ok(bytes_read) => bytes_read,
        Err(_) => break,
      };

      if bytes_read == 0 { break; }
      let output = String::from_utf8_lossy(&buf[..bytes_read]);
      impl_log(&output);

      app::awake();
    }
  });


  // Wait for back-end to execute
  let some_status : Option<std::process::ExitStatus> = arc_handle.lock().unwrap().wait().ok();

  // Get return status
  let status = some_status
    .ok_or(ah!("Failed to read status code"))?
    .code()
    .ok_or(ah!("Failed to extract status code"))?;

  // Check return status
  if status != 0
  {
    return Err(ah!(format!("Exited backend with code {}", status)));
  } // if

  Ok(status)
} // fn: gameimage_cmd }}}

// pub fn image_resize() {{{
pub fn image_resize(path_out : PathBuf, path_in : PathBuf, width : u32, height : u32) -> anyhow::Result<()>
{
  let mut img = image::io::Reader::open(path_in)?.decode()?;
  img = img.resize(width, height, image::imageops::FilterType::CatmullRom);
  Ok(img.save(path_out)?)
} // }}}

// pub trait WidgetExtExtra {{{
pub trait WidgetExtExtra
{
  fn right_bottom_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn top_left_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
}

impl<T: WidgetExt + Clone> WidgetExtExtra for T
{
  fn right_bottom_of<W: WidgetExt>(&mut self, other: &W, offset : i32) -> Self
  {
    self.set_pos(
        other.x() + other.w() - self.w() + offset
      , other.y() + other.h() - self.h() + offset
    );
    self.clone()
  }

  fn top_left_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self
  {
    self.set_pos(
        other.x() + offset
      , other.y() + offset
    );
    self.clone()
  }
}
// }}}

// pub trait OsStrExt {{{
pub trait OsStrExt
{
  fn string(&self) -> String;
}

impl OsStrExt for OsStr
{
  fn string(&self) -> String
  {
    self.to_string_lossy().into_owned()
  } // fn: string
}
// }}}

// pub trait PathBufExt {{{
pub trait PathBufExt
{
  fn string(&self) -> String;
}

impl PathBufExt for PathBuf
{
  fn string(&self) -> String
  {
    self.clone().into_os_string().into_string().unwrap_or(String::new())
  } // fn: string
}
// }}}

// pub fn common {{{
pub fn common() -> anyhow::Result<()>
{
  // Enter build dir
  dir_build()?;

  Ok(())
} // fn: common
// }}}

// // Imports {{{
// // Constants
// use crate::dimm;
//
// // Concurrency
// use std::sync::Arc;
// use std::sync::Mutex;
//
// // Urls
// use url as Url;
//
// // Paths
// use std::path::PathBuf;
// use std::ffi::OsStr;
//
// // Search
// use rust_search::{FileSize, FilterExt, SearchBuilder};
//
// // Rust std
// use std::env;
// use std::path::Path;
//
// // Downloads
// use crate::download;
//
// // Gui
// use fltk::prelude::*;
// use fltk::{
//   widget::Widget,
//   image::SharedImage,
//   app::App,
//   input::{Input,FileInput},
//   dialog::{dir_chooser, file_chooser},
//   text::{TextDisplay,TextBuffer},
//   output::Output,
//   group::{Group, PackType, Scroll},
//   window::Window,
//   menu::MenuButton,
//   button::Button,
//   frame::Frame,
//   enums::{Align,FrameType,Color},
//   misc::Progress,
// };
//
// // Error
// use anyhow::anyhow as ah;
// // }}}
//
// // pub fn frame_info() {{{
// pub fn frame_info(data_frame_default : DataFrameDefault)
// {
//   let group_content = data_frame_default.group_content.clone();
//
//   let header = data_frame_default.header.clone().with_label("Enter the Game Details");
//
//   group_content.begin();
//
//   // Game name
//   let mut input_name = Input::new(dimm::BORDER
//     , dimm::BORDER
//     , dimm::WIDTH - dimm::BORDER*2
//     , dimm::HEIGHT_BUTTON_WIDE
//     , "Application Name")
//     .below_of(&group_content, - group_content.h() + dimm::HEIGHT_TEXT + dimm::BORDER)
//     .with_align(Align::Top | Align::Left);
//
//   // // Check if GIMG_NAME exists
//   if let Some(env_name) = env::var("GIMG_NAME").ok()
//   {
//     input_name.set_value(&env_name);
//   } // if
//
//   // // Set input_name callback
//   input_name.set_callback(|e|
//   {
//     env::set_var("GIMG_NAME", e.value());
//   });
//
//   // Icon
//   let mut input_icon = FileInput::new(dimm::BORDER
//     , dimm::BORDER
//     , dimm::WIDTH - dimm::BORDER*2
//     , dimm::HEIGHT_BUTTON_WIDE + dimm::HEIGHT_TEXT
//     , "Application Icon")
//     .below_of(&input_name, dimm::BORDER + dimm::HEIGHT_TEXT)
//     .with_align(Align::Top | Align::Left);
//   input_icon.set_readonly(true);
//
//   // // Check if GIMG_ICON exists
//   if let Some(env_icon) = env::var("GIMG_ICON").ok()
//   {
//     input_icon.set_value(&env_icon);
//   } // if
//
//   // // Set input_icon callback
//   input_icon.set_callback(|e|
//   {
//     let choice = file_chooser("Select the icon", "*.jpg|*.png|*.svg", ".", false);
//     let str_choice = choice.unwrap_or(String::from(""));
//     e.set_value(str_choice.as_str());
//     env::set_var("GIMG_ICON", str_choice.as_str());
//   });
//   
//   // Set callback for previous frame
//   let clone_data_frame_default = data_frame_default.clone();
//   data_frame_default.btn_prev.clone().set_callback(move |_|
//   {
//     frame_default_reset(clone_data_frame_default.clone(), |e| { frame_fetch_tools(e); });
//   });
//
//   // Set callback for next frame
//   let clone_data_frame_default = data_frame_default.clone();
//   let clone_name = input_name.clone();
//   let clone_input_icon = input_icon.clone();
//   clone_data_frame_default.btn_next.clone().set_callback(move |_|
//   {
//     let mut clone_text_status = data_frame_default.text_status.clone();
//     // Check name field
//     if clone_name.value().is_empty()
//     {
//       clone_text_status.set_value("Name field is empty");
//       return;
//     } // if
//     // Check icon field
//     if clone_input_icon.value().is_empty()
//     {
//       clone_text_status.set_value("No icon was selected");
//       return;
//     } // if
//     frame_default_reset(clone_data_frame_default.clone(), |e| { frame_configure(e); });
//   });
//
//   // Finish group
//   group_content.end();
// }
// // }}}
//
// // pub fn frame_configure() {{{
// fn frame_configure(data_frame_default : DataFrameDefault)
// {
//   let mut group_content = data_frame_default.group_content.clone();
//
//   let header = data_frame_default.header.clone().with_label("Configure and Install");
//
//   // Save widgets to disable GUI on button click
//   let arc_widgets : Arc<Mutex<Vec<Widget>>> = Arc::new(Mutex::new(vec![]));
//
//   let clone_arc_widgets = arc_widgets.clone();
//   let f_register_widgets = move |vec_w : Vec<Widget>|
//   {
//     // Include button & labels in arc
//     if let Some(mut lock) = clone_arc_widgets.lock().ok()
//     {
//       for w in vec_w { lock.push(w.clone()); } // for
//     } // if
//   };
//
//   // Register prev & next buttons
//   f_register_widgets(vec![
//     data_frame_default.btn_prev.clone().as_base_widget(),
//     data_frame_default.btn_next.clone().as_base_widget()]
//   );
//
//   group_content.begin();
//
//   let clone_arc_widgets = arc_widgets.clone();
//   let clone_f_register_widgets = f_register_widgets.clone();
//   let mut f_install_tool = move |mut parent: Widget
//     , stage: &str
//     , name: &str
//     , desc : &str
//     , env: Vec<(&str,&str)>|
//   {
//     // Label
//     let mut label = Frame::new(dimm::BORDER
//       , dimm::BORDER
//       , dimm::WIDTH_BUTTON_WIDE
//       , dimm::HEIGHT_BUTTON_WIDE
//       , desc)
//       .below_of(&parent, dimm::BORDER)
//       .with_align(Align::Inside | Align::Center);
//     label.set_pos(parent.x(), label.y());
//     label.set_frame(FrameType::BorderBox);
//     label.set_size(dimm::WIDTH - dimm::BORDER*3 - dimm::WIDTH_BUTTON_WIDE, dimm::HEIGHT_BUTTON_WIDE);
//
//     // Button
//     let mut btn = Button::new(dimm::BORDER
//       , dimm::BORDER
//       , dimm::WIDTH_BUTTON_WIDE
//       , dimm::HEIGHT_BUTTON_WIDE
//       , name)
//       .right_of(&label, dimm::BORDER);
//     btn.set_color(Color::Blue);
//
//     // Register widgets to disable when required
//     clone_f_register_widgets(vec![btn.clone().as_base_widget(), label.clone().as_base_widget()]);
//
//     // Callback
//     let clone_env: Vec<_> = env.into_iter().map(|(k,v)| (k.to_string(), v.to_string()) ).collect();
//     let clone_stage = stage.to_owned();
//     let clone_arc_widgets = clone_arc_widgets.clone();
//     btn.set_callback(move |e|
//     {
//       let mut clone_e = e.clone();
//       let clone_env = clone_env.clone();
//       let clone_stage = clone_stage.clone();
//       let clone_arc_widgets = clone_arc_widgets.clone();
//       std::thread::spawn(move ||
//       {
//         if let Some(vec) = clone_arc_widgets.lock().ok()
//         {
//           for mut w in vec.clone().into_iter()
//           {
//             w.deactivate();
//           } // for
//         } // if
//         clone_e.deactivate();
//         let _ = dispatch_gameimage_cmd(clone_stage.as_str(), clone_env
//           .iter()
//           .map(|(k, v)| (k.as_str(), v.as_str()))
//           .collect());
//         clone_e.activate();
//         fltk::app::awake();
//         if let Some(vec) = clone_arc_widgets.lock().ok()
//         {
//           for mut w in vec.clone().into_iter()
//           {
//             w.activate();
//           } // for
//         } // if
//       });
//     });
//
//     parent = label.as_base_widget();
//
//     (btn.clone(), label.clone())
//   };
//
//   let (btn, label) = f_install_tool(data_frame_default.sep_top.clone().as_base_widget()
//     , "configure"
//     , "DXVK"
//     , "Install DXVK for directx 9/10/11"
//     , vec![("GIMG_INSTALL_DXVK", "1"), ("GIMG_WINETRICKS_CUSTOM", ""), ("GIMG_ARCH", "win64")]);
//
//   let (btn, label) = f_install_tool(label.clone().as_base_widget()
//     , "configure"
//     , "VKD3D"
//     , "Install VKD3D for directx 12"
//     , vec![("GIMG_INSTALL_VKD3D", "1"), ("GIMG_WINETRICKS_CUSTOM", ""), ("GIMG_ARCH", "win64")]);
//
//   let (mut btn, label) = f_install_tool(label.clone().as_base_widget()
//     , "configure"
//     , "Tricks"
//     , "Run a custom winetricks command"
//     , vec![("GIMG_ARCH", "win64")]);
//
//   // Push it down to move label above
//   btn.set_pos(btn.x(), btn.y() + dimm::HEIGHT_TEXT + dimm::BORDER);
//
//   let mut input_custom_cmd = Input::new(dimm::BORDER
//     , dimm::BORDER
//     , label.w()
//     , dimm::HEIGHT_BUTTON_WIDE
//     , "")
//     .left_of(&btn, dimm::BORDER);
//
//   // Check existing value
//   if let Some(env_var) = env::var("GIMG_WINETRICKS_CUSTOM").ok()
//   {
//     input_custom_cmd.set_value(&env_var);
//   } // if
//
//   // Register callback
//   input_custom_cmd.set_callback(|e|
//   {
//     env::set_var("GIMG_WINETRICKS_CUSTOM", e.value());
//   });
//
//   // Register in widgets vec
//   f_register_widgets(vec![input_custom_cmd.clone().as_base_widget()]);
//
//   let (mut btn, mut label) = f_install_tool(input_custom_cmd.clone().as_base_widget()
//     , "install"
//     , "Wine"
//     , "Install a program with wine"
//     , vec![("GIMG_ARCH", "win64")]);
//
//   // Push it down to move label above
//   btn.set_pos(btn.x(), btn.y() + dimm::HEIGHT_TEXT + dimm::BORDER);
//
//   let mut input_custom_cmd = Input::new(dimm::BORDER
//     , dimm::BORDER
//     , label.w()
//     , dimm::HEIGHT_BUTTON_WIDE
//     , "")
//     .left_of(&btn, dimm::BORDER);
//
//   // Check existing value
//   if let Some(env_var) = env::var("GIMG_WINE_INSTALL_CUSTOM").ok()
//   {
//     input_custom_cmd.set_value(&env_var);
//   } // if
//
//   // Register callback
//   input_custom_cmd.set_callback(|e|
//   {
//     let some_choice = file_chooser("Select the executable", "*.exe|*.msi", ".", false);
//     if let Some(choice) = some_choice
//     {
//       e.set_value(&choice);
//       env::set_var("GIMG_WINE_INSTALL_CUSTOM", e.value());
//     } // if
//   });
//   input_custom_cmd.set_readonly(true);
//
//   // Register in widgets vec
//   f_register_widgets(vec![input_custom_cmd.clone().as_base_widget()]);
//
//   // Set callback for previous frame
//   let clone_data_frame_default = data_frame_default.clone();
//   data_frame_default.btn_prev.clone().set_callback(move |_|
//   {
//     frame_default_reset(clone_data_frame_default.clone(), |e| { frame_info(e); });
//   });
//
//   // Set callback for next frame
//   let clone_data_frame_default = data_frame_default.clone();
//   data_frame_default.btn_next.clone().set_callback(move |_|
//   {
//     frame_default_reset(clone_data_frame_default.clone(), |e| { frame_test(e); });
//   });
//
//   group_content.end();
//   
// } // fn: frame_configure }}}
//
// // pub fn frame_test() {{{
// fn frame_test(data_frame_default : DataFrameDefault)
// {
//   let mut group_content = data_frame_default.group_content.clone();
//
//   let header = data_frame_default.header.clone().with_label("Test the Installed Application");
//
//   // Save widgets to disable GUI on button click
//   let arc_widgets : Arc<Mutex<Vec<Widget>>> = Arc::new(Mutex::new(vec![]));
//
//   let clone_arc_widgets = arc_widgets.clone();
//   let f_register_widgets = move |vec_w : Vec<Widget>|
//   {
//     // Include button & labels in arc
//     if let Some(mut lock) = clone_arc_widgets.lock().ok()
//     {
//       for w in vec_w { lock.push(w.clone()); } // for
//     } // if
//   };
//
//   // Register prev & next buttons
//   f_register_widgets(vec![
//     data_frame_default.btn_prev.clone().as_base_widget(),
//     data_frame_default.btn_next.clone().as_base_widget()]
//   );
//
//   group_content.begin();
//
//   let clone_arc_widgets = arc_widgets.clone();
//   let clone_f_register_widgets = f_register_widgets.clone();
//   let mut f_test_binary = move |mut parent: Widget
//     , stage: &str
//     , name: &str
//     , desc : &str
//     , env: Vec<(&str,&str)>|
//   {
//     // Label
//     let mut label = TextDisplay::new(dimm::BORDER
//       , dimm::BORDER
//       , dimm::WIDTH_BUTTON_WIDE
//       , dimm::HEIGHT_BUTTON_WIDE
//       , "")
//       .below_of(&parent, dimm::BORDER)
//       .with_align(Align::Inside | Align::Center);
//
//     let height_btn = ( dimm::HEIGHT_BUTTON_WIDE as f64 *1.2 ) as i32;
//
//     let mut buf = fltk::text::TextBuffer::default();
//     label.set_buffer(buf);
//     label.set_pos(parent.x(), label.y());
//     label.insert(desc);
//     label.set_frame(FrameType::BorderBox);
//     label.set_size(dimm::WIDTH - dimm::BORDER*5 - dimm::WIDTH_BUTTON_WIDE, height_btn);
//     label.set_scrollbar_size(dimm::HEIGHT_TEXT/2);
//     label.show_insert_position();
//
//     // Button
//     let mut btn = Button::new(dimm::BORDER
//       , dimm::BORDER
//       , dimm::WIDTH_BUTTON_WIDE
//       , height_btn
//       , name)
//       .right_of(&label, dimm::BORDER);
//     btn.set_color(Color::Blue);
//
//     // Register widgets to disable when required
//     clone_f_register_widgets(vec![btn.clone().as_base_widget(), label.clone().as_base_widget()]);
//
//     // Callback
//     let clone_env: Vec<_> = env.into_iter().map(|(k,v)| (k.to_string(), v.to_string()) ).collect();
//     let clone_stage = stage.to_owned();
//     let clone_arc_widgets = clone_arc_widgets.clone();
//     let clone_label = label.clone();
//     btn.set_callback(move |e|
//     {
//       let mut clone_e = e.clone();
//       let clone_env = clone_env.clone();
//       let clone_stage = clone_stage.clone();
//       let clone_arc_widgets = clone_arc_widgets.clone();
//       let clone_label = clone_label.clone();
//       std::thread::spawn(move ||
//       {
//         if let Some(vec) = clone_arc_widgets.lock().ok()
//         {
//           for mut w in vec.clone().into_iter()
//           {
//             w.deactivate();
//           } // for
//         } // if
//
//         // Set test var for test phase
//         env::set_var("GIMG_WINE_TEST_CUSTOM", clone_label.buffer().unwrap().text().as_str());
//
//         clone_e.deactivate();
//         let _ = dispatch_gameimage_cmd(clone_stage.as_str(), clone_env
//           .iter()
//           .map(|(k, v)| (k.as_str(), v.as_str()))
//           .collect());
//         clone_e.activate();
//         fltk::app::awake();
//         if let Some(vec) = clone_arc_widgets.lock().ok()
//         {
//           for mut w in vec.clone().into_iter()
//           {
//             w.activate();
//           } // for
//         } // if
//       });
//     });
//
//     parent = label.as_base_widget();
//
//     (btn.clone(), label.clone())
//   };
//
//   // Create scrollbar
//   let scrollbar = Scroll::default()
//     .with_pos(group_content.x(), group_content.y())
//     .with_size(group_content.w(), group_content.h());
//
//   // Search for files in drive_c
//   if let Some(mut str_prefix) = env::var("GIMG_DIR").ok()
//   {
//     str_prefix.to_string().push_str("/build/AppDir/app/wine/drive_c");
//
//       // .custom_filter(|dir| dir.metadata().unwrap().is_file() )
//     let search: Vec<String> = SearchBuilder::default()
//       .location(str_prefix)
//       // Exclude windows/
//       .custom_filter(|e|
//       {
//         // Requires re-fetch of env var, because this library does not support FnMut on 2.1.0
//         let mut str_prefix = env::var("GIMG_DIR").unwrap();
//         let str_windows = format!("{}/drive_c/windows/", str_prefix);
//         e.path().file_name() != Some(OsStr::new("windows"))
//       })
//       // Exclude files not ending with .exe or .msi
//       .custom_filter(|e|
//       {
//         if e.path().is_file()
//         {
//           return e.path().extension() == Some(OsStr::new("exe"))
//             || e.path().extension() == Some(OsStr::new("msi"))
//         }
//         return true;
//       })
//       .build()
//       .filter(|e| Path::new(e).is_file() )
//       .collect();
//
//     let mut parent = data_frame_default.sep_top.clone().as_base_widget();
//     for entry in search
//     {
//       // log!("Result: {}", entry);
//       let (mut btn, label) = f_test_binary(parent
//         , "test"
//         , "Run"
//         , &entry
//         , vec![("GIMG_ARCH", "win64")]);
//       parent = label.clone().as_base_widget();
//     } // for
//   } // if
//
//   scrollbar.end();
//
//   group_content.end();
//   
// } // fn: frame_test }}}
 
// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
