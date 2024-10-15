use std::
{
  sync::{Mutex,OnceLock},
  env,
  path,
};

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

  DrawLinuxName,
  DrawLinuxIcon,
  DrawLinuxRom,
  DrawLinuxDefault,
  DrawLinuxTest,
  DrawLinuxCompress,

  DrawWineName,
  DrawWineIcon,
  DrawWineConfigure,
  DrawWineEnvironment,
  DrawWineRom,
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

  DrawRpcs3Name,
  DrawRpcs3Icon,
  DrawRpcs3Rom,
  DrawRpcs3Bios,
  DrawRpcs3Test,
  DrawRpcs3Compress,

  DrawRyujinxName,
  DrawRyujinxIcon,
  DrawRyujinxRom,
  DrawRyujinxBios,
  DrawRyujinxKeys,
  DrawRyujinxTest,
  DrawRyujinxCompress,

  DrawFinish,

  WindActivate,
  WindDeactivate,

  Quit,
}

// Constants {{{
pub const STR_DESC_LINUX : &str = "GameImage enhances the Linux gaming experience by offering unparalleled portability and compatibility. In Linux-native games, the tool repackages your favorite titles, ensuring they run flawlessly across a myriad of Linux distributions. Whether you’re facing library incompatibilities or missing dependencies, GameImage bridges the gap, so you can enjoy your games without the hassle. Embrace the freedom to play your way, on any Linux environment, with GameImage.";

pub const STR_DESC_WINE : &str = "Wine is a program which allows running Microsoft Windows programs (including DOS, Windows 3.x, Win32, and Win64 executables) on Unix. It consists of a program loader which loads and executes a Microsoft Windows binary, and a library (called Winelib) that implements Windows API calls using their Unix, X11 or Mac equivalents.  The library may also be used for porting Windows code into native Unix executables. Wine is free software, released under the GNU LGPL.";

pub const STR_DESC_RETR : &str = "RetroArch is a frontend for emulators, game engines and media players. It enables you to run classic games on a wide range of computers and consoles through its slick graphical interface. Settings are also unified so configuration is done once and for all. In addition to this, you are able to run original game discs (CDs) from RetroArch. RetroArch has advanced features like shaders, netplay, rewinding, next-frame response times, runahead, machine translation, blind accessibility features, and more!";

pub const STR_DESC_PCSX2 : &str = "Being almost as old as the console it is emulating, PCSX2 not only has a lot of history behind it, but a continually evolving future. PCSX2 is a free and open-source PlayStation 2 (PS2) emulator. Its purpose is to emulate the PS2's hardware, using a combination of MIPS CPU Interpreters, Recompilers and a Virtual Machine which manages hardware states and PS2 system memory. The project has been running for almost 20 years. Past versions could only run a few public domain game demos, but newer versions can run most games at full speed, including popular titles such as Final Fantasy X and Devil May Cry 3. A significant majority of the official PS2 library is considered playable or perfect, with the remainder at least making it to the menus. PCSX2 allows you to play PS2 games on your PC, with many additional features and benefits. A few of those benefits include: custom resolutions and upscaling, virtual and sharable memory cards, save-states, patching system, internal recorder to achieve lossless quality at full speed.";

pub const STR_DESC_RPCS3 : &str = "RPCS3 is a multi-platform open-source Sony PlayStation 3 emulator and debugger written in C++ for Windows, Linux, macOS and FreeBSD. The purpose of the project is to completely and accurately emulate the Sony PlayStation 3 Computer Entertainment System in its entirety with the power of open-source community and reverse engineering. Our goal is to preserve the legacy of the PlayStation 3 hardware and its vast library by bringing it and its exclusives to the PC platform. We want to achieve this by targeting and supporting multiple operating systems as well as being compatible with a wide range of computer hardware with realistic requirements.";

#[allow(dead_code)]
pub const STR_DESC_RYUJINX : &str = "Ryujinx is an open-source Nintendo Switch emulator created by gdkchan and written in C#. This emulator aims at providing excellent accuracy and performance, a user-friendly interface, and consistent builds.";
// }}}

// impl_log() {{{
pub fn impl_log(value : &str)
{
  static TX: OnceLock<std::sync::mpsc::Sender<String>> = OnceLock::new();

  // Initialize the logging channel and logger thread the first time impl_log is called
  let sender = TX.get_or_init(||
  {
    // Create a channel for log messages
    let (tx, rx) = std::sync::mpsc::channel::<String>();

    // Create a new terminal to append logs (shared between threads)
    let term = std::sync::Arc::new(Mutex::new(frame::term::Term::new(
      dimm::border(),
      dimm::width_wizard() - dimm::border() * 2,
      dimm::height_wizard() - dimm::border() * 2,
      dimm::border(),
      dimm::border(),
    )));

    // Spawn a singleton logger thread that will consume messages and append them to the terminal
    let term_clone = std::sync::Arc::clone(&term);
    std::thread::spawn(move ||
    {
      while let Ok(log_message) = rx.recv()
      {
        // Lock the terminal and append the log message
        if let Ok(term) = term_clone.try_lock()
        {
          term.append(&log_message);
        } // if
        else
        {
          eprintln!("Failed to acquire terminal lock");
        } // else
      }
    }); // std::thread

    // Return the sender for future calls to impl_log
    tx
  });

  // Send the log message to the logger thread
  if let Err(e) = sender.send(value.to_string()) {
      println!("Failed to send log message: {}", e);
  }
} // impl_log() }}}

// macro_rules log! {{{
#[macro_export]
macro_rules! log
{
  ($($arg:tt)*) =>
  {
    {
      let output = format!($($arg)*);
      common::impl_log(output.as_str());
      eprintln!("{}", output);
    }
  }
}
// }}}

// macro_rules log_err! {{{
#[macro_export]
macro_rules! log_err
{
  ($result:expr) =>
  {
    match $result
    {
      Ok(()) => (),
      Err(e) => log!("{:?}", e),
    }
  }
}
// }}}

// macro_rules log_return_err! {{{
#[macro_export]
macro_rules! log_return_err
{
  ($($arg:tt)*) => { { log!($($arg)*); return Err(ah!($($arg)*)); } }
}
// }}}

// macro_rules log_return_void! {{{
#[macro_export]
macro_rules! log_return_void
{
  ($($arg:tt)*) => { { log!($($arg)*); return; } }
}
// }}}

// fn: log_fd() {{{
pub fn log_fd<T: std::io::Read, F: FnMut(std::sync::mpsc::Sender<String>, String)>(mut fd: T
  , tx: std::sync::mpsc::Sender<String>
  , mut f_callback: F) -> impl FnMut() -> ()
{
  return move ||
  {
    // Use buf to write buf to stdout
    loop
    {
      let mut buf = vec![0; 4096];
      let bytes_read = match fd.read(&mut buf)
      {
        Ok(bytes_read) => bytes_read,
        Err(_) => break,
      }; // match
      if bytes_read == 0 { break; }
      let mut output = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
      output = output.trim().to_string();
      f_callback(tx.clone(),output);
    } // loop
  }; // return
} // fn: log_fd() }}}

// pub fn wizard_by_platform() {{{
pub fn wizard_by_platform() -> anyhow::Result<Msg>
{

  match env::var("GIMG_PLATFORM")?.to_lowercase().as_str()
  {
    "linux"      => Ok(Msg::DrawLinuxName),
    "wine"      => Ok(Msg::DrawWineName),
    "retroarch" => Ok(Msg::DrawRetroarchName),
    "pcsx2"     => Ok(Msg::DrawPcsx2Name),
    "rpcs3"     => Ok(Msg::DrawRpcs3Name),
    "ryujinx"      => Ok(Msg::DrawRyujinxName),
    _           => Err(ah!("Unrecognized platform")),
  } // match

} // fn: wizard_by_platform }}}

// pub fn dir_build() {{{
pub fn dir_build() -> anyhow::Result<()>
{
  Ok(env::set_current_dir(path::PathBuf::from(env::var("GIMG_DIR")?))?)
} // fn: dir_build }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
