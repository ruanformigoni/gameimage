use fltk::app::Sender;

use crate::common;
use crate::wizard;

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>, title: &str)
{
  wizard::name::name(tx.clone()
    , title
    , common::Msg::DrawCreator
    , common::Msg::DrawRyujinxIcon);
} // }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>, title: &str)
{
  wizard::icon::icon(tx.clone()
    , title
    , common::Msg::DrawRyujinxName
    , common::Msg::DrawRyujinxIcon
    , common::Msg::DrawRyujinxRom
  );
} // }}}

// pub fn rom() {{{
pub fn rom(tx: Sender<common::Msg>, title: &str)
{
  wizard::install::install(tx.clone()
    , title
    , "rom"
    , common::Msg::DrawRyujinxIcon
    , common::Msg::DrawRyujinxRom
    , common::Msg::DrawRyujinxBios);
} // }}}

// pub fn bios() {{{
pub fn bios(tx: Sender<common::Msg>, title: &str)
{
  wizard::install::install(tx.clone()
    , title
    , "bios"
    , common::Msg::DrawRyujinxRom
    , common::Msg::DrawRyujinxBios
    , common::Msg::DrawRyujinxKeys);
} // }}}

// pub fn keys() {{{
pub fn keys(tx: Sender<common::Msg>, title: &str)
{
  wizard::install::install(tx.clone()
    , title
    , "keys"
    , common::Msg::DrawRyujinxBios
    , common::Msg::DrawRyujinxKeys
    , common::Msg::DrawPcsx2Test);
} // }}}

// pub fn test() {{{
pub fn test(tx: Sender<common::Msg>, title: &str)
{
  wizard::test::test(tx.clone()
    , title
    , common::Msg::DrawRyujinxKeys
    , common::Msg::DrawRyujinxTest
    , common::Msg::DrawRyujinxCompress);
} // }}}

// pub fn compress() {{{
pub fn compress(tx: Sender<common::Msg>, title: &str)
{
  wizard::compress::compress(tx.clone()
    , title
    , common::Msg::DrawRyujinxTest
    , common::Msg::DrawRyujinxCompress
    , common::Msg::DrawCreator);
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
