#![feature(let_chains)]

use fltk::{
  app,
  app::{Sender,Receiver},
  prelude::*,
  app::App,
  window::Window,
  enums::{FrameType,Color},
};
use fltk_theme::{ColorTheme, color_themes};

// Modules {{{
mod common;
mod scaling;
mod frame;
mod dimm;
mod download;
mod db;
mod svg;

use common::Msg;
// }}}

// struct: Gui {{{
#[derive(Debug)]
struct Gui
{
  app       : App,
  wind_main : Window,
  wind_log  : Window,
  tx        : Sender<Msg>,
  rx        : Receiver<Msg>,
} // struct: Gui }}}

// impl: Gui {{{
impl Gui
{

  // fn: new {{{
  pub fn new() -> Self
  {
    let app =  app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind_main = Window::default()
      .with_label("GameImage")
      .with_size(dimm::width(), dimm::height())
      .center_screen();
    wind_main.begin();
    wind_main.end();

    let mut wind_log = Window::default()
      .with_label("Logger")
      .with_size(dimm::width(), dimm::height())
      .left_of(&wind_main, 0);
    wind_log.begin();
    wind_log.end();

    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();
    app::set_font_size(dimm::height_text());
    app::set_color(Color::White, 230, 230, 230);
    app::set_color(Color::Blue, 55, 113, 200);
    app::set_frame_color(Color::White);
    app::foreground(230,230,230);
    let color = Color::from_hex_str("#5294e2").unwrap().to_rgb();
    app::set_selection_color(color.0, color.1, color.2);
    app::set_frame_type(FrameType::BorderBox);

    // Window icon
    if let Some(image) = fltk::image::SvgImage::from_data(svg::ICON_GAMEIMAGE).ok()
    {
      wind_main.set_icon(Some(image.clone()));
      wind_log.set_icon(Some(image));
    } // if
    else
    {
      log!("Failed to load icon image");
    } // else

    let (tx, rx) = fltk::app::channel();

    Gui
    {
      app,
      wind_main,
      wind_log,
      tx,
      rx,
    }
  } // fn: new }}}

// fn redraw() {{{
fn redraw(&mut self, msg : Msg)
{
  self.wind_main.clear();
  self.wind_main.begin();

  match msg
  {
    //
    // Common
    //
    Msg::DrawWelcome =>
    {
      frame::welcome::welcome(self.tx, "Welcome to GameImage");
    }
    Msg::DrawPlatform =>
    {
      frame::platform::platform(self.tx, "Select the Game Platform");
    }
    Msg::DrawFetch =>
    {
      frame::fetch::fetch(self.tx, "Fetch the Required Files");
    }
    Msg::DrawCreator =>
    {
      frame::creator::creator(self.tx, "Create Packages to Include in the Image");
    }
    Msg::DrawDesktop =>
    {
      frame::desktop::desktop(self.tx, "Select the Image Desktop Icon");
    }
    Msg::DrawName =>
    {
      frame::name::name(self.tx, "Select the File Name");
    }

    //
    // Wine
    //
    Msg::DrawWineName =>
    {
      frame::wizard::wine::name(self.tx, "Select the Application Name");
    }
    Msg::DrawWineIcon =>
    {
      frame::wizard::wine::icon(self.tx, "Select the Application Icon");
    }
    Msg::DrawWineConfigure =>
    {
      frame::wizard::wine::configure(self.tx, "Configure Wine");
    }
    Msg::DrawWineRom =>
    {
      frame::wizard::wine::rom(self.tx, "Install/Test the Application(s)");
    }
    Msg::DrawWineDefault =>
    {
      frame::wizard::wine::default(self.tx, "Select the Default Executable");
    }
    Msg::DrawWineCompress =>
    {
      frame::wizard::wine::compress(self.tx, "Compress the Created Package");
    }

    //
    // Retroarch
    //
    Msg::DrawRetroarchName =>
    {
      frame::wizard::retroarch::name(self.tx, "Select the Application Name");
    }
    Msg::DrawRetroarchIcon =>
    {
      frame::wizard::retroarch::icon(self.tx, "Select the Application Icon");
    }
    Msg::DrawRetroarchRom =>
    {
      frame::wizard::retroarch::rom(self.tx, "Install the Rom File(s)");
    }
    Msg::DrawRetroarchCore =>
    {
      frame::wizard::retroarch::core(self.tx, "Install the Core File(s)");
    }
    Msg::DrawRetroarchBios =>
    {
      frame::wizard::retroarch::bios(self.tx, "Install the Bios File(s)");
    }
    Msg::DrawRetroarchTest =>
    {
      frame::wizard::retroarch::test(self.tx, "Test the Created Package");
    }
    Msg::DrawRetroarchCompress =>
    {
      frame::wizard::retroarch::compress(self.tx, "Compress the Created Package");
    }

    //
    // Pcsx2
    //
    Msg::DrawPcsx2Name =>
    {
      frame::wizard::pcsx2::name(self.tx, "Select the Application Name");
    }
    Msg::DrawPcsx2Icon =>
    {
      frame::wizard::pcsx2::icon(self.tx, "Select the Application Icon");
    }
    Msg::DrawPcsx2Rom =>
    {
      frame::wizard::pcsx2::rom(self.tx, "Install the Rom File(s)");
    }
    Msg::DrawPcsx2Bios =>
    {
      frame::wizard::pcsx2::bios(self.tx, "Install the Bios File(s)");
    }
    Msg::DrawPcsx2Test =>
    {
      frame::wizard::pcsx2::test(self.tx, "Test the Created Package");
    }
    Msg::DrawPcsx2Compress =>
    {
      frame::wizard::pcsx2::compress(self.tx, "Compress the Created Package");
    }

    //
    // Rpcs3
    //
    Msg::DrawRpcs3Name =>
    {
      frame::wizard::rpcs3::name(self.tx, "Select the Application Name");
    }
    Msg::DrawRpcs3Icon =>
    {
      frame::wizard::rpcs3::icon(self.tx, "Select the Application Icon");
    }
    Msg::DrawRpcs3Rom =>
    {
      frame::wizard::rpcs3::rom(self.tx, "Install the Rom Directory(ies)");
    }
    Msg::DrawRpcs3Bios =>
    {
      frame::wizard::rpcs3::bios(self.tx, "Install the Bios and DLC Files");
    }
    Msg::DrawRpcs3Test =>
    {
      frame::wizard::rpcs3::test(self.tx, "Test the Created Package");
    }
    Msg::DrawRpcs3Compress =>
    {
      frame::wizard::rpcs3::compress(self.tx, "Compress the Created Package");
    }

    //
    // Yuzu
    //
    Msg::DrawYuzuName =>
    {
      frame::wizard::yuzu::name(self.tx, "Select the Application Name");
    }
    Msg::DrawYuzuIcon =>
    {
      frame::wizard::yuzu::icon(self.tx, "Select the Application Icon");
    }
    Msg::DrawYuzuRom =>
    {
      frame::wizard::yuzu::rom(self.tx, "Install the Rom File(s)");
    }
    Msg::DrawYuzuBios =>
    {
      frame::wizard::yuzu::bios(self.tx, "Install the firmware Files");
    }
    Msg::DrawYuzuKeys =>
    {
      frame::wizard::yuzu::keys(self.tx, "Install the Decryption Keys");
    }
    Msg::DrawYuzuTest =>
    {
      frame::wizard::yuzu::test(self.tx, "Test the Created Package");
    }
    Msg::DrawYuzuCompress =>
    {
      frame::wizard::yuzu::compress(self.tx, "Compress the Created Package");
    }

    //
    // Quit
    //
    Msg::Quit =>
    {
      app::quit();
      app::flush();
    }
    _ => (),
  } // match

  self.wind_main.end();
  app::redraw();
  app::flush();
  app::awake();
} // }}}

} // }}}

// impl: Drop for Gui {{{
impl Drop for Gui
{
  fn drop(&mut self)
  {
    // Create & show logging window
    self.wind_log.begin();
    log!("Initialized logging!");
    self.wind_log.end();
    self.wind_log.show();

    // Show main window
    self.wind_main.show();

    // Set log window to the left of the main window
    self.wind_log.set_pos(self.wind_main.x() - self.wind_main.w(), self.wind_main.y());

    self.tx.send(Msg::DrawRetroarchIcon);
    // self.tx.send(Msg::DrawWelcome);
    while self.app.wait()
    {
      match self.rx.recv()
      {
        Some(common::Msg::WindActivate) =>
        {
          let children = self.wind_main.children();
          for i in 0..children {
            let mut widget = self.wind_main.child(i).unwrap();
            widget.activate();
          }
          app::flush();
          app::awake();
        }
        Some(common::Msg::WindDeactivate) =>
        {
          let children = self.wind_main.children();
          for i in 0..children
          {
            let mut widget = self.wind_main.child(i).unwrap();
            widget.deactivate();
          }
          app::flush();
          app::awake();
        }
        Some(value) => self.redraw(value),
        None => (),
      } // match
    } // while
  }
} // }}}

// fn: main {{{
fn main() {
  let _ = Gui::new();
} // fn: main }}}

// cmd: !GIMG_PKG_TYPE=flatimage cargo run --release

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
