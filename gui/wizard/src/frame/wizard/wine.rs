#![allow(warnings)]

// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  widget::Widget,
  browser::MultiBrowser,
  text::{TextBuffer,TextDisplay},
  menu::MenuButton,
  button::Button,
  group::Group,
  image::SharedImage,
  input::FileInput,
  group::PackType,
  frame::Frame,
  dialog,
  dialog::{dir_chooser,file_chooser},
  enums::{Align,FrameType,Color},
  misc::Progress,
};

use crate::svg;
use crate::dimm;
use crate::common;
use crate::frame;
use crate::frame::wizard;

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>, title: &str)
{
  wizard::name::name(tx.clone()
    , title
    , common::Msg::DrawCreator
    , common::Msg::DrawWineIcon);
} // }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>, title: &str)
{
  wizard::icon::icon(tx.clone()
    , title
    , common::Msg::DrawWineName
    , common::Msg::DrawWineConfigure
  );
} // }}}

// pub fn configure() {{{
pub fn configure(tx: Sender<common::Msg>, title: &str)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_header = ret_frame_header.frame.clone();
  let frame_content = ret_frame_header.frame_content.clone();
  let frame_footer = ret_frame_footer.frame.clone();

  // Set previous frame
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), common::Msg::DrawWineIcon);
  ret_frame_footer.btn_next.clone().emit(tx.clone(), common::Msg::DrawWineRom);

  let clone_tx = tx.clone();
  let f_add_entry = |w: Widget
    , entry_label: &str
    , some_args: Option<Vec<String>>| -> (Button, Frame)
  {
    let mut label = Frame::default()
      .with_size(w.w(), w.h())
      .with_label(entry_label)
      .below_of(&w, dimm::border());

    label.set_frame(FrameType::BorderBox);

    if w.is_same(&frame_content.as_base_widget())
    {
      label.set_size(frame_content.w() - dimm::border()*3 - dimm::width_button_rec()
        , dimm::height_button_rec()
      );
      label.clone().set_pos(frame_content.x() + dimm::border(), frame_content.y() + dimm::border());
    } // if

    let mut btn = Button::default()
      .with_size(dimm::width_button_rec(), dimm::height_button_rec())
      .right_of(&label, dimm::border());
    btn.set_color(Color::Green);
    btn.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_gear(1.0).as_str()).unwrap()));
    if let Some(args) = some_args
    {
      btn.set_callback(move |_|
      {
        clone_tx.send(common::Msg::WindDeactivate);
        let clone_args = args.clone();
        std::thread::spawn(move ||
        {
          common::gameimage_cmd(clone_args);
          clone_tx.send(common::Msg::WindActivate);
        });
      });
    } // if

    (btn, label)
  };

  let (mut btn, label) = f_add_entry(frame_content.as_base_widget()
    , "Install DXVK for directx 9/10/11"
    , Some(vec!["install".to_string(), "dxvk".to_string()])
  );

  let (mut btn, label) = f_add_entry(label.clone().as_base_widget()
    , "Install VKD3D for directx 12"
    , Some(vec!["install".to_string(), "vkd3d".to_string()])
  );

  let (mut btn, label) = f_add_entry(label.clone().as_base_widget()
    , "Run a custom winetricks command"
    , None
  );
  let clone_tx = tx.clone();
  btn.set_callback(move |_|
  {
    let some_value = dialog::input_default("Enter the winetricks command to execute", "");
    if let Some(value) = some_value
    {
      clone_tx.send(common::Msg::WindDeactivate);
      let clone_value = value.clone();
      std::thread::spawn(move ||
      {
        common::gameimage_cmd(vec![
            "install".to_string()
          , "winetricks".to_string()
          , clone_value
        ]);
        clone_tx.send(common::Msg::WindActivate);
      });
    } // if
  });

} // fn: configure }}}

// pub fn rom() {{{
pub fn rom(tx: Sender<common::Msg>, title: &str)
{
  wizard::install::install(tx.clone()
    , title
    , "rom"
    , common::Msg::DrawWineConfigure
    , common::Msg::DrawWineRom
    , common::Msg::DrawWineRom);
} // }}}

// // pub fn bios() {{{
// pub fn bios(tx: Sender<common::Msg>, title: &str)
// {
//   wizard::install::install(tx.clone()
//     , title
//     , "bios"
//     , common::Msg::DrawPcsx2Rom
//     , common::Msg::DrawPcsx2Bios
//     , common::Msg::DrawPcsx2Test);
// } // }}}
//
// // pub fn test() {{{
// pub fn test(tx: Sender<common::Msg>, title: &str)
// {
//   wizard::test::test(tx.clone()
//     , title
//     , common::Msg::DrawPcsx2Bios
//     , common::Msg::DrawPcsx2Test
//     , common::Msg::DrawPcsx2Compress);
// } // }}}
//
// // pub fn compress() {{{
// pub fn compress(tx: Sender<common::Msg>, title: &str)
// {
//   wizard::compress::compress(tx.clone()
//     , title
//     , common::Msg::DrawPcsx2Test
//     , common::Msg::DrawPcsx2Compress
//     , common::Msg::DrawCreator);
// } // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
