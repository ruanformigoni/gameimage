use std::fs;
use std::env;
use std::path::PathBuf;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::io::{BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::collections::HashSet;

use walkdir::WalkDir;
use closure::closure;
use fltk::{
  app,
  app::App,
  button::{Button,CheckButton},
  dialog::dir_chooser,
  group::{Group, PackType},
  input::{Input,FileInput},
  output::Output,
  menu::MenuButton,
  prelude::{ImageExt, DisplayExt, InputExt, GroupExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
  window::Window,
  enums::{Align,FrameType,Color},
  frame::Frame,
  text::SimpleTerminal,
  image::SharedImage,
};
use fltk_theme::{ColorTheme, color_themes};

type SharedPtr<T> = Rc<RefCell<T>>;

// struct: Gui {{{
#[derive(Debug)]
struct Gui
{
  app: App,
  wind: Arc<Mutex<Window>>,
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
    let wind = Arc::new(Mutex::new(Window::default()
      .with_label("GameImage")
      .with_size(width, height)
      .center_screen()));
    let map_yaml = Rc::new(RefCell::new(BTreeMap::<String,String>::new()));

    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();
    app::foreground(230,230,230);
    let color = Color::from_hex_str("#5294e2").unwrap().to_rgb();
    app::set_selection_color(color.0, color.1, color.2);
    app::set_frame_type(FrameType::BorderBox);

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

  // fn: make_frame {{{
  fn make_frame(&self, width: i32, height: i32) -> Frame
  {
    let mut frame = Frame::default()
          .with_size(width, height)
          .with_label("");
    frame.set_frame(FrameType::NoBox);
    frame.set_type(PackType::Vertical);
    // frame.set_frame(FrameType::PlasticUpBox);

    frame
  } // fn: make_frame }}}

  // fn: frame_1 {{{
  fn frame_1(&self) -> FrameInstance
  {
    let mut group = Group::default().with_size(self.width, self.height);
    group.set_frame(FrameType::FlatBox);

    // Frame top {{{
    let frame_top = self.make_frame(self.width, self.height);

    // Header
    let mut output = Output::default()
      .with_pos(self.border, self.border)
      .with_align(Align::Right)
      .with_label("GameImage");
    output.set_frame(FrameType::NoBox);
    output.set_label_size(20);

    //
    // Widgets
    //

    // Game name
    let mut input = Input::new(self.border, self.border, self.width - self.border*2, 30, "")
      .below_of(&output, 50);
    input.set_label("Enter the game name");
    input.set_align(Align::TopLeft);

    // Packaging dir
    let mut remaining_width = self.width-self.border*2;
    let mut dir_root_path = FileInput::new(self.border
      , self.border
      , remaining_width-50-self.border*2
      , 40, "Select the game directory")
        .below_of(&input, self.border)
        .with_align(Align::TopLeft);
    dir_root_path.set_readonly(true);
    remaining_width -= dir_root_path.width();

    let mut btn_dir = Button::default().with_size(remaining_width-10, 40).with_label("Open")
      .right_of(&dir_root_path, 10);

    // Select Platform
    let mut remaining_width = self.width-self.border*2;
    let mut btn_platform = MenuButton::default().with_size(100, 40).with_label("platform")
      .below_of(&dir_root_path, 10);
    btn_platform.add_choice("retroarch|pcsx2|rpcs3|yuzu|wine");
    remaining_width -= btn_platform.width();

    // Select rom
    let mut btn_rom = MenuButton::default().with_size(remaining_width-10, 40).with_label("rom")
      .right_of(&btn_platform, 10);

    // Select arch (wine)
    let mut btn_arch = MenuButton::default().with_size(btn_platform.width(), 40).with_label("arch")
      .below_of(&btn_platform, 10);
    btn_arch.add_choice("win32|win64");

    // Cover image
    let mut frame_image = self.make_frame(200, 200)
      .below_of(&btn_rom, 10);
    frame_image.set_align(Align::Center);

    //
    // Callbacks
    //

    // Game name
    input.set_callback(closure!(clone self.map_yaml, |e| {
      map_yaml.borrow_mut().insert("name".to_string(), e.value());
    }));

    // button platform
    btn_platform.set_callback(closure!(clone self.map_yaml, clone mut btn_arch, clone mut btn_rom, |e|
    {
      if let Some(f) = e.choice()
      {
        e.set_label(&f);

        env::set_var("GIMG_PLATFORM", &f);

        map_yaml.borrow_mut().insert("platform".to_string(), f.clone());
        match f.as_str()
        {
          "rpcs3" | "pcsx2" | "retroarch" | "yuzu" =>
          {
            btn_rom.show();
            btn_arch.hide();
          }
          _ => 
          {
            btn_rom.show();
            btn_arch.show();
          }
        }
      }
    }));

    // button rom
    btn_rom.set_callback(closure!(clone self.map_yaml, |e|
    {
      let path = e.choice().unwrap();
      let path_buf = PathBuf::from(path.as_str());
      e.set_label(path_buf.file_name().unwrap().to_str().unwrap());
      map_yaml.borrow_mut().insert("rom".to_string(), path.to_string());
    }));
    btn_rom.hide();

    btn_arch.set_callback(closure!(clone self.map_yaml, |e|
    {
      let choice = e.choice().unwrap();
      e.set_label(choice.as_str());
      map_yaml.borrow_mut().insert("arch".to_string(), choice.to_string());
    }));
    btn_arch.hide();

    // Button select dir
    btn_dir.set_callback(closure!(clone mut frame_image, clone mut dir_root_path, clone self.map_yaml, |_|
    {
      let diag_dir_chooser = dir_chooser("", "", false);

      if diag_dir_chooser.is_some()
      {
        dir_root_path.set_value(&diag_dir_chooser.unwrap());
        map_yaml.borrow_mut().insert("dir".to_string(), dir_root_path.value());

        btn_rom.clear();

        let _ = WalkDir::new(dir_root_path.value() + "/rom")
          .follow_links(true)
          .sort_by_file_name()
          .into_iter()
          .filter_map(|e| e.ok() )
          .filter(|e| { let str_file = e.path().to_str().unwrap().to_lowercase();
                        str_file.ends_with(".exe") ||
                        str_file.ends_with(".msi") ||
                        str_file.ends_with(".iso") ||
                        str_file.ends_with(".sfb") ||
                        str_file.ends_with(".bin") ||
                        str_file.ends_with(".nsp") ||
                        str_file.ends_with(".zip") ||
                        str_file.ends_with(".cue")} )
          .take(20)
          .for_each(|f| { btn_rom.add_choice(f.path().to_str().unwrap()); } );

        let _ = WalkDir::new(dir_root_path.value() + "/icon")
          .follow_links(true)
          .into_iter()
          .filter_map(|e| e.ok().filter(|f| f.path().is_file() ) )
          .for_each(|e|
          {
            let image = SharedImage::load(e.path().to_str().unwrap())
              .and_then(|mut img| { img.scale(180,180,true,true); Ok(img) } );
            frame_image.set_image(Some(image.unwrap()));
            frame_image.redraw();
          });
      }
    }));
    // }}}

    // frame_bottom {{{
    let frame_bottom = self.make_frame(self.width, 50) .below_of(&frame_top, -50);

    // Write yaml on click in 'next'
    let mut btn_next = Button::default()
      .with_size(60, self.border)
      .with_label("Next")
      .center_of(&frame_bottom);
    btn_next.set_color(Color::DarkGreen);

    // }}}

    group.end();

    let frame_instance = FrameInstance {
      group,
      buttons: vec![btn_next],
    };

    frame_instance

  } // fn: frame_1 }}}

  // fn: frame_2 {{{
  fn frame_2(&self) -> FrameInstance
  {
    let mut group = Group::default().with_size(self.width, self.height);
    group.set_frame(FrameType::FlatBox);

    // Check if is flatimage {{{
    let is_flatimage : bool = env::var("GIMG_PKG_TYPE").is_ok_and(|e| e == "flatimage");

    // }}}

    // frame_top {{{
    let frame_top = self.make_frame(self.width, self.height);
    let mut header = Output::default()
      .with_pos(self.border, self.border)
      .with_align(Align::Right)
      .with_label("Environment Variable Configuration");
    header.set_frame(FrameType::NoBox);
    header.set_label_size(20);
    // }}}

    // frame_content {{{

    //
    // Create layout
    //
    let width_btn = 120;
    let height_btn = 30;

    // Create label to the far left and menubutton to the far right
    let f_make_entry_menubutton = |txt: &str, row: i32|
    {
      let btn = MenuButton::default()
        .with_size(width_btn, height_btn)
        .with_pos(frame_top.width() - width_btn - self.border, (frame_top.y() + self.border*2)*row);
      let mut lbl = Output::default()
        .with_pos(self.border, btn.y() + self.border / 2)
        .with_align(Align::Right)
        .with_label(txt);
      lbl.set_frame(FrameType::NoBox);
      btn
    };

    // Create checkbutton
    let f_make_entry_checkbutton = |txt: &str, row: i32|
    {
      let btn = CheckButton::default()
        .with_size(width_btn, height_btn)
        .with_pos(frame_top.width() - width_btn - self.border, (frame_top.y() + self.border*2)*row);
      let mut lbl = Output::default()
        .with_pos(self.border, btn.y() + self.border / 2)
        .with_align(Align::Right)
        .with_label(txt);
      lbl.set_frame(FrameType::NoBox);
      btn
    };

    // Select the package type
    let mut btn_package_type = f_make_entry_menubutton("Package Type", 1);
    btn_package_type.add_choice("overlayfs|unionfs|readonly|prefix");
    let hash_package_type : HashSet<&str> = vec!["overlayfs", "unionfs", "copy", "prefix"].into_iter().collect();

    // Select the wine dist
    let mut btn_wine_dist = f_make_entry_menubutton("Wine Distribution", 2);

    if is_flatimage
    {
      btn_wine_dist.add_choice("ge|staging|caffe|vaniglia|soda|tkg|osu-tkg");
    } // if
    else
    {
      btn_wine_dist.add_choice("ge|staging|caffe|vaniglia|soda");
    } // else

    let mut btn_check_dxvk = f_make_entry_checkbutton("Install dxvk for directx 9/10/11?", 3);

    let mut btn_check_vkd3d = f_make_entry_checkbutton("Install vkd3d for directx 12?", 4);

    //
    // Initialize defaults
    //
    let f_initialize_entry = |var: &str, default: &str, hash: HashSet<&str>, btn: &mut MenuButton|
    {
      match env::var(var)
      {
        Ok(e) =>
        {
          if hash.contains(e.as_str())
          {
            btn.set_label(e.as_str());
          }
          else
          {
            btn.set_label(default);
          }
        },
        Err(_) => btn.set_label(default),
      }
    };

    f_initialize_entry("GIMG_PKG_METHOD", "overlayfs", hash_package_type, &mut btn_package_type);

    if is_flatimage
    {
      f_initialize_entry("GIMG_WINE_DIST"
        , "ge"
        , vec!["ge","staging","caffe","vaniglia","soda","tkg","osu-tkg"]
          .into_iter()
          .collect()
        , &mut btn_wine_dist);
    } // if
    else
    {
      f_initialize_entry("GIMG_WINE_DIST"
        , "ge"
        , vec!["ge","staging","caffe","vaniglia","soda"]
          .into_iter()
          .collect()
        , &mut btn_wine_dist);
    } // else

    env::set_var("GIMG_INSTALL_DXVK", "0");

    env::set_var("GIMG_INSTALL_VKD3D", "0");

    //
    // Set callbacks
    //
    btn_package_type.set_callback(|e|
    {
      e.choice().as_ref().map(|f| { e.set_label(f); env::set_var("GIMG_PKG_METHOD", f); });
    });

    btn_wine_dist.set_callback(|e|
    {
      e.choice().as_ref().map(|f| { e.set_label(f); env::set_var("GIMG_WINE_DIST", f); });
    });

    btn_check_dxvk.set_callback(|e|
    {
      env::set_var("GIMG_INSTALL_DXVK", if e.is_checked() { "1" } else { "0" });
    });

    btn_check_vkd3d.set_callback(|e|
    {
      env::set_var("GIMG_INSTALL_VKD3D", if e.is_checked() { "1" } else { "0" });
    });
    // }}}

    // frame_bottom {{{

    let frame_bottom = self.make_frame(self.width, 50).below_of(&frame_top, -50);

    // Write yaml on click in 'next'
    let mut btn_prev = Button::default()
      .with_size(60, 30)
      .with_label("Prev")
      .center_y(&frame_bottom);
    btn_prev.set_pos(30, btn_prev.y());

    let mut btn_next = Button::default()
      .with_size(60, self.border)
      .with_label("Next")
      .center_of(&frame_bottom);
    btn_next.set_color(Color::DarkGreen);

    // }}}

    group.end();

    let frame_instance = FrameInstance {
      group,
      buttons: vec![btn_prev,btn_next],
    };

    frame_instance
  } // fn: frame_2 }}}

  // fn: frame_3 {{{
  fn frame_3(&self) -> FrameInstance
  {
    let mut group = Group::default().with_size(self.width, self.height);
    group.set_frame(FrameType::FlatBox);

    // Frame top {{{
    let frame_top_height = self.height - 110;
    let frame_top = self.make_frame(self.width, frame_top_height);

    // Terminal to output script execution
    let mut term = SimpleTerminal::new(self.border, self.border, self.width, frame_top_height, "")
      .above_of(&frame_top, - frame_top_height);
    term.set_text_color(self.wind.lock().unwrap().label_color());
    term.set_text_size(10);
    // }}}

    // Frame bottom {{{

    // Frame CMD input {{{

    let frame_cmd = self.make_frame(self.width, 50).below_of(&frame_top, 10);

    let input_cmd = Input::new(self.border
      , self.border
      , self.width-60-2*30-10
      , 30
      , "Input commands for the terminal")
        .center_y(&frame_cmd)
        .with_align(Align::TopLeft);

    let mut btn_send = Button::default()
      .with_size(60, 30)
      .with_label("Send")
      .center_y(&frame_cmd);
    btn_send.set_color(Color::DarkGreen);
    btn_send.set_pos(self.width-90, btn_send.y());

    // }}}

    // Frame Buttons {{{
    let frame_buttons = self.make_frame(self.width, 50).below_of(&frame_cmd, 0);

    let mut btn_prev = Button::default()
      .with_size(60, 30)
      .with_label("Prev")
      .center_y(&frame_buttons);
    btn_prev.set_pos(30, btn_prev.y());

    let mut btn_exit = Button::default()
      .with_size(60, 30)
      .with_label("Exit")
      .center_y(&frame_buttons);
    btn_exit.set_pos(self.width-90, btn_exit.y());
    btn_exit.set_color(Color::Red);
    btn_exit.set_callback(|_|
    {
      app::quit();
    });

    let mut btn_build = Button::default()
      .with_size(60, 30)
      .with_label("Build")
      .center_of(&frame_buttons);

    btn_build.set_color(Color::DarkGreen);

    btn_build.set_callback(closure!(clone mut btn_prev, clone term
      , clone mut btn_send, clone input_cmd, |e|
    {
      let mut btn_build = e.clone();
      let env_appdir = env::var("APPDIR").unwrap_or(String::from("."));
      let cmd_main = format!("{}{}{}", env_appdir, "/usr/bin/", "main.sh");

      btn_build.deactivate();
      btn_prev.deactivate();

      // Spawn command
      let reader_cmd = Command::new("bash")
        .args(["-c", format!("{} --yaml 2>&1", cmd_main).as_str()])
        .stdin(Stdio::piped())
        .stderr(Stdio::inherit())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not start gameimage main script");

      // Create arc reader for stdin and stdout
      let arc_reader_stdin = Arc::new(Mutex::new(reader_cmd.stdin));
      let arc_reader_stdout = Arc::new(Mutex::new(reader_cmd.stdout));

      // Set callback to send commands to terminal stdin,
      // when 'send' is pressed
      btn_send.set_callback(closure!(clone mut input_cmd, clone arc_reader_stdin, |_|
      {
        let guard_result = arc_reader_stdin.lock();

        if let Err(_) = guard_result
        {
          println!("Failed to acquire stdin lock");
          return;
        }

        let mut guard = guard_result.unwrap();
        let stdin_opt = guard.as_mut();

        if let None = stdin_opt
        {
          println!("Failed to acquire mut stdin");
          return;
        }

        let stdin = stdin_opt.unwrap();
        let value = input_cmd.value();
        input_cmd.set_value("");
        writeln!(stdin, "{}", value).expect("Failed to write to child stdin");
      }));

      // Write stdout to terminal
      std::thread::spawn(closure!(clone mut term, clone arc_reader_stdout, ||
      {
        // Acquire lock
        let lock = (&*arc_reader_stdout).lock();
        if let Err(_) = lock { println!("Failed to acquire stdout lock"); return; }

        // Acquire stdout
        let mut guard = lock.unwrap();
        let stdout = guard.as_mut();
        if stdout.is_none() { println!("Failed to acquire mut stdout"); return; }

        // Create buf
        let mut buf_reader = BufReader::new(stdout.unwrap());
        let mut buf = vec![0; 4096];

        // Write buf to stdout
        loop
        {
          std::thread::sleep(std::time::Duration::from_millis(50));

          let bytes_read = match buf_reader.read(&mut buf) {
            Ok(bytes_read) => bytes_read,
            Err(_) => break,
          };

          if bytes_read == 0 { break; }
          let output = String::from_utf8_lossy(&buf[..bytes_read]);
          term.insert(&output);
          term.show_insert_position();
          app::awake();
        }
      }));

      // Unlock GUI buttons after script finishes
      // When this happens, stdout and stderr will be released
      std::thread::spawn(closure!(clone mut btn_prev, clone arc_reader_stdout, ||
      {
        // Wait for stdout lock in previously launched thread
        std::thread::sleep(std::time::Duration::from_millis(500));
        // Try to re-enable buttons after release
        match (&*arc_reader_stdout).lock()
        {
          Ok(_) => { btn_build.activate(); btn_prev.activate(); app::awake(); }
          Err(_) => { println!("Failure to restore button state"); }
        }
      }));

    })); // btn_build.set_callback...

    // }}}

    // }}}

    group.end();

    let frame_instance = FrameInstance {
      group,
      buttons: vec![btn_prev],
    };

    frame_instance

  } // fn: frame_3 }}}

// fn: frame_switcher {{{
fn frame_switcher(&self)
{
  // Create frames
  let mut frame_1 = self.frame_1();
  let mut frame_2 = self.frame_2();
  let mut frame_3 = self.frame_3();

  // Show first frame by default
  frame_1.group.show();
  frame_2.group.hide();
  frame_3.group.hide();

  // Fetch frame groups & buttons
  let frame_1_group = frame_1.group.clone();
  let mut frame_1_btn_next = frame_1.buttons[0].clone();
  let frame_2_group = frame_2.group.clone();
  let mut frame_2_btn_prev = frame_2.buttons[0].clone();
  let mut frame_2_btn_next = frame_2.buttons[1].clone();
  let frame_3_group = frame_3.group.clone();
  let mut frame_3_btn_prev = frame_3.buttons[0].clone();

  // Setup access to groups from different callbacks
  let arc_frame_1_group = Arc::new(Mutex::new(frame_1_group));
  let arc_frame_2_group = Arc::new(Mutex::new(frame_2_group));
  let arc_frame_3_group = Arc::new(Mutex::new(frame_3_group));

  let arc_frame_1_group_clone = Arc::clone(&arc_frame_1_group);
  let arc_frame_2_group_clone = Arc::clone(&arc_frame_2_group);
  let arc_frame_3_group_clone = Arc::clone(&arc_frame_3_group);
  let map_yaml_clone = self.map_yaml.clone();
  frame_1_btn_next.set_callback(move |_|
  {
    let mut frame_1_group = arc_frame_1_group_clone.lock().unwrap();
    let mut frame_2_group = arc_frame_2_group_clone.lock().unwrap();
    let mut frame_3_group = arc_frame_3_group_clone.lock().unwrap();
    if env::var("GIMG_PLATFORM").is_ok_and(|x| x == "wine")
    {
      frame_1_group.hide();
      frame_2_group.show();
    } // if
    else
    {
      frame_1_group.hide();
      frame_3_group.show();
    } // else
    let yaml = serde_yaml::to_string(map_yaml_clone.as_ref()).unwrap();
    assert!(fs::write("/tmp/gameimage.yml", yaml).is_ok());
  });

  let arc_frame_1_group_clone = Arc::clone(&arc_frame_1_group);
  let arc_frame_2_group_clone = Arc::clone(&arc_frame_2_group);
  frame_2_btn_prev.set_callback(move |_|
  {
    let mut frame_1_group = arc_frame_1_group_clone.lock().unwrap();
    let mut frame_2_group = arc_frame_2_group_clone.lock().unwrap();
    frame_2_group.hide();
    frame_1_group.show();
  });

  let arc_frame_2_group_clone = Arc::clone(&arc_frame_2_group);
  let arc_frame_3_group_clone = Arc::clone(&arc_frame_3_group);
  let map_yaml_clone = self.map_yaml.clone();
  frame_2_btn_next.set_callback(move |_|
  {
    let mut frame_2_group = arc_frame_2_group_clone.lock().unwrap();
    let mut frame_3_group = arc_frame_3_group_clone.lock().unwrap();
    frame_2_group.hide();
    frame_3_group.show();
    let yaml = serde_yaml::to_string(map_yaml_clone.as_ref()).unwrap();
    assert!(fs::write("/tmp/gameimage.yml", yaml).is_ok());
  });

  let arc_frame_1_group_clone = Arc::clone(&arc_frame_1_group);
  let arc_frame_2_group_clone = Arc::clone(&arc_frame_2_group);
  let arc_frame_3_group_clone = Arc::clone(&arc_frame_3_group);
  frame_3_btn_prev.set_callback(move |_|
  {
    let mut frame_1_group = arc_frame_1_group_clone.lock().unwrap();
    let mut frame_2_group = arc_frame_2_group_clone.lock().unwrap();
    let mut frame_3_group = arc_frame_3_group_clone.lock().unwrap();
    if env::var("GIMG_PLATFORM").is_ok_and(|x| x == "wine")
    {
      frame_3_group.hide();
      frame_2_group.show();
    } // if
    else
    {
      frame_3_group.hide();
      frame_1_group.show();
    } // else
  });
} // fn: frame_switcher }}}

} // }}}

// impl: Drop for Gui {{{
impl Drop for Gui
{
  fn drop(&mut self)
  {
    let mut wind = self.wind.lock().unwrap();
    wind.make_resizable(false);
    wind.end();
    wind.show();
    self.app.run().unwrap();
  }
} // }}}

// fn: main {{{
fn main() {
  // Tell GameImage that GUI is used
  env::set_var("GIMG_GUI", "Yes");

  // Init GUI
  let _gui = Gui::new().frame_switcher();
} // fn: main }}}

// cmd: !GIMG_PKG_TYPE=flatimage cargo run --release

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
