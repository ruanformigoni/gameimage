use std::cell::RefCell;
use std::rc::Rc;
use std::collections::BTreeMap;

use fltk::{
  app,
  app::{Sender,Receiver},
  prelude::*,
  app::App,
  button::Button,
  group::Group,
  window::Window,
  enums::{FrameType,Color},
};
use fltk_theme::{ColorTheme, color_themes};

type SharedPtr<T> = Rc<RefCell<T>>;

// Modules {{{
mod common;
mod scaling;
mod frame;
mod dimm;
mod download;
mod db;
// }}}

use common::Msg;


// struct: Gui {{{
#[derive(Debug)]
struct Gui
{
  app: App,
  wind: Window,
  map_yaml: SharedPtr<BTreeMap::<String,String>>,
  width: i32,
  height: i32,
  border: i32,
  rx : Receiver<Msg>,
  tx : Sender<Msg>,
} // struct: Gui }}}

// struct: FrameInstance {{{
#[derive(Debug)]
struct FrameInstance
{
  group: Group,
  buttons: Vec<Button>,
} // struct FrameInstance }}}

// impl: Gui {{{
impl Gui
{

  // fn: new {{{
  pub fn new() -> Self
  {
    let width = 500;
    let height = width;
    let border = 30;
    let app =  app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default()
      .with_label("GameImage")
      .with_size(width, height)
      .center_screen();
    let map_yaml = Rc::new(RefCell::new(BTreeMap::<String,String>::new()));

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
    if let Some(shared_image) = fltk::image::PngImage::load("/tmp/gameimage/gameimage.png").ok()
    {
      wind.set_icon(Some(shared_image));
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
      map_yaml,
      width,
      height,
      border,
      rx,
      tx
    }
  } // fn: new }}}

// fn process() {{{
fn process(&mut self, msg : Msg)
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
      frame::platform::platform(self.tx, "Select the game platform");
    }
    Msg::DrawFetch =>
    {
      frame::fetch::fetch(self.tx, "Fetch the required files");
    }
    Msg::Quit =>
    {
      app::quit();
      app::flush();
    }
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
    self.tx.send(Msg::DrawWelcome);
    while self.app.wait()
    {
      match self.rx.recv()
      {
        Some(value) => self.process(value),
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
