use std::env;

// Multithreading
use std::sync::mpsc;
use std::sync::{Arc,Mutex};
use std::path::Path;

// Tray
use tray_item::TrayItem;
use gtk;
use glib;

// Gui
use fltk::{
  app,
  app::App,
  button::Button,
  group::{Group, PackType},
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

// Icons {{{
const ICON_BACKGROUND: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="317" height="50" class="bi bi-play-fill" viewBox="0 0 30 20">
  <rect width="100%" height="100%" fill="#2A2E32" opacity="0.65"></rect>
</svg>
"##;

const ICON_PLAY: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="white" class="bi bi-play-fill" viewBox="0 0 16 16">
  <path d="m11.596 8.697-6.363 3.692c-.54.313-1.233-.066-1.233-.697V4.308c0-.63.692-1.01 1.233-.696l6.363 3.692a.802.802 0 0 1 0 1.393z"/>
</svg>
"#;

const ICON_CONFIGURE: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="white" class="bi bi-tools" viewBox="0 0 16 16">
  <path d="M1 0 0 1l2.2 3.081a1 1 0 0 0 .815.419h.07a1 1 0 0 1 .708.293l2.675 2.675-2.617 2.654A3.003 3.003 0 0 0 0 13a3 3 0 1 0 5.878-.851l2.654-2.617.968.968-.305.914a1 1 0 0 0 .242 1.023l3.27 3.27a.997.997 0 0 0 1.414 0l1.586-1.586a.997.997 0 0 0 0-1.414l-3.27-3.27a1 1 0 0 0-1.023-.242L10.5 9.5l-.96-.96 2.68-2.643A3.005 3.005 0 0 0 16 3c0-.269-.035-.53-.102-.777l-2.14 2.141L12 4l-.364-1.757L13.777.102a3 3 0 0 0-3.675 3.68L7.462 6.46 4.793 3.793a1 1 0 0 1-.293-.707v-.071a1 1 0 0 0-.419-.814zm9.646 10.646a.5.5 0 0 1 .708 0l2.914 2.915a.5.5 0 0 1-.707.707l-2.915-2.914a.5.5 0 0 1 0-.708M3 11l.471.242.529.026.287.445.445.287.026.529L5 13l-.242.471-.026.529-.445.287-.287.445-.529.026L3 15l-.471-.242L2 14.732l-.287-.445L1.268 14l-.026-.529L1 13l.242-.471.026-.529.445-.287.287-.445.529-.026z"/>
</svg>
"#;

const ICON_BACK: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="white" class="bi bi-arrow-left" viewBox="0 0 16 16">
  <path fill-rule="evenodd" d="M15 8a.5.5 0 0 0-.5-.5H2.707l3.147-3.146a.5.5 0 1 0-.708-.708l-4 4a.5.5 0 0 0 0 .708l4 4a.5.5 0 0 0 .708-.708L2.707 8.5H14.5A.5.5 0 0 0 15 8"/>
</svg>
"#;

// }}}

const HEIGHT_BUTTON_WIDE : i32 = 40;
const WIDTH_BUTTON_WIDE  : i32 = 60;

const HEIGHT_BUTTON_REC : i32 = 40;
const WIDTH_BUTTON_REC  : i32 = 40;

const HEIGHT_BUTTON_CHECK : i32 = 20;
const WIDTH_BUTTON_CHECK  : i32 = 20;

// struct: Gui {{{
#[derive(Debug)]
struct Gui
{
  app: App,
  wind: Window,
  width: i32,
  height: i32,
  border: i32,
  sender: mpsc::Sender<i32>,
  receiver: mpsc::Receiver<()>,
} // struct: Gui }}}

// struct: FrameInstance {{{
#[derive(Debug)]
struct FrameInstance
{
  group: Group,
  buttons: Vec<Button>,
} // struct FrameInstance }}}

// fn: yaml_write {{{
fn yaml_write(key: String, some_value: Option<serde_yaml::Value>) -> Option<()>
{
  if let Some(value) = some_value
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
        yaml_value[key] = value;

        // Write yaml
        serde_yaml::to_string(&yaml_value).map_or_else(
          |_| { println!("Could not generate yaml string"); None },
          |str_yaml| { std::fs::write(str_file.clone(), str_yaml.clone()).ok() })
      });

    return Some(());
  }

  None
} // }}}

// fn: yaml_read {{{
fn yaml_read(query: &str) -> Option<serde_yaml::Value>
{
  if let Some(var) = env::var("GIMG_CONFIG_FILE").ok()
  {
    if let Some(file) = std::fs::File::open(var).ok()
    {
      if let Some(yaml) = serde_yaml::from_reader::<std::fs::File, serde_yaml::Value>(file).ok()
      {
        if let Some(key) = yaml.get(query)
        {
          return Some(key.clone());
        } // if
        else
        {
          println!("Could not extract key from YAML file");
        } // else
      } // if
      else
      {
        println!("Could not parse config file");
      } // else
    } // if
    else
    {
      println!("Could not open config file for read");
    } // else
  } // if
  else
  {
    println!("Could not read GIMG_CONFIG_FILE variable");
  } // else
  None
} // }}}

// fn: tag_expand {{{
// Replace placeholder with value in environment variable 'var'
fn tag_expand(mut src: String) -> String
{
  for (tag, env) in vec![ ("{wine}", "BIN_WINE"), ("{exec}", "GIMG_DEFAULT_EXEC"), ("{here}", "DIR_CALL"), ("{appd}", "DIR_APP") ]
  {
    if let Some(value) = env::var(env).ok()
    {
      src = src.replace(tag, format!("\"{}\"", value).as_str()); 
    } // if
  }
  return src;
} // }}}

// impl: Gui {{{
impl Gui
{

  // fn: new {{{
  pub fn new(sender: mpsc::Sender<i32>, receiver: mpsc::Receiver<()>) -> Self
  {
    let width = 264;
    let height = 352;
    let border = 2;
    let app =  app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default()
      .with_label("GameImage")
      .with_size(width, height)
      .center_screen();
    app::set_frame_type(FrameType::BorderBox);

    // Window icon
    if let Some(env_image_launcher) = env::var("GIMG_LAUNCHER_IMG").ok()
    {
      if let Some(shared_image) = fltk::image::PngImage::load(env_image_launcher).ok()
      {
        wind.set_icon(Some(shared_image));
      } // if
      else
      {
        println!("Failed to load icon image");
      } // else
    } // if
    else
    {
      println!("Failed to fetch environment variable GIMG_LAUNCHER_IMG")
    } // else

    Gui
    {
      app,
      wind,
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

  // fn: frame_cover {{{
  fn frame_cover(&self) -> FrameInstance
  {
    let mut group = Group::default().size_of(&self.wind);
    group.set_frame(FrameType::FlatBox);

    // Frame cover {{{
    let mut frame_cover = self.make_frame(group.width(), group.height());
    frame_cover.set_frame(FrameType::FlatBox);
    // frame_cover.set_color(Color::Green);

    // Cover image
    if let Some(env_image_launcher) = env::var("GIMG_LAUNCHER_IMG").ok()
    {
      if let Some(mut shared_image) = SharedImage::load(env_image_launcher).ok()
      {
        let img_height = frame_cover.h();
        let img_width  = frame_cover.w();
        let mut frame_image = self.make_frame(img_width, img_height).with_pos(frame_cover.x(), frame_cover.y());
        frame_image.draw(move |f| {
          shared_image.scale(f.w(), f.h(), true, true);
          shared_image.draw(f.x() + (img_width - shared_image.width())/2, f.y(), f.w(), f.h());
        });
      } // if
      else
      {
        println!("Failed to fetch load provided image");
      } // else
    } // if
    else
    {
      println!("Failed to fetch environment variable GIMG_LAUNCHER_IMG")
    } // else
    // frame_cover }}}

    let mut btn_background = Button::default()
      .with_size(frame_cover.width(), HEIGHT_BUTTON_WIDE)
      .below_of(&frame_cover, -HEIGHT_BUTTON_WIDE);
    btn_background.set_frame(FrameType::NoBox);
    btn_background.set_image(Some(fltk::image::SvgImage::from_data(ICON_BACKGROUND).unwrap()));
    btn_background.deactivate();


    // Configure application
    let mut btn_configure = Button::default()
      .with_size(WIDTH_BUTTON_WIDE, HEIGHT_BUTTON_WIDE)
      .below_of(&frame_cover, -HEIGHT_BUTTON_WIDE);
    btn_configure.set_color(Color::BackGround);
    btn_configure.set_pos(btn_configure.x() + self.border, btn_configure.y() - self.border);
    btn_configure.set_frame(FrameType::NoBox);
    btn_configure.set_image(Some(fltk::image::SvgImage::from_data(ICON_CONFIGURE).unwrap()));

    // Lauch application
    let mut btn_launch = Button::default()
      .with_size(WIDTH_BUTTON_WIDE, HEIGHT_BUTTON_WIDE)
      .below_of(&frame_cover, -HEIGHT_BUTTON_WIDE);
    btn_launch.set_color(Color::DarkGreen);
    btn_launch.set_pos(btn_launch.x() + frame_cover.width() - btn_launch.width() - self.border, btn_launch.y() - self.border);
    btn_launch.set_frame(FrameType::NoBox);
    btn_launch.set_image(Some(fltk::image::SvgImage::from_data(ICON_PLAY).unwrap()));

    group.end();

    let frame_instance = FrameInstance
    {
      group,
      buttons: vec![btn_configure,btn_launch],
    };

    frame_instance
  } // fn: frame_cover }}}

  // fn: frame_config_wine {{{
  fn frame_config_wine(&self) -> FrameInstance
  {
    let mut group = Group::default().size_of(&self.wind);
    group.set_frame(FrameType::FlatBox);

    //
    // Frame config
    //
    let mut frame_config = self.make_frame(self.width, self.height);
    frame_config.set_frame(FrameType::FlatBox);

    //
    // Layout
    //

    let size_font : i32 = 14;
    let size_spacing : i32 = 5;

    let f_make_output = |label : &str|
    {
      let mut output = self.make_frame(100, 20);
      output.set_label_size(size_font);
      output.set_frame(FrameType::NoBox);
      output.set_align(Align::Left | Align::Inside);
      output.set_label(label);
      output
    };

    // Default application rom to execute
    let output = f_make_output("Binary to execute");
    output.clone().set_pos(self.border, self.border);
    let mut menu_binaries = MenuButton::default()
      .with_size(frame_config.width() - self.border*2, 40)
      .with_align(Align::Inside)
      .below_of(&output, 5);
    menu_binaries.set_frame(FrameType::BorderBox);

    // Default launch command
    let output = f_make_output("Default command");
    output.clone().below_of(&menu_binaries, self.border + size_spacing);
    let mut input_default_cmd = Input::default()
      .with_size(frame_config.width() - self.border*2, HEIGHT_BUTTON_WIDE)
      .with_align(Align::TopLeft)
      .below_of(&output, self.border);
    input_default_cmd.set_color(Color::BackGround);

    // Input to select default runner
    let mut group_runner_default = Group::default().size_of(&self.wind)
      .below_of(&input_default_cmd, self.border);
    group_runner_default.set_frame(FrameType::FlatBox);
    group_runner_default.set_size(group_runner_default.width(),
      self.border + size_spacing + HEIGHT_BUTTON_WIDE + HEIGHT_BUTTON_CHECK
    );

    // // Title
    let output = f_make_output("Alternative wine runner");
    output.clone().below_of(&input_default_cmd, self.border + size_spacing);
    output.clone().set_pos(output.x() + WIDTH_BUTTON_CHECK, output.y());

    // // Input
    let mut input_default_runner = Input::default()
      .with_size(frame_config.width() - self.border*2 - WIDTH_BUTTON_REC, HEIGHT_BUTTON_WIDE)
      .below_of(&output, self.border);
    input_default_runner.set_pos(input_default_runner.x() - WIDTH_BUTTON_CHECK, input_default_runner.y());
    input_default_runner.set_align(Align::TopLeft);
    input_default_runner.set_color(Color::BackGround);
    input_default_runner.deactivate();
    
    // // Use default wine path?
    let mut btn_use_wine_custom = fltk::button::CheckButton::default()
      .with_size(WIDTH_BUTTON_CHECK, HEIGHT_BUTTON_CHECK)
      .left_of(&output, 0);
    btn_use_wine_custom.set_checked(false);

    // // Click three dots to open file picker
    let mut btn_default_runner_picker = Button::default()
      .with_size(WIDTH_BUTTON_REC, HEIGHT_BUTTON_REC)
      .with_label("...")
      .right_of(&input_default_runner, 0);
    btn_default_runner_picker.set_frame(FrameType::BorderBox);
    btn_default_runner_picker.deactivate();
    group_runner_default.end();
    group_runner_default.hide();

    // Conditionally enable group
    if let Some(path_binary_wine) = env::var("BIN_WINE").ok()
    {
      // Activate if default runner exists and is not flatimage
      if let Some(var) = env::var("GIMG_PKG_TYPE").ok()
      {
        let path = Path::new(&path_binary_wine);
        if path.exists() && var != "flatimage"
        {
          group_runner_default.show();
        }
        else
        {
          println!("Path set in 'BIN_WINE' does not exist");
        }
      } // if
      else
      {
        println!("Could not fetch 'GIMG_PKG_TYPE' variable");
      } // else
    }
    else
    {
      println!("Could fetch RUNNER variable");
    }

    // Set bottom backgroud
    let mut btn_background = Button::default()
      .with_size(frame_config.width(), HEIGHT_BUTTON_WIDE)
      .below_of(&frame_config, -HEIGHT_BUTTON_WIDE);
    btn_background.set_frame(FrameType::NoBox);
    btn_background.set_image(Some(fltk::image::SvgImage::from_data(ICON_BACKGROUND).unwrap()));
    btn_background.deactivate();

    // Lauch application
    let mut btn_back = Button::default()
      .with_size(WIDTH_BUTTON_WIDE, HEIGHT_BUTTON_WIDE)
      .below_of(&frame_config, -HEIGHT_BUTTON_WIDE);
    btn_back.set_pos(btn_back.x() + self.border, btn_back.y() - self.border);
    btn_back.set_frame(FrameType::NoBox);
    btn_back.set_image(Some(fltk::image::SvgImage::from_data(ICON_BACK).unwrap()));

    //
    // Initial Values
    //

    // Default rom to execute
    menu_binaries.add_choice(env::var("GIMG_LAUNCHER_EXECUTABLES").unwrap_or(String::new()).as_str());
    menu_binaries.set_label(env::var("GIMG_DEFAULT_EXEC").unwrap_or(String::new()).as_str());

    // Default launch command
    if let Some(value) = yaml_read("cmd")
    {
      if let Some(string) = value.as_str()
      {
        input_default_cmd.set_value(string);
        env::set_var("GIMG_LAUNCHER_CMD", string);
      } // if
      else
      {
        println!("Could convert 'cmd' to string");
      } // else
    } // if
    else
    {
      println!("Could not read launcher cmd");
    } // else

    // Fetch previous state for runner_custom
    if let Some(value) = yaml_read("runner_custom")
    {
      if let Some(boolean) = value.as_bool()
      {
        btn_use_wine_custom.set_checked(boolean);
      }
      else
      {
        println!("Could convert 'runner_custom' to bool");
      } // else
    }
    else
    {
      println!("Could not read runner_custom in YAML");
    } // else

    // Display default runner
    if let Some(value) = yaml_read("runner")
    {
      if let Some(string) = value.as_str()
      {
        // Set field text
        input_default_runner.set_value(string);

        // Check if can activate
        if btn_use_wine_custom.is_checked()
        {
          btn_default_runner_picker.activate();
          input_default_runner.activate();
          env::set_var("BIN_WINE", string);
        }
      }
      else
      {
        println!("Could convert 'runner_custom' to string");
      } // else
    } // if
    else
    {
      println!("Could not read runner");
    } // else

    // Expand GIMG_LAUNCHER_CMD
    let f_launcher_cmd_update = ||
    {
      if let Some(var) = env::var("GIMG_LAUNCHER_CMD").ok()
      {
        env::set_var("GIMG_LAUNCHER_CMD_EXP", tag_expand(var));
      }
      else
      {
        println!("Could not fetch env var GIMG_LAUNCHER_CMD");
      }
    };
    f_launcher_cmd_update();

    //
    // Callbacks
    //

    // Update executable to execute
    menu_binaries.set_callback(move |e|
    {
      let choice = e.choice().unwrap();
      e.set_label(choice.as_str());
      env::set_var("GIMG_DEFAULT_EXEC", choice.as_str());
      f_launcher_cmd_update();
    });

    // Update default command to execute
    let f_input_default_cmd_update = move |e: &str|
    {
      // Perform strings replacements
      if yaml_write("cmd".to_string(), serde_yaml::to_value::<String>(e.into()).ok()).is_some()
      {
        env::set_var("GIMG_LAUNCHER_CMD_EXP", tag_expand(e.to_string()));
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

    // Checkbox to use custom wine path
    let mut _input_default_runner = input_default_runner.clone();
    let mut _btn_default_runner_picker = btn_default_runner_picker.clone();
    btn_use_wine_custom.set_callback(move |e|
    {
      if e.is_checked()
      {
        // Activate buttons
        _input_default_runner.activate();
        _btn_default_runner_picker.activate();
        // Update wine path
        env::set_var("BIN_WINE", _input_default_runner.value());
        // Update wine path & checkbutton state
        yaml_write("runner".to_string(), serde_yaml::to_value(_input_default_runner.value()).ok());
        yaml_write("runner_custom".to_string(), serde_yaml::to_value(true).ok());
        // Re-Expand GIMG_LAUNCHER_CMD_EXP
        f_launcher_cmd_update();
      }
      else
      {
        _input_default_runner.deactivate();
        _btn_default_runner_picker.deactivate();
        env::set_var("BIN_WINE", "$APPDIR/usr/bin/wine");
        yaml_write("runner_custom".to_string(), serde_yaml::to_value(false).ok());
        f_launcher_cmd_update();
      }
    });

    // Select path to custom wine runner
    let mut _input_default_runner = input_default_runner.clone();
    btn_default_runner_picker.set_callback(move |_|
    {
      file_chooser("", "", "", false)
        .map_or_else(|| { println!("Could not pick new path"); None },
          |e| { _input_default_runner.set_value(e.as_str()); Some(e) })
        .map_or_else(|| { println!("Could not set value for input widget"); None },
          |e| { env::set_var("BIN_WINE", e.clone()); Some(e) })
        .map_or_else(|| { println!("Could not set env variable value"); None },
          |e| { yaml_write("runner".to_string(), serde_yaml::to_value(e.to_string()).ok()); Some(e) })
        .map_or_else(|| { println!("Could not update default command") },
          |_| { f_input_default_cmd_update(input_default_cmd.value().as_str()); });
    });

    group.end();

    let frame_instance = FrameInstance
    {
      group,
      buttons: vec![btn_back],
    };

    frame_instance
  } // fn: frame_config_wine }}}

  // fn: frame_switcher {{{
  fn frame_switcher(&self)
  {
    // Create frames
    let frame_cover = self.frame_cover();
    let frame_config = self.frame_config_wine();

    // Fetch groups/buttons
    let mut frame_cover_group         = frame_cover.group.clone();
    let mut frame_cover_btn_configure = frame_cover.buttons[0].clone();
    let mut frame_cover_btn_launch    = frame_cover.buttons[1].clone();
    let mut frame_config_group        = frame_config.group.clone();
    let mut frame_config_btn_back     = frame_config.buttons[0].clone();

    // Startup group
    // frame_cover_group.hide();
    // frame_config_group.show();
    frame_cover_group.show();
    frame_config_group.hide();

    // Mutual exclusive access to groups
    let arc_frame_cover_group = Arc::new(Mutex::new(frame_cover_group));
    let arc_frame_config_group = Arc::new(Mutex::new(frame_config_group));


    // fn: Callbacks {{{
    let arc_frame_config_group_clone = Arc::clone(&arc_frame_config_group);
    let arc_frame_cover_group_clone = Arc::clone(&arc_frame_cover_group);
    frame_config_btn_back.set_callback(move |_|
    {
      let mut arc_frame_config_group = arc_frame_config_group_clone.lock().unwrap();
      let mut arc_frame_cover_group = arc_frame_cover_group_clone.lock().unwrap();
      arc_frame_cover_group.show();
      arc_frame_config_group.hide();
    });

    let arc_frame_config_group_clone = Arc::clone(&arc_frame_config_group);
    let arc_frame_cover_group_clone = Arc::clone(&arc_frame_cover_group);
    frame_cover_btn_configure.set_callback(move |_|
    {
      let mut arc_frame_config_group = arc_frame_config_group_clone.lock().unwrap();
      let mut arc_frame_cover_group = arc_frame_cover_group_clone.lock().unwrap();
      arc_frame_cover_group.hide();
      arc_frame_config_group.show();
    });

    let sender = self.sender.clone();
    frame_cover_btn_launch.set_callback(move |_|
    {
      app::quit();
      app::flush();

      env::var("GIMG_LAUNCHER_CMD_EXP").ok().and_then(|e|
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
    }).or_else(|| { println!("Variable GIMG_LAUNCHER_CMD_EXP is not defined"); sender.send(1).ok(); None });
  });
  // }}}

  } // fn: frame_switcher }}}

} // impl: Gui }}}

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

// fn: theme {{{
fn theme()
{
  app::background(42, 46, 50); 
  app::foreground(255, 255, 255); 
} // }}}

// fn: main {{{
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
    Gui::new(s1, r0).frame_switcher();
  });

  t.join().unwrap();
  u.join().unwrap();

  std::process::exit(*status_exit.lock().unwrap());
} // }}}

// cmd: !BIN_WINE="/home/ruan/Experiments/test.lua" GIMG_PKG_TYPE=appimage GIMG_LAUNCHER_IMG=/home/ruan/Pictures/prostreet.png cargo run --release

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
