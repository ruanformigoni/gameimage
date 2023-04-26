use std::env;

// Multithreading
use std::sync::mpsc;
use std::sync::{Arc,Mutex};

// Tray
use tray_item::TrayItem;
use gtk;
use glib;

// Gtk Colors
use gtk::prelude::*;

// Gui
use fltk::{
  app,
  app::App,
  button::Button,
  group::{Group, PackType, Wizard},
  input::Input,
  menu::MenuButton,
  prelude::{ImageExt, InputExt, GroupExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
  window::Window,
  enums::{Align,FrameType,Color},
  frame::Frame,
  image::SharedImage,
};

use fltk_theme::{ColorTheme, color_themes};

// struct: Gui {{{
#[derive(Debug)]
struct Gui
{
  app: App,
  wind: Window,
  wizard: Wizard,
  width: i32,
  height: i32,
  border: i32,
  sender: mpsc::Sender<i32>,
  receiver: mpsc::Receiver<()>,
} // struct: Gui }}}

// impl: Gui {{{
impl Gui
{

  // fn: new {{{
  pub fn new(sender: mpsc::Sender<i32>, receiver: mpsc::Receiver<()>) -> Self
  {
    let width = 792;
    let height = 352;
    let border = 30;
    let app =  app::App::default().with_scheme(app::Scheme::Gtk);
    let wind = Window::default()
      .with_label("GameImage")
      .with_size(width, height)
      .center_screen();
    let wizard = Wizard::default().with_size(width, height);

    Gui
    {
      app,
      wind,
      wizard,
      width,
      height,
      border,
      sender,
      receiver,
    }
  } // fn: new }}}

  // fn: make_frame {{{
  fn make_frame(&self, width: i32, height: i32) -> Frame
  {
    let mut frame = Frame::default()
          .with_size(width, height)
          .with_label("");
    frame.set_frame(FrameType::FlatBox);
    frame.set_type(PackType::Vertical);
    // frame.set_frame(FrameType::PlasticUpBox);

    frame
  } // fn: make_frame }}}

  // fn: frame_1 {{{
  fn frame_1(&self)
  {
    let mut group1 = Group::default().size_of(&self.wizard);
    group1.set_frame(FrameType::FlatBox);

    // Frame top {{{
    let mut frame_top = self.make_frame(self.width - self.border, self.height - self.border)
      .with_pos(self.border/2, self.border/2);
    frame_top.set_frame(FrameType::FlatBox);
    frame_top.set_color(Color::DarkBlue);
    // }}}

    // Frame left {{{
    let mut frame_left = self.make_frame(frame_top.width()/3, frame_top.height())
      .with_pos(frame_top.x(),frame_top.y());
    frame_left.set_frame(FrameType::FlatBox);
    frame_left.set_color(Color::Green);

    // Cover image
    env::var("GIMG_LAUNCHER_IMG")
    .map_err(|_| "Failed to fetch environment variable GIMG_LAUNCHER_IMG")
    .and_then(|cover|
    {
      SharedImage::load(cover)
        .map_err(|_| "Failed to load cover image")
        .and_then(|mut img|
        {
          let img_height = frame_left.h();
          let img_width  = frame_left.w();
          let mut frame_image = self.make_frame(img_width, img_height).with_pos(frame_left.x(), frame_left.y());
          frame_image.draw(move |f| {
            img.scale(f.w(), f.h(), true, true);
            // img.draw(f.x(), f.y(), f.w(), f.h());
            img.draw(f.x() + (frame_left.w() - img.width())/2, f.y(), f.w(), f.h());
          });
          Ok(())
        })
    }).unwrap_or_else(|_|{ println!("Failed to fetch environment variable GIMG_LAUNCHER_IMG"); });
    // }}}

    // Frame right {{{
    let mut frame_right = self.make_frame(frame_top.width()-frame_top.width()/3, frame_top.height())
      .with_pos(frame_top.width()/3 + self.border/2, frame_top.y());
    frame_right.set_frame(FrameType::FlatBox);
    // frame_right.set_color(Color::Red);
    frame_right.redraw();

    // Application name
    let name = env::var("GIMG_LAUNCHER_NAME").unwrap_or(String::from("GameImage"));
    let mut output = self.make_frame(100, 50)
      .above_of(&frame_right, -35);
    output.set_pos(self.border/2 + output.x(), output.y());
    output.set_label_size(20);
    output.set_frame(FrameType::NoBox);
    output.set_align(Align::Left | Align::Inside);
    output.set_label(name.as_str());

    // Application binaries
    let mut menu_binaries = MenuButton::default()
      .with_size(frame_right.width() - self.border, 40)
      .with_align(Align::Inside)
      .below_of(&output, 20);
    menu_binaries.set_frame(FrameType::BorderBox);
    menu_binaries.add_choice(env::var("GIMG_LAUNCHER_EXECUTABLES")
      .unwrap_or(String::new()).as_str());
    menu_binaries.set_label(env::var("GIMG_DEFAULT_EXEC")
      .unwrap_or(String::new()).as_str());

    // Default launch command
    let mut input = Input::default()
      .with_size(frame_right.width() - self.border, 40)
      .below_of(&menu_binaries, 40);
    input.set_label("Default command");
    input.set_align(Align::TopLeft);
    env::var("GIMG_CONFIG_FILE").ok()
      .map_or_else(|| { println!("Could not read GIMG_CONFIG_FILE variable"); }, |var|
      {
        std::fs::File::open(var).ok().and_then(|f|{ serde_yaml::from_reader(f).ok() })
          .map(|y: serde_yaml::Value|
          {
            y.get("cmd").map(|cmd| { cmd.as_str().map(|s| { input.set_value(s); }); })
          });
      });

    // Lauch application
    let mut btn_launch = Button::default()
      .with_size(60, 40)
      .with_label("Launch")
      .below_of(&frame_right, -40);
    btn_launch.set_color(Color::DarkGreen);
    btn_launch.set_pos(btn_launch.x() + self.border/2, btn_launch.y());
    btn_launch.set_frame(FrameType::BorderBox);

    // Quit application
    let mut btn_quit = Button::default()
      .with_size(60, 40)
      .with_label("Quit")
      .below_of(&frame_right, -40);
    btn_quit.set_color(Color::Red);
    btn_quit.set_pos(btn_quit.x() + frame_right.width() - 60 - self.border/2, btn_quit.y());
    btn_quit.set_frame(FrameType::BorderBox);

    //
    // Callbacks
    //

    menu_binaries.set_callback(move |e|
    {
      let choice = e.choice().unwrap();
      e.set_label(choice.as_str());
      env::set_var("GIMG_DEFAULT_EXEC", choice.as_str());
    });

    input.set_callback(move |e|
    {
      // Perform strings replacements
      let mut str_cmd = e.value();
      str_cmd = str_cmd.replace("{wine}", env::var("WINE").unwrap().as_str());
      str_cmd = str_cmd.replace("{exec}", env::var("GIMG_DEFAULT_EXEC").unwrap().as_str());
      str_cmd = str_cmd.replace("{here}", env::var("DIR_CALL").unwrap().as_str());
      str_cmd = str_cmd.replace("{appd}", env::var("DIR_APP").unwrap().as_str());

      // Read file from global variable
      let (file, var) = env::var("GIMG_CONFIG_FILE").ok().map_or_else(
          || { println!("Could not read GIMG_CONFIG_FILE variable"); (None, String::new()) },
          |var| { (std::fs::File::open(var.clone()).ok(), var) }
        );

      // Parse file into yaml with serde_yaml
      let yaml: Option<serde_yaml::Value> = file.map_or_else(
          || { println!("Could not open file {} for read", var);  None },
          |file| { serde_yaml::from_reader(file).ok() }
        );

      // Update cmd in the yaml file and the GIMG_LAUNCHER_CMD variable
      yaml.map_or_else(|| { println!("Could parse yaml file");  }, |mut yaml: serde_yaml::Value|
      {
        // Update cmd value with the 'aliased' command
        yaml.get_mut("cmd").map_or_else(
            || { println!("Failed to update variable");  },
            |key| { *key = e.value().as_str().into(); }
          );
        // Write new cmd value to file and update GIMG_LAUNCHER_CMD with resolved aliases
        serde_yaml::to_string(&yaml).map_or_else(|_| { println!("Could not generate yaml string") }, |str_yaml|
        {
          std::fs::write(var.clone(), str_yaml)
            .map_or_else(
              |_| { println!("Could not write to file {}", var); },
              |_| { env::set_var("GIMG_LAUNCHER_CMD", str_cmd); }
            );
        });
      });
    });

    let sender = self.sender.clone();
    btn_quit.set_callback(move |_|
    {
      app::quit();
      app::flush();
      sender.send(0).ok();
    });

    let sender = self.sender.clone();
    btn_launch.set_callback(move |_|
    {
      app::quit();
      app::flush();

      env::var("GIMG_LAUNCHER_CMD").ok().and_then(|e|
      {
        env::set_var("GIMG_LAUNCHER_DISABLE", "1");
        let handle = std::process::Command::new("sh")
          .args(["-c", format!("''{} 2>&1''", e.as_str()).as_str()])
          .spawn();
        handle.ok().and_then(|mut h|
        {
          h
          .wait()
          .ok()
          .and_then(|e| { sender.send(e.code().unwrap_or(1)).ok() })
          .or_else( || { sender.send(1).ok() })
        }).or_else(|| { sender.send(1).ok() })
      }).or_else(|| { println!("Variable GIMG_LAUNCHER_CMD is not defined"); sender.send(1).ok(); None });
    });

    // }}}

    group1.end();
  } // fn: frame_1 }}}
} // impl: Gui

// impl: Drop for Gui {{{
impl Drop for Gui
{
  fn drop(&mut self)
  {
    self.receiver.recv().unwrap(); // wait for theme to be applied
    self.wind.make_resizable(false);
    self.wind.end();
    self.wind.show();
    self.app.run().unwrap_or_else(|_|{ println!("Failed to run GUI"); });
    self.sender.send(0).ok();
  }
} // }}}

fn theme()
{
  let label = gtk::Label::new(Some(""));
  let style_context = gtk::prelude::WidgetExt::style_context(&label);
  let color_bg = style_context.lookup_color("theme_bg_color");
  let color_fg = style_context.lookup_color("theme_fg_color");
  let color_sc = style_context.lookup_color("success_color");
  let color_er = style_context.lookup_color("error_color");

  // Set the theme to correspond to the current gtk context
  let ci = |c: f64| { (c*255.0) as u8 }; // normalized color f64 to un-normalized u8

  if let Some(c) = color_bg { app::background(ci(c.red()), ci(c.green()), ci(c.blue())); }
  else { println!("Failed to set background color"); }

  if let Some(c) = color_fg { app::foreground(ci(c.red()), ci(c.green()), ci(c.blue())); }
  else { println!("Failed to set foreground color"); }

  if let Some(c) = color_sc { app::set_color(Color::DarkGreen, ci(c.red()), ci(c.green()), ci(c.blue())); }
  else { println!("Failed to set success color"); }

  if let Some(c) = color_er { app::set_color(Color::Red, ci(c.red()), ci(c.green()), ci(c.blue())); }
  else { println!("Failed to set error color"); }

}

fn main()
{
  // Exit status
  let status_exit = Arc::new(Mutex::new(0));

  // Set starting theme as dark
  ColorTheme::new(color_themes::BLACK_THEME).apply(); // Start with a default dark theme

  let (s0, r0) = mpsc::channel::<()>();
  let (s1, r1) = mpsc::channel::<i32>();

  // Run tray in new thread
  let status_exit_clone = Arc::clone(&status_exit);
  let t = std::thread::spawn(move ||
  {
    // Application name
    let name = env::var("GIMG_LAUNCHER_NAME").unwrap_or(String::from("GameImage"));
    // Application image
    let cover = env::var("GIMG_LAUNCHER_IMG");

    // Init gtk
    gtk::init().unwrap();

    // Set fltk theme
    theme();

    // Tell fltk the theme is set
    s0.send(()).unwrap_or_else(|_|{ println!("Failed to send 'theme applied' signal to fltk"); });

    // Init tray
    TrayItem::new("Gameimage", cover.unwrap_or(String::from("system-run")).as_str())
      .and_then(|mut e|
        {
          e.add_label(name.as_str()).and_then(|_|{ Ok(e) })
        })
      .and_then(|mut e|
        {
          e.add_menu_item("Quit", || { gtk::main_quit(); })
        })
      .unwrap_or_else(|_|{ println!("Failed to initialize tray"); });

    // Wait for launched application to finish, and exit tray icon
    glib::source::timeout_add(std::time::Duration::from_millis(1000), move ||
    {
      match r1.try_recv().ok()
      {
        Some(e) =>
        {
          *status_exit_clone.lock().unwrap() = e;
          gtk::main_quit();
          glib::Continue(false)
        }
        None => { glib::Continue(true) }
      }
    });

    // Start gtk tray
    gtk::main();
  });

  // Run main fltk window in new thread
  let u = std::thread::spawn(||
  {
    Gui::new(s1, r0).frame_1();
  });

  t.join().unwrap();
  u.join().unwrap();

  std::process::exit(*status_exit.lock().unwrap());
}

// cmd: !cargo run

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
