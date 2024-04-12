use std::
{
  io::Read,
  sync::{Arc,Mutex,mpsc,OnceLock},
  env,
  path,
};

use fltk::app;

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
  static TX : OnceLock<Mutex<frame::term::Term>> = OnceLock::new();

  let lock = TX.get_or_init(|| Mutex::new(frame::term::Term::new(dimm::border()
    , dimm::width_wizard() - dimm::border()*2
    , dimm::height_wizard() - dimm::border()*2
    , dimm::border()
    , dimm::border()
  )));

  let clone_value = value.to_owned();
  std::thread::spawn(move ||
  {
    if let Ok(term) = lock.try_lock()
    {
      term.append(&clone_value);
    } // if
  });
} // impl_log() }}}

// macro_rules log! {{{
#[macro_export]
macro_rules! log
{
  ($($arg:tt)*) =>
  {
    {
      let mut output = format!($($arg)*);
      output.push('\n');
      common::impl_log(output.as_str());
    }
  }
}
// }}}

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

// pub fn gameimage_async() {{{
pub fn gameimage_async(args : Vec<&str>) -> anyhow::Result<mpsc::Receiver<i32>>
{
  dir_build()?;

  let path_binary_gameimage = path::PathBuf::from(env::var("GIMG_BACKEND")?);

  let handle = std::process::Command::new(path_binary_gameimage)
    .env_remove("LD_PRELOAD")
    .env("FIM_FIFO", "0")
    .stderr(std::process::Stdio::inherit())
    .stdout(std::process::Stdio::piped())
    .args(args)
    .spawn()?;


  // Create arc reader for stdout
  let arc_handle = Arc::new(Mutex::new(handle));

  // Clone process handle
  let clone_arc_handle = arc_handle.clone();

  // Create t/r
  let (tx, rx) = mpsc::channel();
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
        impl_log("Could not acquire lock");
        let _ = tx.send(1);
        return;
      }; // else

    // Create buf
    let mut buf = vec![0; 4096];

    // Use buf to write buf to stdout & stderr
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
    }

    if let Ok(status) = lock.wait() && let Some(code) = status.code()
    {
      let _ = tx.send(code);
    }
    else
    {
      let _ = tx.send(1);
    } // else

    app::awake();
  });

  Ok(rx)
} // fn: gameimage_async }}}

// pub fn gameimage_sync() {{{
pub fn gameimage_sync(args : Vec<&str>) -> i32
{
  if let Ok(rx) = gameimage_async(args)
  && let Ok(code) = rx.recv()
  {
    return code;
  } // if

  impl_log("Could not retrieve exit code from backend");

  1
} // fn: gameimage_sync }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
