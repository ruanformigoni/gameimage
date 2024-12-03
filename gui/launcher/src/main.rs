#![feature(let_chains, proc_macro_hygiene, stmt_expr_attributes)]

// Gui
use fltk::{
  app,
  prelude::*,
  window::Window,
  enums::FrameType,
};

use shared::svg;

mod games;
mod frame;
mod common;
mod db;

use common::Msg;

use shared::dimm;
use shared::std::PathBufExt;
use fltk_theme::{ColorTheme, color_themes};
use clap::Parser;

// struct: Gui {{{
#[derive(Debug)]
struct Gui
{
  app: fltk::app::App,
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
  wind.make_resizable(true);

  shared::fltk::theme();

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
    Some(Msg::DrawCover) => frame::cover::new(self.tx),
    Some(Msg::DrawSelector) => frame::selector::new(self.tx),
    Some(Msg::DrawExecutables) => frame::menu::executables::new(self.tx),
    Some(Msg::DrawEnv) => frame::menu::environment::new(self.tx),
    Some(Msg::DrawMenu) => frame::menu::new(self.tx),
    _ => (),
  }
  self.wind.end();
} // fn: redraw }}}

// init() {{{
fn init(&mut self)
{
  let vec_games = match games::games()
  {
    Ok(vec_games) => vec_games,
    Err(_) => { frame::fail::new(); vec![] }
  }; // match

  // Fetch game entries
  if vec_games.is_empty()
  {
    frame::fail::new();
  } // if
  else
  {
    // Create initial cover frame
    self.tx.send(common::Msg::DrawCover);
    // Select the first game as the current
    games::select(vec_games.first().unwrap());
  } // else

  // Show window
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

// struct Cli {{{
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli
{
  #[arg(long, value_name = "INDEX")]
  select_index: Option<i32>,
  #[arg(long)]
  select_list: bool,
} // struct Cli }}}

// fn: main {{{
fn main() -> anyhow::Result<()>
{
  let args = Cli::parse();

  // Launch game directly
  if let Some(index) = args.select_index
  {
    match games::select_by_index(index as usize)
    {
      Ok(()) => games::launch(),
      Err(e) => { eprintln!("Could not select index '{}': '{}'", index, e); }
    } // match
    return Ok(());
  } // if
  else if args.select_list
  {
    for (index, game) in games::games()?.into_iter().enumerate()
    {
      println!("{}: {}", index, game.path_root.file_name_string());
    } // for
    return Ok(());
  } // else if

  // Set theme
  theme();

  // Start GUI
  Gui::new().init();

  Ok(())
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
