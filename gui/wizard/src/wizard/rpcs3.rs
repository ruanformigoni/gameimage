use fltk::
{
  app::Sender,
  enums::{
    FrameType,
    Color,
  },
  browser::MultiBrowser,
  button,
  dialog,
  output,
  prelude::*,
};

use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;
use shared::dimm;

use crate::common;
use shared::std::PathBufExt;
use crate::log;
use crate::frame;
use crate::gameimage;
use crate::wizard;

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>, title: &str)
{
  wizard::name::name(tx.clone()
    , title
    , common::Msg::DrawPlatform
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
    .with_size(frame_content.width() - dimm::border() - dimm::width_button_rec(), frame_content.height())
    .with_pos_of(&frame_content);
  frame_list.set_frame(FrameType::BorderBox);
  frame_list.set_text_size(dimm::height_text());

  // Insert items in list of currently installed items
  let result_vec_items = gameimage::search::search_local("rom");
  if let Ok(vec_items) = result_vec_items
  {
    for item in vec_items { frame_list.add(&item.string()); } // for
  } // if

  // Add new item
  let mut btn_add = shared::fltk::button::rect::add()
    .right_of(&frame_list, dimm::border())
    .with_color(Color::Green);
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
    match gameimage::install::install("rom", vec![str_choice])
    {
      Ok(_) => log!("Successfully installed rom"),
      Err(e) => log!("Failed to install rom: {}", e),
    } // match

    clone_tx.send_awake(common::Msg::WindActivate);
    clone_tx.send_awake(common::Msg::DrawRpcs3Rom);
  });

  // Erase package
  let mut btn_del = shared::fltk::button::rect::del()
    .below_of(&btn_add, dimm::border())
    .with_color(Color::Red);
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

    // Run backend
    match gameimage::install::remove("rom", vec_items)
    {
      Ok(_) => log!("Removed rom(s) successfully"),
      Err(e) => log!("Could not remove rom(s): '{}'", e),
    } // match

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
    .with_size(frame_content.w(), frame_content.h() - dimm::border() - dimm::height_button_wide())
    .with_pos_of(&frame_content);
  frame_text.set_color(Color::BackGround);
  frame_text.set_frame(FrameType::BorderBox);
  frame_text.set_text_size(dimm::height_text());

  let _ = frame_text.append("Here you can install the firmware and the .pkg, .rap and .edat files\n");
  let _ = frame_text.append("Clicking on 'Open' will open RPCS3\n");
  let _ = frame_text.append("Go to 'File -> Install Packages/Raps/Edats' for DLC\n");
  let _ = frame_text.append("Go to 'File -> Install Firmware' for the BIOS\n");

  // Button to launch rpcs3 and install files
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
        match gameimage::install::gui()
        {
          Ok(_) => log!("Gui exited successfully"),
          Err(e) => log!("Install gui exited with error: {}", e),
        }; // match

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
