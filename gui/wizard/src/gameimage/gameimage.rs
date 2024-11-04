use std::
{
  env,
  sync::{Arc,Mutex,mpsc},
};

use anyhow::anyhow as ah;

use crate::lib;
use crate::common;
use crate::log_err;
use crate::log;



// fn binary() {{{
pub fn binary() -> anyhow::Result<std::path::PathBuf>
{
  Ok(which::which("gameimage-cli")?)
} // }}}

// pub fn dir_build() {{{
pub fn dir_build() -> anyhow::Result<()>
{
  Ok(env::set_current_dir(std::path::PathBuf::from(env::var("GIMG_DIR")?))?)
} // fn: dir_build }}}

// pub fn gameimage_async() {{{
pub fn gameimage_async(args : Vec<&str>) -> anyhow::Result<(mpsc::Receiver<String>, mpsc::Receiver<i32>)>
{
  dir_build()?;

  let path_binary_gameimage = binary()?;

  // Open ipc
  let ipc = match lib::ipc::Ipc::new()
  {
    Ok(ipc) => ipc,
    Err(e) => { return Err(ah!("Could not create ipc instance: {}", e)); },
  }; // match

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

    // Close IPC
    lib::ipc::Ipc::close();

    // Send exit code
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

  let (tx_ipc, rx_ipc) = mpsc::channel();
  // Write from ipc to channel
  std::thread::spawn(move ||
  {
    // Write received message to transmitter
    while let Ok(msg) = ipc.recv()
    {
      if let Err(e) = tx_ipc.send(msg)
      {
        log!("Could not send ipc retrieved message: {}", e);
      } // if
    } // while
  });

  Ok((rx_ipc, rx_code))
} // fn: gameimage_async }}}

// pub fn gameimage_sync() {{{
pub fn gameimage_sync(args : Vec<&str>) -> i32
{
  let (_, rx_code) = match gameimage_async(args)
  {
    Ok((rx_ipc, rx_code)) => (rx_ipc, rx_code),
    Err(e) => { log!("Could not start backend: {}", e); return 1; },
  }; // if

  rx_code.recv().unwrap_or(1)
} // fn: gameimage_sync }}}

// pub fn gameimage_sync_ipc() {{{
pub fn gameimage_sync_ipc<F>(args : Vec<&str>, mut f: F) -> i32
  where F: FnMut(mpsc::Receiver<String>) + Send + 'static
{
  let (rx_ipc, rx_code) = match gameimage_async(args)
  {
    Ok((rx_ipc, rx_code)) => (rx_ipc, rx_code),
    Err(e) => { log!("Could not start backend: {}", e); return 1; },
  }; // if

  // Spawn message receiver thread
  std::thread::spawn(move || { f(rx_ipc); });

  rx_code.recv().unwrap_or(1)
} // fn: gameimage_sync_ipc }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
