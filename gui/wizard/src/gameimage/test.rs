use anyhow::anyhow as ah;

use crate::log;
use crate::common;
use crate::gameimage::gameimage;

// pub fn test() {{{
pub fn test() -> anyhow::Result<()>
{
  // Wait for message & check return value
  match gameimage::gameimage_sync(vec!["test"])
  {
    0 => { log!("test returned successfully"); return Ok(()) },
    ret => return Err(ah!("test returned with error: {}", ret)),
  } // match
} // fn: test }}}
