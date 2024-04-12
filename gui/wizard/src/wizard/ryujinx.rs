use fltk::
{
  app::Sender,
  enums::{
    FrameType,
    Color,
  },
  button,
  output,
  prelude::*,
};

use shared::fltk::WidgetExtExtra;
use shared::dimm;

use crate::common;
use crate::log;
use crate::frame;
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
  frame::icon::project(tx.clone()
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
    , common::Msg::DrawRyujinxKeys);
} // }}}

// pub fn keys() {{{
pub fn keys(tx: Sender<common::Msg>, title: &str)
{
  let ret = wizard::install::install(tx.clone()
    , title
    , "keys"
    , common::Msg::DrawRyujinxRom
    , common::Msg::DrawRyujinxKeys
    , common::Msg::DrawRyujinxBios);

  // Get keys list
  let mut frame_list = ret.frame_list.clone();

  // Get frame contents frame
  let frame_content = ret.ret_frame_header.frame_content.clone();

  // Resize keys list to fit text box below
  frame_list.set_size(frame_list.w(), ( frame_content.h() as f32 * 0.8 ) as i32 - dimm::border()*3);

  // Box with explanation under keys list
  let mut frame_text = output::MultilineOutput::default()
    .with_width(frame_list.w())
    .with_height((frame_content.h() as f32 * 0.2) as i32)
    .below_of(&frame_list, dimm::border());
  frame_text.set_color(Color::BackGround);
  frame_text.set_frame(FrameType::BorderBox);
  frame_text.set_text_size(dimm::height_text());

  let _ = frame_text.append("Here you can install a zipfile with the '.keys' inside.\n");
  let _ = frame_text.append("You can also install the extracted '.keys' file(s) one by one.\n");
} // }}}

// pub fn bios() {{{
pub fn bios(tx: Sender<common::Msg>, title: &str)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

  // Set bottom callbacks
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), common::Msg::DrawRyujinxKeys);
  ret_frame_footer.btn_next.clone().emit(tx.clone(), common::Msg::DrawRyujinxTest);

  // Box with explanation text
  let mut frame_text = output::MultilineOutput::default()
    .with_size(
        frame_content.w() - dimm::border()*2
      , frame_content.h() - dimm::border()*3 - dimm::height_button_wide()
    )
    .top_center_of(&frame_content, dimm::border());
  frame_text.set_color(Color::BackGround);
  frame_text.set_frame(FrameType::BorderBox);
  frame_text.set_text_size(dimm::height_text());

  let _ = frame_text.append("Here you can install the firmware\n");
  let _ = frame_text.append("Clicking on 'Open' will open Ryujinx\n");
  let _ = frame_text.append("Go to 'Tools -> Install Firmware' to install the firmware\n");
  let _ = frame_text.append("Then close Ryujinx and click on next\n");

  // Button to launch ryujinx and install files
  let _frame_bottom = fltk::frame::Frame::default()
    .with_size(frame_text.w(), frame_content.h() - frame_text.h() - dimm::border())
    .with_frame(FrameType::BorderBox)
    .below_of(&frame_text, 0);

  let _btn_launch = button::Button::default()
    .with_size(dimm::width_button_wide(), dimm::height_button_wide())
    .below_center_of(&frame_text, dimm::border())
    .with_color(Color::Green)
    .with_label("Open")
    .with_callback(move |_|
    {
      tx.send(common::Msg::WindDeactivate);
      std::thread::spawn(move ||
      {
        if common::gameimage_sync(vec!["install", "gui"]) != 0
        {
          log!("Install gui exited with error");
        } // if

        tx.send(common::Msg::WindActivate);
      });
    });

} // }}}

// pub fn test() {{{
pub fn test(tx: Sender<common::Msg>, title: &str)
{
  wizard::test::test(tx.clone()
    , title
    , common::Msg::DrawRyujinxBios
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
