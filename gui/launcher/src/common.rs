use anyhow::anyhow as ah;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Msg
{
  DrawCover,
  DrawSelector,
  DrawEnv,
  DrawExecutables,
  DrawMenu,
  WindActivate,
  WindDeactivate,
  Quit,
} // enum

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Platform
{
  LINUX,
  WINE,
  RETROARCH,
  PCSX2,
  RPCS3,
} // Platform

// impl Platform {{{
impl Platform
{
  pub fn as_str(&self) -> &'static str
  {
    match self
    {
      Platform::LINUX     => "linux",
      Platform::WINE      => "wine",
      Platform::RETROARCH => "retroarch",
      Platform::PCSX2     => "pcsx2",
      Platform::RPCS3     => "rpcs3",
    } // match
  } // as_str

  pub fn from_str(src : &str) -> anyhow::Result<Platform>
  {
    match src.to_lowercase().as_str()
    {
      "linux"     => Ok(Platform::LINUX),
      "wine"      => Ok(Platform::WINE),
      "retroarch" => Ok(Platform::RETROARCH),
      "pcsx2"     => Ok(Platform::PCSX2),
      "rpcs3"     => Ok(Platform::RPCS3),
      &_ => Err(ah!("Could not determine platform with string")),
    } // match
  } // as_str
} // impl Platform }}}

#[macro_export]
macro_rules! assign_to_arc_mutex
{
  ($arc_mutex:expr, $value:expr) =>
  {
    {
      let mut data = $arc_mutex.lock().unwrap();
      *data = $value;
    }
  };
}

#[macro_export]
macro_rules! call_with_args
{
  ($func:ident, $( $obj:expr ),* ) =>
  {
    $(
      $obj.$func();
    )*
  };
}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
