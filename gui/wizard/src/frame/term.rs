use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::io::{Read, Write};
use std::process::{Command, Stdio, Child};

use fltk::{
  app,
  // app::{Sender,Receiver},
  button::Button,
  dialog::file_chooser,
  prelude::*,
  enums::{FrameType,Color},
  text::SimpleTerminal,
};

use anyhow::anyhow as ah;

use crate::dimm;
use crate::common;
use crate::log;
use crate::lib::svg;

pub struct Term
{
  pub term : SimpleTerminal,
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
  let mut clone_term = term.clone();
  btn_save.set_callback(move |_|
  {
    // Get file name
    let some_path_file_dest : Option<PathBuf> = file_chooser("Save as...",  "*.txt", ".", true)
      .map(|e| PathBuf::from(e) );

    if some_path_file_dest.is_none()
    {
      clone_term.append("No file selected\n");
      return;
    } // if

    let path_file_dest = some_path_file_dest.unwrap();

    // Get contents of terminal
    let str_contents = clone_term.text();

    // Open dest file as write
    let file_dest = fs::File::create(path_file_dest.clone());

    if file_dest.is_err()
    {
      clone_term.append(format!("Failed to open file {}", path_file_dest.to_str().unwrap()).as_str());
    } // if

    // Write to file
    let _ = writeln!(&mut file_dest.unwrap(), "{}", str_contents);
  });

  // Return new term
  Term{ term }
} // new() }}}

// pub fn dispatch() {{{
pub fn dispatch<F>(&self, args : Vec<&str>, callback : F) -> anyhow::Result<Arc<Mutex<Child>>>
  where F : Fn(i32) + Send + 'static
{
  let (cmd_base, cmd_args) = args.split_first().ok_or(ah!("No command to execute"))?;

  let reader_cmd = Command::new(cmd_base)
    .env_remove("LD_PRELOAD")
    .env("FIM_FIFO", "0")
    .args(cmd_args)
    .stderr(Stdio::inherit())
    .stdout(Stdio::piped())
    .spawn()?;

  // Create arc reader for stdout
  let arc_reader = Arc::new(Mutex::new(reader_cmd));

  // Write stdout to terminal
  let mut clone_term = self.term.clone();
  let clone_arc_reader = arc_reader.clone();
  std::thread::spawn(move ||
  {
    // Acquire stdout
    let mut lock =
      if let Ok(lock) = clone_arc_reader.lock() && lock.stdout.is_some()
      {
        lock
      }
      else
      {
        clone_term.append("Failed to acquire mut stdout\n");
        return;
      }; // else

    // Create buf
    let mut buf = vec![0; 4096];

    // Write buf to stdout
    loop
    {
      std::thread::sleep(std::time::Duration::from_millis(50));

      let bytes_read =
        if   let Some(stdout) = lock.stdout.as_mut()
          && let Ok(bytes_read) = stdout.read(&mut buf)
        {
          bytes_read
        }
        else
        {
          log!("Could not get stdout");
          break;
        };

      if bytes_read == 0 { break; }
      let output = String::from_utf8_lossy(&buf[..bytes_read]);
      clone_term.insert(&output);
      clone_term.show_insert_position();

      app::awake();
    }

    let code_return : i32 =
      if let Ok(status) = lock.wait()
      && let Some(code) = status.code()
    {
      code
    }
    else
    {
      1
    }; // else

    callback(code_return);
  });


  Ok(arc_reader.clone())
} // dispatch() }}}

// pub fn append() {{{
pub fn append(&self, value: &str)
{
  let mut clone_term = self.term.clone();
  clone_term.append(value);
  clone_term.show_insert_position();
  app::awake();
} // fn: append }}}

} // impl

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
