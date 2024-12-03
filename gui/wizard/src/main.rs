#![feature(let_chains,proc_macro_hygiene, stmt_expr_attributes)]
#![allow(special_module_name)]

use std::sync::{Mutex,LazyLock};

use fltk::{
  app,
  app::{Sender,Receiver},
  prelude::*,
  app::App,
  window::Window,
  dialog,
  group::Group,
  button::Button,
  output::Output,
  frame::Frame,
  enums::{FrameType,Color,Font},
};
use fltk_theme::{ColorTheme, color_themes};

use shared::dimm;
use shared::svg;
use shared::fltk::SenderExt;

// Modules {{{
mod common;
mod frame;
mod lib;
mod db;
mod wizard;
mod gameimage;
// }}}

use common::Msg;

static GUI: LazyLock<Mutex<Gui>> = LazyLock::new(|| Mutex::new(Gui::new()));

pub struct Ui
{
  title: Frame,
  group: Group,
  btn_prev: Button,
  btn_next: Button,
  status: Output,
}

// struct: Gui {{{
#[derive(Debug, Clone)]
struct Gui
{
  app       : App,
  wind_main : Window,
  tx        : Sender<Msg>,
  rx        : Receiver<Msg>,
  ui        : fn(&str) -> Ui
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
      .with_size(dimm::width_wizard(), dimm::height_wizard())
      .center_screen();
    wind_main.end();

    // Apply theme
    shared::fltk::theme();

    // Window icon
    if let Some(image) = fltk::image::SvgImage::from_data(svg::ICON_GAMEIMAGE).ok()
    {
      wind_main.set_icon(Some(image.clone()));
    } // if
    else
    {
      log!("Failed to load icon image");
    } // else

    let (tx, rx) = fltk::app::channel();

    let ui = |title: &str|
    {
      let mut ui =  Ui
      {
        title    : fltk::app::widget_from_id("header_title").unwrap(),
        group    : fltk::app::widget_from_id("content").unwrap(),
        btn_next : fltk::app::widget_from_id("footer_next").unwrap(),
        btn_prev : fltk::app::widget_from_id("footer_prev").unwrap(),
        status   : fltk::app::widget_from_id("footer_status").unwrap(),
      };
      ui.title.set_label(title);
      ui.btn_prev.set_callback(|_|{});
      ui.btn_next.set_callback(|_|{});
      ui.btn_next.set_label("Next");
      ui.btn_prev.set_label("Prev");
      ui.btn_next.show();
      ui.btn_prev.show();
      ui.btn_next.show();
      ui
    };

    Gui { app, wind_main, tx, rx, ui }
  } // fn: new }}}

// fn redraw() {{{
fn redraw(&mut self, tx: Sender<common::Msg>, msg : Msg)
{
  let mut content: Group = fltk::app::widget_from_id("content").unwrap();
  content.clear();
  content.begin();

  match msg
  {
    // Common
    Msg::DrawFinish => frame::finish::finish(self.tx, "Thank You for Using GameImage!"),
    Msg::DrawWelcome => frame::welcome::welcome(self.tx, "Welcome to GameImage"),
    Msg::DrawPlatform => frame::platform::platform(self.tx, "Select a Platform"),
    Msg::DrawCreator => frame::creator::creator(self.tx, "Create Packages to Include in the Image"),
    Msg::DrawDesktop => frame::desktop::desktop(self.tx, "Select the Desktop Icon"),
    // Linux
    Msg::DrawLinuxName => wizard::linux::name(self.tx, "Select the Application Name"),
    Msg::DrawLinuxIcon => wizard::linux::icon(self.tx, "Select the Application Icon"),
    Msg::DrawLinuxMethod => wizard::linux::method(self.tx, "Select How to Install the Application"),
    Msg::DrawLinuxRom => wizard::linux::rom(self.tx, "Install the Application"),
    Msg::DrawLinuxDefault(is_update) => wizard::linux::default(self.tx, "Select the Main Binary", is_update),
    Msg::DrawLinuxCompress => wizard::linux::compress(self.tx, "Compress the Created Package"),
    // Wine
    Msg::DrawWineName => wizard::wine::name(self.tx, "Select the Application Name"),
    Msg::DrawWineIcon => wizard::wine::icon(self.tx, "Select the Application Icon"),
    Msg::DrawWineConfigure => wizard::wine::configure(self.tx, "Configure Wine"),
    Msg::DrawWineTricks => wizard::wine::winetricks(self.tx, "Install Libraries"),
    Msg::DrawWineEnvironment => wizard::wine::environment(self.tx, "Configure the Environment"),
    Msg::DrawWineRom => wizard::wine::rom(self.tx, "Install/Test the Application(s)"),
    Msg::DrawWineCompress => wizard::wine::compress(self.tx, "Compress the Created Package"),
    // Retroarch
    Msg::DrawRetroarchName => wizard::retroarch::name(self.tx, "Select the Application Name"),
    Msg::DrawRetroarchIcon => wizard::retroarch::icon(self.tx, "Select the Application Icon"),
    Msg::DrawRetroarchRom => wizard::retroarch::rom(self.tx, "Install the Rom File(s)"),
    Msg::DrawRetroarchCore => wizard::retroarch::core(self.tx, "Install the Core File(s)"),
    Msg::DrawRetroarchBios => wizard::retroarch::bios(self.tx, "Install the Bios File(s)"),
    Msg::DrawRetroarchTest => wizard::retroarch::test(self.tx, "Test the Created Package"),
    Msg::DrawRetroarchCompress => wizard::retroarch::compress(self.tx, "Compress the Created Package"),
    // Pcsx2
    Msg::DrawPcsx2Name => wizard::pcsx2::name(self.tx, "Select the Application Name"),
    Msg::DrawPcsx2Icon => wizard::pcsx2::icon(self.tx, "Select the Application Icon"),
    Msg::DrawPcsx2Rom => wizard::pcsx2::rom(self.tx, "Install the Rom File(s)"),
    Msg::DrawPcsx2Bios => wizard::pcsx2::bios(self.tx, "Install the Bios File(s)"),
    Msg::DrawPcsx2Test => wizard::pcsx2::test(self.tx, "Test the Created Package"),
    Msg::DrawPcsx2Compress => wizard::pcsx2::compress(self.tx, "Compress the Created Package"),
    // Rpcs3
    Msg::DrawRpcs3Name => wizard::rpcs3::name(self.tx, "Select the Application Name"),
    Msg::DrawRpcs3Icon => wizard::rpcs3::icon(self.tx, "Select the Application Icon"),
    Msg::DrawRpcs3Rom => wizard::rpcs3::rom(self.tx, "Install the Rom Directory(ies)"),
    Msg::DrawRpcs3Bios => wizard::rpcs3::bios(self.tx, "Install the Bios and DLC Files"),
    Msg::DrawRpcs3Test => wizard::rpcs3::test(self.tx, "Test the Created Package"),
    Msg::DrawRpcs3Compress => wizard::rpcs3::compress(self.tx, "Compress the Created Package"),
    // Quit
    Msg::Quit =>
    {
      app::quit();
      app::flush();
    }
    _ => (),
  } // match

  content.end();
  content.redraw();
  app::redraw();
  app::flush();
  app::awake();
  tx.send_awake(Msg::WindActivate);
} // }}}

// init() {{{
fn init(&mut self)
{
  // Ask the user he really wants to exit gameimage
  let clone_tx = self.tx.clone();
  let f_callback_close = move |_: &mut fltk::window::DoubleWindow|
  {
    if fltk::app::event() == fltk::enums::Event::Close
      && dialog::choice2_default("Exit GameImage?", "No", "Yes", "") == Some(1)
    {
      clone_tx.send_awake(common::Msg::Quit);
    } // if
  };

  self.wind_main.set_callback(f_callback_close.clone());

  self.wind_main.begin();
  frame::common::frame_header("Header");
  frame::common::frame_footer();
  self.wind_main.end();
  self.wind_main.show();

  let clone_tx = self.tx.clone();
  std::thread::spawn(move ||
  {
    loop
    {
      clone_tx.send(common::Msg::WindUpdate);
      std::thread::sleep(std::time::Duration::from_millis(50));
    } // while
  });

  self.tx.send_awake(Msg::DrawWelcome);
  while self.app.wait()
  {
    // Handle messages
    match self.rx.recv()
    {
      Some(common::Msg::WindUpdate) =>
      {
        app::flush();
        app::awake();
      }
      Some(common::Msg::WindActivate) =>
      {
        ( 0..self.wind_main.children() )
          .into_iter()
          .for_each(|e| { self.wind_main.child(e).unwrap().clone().activate() });
        app::flush();
        app::awake();
      }
      Some(common::Msg::WindDeactivate) =>
      {
        ( 0..self.wind_main.children() )
          .into_iter()
          .for_each(|e| { self.wind_main.child(e).unwrap().clone().deactivate() });
        app::flush();
        app::awake();
      }
      Some(value) => self.redraw(self.tx, value),
      None => (),
    } // match
  } // while
} // fn: init }}}

} // }}}

// fn: main {{{
fn main() {
  let mut gui = GUI.lock().unwrap().clone();
  gui.init();
} // fn: main }}}

// cmd: !GIMG_PKG_TYPE=flatimage cargo run --release

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
