use std::env;

// Multithreading
use std::sync::mpsc;
use std::sync::{Arc,Mutex};
use std::path::Path;

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
  dialog::file_chooser,
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
    // Functor to update yaml
    let f_yaml_write = |key: String, value: String|
    {
      // Read file from global variable
      let (file, str_file) = env::var("GIMG_CONFIG_FILE").ok().map_or_else(
          || { println!("Could not read GIMG_CONFIG_FILE variable"); (None, String::new()) },
          |str_file| { (std::fs::File::open(str_file.clone()).ok(), str_file) });

      // Deserialize the YAML into a serde_yaml::Value
      let yaml_value: Option<serde_yaml::Value> = file.map_or_else(
          || { println!("Could not open file {} for read", str_file);  None },
          |file| { serde_yaml::from_reader(file).ok() });

      // Update value in the yaml file
      yaml_value.map_or_else(
        || { println!("Could parse yaml file"); None },
        |mut yaml_value: serde_yaml::Value|
        {
          // Update yaml variable
          yaml_value[key] = serde_yaml::Value::String(value.as_str().into());

          // Write yaml
          serde_yaml::to_string(&yaml_value).map_or_else(
            |_| { println!("Could not generate yaml string"); None },
            |str_yaml| { std::fs::write(str_file.clone(), str_yaml.clone()).ok() })
        })
    };

    let f_yaml_read = |key: &str|
    {
      env::var("GIMG_CONFIG_FILE")
        .ok()
        .map_or_else(|| { println!("Could not read GIMG_CONFIG_FILE variable"); None },
        |var|
        {
          std::fs::File::open(var)
            .ok()
            .map_or_else(
              || { println!("Could not open config file for read"); None },
              |f|{ serde_yaml::from_reader(f).ok() })
            .map_or_else(
              || { println!("Could not parse config file"); None },
              |y: serde_yaml::Value|
              {
                y.get(key)
                  .map_or_else(
                    || { println!("Could not extract key from YAML file"); None },
                    |cmd| { Some(String::from(cmd.as_str().unwrap())) })
              })
        })
    };

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

    //
    // Layout
    //

    // Application name
    let name = env::var("GIMG_LAUNCHER_NAME").unwrap_or(String::from("GameImage"));
    let mut output = self.make_frame(100, 50)
      .above_of(&frame_right, -35);
    output.set_pos(self.border/2 + output.x(), output.y());
    output.set_label_size(20);
    output.set_frame(FrameType::NoBox);
    output.set_align(Align::Left | Align::Inside);
    output.set_label(name.as_str());

    // Default application rom to execute
    let mut menu_binaries = MenuButton::default()
      .with_size(frame_right.width() - self.border, 40)
      .with_align(Align::Inside)
      .below_of(&output, 20);
    menu_binaries.set_frame(FrameType::BorderBox);

    // Default launch command
    let mut input_default_cmd = Input::default()
      .with_size(frame_right.width() - self.border, 40)
      .below_of(&menu_binaries, 40);
    input_default_cmd.set_label("Default command");
    input_default_cmd.set_align(Align::TopLeft);
    input_default_cmd.set_color(Color::BackGround);

    // Use default wine path?
    let mut btn_use_builtin = fltk::button::CheckButton::default()
      .with_label("Use builtin wine?")
      .with_size(15,15)
      .below_of(&input_default_cmd, 15);
    btn_use_builtin.deactivate();

    // Input to select default runner
    let mut input_default_runner = Input::default()
      .with_size(frame_right.width() - self.border - 40, 40)
      .below_of(&input_default_cmd, 60);
    input_default_runner.set_label("Default runner path");
    input_default_runner.set_align(Align::TopLeft);
    input_default_runner.set_color(Color::BackGround);
    input_default_runner.deactivate();

    let mut btn_default_runner_picker = Button::default()
      .with_size(40, 40)
      .with_label("...")
      .right_of(&input_default_runner, 0);

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
    // Initial Values
    //

    // Default rom to execute
    menu_binaries.add_choice(env::var("GIMG_LAUNCHER_EXECUTABLES")
      .unwrap_or(String::new()).as_str());
    menu_binaries.set_label(env::var("GIMG_DEFAULT_EXEC")
      .unwrap_or(String::new()).as_str());

    // Default Launch command
    f_yaml_read("cmd")
      .map_or_else(|| { println!("Could not read cmd"); },
        |s| { input_default_cmd.set_value(s.as_str()); });

    // Default runner exists?
    let use_runner_default = env::var("WINE").ok()
      .map_or_else(|| { println!("Could fetch RUNNER variable"); false },
      |str_path|
      {
        // Activate if default runner exists
        Path::new(&str_path)
        .exists()
        .then(||
        {
          btn_use_builtin.activate();
        })
        .map_or_else(|| { false },
        |_|
        {
          // Fetch previous state
          f_yaml_read("runner_default")
            .map_or_else(||{ println!("Could not read runner_default in YAML"); false }
              , |e| { btn_use_builtin.set_checked(e == "true"); e == "true" })
        })
      });

    // Display default runner
    f_yaml_read("runner")
      .map_or_else(|| { println!("Could not read runner"); },
        |s|
        {
          // Set field text
          input_default_runner.set_value(s.as_str());
          // Check if can activate
          if ! use_runner_default { input_default_runner.activate(); }
        });

    //
    // Callbacks
    //

    menu_binaries.set_callback(move |e|
    {
      let choice = e.choice().unwrap();
      e.set_label(choice.as_str());
      env::set_var("GIMG_DEFAULT_EXEC", choice.as_str());
    });

    let f_input_default_cmd_update = move |e: &str|
    {
      // Perform strings replacements
      let mut str_cmd = e.to_string();

      // Replace placeholder with value in environment variable 'var'
      let mut f_expand = |placeholder, var|
      {
        env::var(var).map_or_else(|_| { println!("Could not expand var {}", var); },
          |value| { str_cmd = str_cmd.replace(placeholder, format!("\"{}\"", value).as_str()); });
      };

      f_expand("{wine}", "WINE");
      f_expand("{exec}", "GIMG_DEFAULT_EXEC");
      f_expand("{here}", "DIR_CALL");
      f_expand("{appd}", "DIR_APP");

      if f_yaml_write("cmd".to_string(), e.into()).is_some()
      {
        env::set_var("GIMG_LAUNCHER_CMD", str_cmd);
      }
      else
      {
        println!("Could not update YAML config file");
      } // else
    };

    input_default_cmd.set_callback(move |e|
    {
      f_input_default_cmd_update(e.value().as_str());
    });

    let mut _input_default_runner = input_default_runner.clone();
    btn_use_builtin.set_callback(move |e|
    {
      if e.is_checked()
      {
        _input_default_runner.deactivate();
        env::set_var("WINE", "$APPDIR/usr/bin/wine");
        f_yaml_write("runner_default".to_string(), "true".to_string());
      }
      else
      {
        _input_default_runner.activate();
        env::set_var("WINE", _input_default_runner.value());
        f_yaml_write("runner".to_string(), _input_default_runner.value());
        f_yaml_write("runner_default".to_string(), "false".to_string());
      }
    });

    let mut _input_default_runner = input_default_runner.clone();
    btn_default_runner_picker.set_callback(move |_|
    {
      file_chooser("", "", "", false)
        .map_or_else(|| { println!("Could not pick new path"); None },
          |e| { _input_default_runner.set_value(e.as_str()); Some(e) })
        .map_or_else(|| { println!("Could not set value for input widget"); None },
          |e| { env::set_var("WINE", e.clone()); Some(e) })
        .map_or_else(|| { println!("Could not set env variable value"); None },
          |e| { f_yaml_write("runner".to_string(), e.to_string()); Some(e) })
        .map_or_else(|| { println!("Could not update default command") },
          |_| { f_input_default_cmd_update(input_default_cmd.value().as_str()); });
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
