use std::env;
use std::fs;
use std::path::PathBuf;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::BTreeMap;
use std::io::{BufRead,BufReader};

use walkdir::WalkDir;
use closure::closure;
use duct::cmd;
use fltk::{
  app,
  app::App,
  button::Button,
  dialog::dir_chooser,
  group::{Group, PackType, Wizard},
  input::{Input,FileInput},
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
  wind: Window,
  wizard: Wizard,
  map_yaml: SharedPtr<BTreeMap::<String,String>>,
  width: i32,
  height: i32,
  border: i32,
} // struct: Gui }}}

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
    let wind = Window::default()
      .with_label("GameImage")
      .with_size(width, height)
      .center_screen();
    let wizard = Wizard::default().with_size(width, height);
    let map_yaml = Rc::new(RefCell::new(BTreeMap::<String,String>::new()));

    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();
    app::foreground(230,230,230);
    let color = Color::from_hex_str("#5294e2").unwrap().to_rgb();
    app::set_selection_color(color.0, color.1, color.2);

    Gui
    {
      app,
      wind,
      wizard,
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
  fn frame_1(&self)
  {
    let group1 = Group::default().size_of(&self.wizard);

    // Frame top {{{
    let frame_top = self.make_frame(self.width, self.height - 50);

    // Header
    let mut output = Frame::new(self.border, 0, self.width - self.border*2, 100, "");
    output.set_label_size(20);
    output.set_frame(FrameType::NoBox);
    // output.set_label_color(Color::White);
    output.set_align(Align::Left | Align::Inside);
    output.set_label("GameImage");

    //
    // Widgets
    //

    // Game name
    let mut input = Input::new(self.border, self.border, self.width - self.border*2, 30, "")
      .below_of(&output, 0);
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
    btn_platform.set_callback(closure!(clone self.map_yaml, clone btn_arch, clone btn_rom, |e| {
      e.choice().and_then(closure!(clone map_yaml, clone mut btn_arch, clone mut btn_rom, |f|
      {
        e.set_label(&f);
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
        Some(f)
      }));
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

      if diag_dir_chooser.is_some() {
        dir_root_path.set_value(&diag_dir_chooser.unwrap());
        map_yaml.borrow_mut().insert("dir".to_string(), dir_root_path.value());

        btn_rom.clear();

        let _ = WalkDir::new(dir_root_path.value() + "/rom")
          .sort_by_file_name()
          .into_iter()
          .take(20)
          .filter_map(|e| e.ok().filter(|f| f.path().is_file() ) )
          .for_each(|f| { btn_rom.add_choice(f.path().to_str().unwrap()); } );

        let _ = WalkDir::new(dir_root_path.value() + "/icon")
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
    let frame_bottom = self.make_frame(self.width, 50) .below_of(&frame_top, 0);

    // Write yaml on click in 'next'
    let mut btn_next = Button::default()
      .with_size(60, self.border)
      .with_label("Next")
      .center_of(&frame_bottom);
    btn_next.set_color(Color::DarkGreen);
    btn_next.set_callback({
      closure!(clone self.map_yaml, clone mut self.wizard, |_|
      {
        let yaml = serde_yaml::to_string(map_yaml.as_ref()).unwrap();
        assert!(fs::write("gameimage.yml", yaml).is_ok());
        wizard.next()
      })
    });

    // }}}

    group1.end();
  } // fn: frame_1 }}}

  // fn: frame_2 {{{
  fn frame_2(&self)
  {
    let group2 = Group::default().size_of(&self.wizard);

    // Frame top {{{
    let frame_top = self.make_frame(self.width, self.height - 50);

    // Terminal to output script execution
    let mut term = SimpleTerminal::new(self.border, self.border, self.width, self.height - 50, "")
      .above_of(&frame_top, - (self.height - 50));
    term.set_text_color(self.wind.label_color());
    term.set_text_size(10);
    // }}}

    // Frame bottom {{{
    let frame_bottom = self.make_frame(self.width, 50).below_of(&frame_top, 0);

    let mut btn_prev = Button::default()
      .with_size(60, 30)
      .with_label("Prev")
      .center_y(&frame_bottom);

    let mut btn_exit = Button::default()
      .with_size(60, 30)
      .with_label("Exit")
      .center_y(&frame_bottom);
    btn_exit.set_pos(self.width-90, btn_exit.y());
    btn_exit.set_color(Color::Red);
    btn_exit.set_callback(|_|
    {
      app::quit();
    });

    btn_prev.set_pos(30, btn_prev.y());
    btn_prev.set_callback({
      closure!(clone mut self.wizard, |_| wizard.prev())
    });

    let mut btn_build = Button::default()
      .with_size(60, 30)
      .with_label("Build")
      .center_of(&frame_bottom);

    btn_build.set_color(Color::DarkGreen);
    btn_build.set_callback(closure!(clone mut btn_build, clone mut btn_prev, clone term, |_|
    {
      let env_appdir = env::var("APPDIR").unwrap_or(String::from("."));
      let cmd_str = format!("{}{}{}", env_appdir, "/usr/bin/", "main.sh");
      let _cmd : Result<(),String> = cmd!(cmd_str, "--yaml")
        .stderr_to_stdout()
        .reader()
        .and_then(|e|
        {
          btn_build.deactivate();
          btn_prev.deactivate();

          let buf = BufReader::new(e);

          std::thread::spawn(closure!(clone mut btn_build, clone mut btn_prev, clone mut term, || {
            buf.lines().filter_map(|line| line.ok()).try_for_each(|line|
            {
                term.insert(&line);
                term.insert("\n");
                term.show_insert_position();
                app::awake();
                Some(())
            });
            btn_build.activate();
            btn_prev.activate();
          }));

          Ok(())
        })
        .or_else(|_|
        {
          term.insert("Invalid gameimage directory\n");
          Ok(())
        });

    }));

    // }}}

    group2.end();
  } // fn: frame_2 }}}

} // }}}

// impl: Drop for Gui {{{
impl Drop for Gui
{
  fn drop(&mut self)
  {
    self.wind.make_resizable(false);
    self.wind.end();
    self.wind.show();
    self.app.run().unwrap();
  }
} // }}}

// fn: main {{{
fn main() {
  let gui = Gui::new();
  gui.frame_1();
  gui.frame_2();
} // fn: main }}}

// cmd: !cargo run

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
