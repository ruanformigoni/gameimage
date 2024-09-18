use std::
{
  io::prelude::*,
  env,
  sync::{Arc,Mutex,mpsc},
};

use crate::common;
use crate::log_err;
use crate::log;

// fn platform() {{{
pub fn platform() -> anyhow::Result<String>
{
  Ok(env::var("GIMG_PLATFORM")?.to_lowercase())
} // }}}

// fn binary() {{{
pub fn binary() -> anyhow::Result<std::path::PathBuf>
{
  Ok(std::path::PathBuf::from(env::var("GIMG_BACKEND")?))
} // }}}

// pub fn dir_build() {{{
pub fn dir_build() -> anyhow::Result<()>
{
  Ok(env::set_current_dir(std::path::PathBuf::from(env::var("GIMG_DIR")?))?)
} // fn: dir_build }}}

// fn: log_fd() {{{
fn log_fd<T: Read>(mut fd : T, tx : mpsc::Sender<String>) -> impl FnMut() -> ()
{
  // let (rx, tx) = mpsc::channel();
  return move ||
  {
    // Use buf to write buf to stdout
    loop
    {
      let mut buf = vec![0; 4096];
      let bytes_read = match fd.read(&mut buf)
      {
        Ok(bytes_read) => bytes_read,
        Err(_) => break,
      }; // match
      if bytes_read == 0 { break; }
      let mut output = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
      output = output.trim().to_string();
      let _ = tx.send(output);
    } // loop
  }; // return
} // fn: log_fd() }}}

// pub fn gameimage_async() {{{
pub fn gameimage_async(args : Vec<&str>) -> anyhow::Result<mpsc::Receiver<i32>>
{
  dir_build()?;

  let path_binary_gameimage = binary()?;

  let mut handle = std::process::Command::new(&path_binary_gameimage)
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::piped())
    .args(&args)
    .spawn()?;

  log!("Dispatch command: {:?} : {:?}", path_binary_gameimage, args);

  // Create arc reader for stdout
  let arc_stdout = Arc::new(Mutex::new(handle.stdout.take()));
  let arc_stderr = Arc::new(Mutex::new(handle.stderr.take()));
  let arc_handle = Arc::new(Mutex::new(handle));

  // Create t/r
  let stdout = arc_stdout.lock().unwrap().take();
  let stderr = arc_stderr.lock().unwrap().take();
  let (tx_code, rx_code) = mpsc::channel();
  std::thread::spawn(move ||
  {
    let (tx_log, rx_log) = mpsc::channel();
    let handle_stdout = std::thread::spawn(log_fd(stdout.unwrap(), tx_log.clone()));
    let handle_stderr = std::thread::spawn(log_fd(stderr.unwrap(), tx_log));
    while let Ok(msg) = rx_log.recv()
    {
      log!("{}", msg);
    } // while

    log_err!(handle_stdout.join());
    log_err!(handle_stderr.join());

    if let Ok(mut guard) = arc_handle.lock()
    && let Ok(status) = guard.wait()
    && let Some(code) = status.code()
    {
      log_err!(tx_code.send(code));
    }
    else
    {
      log_err!(tx_code.send(1));
    } // else

    fltk::app::awake();
  });

  Ok(rx_code)
} // fn: gameimage_async }}}

// pub fn gameimage_sync() {{{
pub fn gameimage_sync(args : Vec<&str>) -> i32
{
  if let Ok(rx) = gameimage_async(args)
  && let Ok(code) = rx.recv()
  {
    return code;
  } // if

  log!("Could not retrieve exit code from backend");

  1
} // fn: gameimage_sync }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
