#![allow(warnings)]

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
  wizard::install::install(tx.clone()
    , title
    , "rom"
    , common::Msg::DrawRetroarchIcon
    , common::Msg::DrawRetroarchRom
    , common::Msg::DrawRetroarchCore);
} // }}}

// pub fn core() {{{
pub fn core(tx: Sender<common::Msg>, title: &str)
{
  wizard::install::install(tx.clone()
    , title
    , "core"
    , common::Msg::DrawRetroarchRom
    , common::Msg::DrawRetroarchCore
    , common::Msg::DrawRetroarchBios);
} // }}}

// pub fn bios() {{{
pub fn bios(tx: Sender<common::Msg>, title: &str)
{
  wizard::install::install(tx.clone()
    , title
    , "bios"
    , common::Msg::DrawRetroarchCore
    , common::Msg::DrawRetroarchBios
    , common::Msg::DrawRetroarchTest);
} // }}}

// pub fn test() {{{
pub fn test(tx: Sender<common::Msg>, title: &str)
{
  wizard::test::test(tx.clone()
    , title
    , common::Msg::DrawRetroarchBios
    , common::Msg::DrawRetroarchTest
    , common::Msg::DrawRetroarchCompress);
} // }}}

// pub fn compress() {{{
pub fn compress(tx: Sender<common::Msg>, title: &str)
{
  wizard::compress::compress(tx.clone()
    , title
    , common::Msg::DrawRetroarchTest
    , common::Msg::DrawRetroarchCompress
    , common::Msg::DrawCreator);
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
