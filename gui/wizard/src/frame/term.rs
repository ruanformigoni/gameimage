use std::{
  fs,
  path::PathBuf,
  sync::{Arc, Mutex},
  io::Write,
  process::{Command, Stdio, Child},
  sync::mpsc,
};

use fltk::{
  app,
  dialog::file_chooser,
  prelude::*,
  enums::Color,
  text::SimpleTerminal,
};

use anyhow::anyhow as ah;
use shared::fltk::WidgetExtExtra;

use crate::dimm;
use crate::common;
use crate::log;
use crate::log_err;

// struct Term {{{
#[derive(Clone)]
pub struct Term
{
  // Current process in the terminal
  opt_proc_thread : Option<(Arc<Mutex<std::process::Child>>, Arc<Mutex<Option<std::thread::JoinHandle<()>>>>)>,
  // Terminal gui
  pub term : SimpleTerminal,
  // Terminal message sender
  pub tx : mpsc::Sender<String>,
  // Save button
  pub btn_save : fltk::button::Button,
  // Group
  pub group : fltk::group::Group,
} // struct Term }}}

impl Drop for Term
{

  // drop() {{{
  fn drop(&mut self)
  {
    self.kill(self.opt_proc_thread.clone());
  } // drop() }}}

} // impl

impl Term
{

// pub fn new() {{{
pub fn new(border : i32, width : i32, height : i32, x : i32, y : i32) -> Term
{
  let group = fltk::group::Group::default()
    .with_size(width, height)
    .with_frame(fltk::enums::FrameType::FlatBox)
    .with_color(Color::BackGround)
    .with_pos(x,y);

  let mut term = SimpleTerminal::new(border
    , border
    , width - dimm::border() - dimm::width_button_rec()
    , height
    , "");
  term.set_pos(x, y);
  term.set_text_color(Color::White);
  term.set_text_size(dimm::height_text());
  term.wrap_mode(fltk::text::WrapMode::None, 0);
  term.set_history_lines(std::i32::MAX);
  term.set_scrollbar_size(dimm::border());

  let mut clone_term = term.clone();
  let btn_save = shared::fltk::button::rect::save()
    .right_of(&term, dimm::border())
    .with_posy_of(&term)
    .with_color(Color::Blue)
    .with_callback (move |_|
    {
      let path_file_dest = match file_chooser("Save as...",  "*.txt", ".", true).map(|e| PathBuf::from(e) )
      {
        Some(e) => PathBuf::from(e),
        None => { clone_term.append("No file selected\n"); return; },
      }; // match

      // Open dest file as write
      let mut file_dest = match fs::File::create(path_file_dest.clone())
      {
        Ok(e) => { clone_term.append(format!("Failed to open file {}", path_file_dest.to_str().unwrap()).as_str()); e },
        Err(e) => { log!("Error to save selected file: {}", e); return; },
      };

      // Write to file
      let _ = writeln!(&mut file_dest, "{}", clone_term.text());
    });

  // Stay at the bottom
  term.set_stay_at_bottom(true);

  // Create sender and receiver
  let (tx, rx) = mpsc::channel::<String>();

  // Dispatch message writer thread
  let mut clone_term = term.clone();
  std::thread::spawn(move ||
  {
    while let Ok(msg) = rx.recv()
    {
      clone_term.append(&msg);
      clone_term.append("\n");
      app::awake();
    } // while
  });

  group.end();

  // Return new term
  Term{ term, opt_proc_thread: None, tx, btn_save, group }
} // new() }}}

// pub fn new_with_id() {{{
pub fn new_with_id(id: &str, border : i32, width : i32, height : i32, x : i32, y : i32) -> Term
{
  let mut term = Self::new(border, width, height, x, y);
  term.term.set_id(id);
  term
} // new_with_id() }}}

// kill() {{{
fn kill(&mut self, opt_proc_thread : Option<(Arc<Mutex<std::process::Child>>, Arc<Mutex<Option<std::thread::JoinHandle<()>>>>)>)
{
  let (proc, thread) = match opt_proc_thread
  {
    Some(e) => (e.0, e.1),
    None => { log!("No process to terminate"); return; },
  }; // match

  // Kill process
  match proc.lock()
  {
    Ok(mut guard) => { let _ = guard.kill(); let _ = guard.wait(); },
    Err(e) => { log!("Could not lock arc with error: {}", e); return; }
  }; // match

  // Wait for thread
  match thread.lock()
  {
    Ok(mut guard) => { let _ = guard.take().unwrap().join(); },
    Err(e) => { log!("Could not lock arc with error: {}", e); return; }
  }; // match
} // kill() }}}

// pub fn dispatch() {{{
pub fn dispatch<F>(&mut self, args : Vec<&str>, mut callback : F) -> anyhow::Result<Arc<Mutex<Child>>>
  where F : FnMut(i32) + Send + 'static
{
  let (cmd_base, cmd_args) = args.split_first().ok_or(ah!("No command to execute"))?;

  let mut reader_cmd = Command::new(cmd_base)
    .env_remove("LD_PRELOAD")
    .env("FIM_FIFO", "0")
    .args(cmd_args)
    .stdin(Stdio::piped())
    .stderr(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;

  // Create arc reader for stdout
  let arc_stdout = Arc::new(Mutex::new(reader_cmd.stdout.take()));
  let arc_stderr = Arc::new(Mutex::new(reader_cmd.stderr.take()));

  // Put child in arc
  let arc_reader = Arc::new(Mutex::new(reader_cmd));

  // Kill existing process if any
  self.kill(self.opt_proc_thread.clone());

  // Setup callback
  let clone_arc_stdout = arc_stdout.clone();
  let clone_arc_stderr = arc_stderr.clone();
  let clone_arc_reader = arc_reader.clone();
  let clone_tx = self.tx.clone();
  let handle = std::thread::spawn(move ||
  {
    let (tx_log, rx_log) = mpsc::channel::<String>();
    let f_callback = move |tx : mpsc::Sender<String>, msg : String| { log_err!(tx.send(msg)); };
    let handle_stdout = std::thread::spawn(common::log_fd(clone_arc_stdout.lock().unwrap().take().unwrap(), tx_log.clone(), f_callback.clone()));
    let handle_stderr = std::thread::spawn(common::log_fd(clone_arc_stderr.lock().unwrap().take().unwrap(), tx_log, f_callback));
    while let Ok(msg) = rx_log.recv()
    {
      let _ = clone_tx.send(msg);
    } // while
    log_err!(handle_stdout.join());
    log_err!(handle_stderr.join());

    // Make callback
    let code_return : i32 = if let Ok(mut lock) = clone_arc_reader.lock()
      && let Ok(status) = lock.wait()
      && let Some(code) = status.code() { code }
    else { 1 }; // else
    callback(code_return);
  });

  // Save proc & thread handle
  self.opt_proc_thread = Some((arc_reader.clone(), Arc::new(Mutex::new(Some(handle)))));

  Ok(arc_reader.clone())
} // dispatch() }}}

// pub fn append() {{{
pub fn append(&self, value: &str)
{
  let _ = self.tx.send(value.to_string());
} // fn: append }}}

} // impl

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
