use std::env;
use std::path::PathBuf;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use walkdir::WalkDir;
use closure::closure;
use fltk::{
  app,
  app::App,
  button::Button,
  dialog::dir_chooser,
  group::{Group, PackType},
  input::{Input,FileInput},
  output::Output,
  menu::MenuButton,
  prelude::{ImageExt, InputExt, GroupExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
  window::Window,
  enums::{Align,FrameType,Color},
  frame::Frame,
  image::SharedImage,
};
use fltk_theme::{ColorTheme, color_themes};

type SharedPtr<T> = Rc<RefCell<T>>;

// Modules {{{
mod dimm;
mod common;
mod wine;
mod download;
// }}}

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

    Gui
    {
      app,
      wind,
      map_yaml,
      width,
      height,
      border,
    }
  } // fn: new }}}

} // }}}

// impl: Drop for Gui {{{
impl Drop for Gui
{
  fn drop(&mut self)
  {
    self.app.run().unwrap();
    println!("Platform: {:?}", env::var("GIMG_PLATFORM"));
  }
} // }}}

// fn: frame_switcher {{{
fn frame_switcher()
{
  // Init GUI
  let arc_gui_clone = Arc::new(Mutex::new(Gui::new()));
  let mut gui = arc_gui_clone.lock().unwrap();
  let mut clone_wind = gui.wind.clone();
  let width_clone = gui.width;
  let height_clone = gui.height;
  let border_clone = gui.border;

  let data_frame_default = common::frame_default(gui.app.clone(), clone_wind.clone(), "");
  let mut btn_prev = data_frame_default.btn_prev.clone();
  let mut btn_next = data_frame_default.btn_next.clone();
  let txt_header = data_frame_default.header.clone();
  let group_content = data_frame_default.group_content.clone();

  common::frame_welcome(data_frame_default.clone());

  let mut clone_wind = gui.wind.clone();
  clone_wind.make_resizable(false);
  clone_wind.end();
  clone_wind.show();

} // fn: frame_switcher }}}

// fn: main {{{
fn main() {
  // Tell GameImage that GUI is used
  env::set_var("GIMG_GUI", "Yes");

  frame_switcher();

} // fn: main }}}

// cmd: !GIMG_PKG_TYPE=flatimage cargo run --release

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
