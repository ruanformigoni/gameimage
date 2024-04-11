use fltk::
{
  app::Sender,
  enums::{
    FrameType,
    Color,
  },
  browser::MultiBrowser,
  button,
  button::Button,
  dialog,
  output,
  prelude::*,
};

use crate::common;
use crate::common::PathBufExt;
use crate::common::WidgetExtExtra;
use crate::common::FltkSenderExt;
use crate::log;
use crate::frame;
use crate::gameimage;
use crate::wizard;
use crate::lib::dimm;
use crate::lib::svg;

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>, title: &str)
{
  wizard::name::name(tx.clone()
    , title
    , common::Msg::DrawCreator
    , common::Msg::DrawRpcs3Icon);
} // }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>, title: &str)
{
  frame::icon::project(tx.clone()
    , title
    , common::Msg::DrawRpcs3Name
    , common::Msg::DrawRpcs3Icon
    , common::Msg::DrawRpcs3Rom
  );
} // }}}

// pub fn rom() {{{
pub fn rom(tx: Sender<common::Msg>, title: &str)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

  // Set previous frame
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), common::Msg::DrawRpcs3Icon);
  ret_frame_footer.btn_next.clone().emit(tx.clone(), common::Msg::DrawRpcs3Bios);

  // List of the currently installed items
  let mut frame_list = MultiBrowser::default()
    .with_size(frame_content.width() - dimm::border()*3 - dimm::width_button_rec()
      , frame_content.height() - dimm::border()*2)
    .with_pos(frame_content.x() + dimm::border(), frame_content.y() + dimm::border());
  frame_list.set_frame(FrameType::BorderBox);
  frame_list.set_text_size(dimm::height_text());

  // Insert items in list of currently installed items
  let result_vec_items = gameimage::search::search_local("rom");
  if let Ok(vec_items) = result_vec_items
  {
    for item in vec_items { frame_list.add(&item.string()); } // for
  } // if

  // Add new item
  let mut btn_add = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .right_of(&frame_list, dimm::border());
  btn_add.set_frame(FrameType::RoundedFrame);
  btn_add.visible_focus(false);
  btn_add.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_add(1.0).as_str()).unwrap()));
  btn_add.set_color(Color::Green);
  let clone_tx = tx.clone();
  btn_add.set_callback(move |_|
  {
    // Pick files to install
    let mut chooser = dialog::FileChooser::new("."
      , "*"
      , dialog::FileChooserType::Directory
      , "Pick a directory with the .SFB file");

    // Start dialog
    chooser.show();

    // Wait for choice(s)
    while chooser.shown() { fltk::app::wait(); } // while

    // Check if choice is valid
    if chooser.value(1).is_none()
    {
      log!("No file selected");
      return;
    } // if

    // Deactivate window
    clone_tx.send_awake(common::Msg::WindDeactivate);

    // Fetch choice
    let str_choice = chooser.value(1).unwrap();

    // Install directory with backend
    if common::gameimage_sync(vec!["install", "rom", &str_choice]) != 0
    {
      log!("Failed to install '{}'", str_choice);
    } // else
    clone_tx.send_awake(common::Msg::WindActivate);
    clone_tx.send_awake(common::Msg::DrawRpcs3Rom);
  });

  // Erase package
  let mut btn_del = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .below_of(&btn_add, dimm::border());
  btn_del.set_frame(FrameType::RoundedFrame);
  btn_del.visible_focus(false);
  btn_del.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_del(1.0).as_str()).unwrap()));
  btn_del.set_color(Color::Red);
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  btn_del.set_callback(move |_|
  {
    let vec_indices = frame_list.selected_items();

    if vec_indices.len() == 0
    {
      clone_output_status.set_value("No item selected for deletion");
      return;
    } // if

    // Get items
    let vec_items : Vec<String> = vec_indices.into_iter().map(|e|{ frame_list.text(e).unwrap() }).collect();

    for item in vec_items
    {
      if common::gameimage_sync(vec!["install", "--remove", "rom", &item]) != 0
      {
        log!("Could not remove '{}", item);
      }; // else
    } // for
    clone_tx.send_awake(common::Msg::DrawRpcs3Rom);
  });
} // }}}

// pub fn bios() {{{
pub fn bios(tx: Sender<common::Msg>, title: &str)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

  // Set bottom callbacks
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), common::Msg::DrawRpcs3Rom);
  ret_frame_footer.btn_next.clone().emit(tx.clone(), common::Msg::DrawRpcs3Test);

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

  let _ = frame_text.append("Here you can install the firmware and the .pkg, .rap and .edat files\n");
  let _ = frame_text.append("Clicking on 'Open' will open RPCS3\n");
  let _ = frame_text.append("Go to 'File -> Install Packages/Raps/Edats' for DLC\n");
  let _ = frame_text.append("Go to 'File -> Install Firmware' for the BIOS\n");

  // Button to launch rpcs3 and install files
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
      tx.send_awake(common::Msg::WindDeactivate);
      std::thread::spawn(move ||
      {
        if common::gameimage_sync(vec!["install", "gui"]) != 0
        {
          log!("Install gui exited with error");
        } // if

        tx.send_awake(common::Msg::WindActivate);
      });
    });

} // }}}

// pub fn test() {{{
pub fn test(tx: Sender<common::Msg>, title: &str)
{
  wizard::test::test(tx.clone()
    , title
    , common::Msg::DrawRpcs3Bios
    , common::Msg::DrawRpcs3Test
    , common::Msg::DrawRpcs3Compress);
} // }}}

// pub fn compress() {{{
pub fn compress(tx: Sender<common::Msg>, title: &str)
{
  wizard::compress::compress(tx.clone()
    , title
    , common::Msg::DrawRpcs3Test
    , common::Msg::DrawRpcs3Compress
    , common::Msg::DrawCreator);
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
