use std::{
  fs,
  path::PathBuf,
  sync::{Arc, Mutex},
  io::{Read, Write},
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

// struct Term {{{
#[derive(Clone)]
pub struct Term
{
  // Current process in the terminal
  opt_proc_thread : Option<(Arc<Mutex<std::process::Child>>, Arc<Mutex<Option<std::thread::JoinHandle<()>>>>)>,
  // Terminate with signal
  opt_tx : Option<std::sync::mpsc::Sender<bool>>,
  // Terminal gui
  pub term : SimpleTerminal,
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

  let mut clone_term = term.clone();
  let _btn_save = shared::fltk::button::rect::save()
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

  // Return new term
  Term{ term, opt_proc_thread: None, opt_tx: None }
} // new() }}}

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

  // Send stop signal to thread
  if let Some(tx) = self.opt_tx.take() { let _ = tx.send(true); } // if

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
    .stderr(Stdio::inherit())
    .stdout(Stdio::piped())
    .spawn()?;

  // Create arc reader for stdout
  let arc_stdout = Arc::new(Mutex::new(reader_cmd.stdout.take()));

  // Put child in arc
  let arc_reader = Arc::new(Mutex::new(reader_cmd));

  // Kill existing process if any
  self.kill(self.opt_proc_thread.clone());

  // Setup callback
  let mut clone_term = self.term.clone();
  let clone_arc_stdout = arc_stdout.clone();
  let (tx, rx) = mpsc::channel::<bool>();
  let clone_arc_reader = arc_reader.clone();
  let handle = std::thread::spawn(move ||
  {
    // Write buf to stdout
    loop
    {
      // Stop loop when requested
      if let Ok(true) = rx.try_recv() { log!("Killed by message"); break; } // if

      // Wait a bit to avoid over consumption
      std::thread::sleep(std::time::Duration::from_millis(50));

      // Acquire stdout
      let mut guard = match clone_arc_stdout.lock()
      {
        Ok(guard) => guard,
        Err(e) => { log!("Failed to acquire mut stdout: {}", e); return; },
      }; // match

      // Create buf
      let mut buf = vec![0; 4096];

      let bytes_read = if let Some(stdout) = guard.as_mut()
      && let Ok(bytes_read) = stdout.read(&mut buf)
      {
        bytes_read
      }
      else
      {
        log!("Could not get stdout");
        break;
      };

      if bytes_read == 0 { log!("Break subprocess due to no bytes left to read"); break; }
      let output = String::from_utf8_lossy(&buf[..bytes_read]);
      clone_term.insert(&output);
      clone_term.show_insert_position();

      app::awake();
    } // loop

    // Make callback
    let code_return : i32 = if let Ok(mut lock) = clone_arc_reader.lock()
      && let Ok(status) = lock.wait()
      && let Some(code) = status.code() { code }
    else { 1 }; // else
    callback(code_return);
  });

  // Save proc & thread handle
  self.opt_proc_thread = Some((arc_reader.clone(), Arc::new(Mutex::new(Some(handle)))));

  // Transmitter to the thread
  self.opt_tx = Some(tx);

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
