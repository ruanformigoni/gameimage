use fltk::{
  prelude::*,
  frame::Frame,
};

use shared::column;

// fn: new {{{
pub fn new()
{
  column!(col, Frame::default().with_label("No game found inside this image"););
} // fn: new }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
