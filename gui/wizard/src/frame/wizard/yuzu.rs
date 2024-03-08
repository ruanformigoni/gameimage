#![allow(warnings)]

use fltk::app::Sender;

use crate::common;
use crate::frame::wizard;

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>, title: &str)
{
  wizard::name::name(tx.clone()
    , title
    , common::Msg::DrawCreator
    , common::Msg::DrawYuzuIcon);
} // }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>, title: &str)
{
  wizard::icon::icon(tx.clone()
    , title
    , common::Msg::DrawYuzuName
    , common::Msg::DrawYuzuRom
  );
} // }}}

// pub fn rom() {{{
pub fn rom(tx: Sender<common::Msg>, title: &str)
{
  wizard::install::install(tx.clone()
    , title
    , "rom"
    , common::Msg::DrawYuzuIcon
    , common::Msg::DrawYuzuRom
    , common::Msg::DrawYuzuBios);
} // }}}

// pub fn bios() {{{
pub fn bios(tx: Sender<common::Msg>, title: &str)
{
  wizard::install::install(tx.clone()
    , title
    , "bios"
    , common::Msg::DrawYuzuRom
    , common::Msg::DrawYuzuBios
    , common::Msg::DrawYuzuKeys);
} // }}}

// pub fn keys() {{{
pub fn keys(tx: Sender<common::Msg>, title: &str)
{
  wizard::install::install(tx.clone()
    , title
    , "keys"
    , common::Msg::DrawYuzuBios
    , common::Msg::DrawYuzuKeys
    , common::Msg::DrawPcsx2Test);
} // }}}

// pub fn test() {{{
pub fn test(tx: Sender<common::Msg>, title: &str)
{
  wizard::test::test(tx.clone()
    , title
    , common::Msg::DrawYuzuKeys
    , common::Msg::DrawYuzuTest
    , common::Msg::DrawYuzuCompress);
} // }}}

// pub fn compress() {{{
pub fn compress(tx: Sender<common::Msg>, title: &str)
{
  wizard::compress::compress(tx.clone()
    , title
    , common::Msg::DrawYuzuTest
    , common::Msg::DrawYuzuCompress
    , common::Msg::DrawCreator);
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
