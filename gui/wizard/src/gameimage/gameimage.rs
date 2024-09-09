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
  let (tx, rx) = mpsc::channel();
  std::thread::spawn(move ||
  {
    let clone_arc_stdout = arc_stdout.clone();
    let clone_tx = tx.clone();
    let handle_stdout = std::thread::spawn(move ||
    {
      // Use buf to write buf to stdout
      loop
      {
        let mut buf = vec![0; 4096];
        let mut output;
        let mut lock_stdout = match clone_arc_stdout.lock()
        {
          Ok(lock) => lock,
          Err(e) => { log!("Could not acquire lock (stdout): {}", e); let _ = clone_tx.send(1); return; },
        };
        let bytes_read = match lock_stdout.as_mut()
        {
          Some(lock) =>
          {
            match lock.read(&mut buf)
            {
              Ok(bytes_read) => bytes_read,
              Err(_) => break,
            } // match
          } // Some
          None => { eprintln!("Could not get lock for stdout"); break; }
        };
        if bytes_read == 0 { break; }
        output = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
        output = output.trim().to_string();
        log!("{}", &output);
      } // loop
    });

    let clone_arc_stderr = arc_stderr.clone();
    let clone_tx = tx.clone();
    let handle_stderr = std::thread::spawn(move ||
    {
      loop
      {
        let mut buf = vec![0; 4096];
        let mut output;
        let mut lock_stderr = match clone_arc_stderr.lock()
        {
          Ok(lock) => lock,
          Err(e) => { log!("Could not acquire lock (stderr): {}", e); let _ = clone_tx.send(1); return; },
        };
        let bytes_read = match lock_stderr.as_mut()
        {
          Some(lock) =>
          {
            match lock.read(&mut buf)
            {
              Ok(bytes_read) => bytes_read,
              Err(_) => break,
            } // match
          } // Some
          None => { eprintln!("Could not get lock for stdout"); break; }
        };
        if bytes_read == 0 { break; }
        output = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
        output = output.trim().to_string();
        log!("{}", &output);
      } // loop
    });

    let _ = handle_stdout.join();
    let _ = handle_stderr.join();

    if let Ok(mut guard) = arc_handle.lock()
    && let Ok(status) = guard.wait()
    && let Some(code) = status.code()
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
