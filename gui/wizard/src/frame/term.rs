#![allow(warnings)]

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
  dialog::{file_chooser, dir_chooser, FileChooser, FileChooserType, FileDialogOptions, NativeFileChooser, FileDialogType, NativeFileChooserOptions},
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

use crate::dimm;
use crate::svg;

pub struct Term
{
  term : SimpleTerminal,
}

impl Term
{

// pub fn new() {{{
pub fn new(border : i32, width : i32, height : i32, x : i32, y : i32) -> Term
{
  let mut term = SimpleTerminal::new(border
    , border
    , width - dimm::border() - dimm::width_button_rec()
    , height
    , "");
  term.set_pos(x, y);
  term.set_text_color(Color::White);
  term.set_text_size((dimm::height_text() as f64 * 0.7) as i32);
  term.wrap_mode(fltk::text::WrapMode::None, 0);

  let mut btn_save = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .right_of(&term, dimm::border());
  btn_save.set_frame(FrameType::RoundedFrame);
  btn_save.visible_focus(false);
  btn_save.set_pos(btn_save.x(), term.y());
  btn_save.set_color(Color::Blue);
  btn_save.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_save(1.0).as_str()).unwrap()));
  let clone_term = term.clone();
  btn_save.set_callback(move |_|
  {
    // Get file name
    let some_path_file_dest : Option<PathBuf> = file_chooser("Save as...",  "*.txt", ".", true)
      .map(|e| PathBuf::from(e) );

    if some_path_file_dest.is_none()
    {
      println!("No file selected");
      return;
    } // if

    let mut path_file_dest = some_path_file_dest.unwrap();

    // Get contents of terminal
    let str_contents = clone_term.text();

    // Open dest file as write
    let file_dest = fs::File::create(path_file_dest.clone());

    if file_dest.is_err()
    {
      println!("Failed to open file {}", path_file_dest.to_str().unwrap());
    } // if

    // Write to file
    writeln!(&mut file_dest.unwrap(), "{}", str_contents);
  });

  // Return new term
  Term{ term }
} // new() }}}

// pub fn dispatch() {{{
pub fn dispatch<F>(&self, args : Vec<&str>, callback : F)
  where F : Fn() + Send + 'static
{
  let reader_cmd = Command::new("sh")
    .env_remove("LD_PRELOAD")
    .env("FIM_FIFO", "0")
    .args(vec!["-c"].into_iter().chain(args).collect::<Vec<&str>>())
    .stdin(Stdio::piped())
    .stderr(Stdio::inherit())
    .stdout(Stdio::piped())
    .spawn()
    .expect("Could not dispatch command");

  // Create arc reader for stdout
  let arc_reader_stdout = Arc::new(Mutex::new(reader_cmd.stdout));

  // Write stdout to terminal
  let mut clone_term = self.term.clone();
  let clone_arc_reader_stdout = arc_reader_stdout.clone();
  std::thread::spawn(move ||
  {
    // Acquire lock
    let lock = (&*clone_arc_reader_stdout).lock();
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
      clone_term.insert(&output);
      clone_term.show_insert_position();

      app::awake();
    }

    callback();
  });
} // dispatch() }}}

} // impl

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
