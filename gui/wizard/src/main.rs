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

// struct: Gui {{{
#[derive(Debug, Clone)]
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
      .with_size(dimm::width_wizard(), dimm::height_wizard())
      .center_screen();
    wind_main.begin();
    wind_main.end();

    if let Ok(font) = Font::load_font("/usr/share/fonts/noto/NotoSans-Regular.ttf")
    {
      Font::set_font(Font::Helvetica, &font);
      app::set_font(Font::Helvetica);
      app::set_font_size(12);
    } // if

    let mut wind_log = Window::default()
      .with_label("Logger")
      .with_size(dimm::width_wizard(), dimm::height_wizard())
      .left_of(&wind_main, 0);
    wind_log.begin();
    wind_log.end();

    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();
    app::set_visible_focus(false);
    app::set_font_size(dimm::height_text());
    let set_color = |c: Color, hex: &str|
    {
      let r = Color::from_hex_str(hex).unwrap();
      let r = Color::darker(&r).to_rgb();
      app::set_color(c, r.0, r.1, r.2);
    };
    let str_black = "#35353A";
    let str_white = "#eeeeee";
    set_color(Color::White       , str_white);
    set_color(Color::Black       , str_black);
    set_color(Color::ForeGround  , str_white);
    set_color(Color::Foreground  , str_white);
    set_color(Color::BackGround  , str_black);
    set_color(Color::Background  , str_black);
    set_color(Color::BackGround2 , &Color::from_hex_str(str_black).unwrap().darker().to_hex_str());
    set_color(Color::Background2 , &Color::from_hex_str(str_black).unwrap().darker().to_hex_str());
    set_color(Color::Red         , "#F05090");
    set_color(Color::Blue        , "#00A0F0");
    set_color(Color::Green       , "#60F080");
    set_color(Color::Yellow      , "#F0F080");
    set_color(Color::Magenta     , "#D080F0");
    set_color(Color::Cyan        , "#70D0F0");
    set_color(Color::DarkRed     , &Color::darker(&Color::DarkRed).to_hex_str());
    set_color(Color::DarkBlue    , &Color::darker(&Color::DarkBlue).to_hex_str());
    set_color(Color::DarkGreen   , &Color::darker(&Color::DarkGreen).to_hex_str());
    set_color(Color::DarkYellow  , &Color::darker(&Color::DarkYellow).to_hex_str());
    set_color(Color::DarkMagenta , &Color::darker(&Color::DarkMagenta).to_hex_str());
    set_color(Color::DarkCyan    , &Color::darker(&Color::DarkCyan).to_hex_str());
    app::set_frame_color(Color::White);
    app::foreground(230,230,230);
    let color = Color::from_hex_str("#5294e2").unwrap().to_rgb();
    app::set_selection_color(color.0, color.1, color.2);
    app::set_frame_type(FrameType::BorderBox);
    fltk_theme::WidgetScheme::new(fltk_theme::SchemeType::Clean).apply();

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

    Gui { app, wind_main, wind_log, tx, rx, }
  } // fn: new }}}

// fn redraw() {{{
fn redraw(&mut self, msg : Msg)
{
  self.wind_main.clear();
  self.wind_main.begin();

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

  self.wind_main.end();
  app::redraw();
  app::flush();
  app::awake();
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

  // Create & show logging window
  self.wind_log.begin();
  log!("Initialized logging!");
  self.wind_log.end();

  // Show main window
  self.wind_main.show();

  // Set log window to the left of the main window
  self.wind_log.set_pos(self.wind_main.x() - self.wind_main.w(), self.wind_main.y());

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
      Some(common::Msg::ToggleTerminal) =>
      {
        match self.wind_log.shown()
        {
          true => self.wind_log.hide(),
          false => self.wind_log.show(),
        } // match
      }
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
      Some(value) => self.redraw(value),
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
