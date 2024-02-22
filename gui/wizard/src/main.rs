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
  app: App,
  wind: Window,
  rx : Receiver<Msg>,
  tx : Sender<Msg>,
} // struct: Gui }}}

// impl: Gui {{{
impl Gui
{

  // fn: new {{{
  pub fn new() -> Self
  {
    let app =  app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default()
      .with_label("GameImage")
      .with_size(dimm::width(), dimm::height())
      .center_screen();

    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();
    app::set_font_size(dimm::HEIGHT_TEXT);
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
      wind.set_icon(Some(image));
    } // if
    else
    {
      println!("Failed to load icon image");
    } // else

    let (tx, rx) = fltk::app::channel();

    Gui
    {
      app,
      wind,
      rx,
      tx
    }
  } // fn: new }}}

// fn redraw() {{{
fn redraw(&mut self, msg : Msg)
{
  self.wind.clear();
  self.wind.begin();

  match msg
  {
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
      frame::wizard::retroarch::test(self.tx, "Test the created package");
    }
    Msg::DrawRetroarchCompress =>
    {
      frame::wizard::retroarch::compress(self.tx, "Compress the created package");
    }
    Msg::Quit =>
    {
      app::quit();
      app::flush();
    }
    _ => (),
  } // match

  self.wind.end();
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
    self.wind.show();
    self.tx.send(Msg::DrawRetroarchTest);
    while self.app.wait()
    {
      match self.rx.recv()
      {
        Some(common::Msg::WindActivate) =>
        {
          let children = self.wind.children();
          for i in 0..children {
            let mut widget = self.wind.child(i).unwrap();
            widget.activate();
          }
          app::flush();
          app::awake();
        }
        Some(common::Msg::WindDeactivate) =>
        {
          let children = self.wind.children();
          for i in 0..children
          {
            let mut widget = self.wind.child(i).unwrap();
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
