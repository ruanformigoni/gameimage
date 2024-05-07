// Gui
use fltk::prelude::*;
use fltk::{
  app::Sender,
  browser::MultiBrowser,
  dialog,
  output,
  enums::{FrameType,Color,Align},
};

use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;

use crate::dimm;
use crate::wizard;
use crate::frame;
use crate::common;
use shared::std::PathBufExt;
use crate::log;
use crate::db;
use crate::gameimage;

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>, title: &str)
{
  wizard::name::name(tx.clone()
    , title
    , common::Msg::DrawCreator
    , common::Msg::DrawRetroarchIcon);
} // }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>, title: &str)
{
  frame::icon::project(tx.clone()
    , title
    , common::Msg::DrawRetroarchName
    , common::Msg::DrawRetroarchIcon
    , common::Msg::DrawRetroarchRom
  );
} // }}}

// pub fn rom() {{{
pub fn rom(tx: Sender<common::Msg>, title: &str)
{
  let install = wizard::install::install(tx.clone()
    , title
    , "rom"
    , common::Msg::DrawRetroarchIcon
    , common::Msg::DrawRetroarchRom
    , common::Msg::DrawRetroarchCore);

  let mut frame_list = install.frame_list.clone();
  let output_status = install.ret_frame_footer.output_status.clone();
  let btn_add = install.btn_add.clone();
  let btn_del = install.btn_del.clone();

  // Adjust to include the field below
  frame_list.set_size(frame_list.w(), frame_list.h() - dimm::border() - dimm::height_text() - dimm::height_button_wide());

  // Show default item below all items
  let mut output_default = output::Output::default()
    .with_width(frame_list.w())
    .with_height(dimm::height_button_wide())
    .below_of(&frame_list, dimm::border() + dimm::height_text())
    .with_align(Align::Top | Align::Left)
    .with_label("Default rom:");
  output_default.deactivate();
  if let Ok(project) = db::project::current()
  && let Ok(path_file_rom) = project.get_path_relative(db::project::EntryName::PathFileRom)
  {
    let _ = output_default.insert(&path_file_rom.file_name_string());
  } // if

  // Update default rom
  let clone_frame_list = frame_list.clone();
  let mut clone_output_status = output_status.clone();
  let clone_tx = tx.clone();
  let btn_default = shared::fltk::button::rect::check()
    .below_of(&btn_add, dimm::border())
    .with_color(Color::Blue)
    .with_callback(move |_|
    {
      let vec_indices = clone_frame_list.selected_items();

      if vec_indices.len() == 0
      {
        clone_output_status.set_value("No item selected to set as default");
        return;
      } // if

      if vec_indices.len() != 1
      {
        clone_output_status.set_value("Only one item can be set as the default");
        return;
      } // if

      let selected = match clone_frame_list.text(*vec_indices.first().unwrap())
      {
        Some(item) => std::path::PathBuf::from(item),
        None => return,
      }; // match

      clone_tx.send_awake(common::Msg::WindDeactivate);
      std::thread::spawn(move ||
      {
        match gameimage::select::select("rom", &selected)
        {
          Ok(_) => log!("Selected rom successfully"),
          Err(e) => log!("Could not select rom file '{}': '{}'", selected.string(), e),
        } // match
        clone_tx.send_awake(common::Msg::DrawRetroarchRom);
      }); // std::thread
    }); // with_callback

  // Update del button position
  let _ = btn_del.below_of(&btn_default, dimm::border());
} // }}}

// pub fn core() {{{
pub fn core(tx: Sender<common::Msg>, title: &str)
{
  let install = wizard::install::install(tx.clone()
    , title
    , "core"
    , common::Msg::DrawRetroarchRom
    , common::Msg::DrawRetroarchCore
    , common::Msg::DrawRetroarchBios);

  // Clone data from preset
  let mut frame_list_installed = install.frame_list.clone();
  let frame_content = install.ret_frame_header.frame_content.clone();
  let output_status = install.ret_frame_footer.output_status.clone();
  let mut btn_add = install.btn_add.clone();
  let btn_del = install.btn_del.clone();

  // Adjust size
  frame_list_installed.set_size(
      frame_content.width() - dimm::border()*3 - dimm::width_button_rec()
    , frame_content.height() / 2 - ( ( dimm::border()*4 + dimm::height_button_wide() + dimm::height_text() ) / 2 )
  );

  // Show default item below all items
  let mut output_default = fltk::output::Output::default()
    .with_size(frame_list_installed.w(), dimm::height_button_wide())
    .center_of(&frame_content)
    .with_posx_of(&frame_list_installed)
    .with_border(0, dimm::border())
    .with_align(Align::Top | Align::Left)
    .with_label("Default core:");
  output_default.deactivate();
  if let Ok(project) = db::project::current()
  && let Ok(path_file_core) = project.get_path_relative(db::project::EntryName::PathFileCore)
  {
    let _ = output_default.insert(&path_file_core.file_name_string());
  } // if


  // Add new item from file manager
  let clone_tx = tx.clone();
  btn_add.set_callback(move |_|
  {
    // Pick files to install
    let mut chooser = dialog::FileChooser::new("."
      , "*"
      , dialog::FileChooserType::Multi
      , "Pick one or multiple files");

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

    // Install files
    clone_tx.send_awake(common::Msg::WindDeactivate);

    // Get items
    let vec_items = (1..chooser.count()+1).into_iter().map(|e| chooser.value(e).unwrap()).collect::<Vec<String>>();
    std::thread::spawn(move ||
    {
      match gameimage::install::install("core", vec_items)
      {
        Ok(_) => log!("Successfully installed cores"),
        Err(e) => log!("Failed to install one or more cores: {}", e),
      }; // match
      // Redraw window
      clone_tx.send_awake(common::Msg::DrawRetroarchCore);
    });
  });

  // Update default core
  let clone_frame_list_installed = frame_list_installed.clone();
  let mut clone_output_status = output_status.clone();
  let btn_default = shared::fltk::button::rect::check()
    .below_of(&btn_add, dimm::border())
    .with_color(Color::Blue)
    .with_callback(move |_|
    {
      let vec_indices = clone_frame_list_installed.selected_items();

      if vec_indices.len() == 0
      {
        clone_output_status.set_value("No item selected to set as default");
        return;
      } // if

      if vec_indices.len() != 1
      {
        clone_output_status.set_value("Only one item can be set as the default");
        return;
      } // if

      let selected = match clone_frame_list_installed.text(*vec_indices.first().unwrap())
      {
        Some(item) => std::path::PathBuf::from(item),
        None => return,
      }; // match

      clone_tx.send_awake(common::Msg::WindDeactivate);
      std::thread::spawn(move ||
      {
        match gameimage::select::select("core", &selected)
        {
          Ok(_) => log!("Selected core successfully"),
          Err(e) => log!("Could not select core file '{}': '{}'", selected.string(), e),
        } // match

        clone_tx.send_awake(common::Msg::DrawRetroarchCore);
      }); // std::thread
    }); // with_callback

  // Erase package
  let clone_frame_list_installed = frame_list_installed.clone();
  let mut clone_output_status = output_status.clone();
  btn_del
    .below_of(&btn_default, dimm::border())
    .with_callback(move |_|
  {
    let vec_indices = clone_frame_list_installed.selected_items();

    if vec_indices.len() == 0
    {
      clone_output_status.set_value("No item selected for deletion");
      clone_tx.send_awake(common::Msg::DrawRetroarchCore);
      return;
    } // if

    // Get items
    let vec_items : Vec<String> = vec_indices.into_iter().map(|e|{ clone_frame_list_installed.text(e).unwrap() }).collect();

    // Run backend
    match gameimage::install::remove("core", vec_items)
    {
      Ok(_) => log!("Removed core successfully"),
      Err(e) => log!("Could not remove core file(s) '{}'", e),
    } // match

    // Redraw
    clone_tx.send_awake(common::Msg::DrawRetroarchCore);
  });

  // List of items to install
  let mut frame_list_remote = MultiBrowser::default()
    .with_size_of(&frame_list_installed)
    .with_posx_of(&frame_list_installed)
    .bottom_of(&frame_content, - dimm::border())
    .with_frame(FrameType::BorderBox);
  frame_list_remote.set_text_size(dimm::height_text());

  // Insert remote items in list
  if let Ok(vec_items) = gameimage::search::search_remote("core")
  {
    vec_items.iter().for_each(|item| frame_list_remote.add(&item.string()) );
  } // if

  // Add new item from remote
  let clone_tx = tx.clone();
  let clone_frame_list_remote = frame_list_remote.clone();
  let _btn_install_from_remote = shared::fltk::button::rect::cloud()
    .right_of(&frame_list_remote, dimm::border())
    .with_color(Color::Green)
    .with_callback(move |_|
    {
      // Install files
      clone_tx.send_awake(common::Msg::WindDeactivate);
      let clone_frame_list_remote = clone_frame_list_remote.clone();
      std::thread::spawn(move ||
      {
        // Get indices
        let vec_indices = clone_frame_list_remote.selected_items();

        // Get text
        let vec_items : Vec<String> = vec_indices.into_iter().map(|e|{ clone_frame_list_remote.text(e).unwrap() }).collect();

        // Install with backend
        match gameimage::install::remote("core", vec_items)
        {
          Ok(_) => log!("Remote cores installed successfully"),
          Err(e) => log!("Failed to install remote cores: {}", e),
        } // match

        clone_tx.send_awake(common::Msg::DrawRetroarchCore);
      });
    });
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
