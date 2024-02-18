#![feature(proc_macro_hygiene, stmt_expr_attributes)]

use std::env;

// Gui
use fltk::{
  app,
  app::*,
  prelude::*,
  window::Window,
  enums::FrameType,
};

mod dimm;
mod scaling;
mod svg;
mod mounts;
mod frame;
mod common;
mod db;

use fltk_theme::{ColorTheme, color_themes};

use common::Msg;

// struct: Gui {{{
#[derive(Debug)]
struct Gui
{
  app: App,
  wind: Window,
  width: i32,
  height: i32,
  border: i32,
  rx : fltk::app::Receiver<Msg>,
  tx : fltk::app::Sender<Msg>,
} // struct: Gui }}}

// impl: Gui {{{
impl Gui
{

// fn: new {{{
pub fn new() -> Self
{
  let mut width : i32 = dimm::WIDTH;
  let mut height : i32 = dimm::HEIGHT;
  let mut border = dimm::BORDER;

  // Try to scale
  let factor : f32 = if let Some(factor) = scaling::factor()
  {
    factor
  }
  else
  {
    1.0
  };

  width = ( width as f32 * factor ) as i32;
  height = ( height as f32 * factor ) as i32;
  border = ( border as f32 * factor ) as i32;

  let app =  app::App::default().with_scheme(app::Scheme::Gtk);
  let mut wind = Window::default()
    .with_label("GameImage")
    .with_size(width, height)
    .center_screen();
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
    width,
    height,
    border,
    rx,
    tx,
  }
} // fn: new }}}

// fn: frame_switcher {{{
fn frame_switcher(&self)
{
  // Fetch game entries
  let vec_entry = mounts::mounts().unwrap_or(vec![]);
  if vec_entry.is_empty()
  {
    frame::fail::new(self.width, self.height, self.border);
    return;
  } // if

  // Update env
  let (path_root, path_icon, path_boot) = vec_entry.first().unwrap();
  env::set_var("GIMG_LAUNCHER_ROOT", path_root.to_str().unwrap_or(""));
  env::set_var("GIMG_LAUNCHER_IMG", path_icon.to_str().unwrap_or(""));
  env::set_var("GIMG_LAUNCHER_BOOT", path_boot.to_str().unwrap_or(""));

  // Create initial cover frame
  frame::cover::new(
      self.tx.clone()
    , self.border
    , self.width
    , self.height
    , 0
    , 0
  );
} // fn: frame_switcher }}}

} // impl: Gui }}}

// impl: Drop for Gui {{{
impl Drop for Gui
{
  fn drop(&mut self)
  {
    self.wind.make_resizable(false);
    self.wind.end();
    self.wind.show();

    let mut some_child : std::option::Option<std::process::Child> = None;
    while self.app.wait()
    {
      match self.rx.recv()
      {
        Some(Msg::DrawCover) =>
        {
          self.wind.clear();
          self.wind.begin();
          frame::cover::new(self.tx, self.border, self.width, self.height, 0, 0);
          self.wind.end();
        }
        Some(Msg::DrawSelector) =>
        {
          self.wind.clear();
          self.wind.begin();
          frame::selector::new(self.tx, self.border, self.width, self.height, 0, 0);
          self.wind.end();
        }
        Some(Msg::DrawEnv) =>
        {
          self.wind.clear();
          self.wind.begin();
          frame::env::new(self.tx, self.border, self.width, self.height, 0, 0);
          self.wind.end();
        }
        Some(Msg::DrawMenu) =>
        {
          self.wind.clear();
          self.wind.begin();
          frame::menu::new(self.tx, self.border, self.width, self.height, 0, 0);
          self.wind.end();
        }
        Some(Msg::Launch) =>
        {
          if let Ok(process) =  std::process::Command::new("sh")
            .args(["-c", "$GIMG_LAUNCHER_BOOT"])
            .spawn()
          {
            some_child = Some(process);
          } // if
        }
        Some(Msg::Quit) =>
        {
          app::quit();
          app::flush();
        }
        None => (),
      } // match
    } // while

    if let Some(ref mut child) = some_child
    {
      let _ = child.wait();
    } // if
  }
} // }}}

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

  Gui::new().frame_switcher();
} // }}}

// cmd: !BIN_WINE="/home/ruan/Experiments/test.lua" GIMG_PLATFORM=retroarch GIMG_PKG_TYPE=flatimage GIMG_LAUNCHER_NAME=prostreet GIMG_LAUNCHER_IMG=/home/ruan/Pictures/prostreet.png cargo run --release

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
