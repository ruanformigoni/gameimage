use std::
{
  io::prelude::*,
  env,
  sync::{Arc,Mutex,mpsc},
};

use crate::common;
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

// pub fn gameimage_async() {{{
pub fn gameimage_async(args : Vec<&str>) -> anyhow::Result<mpsc::Receiver<i32>>
{
  dir_build()?;

  let path_binary_gameimage = binary()?;

  let handle = std::process::Command::new(path_binary_gameimage)
    .env_remove("LD_PRELOAD")
    .env("FIM_FIFO", "0")
    .stderr(std::process::Stdio::inherit())
    .stdout(std::process::Stdio::piped())
    .args(args)
    .spawn()?;


  // Create arc reader for stdout
  let arc_handle = Arc::new(Mutex::new(handle));

  // Clone process handle
  let clone_arc_handle = arc_handle.clone();

  // Create t/r
  let (tx, rx) = mpsc::channel();
  std::thread::spawn(move ||
  {
    // Acquire stdout
    let mut lock =
      if let Ok(lock) = clone_arc_handle.lock() && lock.stdout.is_some()
      {
        lock
      }
      else
      {
        log!("Could not acquire lock");
        let _ = tx.send(1);
        return; 
      }; // else

    // Create buf
    let mut buf = vec![0; 4096];

    // Use buf to write buf to stdout & stderr
    loop
    {
      std::thread::sleep(std::time::Duration::from_millis(50));

      let bytes_read = match lock.stdout.as_mut().unwrap().read(&mut buf)
      {
        Ok(bytes_read) => bytes_read,
        Err(_) => break,
      };

      if bytes_read == 0 { break; }
      let output = String::from_utf8_lossy(&buf[..bytes_read]);
      log!("{}", &output);
    }

    if let Ok(status) = lock.wait() && let Some(code) = status.code()
    {
      let _ = tx.send(code);
    }
    else
    {
      let _ = tx.send(1);
    } // else

    fltk::app::awake();
  });

  Ok(rx)
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
