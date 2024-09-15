use crate::scaling;
use anyhow::anyhow as ah;

pub fn scale() -> anyhow::Result<()>
{
  if fltk::app::screen_count() < 1
  {
    return Err(ah!("Screen count is less than one"));
  } // if

  // Detect proper scale
  match scaling::factor()
  {
    Some(scale) => { fltk::app::set_screen_scale(0, scale); Ok(())},
    None => Err(ah!("Could not get factor to apply scaling"))
  } // match
} // scale

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
