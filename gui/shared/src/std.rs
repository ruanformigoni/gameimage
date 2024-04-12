use std::ffi::{OsStr, OsString};


// pub trait OsStrExt {{{
pub trait OsStrExt
{
  fn string(&self) -> String;
}

impl OsStrExt for OsStr
{
  fn string(&self) -> String
  {
    self.to_string_lossy().into_owned()
  } // fn: string
}
// }}}

// pub trait OsStringExt {{{
pub trait OsStringExt
{
  fn append(&mut self, val: &str) -> &mut Self;
  fn string(&self) -> String;
}

impl OsStringExt for OsString
{
  fn append(&mut self, val: &str) -> &mut Self
  {
    self.push(val);
    self
  }
  fn string(&self) -> String
  {
    self.to_string_lossy().into_owned()
  } // fn: string
}
// }}}

// pub trait PathBufExt {{{
#[allow(warnings)]
pub trait PathBufExt
{
  fn string(&self) -> String;
  fn append_extension(&self, val: &str) -> Self;
  fn prepend(&self, upper: &std::path::PathBuf) -> Self;
  fn file_name_string(&self) -> String;
}

impl PathBufExt for std::path::PathBuf
{
  fn string(&self) -> String
  {
    self.clone().to_string_lossy().into_owned()
  } // fn: string

  fn append_extension(&self, val: &str) -> Self
  {
    std::path::PathBuf::from(self.clone().into_os_string().append(val).string())
  } // fn: extend_extension

  fn prepend(&self, upper: &std::path::PathBuf) -> Self
  {
    upper.join(self)
  } // fn: prepend

  fn file_name_string(&self) -> String
  {
    self.file_name().unwrap_or_default().to_string_lossy().to_string()
  } // fn: file_name_string
}
// }}}

// pub trait VecExt {{{
pub struct VecString
{
  str_owned: Vec<String>,
} // VecString

impl VecString
{
  // Method to access the &str references of the owned strings
  pub fn as_str_slice(&self) -> Vec<&str>
  {
    self.str_owned.iter().map(|s| s.as_str()).collect()
  } // as_str_slice()
} // impl VecString

pub trait VecExt
{
  fn append_strings(&self, other: Vec<String>) -> VecString;
} // trait VecExt

impl VecExt for Vec<&str>
{
  fn append_strings(&self, other: Vec<String>) -> VecString
  {
    // Map self to String
    let mut str_owned: Vec<String> = self.iter().map(|s| s.to_string()).collect();

    // Extend self with other
    str_owned.extend(other);

    // Create VecString
    VecString { str_owned }
  }
} // impl VecExt
// }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
