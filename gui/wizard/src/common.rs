use std::
{
  io::Read,
  sync::{Arc,Mutex,mpsc,OnceLock},
  path::PathBuf,
  ffi::{OsStr, OsString},
  env,
  path,
};

use fltk::
{
  app,
  prelude::*,
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
    , dimm::width() - dimm::border()*2
    , dimm::height() - dimm::border()*2
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

// pub fn image_resize() {{{
pub fn image_resize(path_out : PathBuf, path_in : PathBuf, width : u32, height : u32) -> anyhow::Result<()>
{
  let mut img = image::io::Reader::open(path_in)?.decode()?;
  img = img.resize(width, height, image::imageops::FilterType::CatmullRom);
  Ok(img.save(path_out)?)
} // }}}

// pub trait WidgetExtExtra {{{
#[allow(warnings)]
pub trait WidgetExtExtra
{
  fn with_callback<F>(&mut self, callback : F) -> Self
    where F: FnMut(&mut Self) + 'static;
  fn with_frame(&mut self, frame : fltk::enums::FrameType) -> Self;
  fn with_svg(&mut self, data : &str) -> Self;
  fn with_shared_image(&mut self, path : PathBuf) -> Self;
  fn with_focus(&mut self, use_focus : bool) -> Self;
  fn with_color(&mut self, color : fltk::enums::Color) -> Self;
  fn with_color_selected(&mut self, color : fltk::enums::Color) -> Self;
  fn with_border(&mut self, x_border : i32, y_border : i32) -> Self;
  fn right_bottom_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn top_left_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn top_center_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn bottom_center_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn bottom_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn below_center_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self;
  fn with_pos_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self;
  fn with_posx_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self;
  fn with_posy_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self;
  fn with_size_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self;
  fn with_width(&mut self, width : i32) -> Self;
  fn with_height(&mut self, height : i32) -> Self;
  fn set_width(&mut self, width : i32);
  fn set_height(&mut self, height : i32);
  fn with_width_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self;
  fn with_height_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self;
}

impl<T: WidgetExt + Clone> WidgetExtExtra for T
{
  fn with_callback<F>(&mut self, mut callback : F) -> Self
    where F: FnMut(&mut Self) + 'static
  {
    self.set_callback(move |e| callback(e));
    self.clone()
  }

  fn with_frame(&mut self, frame : fltk::enums::FrameType) -> Self
  {
    self.set_frame(frame);
    self.clone()
  }

  fn with_svg(&mut self, data : &str) -> Self
  {
    self.set_image(Some(fltk::image::SvgImage::from_data(data).unwrap()));
    self.clone()
  }

  fn with_shared_image(&mut self, path : PathBuf) -> Self
  {
    self.set_image_scaled(Some(fltk::image::SharedImage::load(path).unwrap()));
    self.clone()
  }

  fn with_focus(&mut self, use_focus : bool) -> Self
  {
    self.visible_focus(use_focus);
    self.clone()
  }

  fn with_color(&mut self, color : fltk::enums::Color) -> Self
  {
    self.set_color(color);
    self.clone()
  }

  fn with_color_selected(&mut self, color : fltk::enums::Color) -> Self
  {
    self.set_selection_color(color);
    self.clone()
  }

  fn with_border(&mut self, x_border : i32, y_border : i32) -> Self
  {
    self.set_pos(self.x() + x_border, self.y() + y_border);
    self.clone()
  }

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
        other.x()
      , other.y() + offset
    );
    self.clone()
  }

  fn top_center_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self
  {
    self.set_pos(
        other.x() + (other.w() / 2) - (self.w() / 2)
      , other.y() + offset
    );
    self.clone()
  }

  fn bottom_center_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self
  {
    self.set_pos(
        other.x() + (other.w() / 2) - (self.w() / 2)
      , other.y() + other.h() - self.h() + offset
    );
    self.clone()
  }

  fn bottom_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self
  {
    self.set_pos(self.x(), other.y() + other.h() - self.h() + offset);
    self.clone()
  }

  fn below_center_of<W: WidgetExt + Clone>(&mut self, other: &W, offset : i32) -> Self
  {
    self.set_pos(
        other.x() + (other.w() / 2) - (self.w() / 2)
      , other.y() + other.h() + offset
    );
    self.clone()
  }

  fn with_pos_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self
  {
    self.set_pos(other.x(), other.y());
    self.clone()
  }

  fn with_posx_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self
  {
    self.set_pos(other.x(), self.y());
    self.clone()
  }

  fn with_posy_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self
  {
    self.set_pos(self.x(), other.y());
    self.clone()
  }

  fn with_size_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self
  {
    self.set_size(other.w(), other.h());
    self.clone()
  }

  fn with_width(&mut self, width : i32) -> Self
  {
    self.set_size(width, self.h());
    self.clone()
  }

  fn with_height(&mut self, height : i32) -> Self
  {
    self.set_size(self.w(), height);
    self.clone()
  }

  fn set_width(&mut self, width : i32)
  {
    self.set_size(width, self.h());
  }

  fn set_height(&mut self, height : i32)
  {
    self.set_size(self.w(), height);
  }

  fn with_width_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self
  {
    self.set_size(other.w(), self.h());
    self.clone()
  }

  fn with_height_of<W: WidgetExt + Clone>(&mut self, other: &W) -> Self
  {
    self.set_size(self.w(), other.h());
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

// pub trait OsStringExt {{{
pub trait OsStringExt
{
  fn append(&mut self, val: &str) -> &mut Self;
  fn string(&self) -> String;
}

impl OsStringExt for OsString
{
  fn append(&mut self, val: &str) -> &mut Self
  {
    self.push(val);
    self
  }
  fn string(&self) -> String
  {
    self.to_string_lossy().into_owned()
  } // fn: string
}
// }}}

// pub trait PathBufExt {{{
#[allow(warnings)]
pub trait PathBufExt
{
  fn string(&self) -> String;
  fn append_extension(&self, val: &str) -> Self;
  fn prepend(&self, upper: &PathBuf) -> Self;
  fn file_name_string(&self) -> String;
}

impl PathBufExt for PathBuf
{
  fn string(&self) -> String
  {
    self.clone().to_string_lossy().into_owned()
  } // fn: string

  fn append_extension(&self, val: &str) -> Self
  {
    PathBuf::from(self.clone().into_os_string().append(val).string())
  } // fn: extend_extension

  fn prepend(&self, upper: &PathBuf) -> Self
  {
    upper.join(self)
  } // fn: prepend

  fn file_name_string(&self) -> String
  {
    self.file_name().unwrap_or_default().to_string_lossy().to_string()
  } // fn: file_name_string
}
// }}}

// pub trait VecExt {{{
pub struct VecString
{
  str_owned: Vec<String>,
} // VecString

impl VecString
{
  // Method to access the &str references of the owned strings
  pub fn as_str_slice(&self) -> Vec<&str>
  {
    self.str_owned.iter().map(|s| s.as_str()).collect()
  } // as_str_slice()
} // impl VecString

pub trait VecExt
{
  fn append_strings(&self, other: Vec<String>) -> VecString;
} // trait VecExt

impl VecExt for Vec<&str>
{
  fn append_strings(&self, other: Vec<String>) -> VecString
  {
    // Map self to String
    let mut str_owned: Vec<String> = self.iter().map(|s| s.to_string()).collect();

    // Extend self with other
    str_owned.extend(other);

    // Create VecString
    VecString { str_owned }
  }
} // impl VecExt
// }}}

// pub struct ScrollList {{{

pub struct ScrollList
{
  scroll : fltk::group::Scroll,
  opt_current : Option<fltk::widget::Widget>,
} // ScrollList

impl ScrollList
{
  pub fn new(w : i32, h : i32, x : i32, y : i32) -> Self
  {
    let mut scroll = fltk::group::Scroll::default()
      .with_size(w, h)
      .with_pos(x, y);
    scroll.set_scrollbar_size(dimm::border());

    ScrollList{scroll: scroll.clone(), opt_current: None}
  } // new()

  pub fn begin(&self)
  {
    self.scroll.begin();
  } // begin()

  pub fn end(&self)
  {
    self.scroll.end();
  } // end()

  pub fn widget_mut(&mut self) -> &mut fltk::group::Scroll
  {
    &mut self.scroll
  } // widget_mut()

  pub fn widget_ref(&self) -> &fltk::group::Scroll
  {
    &self.scroll
  } // widget_ref()

  pub fn add(&mut self, w : &mut fltk::widget::Widget, border : i32)
  {
    let (x, y) = match &self.opt_current
    {
      Some(current) => ( current.x(), current.y() + current.h() ),
      None => ( self.scroll.x(), self.scroll.y() ),
    }; // match

    w.set_pos(x, y + border);

    self.opt_current = Some(w.as_base_widget());
  } // add()

} // impl ScrollList }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
