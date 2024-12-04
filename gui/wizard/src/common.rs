use std::
{
  sync::OnceLock,
  env,
  path,
};

// pub enum Platform {{{
#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Platform
{
  Linux,
  Wine,
  Retroarch,
  Pcsx2,
  Rcps3,
} // }}}

// impl Platform {{{
impl Platform
{
  pub fn as_str(&self) -> &'static str
  {
    match self
    {
      Platform::Linux     => "linux",
      Platform::Wine      => "wine",
      Platform::Retroarch => "retroarch",
      Platform::Pcsx2     => "pcsx2",
      Platform::Rcps3     => "rpcs3",
    } // match
  } // as_str

  pub fn from_str(src : &str) -> Option<Platform>
  {
    match src
    {
      "linux"     => Some(Platform::Linux),
      "wine"      => Some(Platform::Wine),
      "retroarch" => Some(Platform::Retroarch),
      "pcsx2"     => Some(Platform::Pcsx2),
      "rpcs3"     => Some(Platform::Rcps3),
      _           => None,
    } // match
  } // as_str
} // impl IconFrame }}}

// pub enum Msg {{{
#[derive(Debug, Clone, Copy)]
pub enum Msg
{
  DrawWelcome,
  DrawPlatform,
  DrawCreator,
  DrawDesktop,

  DrawLinuxName,
  DrawLinuxIcon,
  DrawLinuxMethod,
  DrawLinuxRom,
  DrawLinuxDefault,
  DrawLinuxCompress,

  DrawWineName,
  DrawWineIcon,
  DrawWineConfigure,
  DrawWineTricks,
  DrawWineEnvironment,
  DrawWineRom,
  DrawWineCompress,

  DrawRetroarchName,
  DrawRetroarchIcon,
  DrawRetroarchRom,
  DrawRetroarchCore,
  DrawRetroarchBios,
  DrawRetroarchTest,
  DrawRetroarchCompress,

  DrawPcsx2Name,
  DrawPcsx2Icon,
  DrawPcsx2Rom,
  DrawPcsx2Bios,
  DrawPcsx2Test,
  DrawPcsx2Compress,

  DrawRpcs3Name,
  DrawRpcs3Icon,
  DrawRpcs3Rom,
  DrawRpcs3Bios,
  DrawRpcs3Test,
  DrawRpcs3Compress,

  DrawFinish,

  WindActivate,
  WindDeactivate,
  WindUpdate,

  Quit,
} // }}}

// impl_log() {{{
pub fn impl_log(value : &str)
{
  static TX: OnceLock<std::sync::mpsc::Sender<String>> = OnceLock::new();

  // Initialize the logging channel and logger thread the first time impl_log is called
  let sender = TX.get_or_init(||
  {
    // Create a channel for log messages
    let (tx, rx) = std::sync::mpsc::channel::<String>();

    // Spawn a singleton logger thread that will consume messages and append them to the terminal
    std::thread::spawn(move ||
    {
      // Fetch terminal
      let mut term: Option<fltk::text::SimpleTerminal>;
      loop
      {
        let widget: Option<fltk::text::SimpleTerminal> = fltk::app::widget_from_id("term_log");
        if widget.is_some() { term = widget; break; }
      }

      while let Ok(mut log_message) = rx.recv()
      {
        if ! log_message.ends_with("\n") { log_message = format!("{}\n", log_message); }
        term.as_mut().unwrap().append(&log_message);
      }
    }); // std::thread

    // Return the sender for future calls to impl_log
    tx
  });

  // Send the log message to the logger thread
  if let Err(e) = sender.send(value.to_string()) {
    println!("Failed to send log message: {}", e);
  }
} // impl_log() }}}

// macro_rules log! {{{
#[macro_export]
macro_rules! log
{
  ($($arg:tt)*) =>
  {
    {
      let output = format!("{}:{}: "
        , std::path::PathBuf::from(file!()).file_name().unwrap_or_default().to_str().unwrap_or("Unknown file")
        , line!()) + &format!($($arg)*);
      common::impl_log(output.as_str());
      eprintln!("{}", output);
    }
  }
}
// }}}

// macro_rules log_alert! {{{
#[macro_export]
macro_rules! log_alert
{
  ($($arg:tt)*) =>
  {
    {
      let output = format!($($arg)*);
      common::impl_log(output.as_str());
      fltk::dialog::alert_default(output.as_str());
      eprintln!("{}", output);
    }
  }
}
// }}}

// macro_rules log_err! {{{
#[macro_export]
macro_rules! log_err
{
  ($result:expr) =>
  {
    match $result
    {
      Ok(()) => (),
      Err(e) => log!("{:?}", e),
    }
  }
}
// }}}

// macro_rules log_status! {{{
#[macro_export]
macro_rules! log_status
{
  ($($arg:tt)*) =>
  {
    {
      let output = format!($($arg)*);
      common::impl_log(output.as_str());
      let mut status: fltk::output::Output = fltk::app::widget_from_id("footer_status").unwrap();
      status.set_value(&output);
      eprintln!("{}", output);
    }
  }
}
// }}}

// macro_rules log_err_status! {{{
#[macro_export]
macro_rules! log_err_status
{
  ($result:expr) =>
  {
    if let Err(e) = $result
    {
      let err = e.to_string();
      common::impl_log(err.as_str());
      let mut status: fltk::output::Output = fltk::app::widget_from_id("footer_status").unwrap();
      status.set_value(&err);
      eprintln!("{}", err);
    }
  }
}
// }}}

// macro_rules log_return_err! {{{
#[macro_export]
macro_rules! log_return_err
{
  ($($arg:tt)*) => { { log!($($arg)*); return Err(ah!($($arg)*)); } }
}
// }}}

// macro_rules log_return_void! {{{
#[macro_export]
macro_rules! log_return_void
{
  ($($arg:tt)*) => { { log!($($arg)*); return; } }
}
// }}}

// fn: log_fd() {{{
pub fn log_fd<T: std::io::Read, F: FnMut(std::sync::mpsc::Sender<String>, String)>(mut fd: T
  , tx: std::sync::mpsc::Sender<String>
  , mut f_callback: F) -> impl FnMut() -> ()
{
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
      f_callback(tx.clone(),output);
    } // loop
  }; // return
} // fn: log_fd() }}}

// pub fn dir_build() {{{
pub fn dir_build() -> anyhow::Result<()>
{
  Ok(env::set_current_dir(path::PathBuf::from(env::var("GIMG_DIR")?))?)
} // fn: dir_build }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
