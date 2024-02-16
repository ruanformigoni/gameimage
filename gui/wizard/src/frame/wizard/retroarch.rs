#![allow(dead_code)]
#![allow(unused_variables)]

use fltk::app::Sender;

use crate::common;
use crate::frame::wizard;

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>, title: &str)
{
  wizard::name::name(tx.clone(), title);
} // }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>, title: &str)
{
  wizard::icon::icon(tx.clone(), title);
} // }}}

// pub fn rom() {{{
pub fn rom(tx: Sender<common::Msg>, title: &str)
{
  wizard::rom::rom(tx.clone(), title);
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
