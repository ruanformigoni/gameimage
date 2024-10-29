use std::
{
  env,
  sync::{Arc,Mutex,mpsc},
};

use crate::common;
use crate::log_err;
use crate::log;

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
    let f_callback = |tx : mpsc::Sender<String>, msg| { log_err!(tx.send(msg)); };
    let handle_stdout = std::thread::spawn(common::log_fd(stdout.unwrap(), tx_log.clone(), f_callback));
    let handle_stderr = std::thread::spawn(common::log_fd(stderr.unwrap(), tx_log, f_callback));
    while let Ok(msg) = rx_log.recv()
    {
      log!("{}", msg);
      fltk::app::awake();
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
