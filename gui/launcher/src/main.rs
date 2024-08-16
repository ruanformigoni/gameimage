#![feature(let_chains, proc_macro_hygiene, stmt_expr_attributes)]

use std::env;

// Gui
use fltk::{
  app,
  app::*,
  prelude::*,
  window::Window,
  enums::{FrameType,Font},
};

use shared::svg;

mod mounts;
mod frame;
mod common;

use shared::dimm;

use fltk_theme::{ColorTheme, color_themes};

use common::Msg;

// struct: Gui {{{
#[derive(Debug)]
struct Gui
{
  app: App,
  wind: Window,
  rx : fltk::app::Receiver<Msg>,
  tx : fltk::app::Sender<Msg>,
} // struct: Gui }}}

// impl: Gui {{{
impl Gui
{

// fn: new {{{
pub fn new() -> Self
{
  let app =  app::App::default().with_scheme(app::Scheme::Gtk);
  app::set_frame_type(FrameType::BorderBox);
  app::set_font_size(dimm::height_text());
  let mut wind = Window::default()
    .with_label("GameImage")
    .with_size(dimm::width_launcher(), dimm::height_launcher())
    .center_screen();

  // Font
  if let Ok(font) = Font::load_font("/usr/share/fonts/noto/NotoSans-Regular.ttf")
  {
    Font::set_font(Font::Helvetica, &font);
    app::set_font(Font::Helvetica);
    app::set_font_size(12);
  } // if

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
    tx,
  }
} // fn: new }}}

// fn redraw() {{{
fn redraw(&mut self, msg: Msg)
{
  self.wind.clear();
  self.wind.begin();

  match Some(msg)
  {
    Some(Msg::DrawCover) =>
    {
      frame::cover::new(self.tx, 0, 0);
    }
    Some(Msg::DrawSelector) =>
    {
      frame::selector::new(self.tx, 0, 0);
    }
    Some(Msg::DrawExecutables) =>
    {
      frame::executables::new(self.tx, 0, 0);
    }
    Some(Msg::DrawEnv) =>
    {
      frame::env::new(self.tx, 0, 0);
    }
    Some(Msg::DrawMenu) =>
    {
      frame::menu::new(self.tx, 0, 0);
    }
    _ => (),
  }
  self.wind.end();
} // fn: redraw }}}

// init() {{{
fn init(&mut self)
{
  // Fetch game entries
  if let Ok(vec_entry) = mounts::mounts()
  {
    // Create initial cover frame
    self.tx.send(common::Msg::DrawCover);
    // Update env
    let data = vec_entry.first().unwrap();
    if let Ok(platform) = data.platform.as_ref()
    {
      env::set_var("GIMG_PLATFORM", platform.as_str());
    } // if
    env::set_var("GIMG_LAUNCHER_ROOT", data.path_root.to_str().unwrap_or(""));
    env::set_var("GIMG_LAUNCHER_BOOT", data.path_boot.to_str().unwrap_or(""));
    env::set_var("GIMG_LAUNCHER_IMG", data.path_icon.to_str().unwrap_or(""));
    env::set_var("GIMG_LAUNCHER_IMG_GRAYSCALE", data.path_icon_grayscale.to_str().unwrap_or(""));
  } // if
  else
  {
    frame::fail::new(dimm::width_launcher(), dimm::height_launcher(), dimm::border());
  } // else

  self.wind.make_resizable(false);
  self.wind.end();
  self.wind.show();

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
      Some(Msg::Quit) =>
      {
        app::quit();
        app::flush();
      }
      Some(value) => self.redraw(value),
      None => (),
    } // match
  } // while
} // init() }}}

} // impl: Gui }}}

// fn: theme {{{
fn theme()
{
  // Set starting theme as dark
  ColorTheme::new(color_themes::BLACK_THEME).apply(); // Start with a default dark theme
  // Adjust it a bit
  app::background(42, 46, 50);
  app::foreground(255, 255, 255);
} // }}}

// fn: main {{{
fn main()
{
  // Set theme
  theme();

  let _ = Gui::new().init();
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
