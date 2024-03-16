// Gui
use std::
{
  path,
  path::PathBuf,
  sync::{Arc,Mutex}
};

use fltk::prelude::*;
use fltk::{
  app::Sender,
  widget::Widget,
  button,
  button::Button,
  group,
  output,
  frame::Frame,
  dialog,
  enums::{FrameType,Color},
};

use anyhow::anyhow as ah;

use crate::log;
use crate::db;
use crate::common;
use crate::common::PathBufExt;
use crate::common::WidgetExtExtra;
use crate::frame;
use crate::wizard;
use crate::lib::svg;
use crate::lib::dimm;

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
    , common::Msg::DrawWineIcon
    , common::Msg::DrawWineConfigure
  );
} // }}}

// pub fn configure() {{{
pub fn configure(tx: Sender<common::Msg>, title: &str)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();
  let output_status = ret_frame_footer.output_status.clone();

  // Set previous frame
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), common::Msg::DrawWineIcon);

  // Set next frame
  let clone_tx = tx.clone();
  let mut clone_output_status = output_status.clone();
  ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    // Get path to wine prefix
    let path_dir_wine_prefix = if let Ok(project) = db::project::current()
      && let Ok(path_dir_self) = project.get_dir_self()
      {
        path_dir_self.join("wine")
      } // if
      else
      {
        log!("Could not get path to current project");
        return;
      }; // else

    if ! path_dir_wine_prefix.exists()
    {
      clone_output_status.set_value("Wine prefix does not exist");
      log!("Wine prefix does not exist");
      return;
    } // if

    clone_tx.send(common::Msg::DrawWineRom);
  });

  let clone_tx = tx.clone();
  let f_add_entry = |w: Widget
    , entry_label: &str
    , some_args: Option<Vec<&str>>| -> (Button, Frame)
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
    let args = if let Some(args) = some_args { args } else { return (btn, label); };
    let args_owned : Vec<String> = args.iter().map(|s| s.to_string()).collect();
    btn.set_callback(move |_|
    {
      clone_tx.send(common::Msg::WindDeactivate);
      let args_owned = args_owned.clone();
      std::thread::spawn(move ||
      {
        let slices: Vec<&str> = args_owned.iter().map(|s| s.as_str()).collect();
        if common::gameimage_sync(slices) != 0
        {
          log!("Command exited with non-zero status");
        } // else
        clone_tx.send(common::Msg::WindActivate);
      });
    });

    (btn, label)
  };

  let (_, label) = f_add_entry(frame_content.as_base_widget()
    , "Install DXVK for directx 9/10/11"
    , Some(vec!["install", "dxvk"])
  );

  let (_, label) = f_add_entry(label.clone().as_base_widget()
    , "Install VKD3D for directx 12"
    , Some(vec!["install", "vkd3d"])
  );

  let (_, label) = f_add_entry(label.clone().as_base_widget()
    , "Run regedit"
    , Some(vec!["install", "wine", "regedit"])
  );

  let (_, label) = f_add_entry(label.clone().as_base_widget()
    , "Run add/remove programs"
    , Some(vec!["install", "wine", "uninstaller"])
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
        if common::gameimage_sync(vec!["install", "winetricks", &clone_value]) != 0
        {
          log!("Command exited with non zero status");
        } // else

        clone_tx.send(common::Msg::WindActivate);
      });
    } // if
  });

  let (mut btn, _) = f_add_entry(label.clone().as_base_widget()
    , "Run a custom wine command"
    , None
  );
  let clone_tx = tx.clone();
  btn.set_callback(move |_|
  {
    let some_value = dialog::input_default("Enter the wine command to execute", "");
    if let Some(value) = some_value
    {
      clone_tx.send(common::Msg::WindDeactivate);
      let clone_value = value.clone();
      std::thread::spawn(move ||
      {
        if common::gameimage_sync(vec!["install", "wine", &clone_value]) != 0
        {
          log!("Command exited with non zero status");
        } // else

        clone_tx.send(common::Msg::WindActivate);
      });
    } // if
  });

} // fn: configure }}}

// find_roms() {{{
fn find_roms() -> anyhow::Result<Vec<PathBuf>>
{
  // Ask back-end for the item files
  if common::gameimage_sync(vec!["search", "--json", "gameimage.search.json", "rom"]) != 0
  {
    return Err(ah!("No items found (dir not found)"));
  } // else

  // Fetch items from db generated by backend
  let result_entry = db::search::read();

  Ok(result_entry?.rom.ok_or(ah!("No items found"))?)
} // find_roms() }}}

// pub fn rom() {{{
pub fn rom(tx: Sender<common::Msg>, title: &str)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

  // Set previous frame
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), common::Msg::DrawWineConfigure);
  ret_frame_footer.btn_next.clone().emit(tx.clone(), common::Msg::DrawWineDefault);

  // List of the currently installed items
  let frame_list = Frame::default()
    .with_size(frame_content.width() - dimm::border()*3 - dimm::width_button_rec()
      , frame_content.height() - dimm::border()*2)
    .with_pos(frame_content.x() + dimm::border(), frame_content.y() + dimm::border())
    .with_frame(FrameType::BorderBox);

  let mut scroll = group::Scroll::default()
    .with_size(frame_list.w(), frame_list.h())
    .with_pos(frame_list.x(), frame_list.y())
    .with_frame(FrameType::BorderBox);

  scroll.set_scrollbar_size(dimm::border());

  scroll.begin();

  // Insert items in list of currently installed items
  let vec_radio_path = Arc::new(Mutex::new(Vec::<(button::RadioButton, path::PathBuf)>::new()));

  let mut parent = scroll.as_base_widget();
  if let Ok(vec_items) = find_roms()
  {
    for item in vec_items
    {
      // Checkbutton
      let mut btn_check = button::RadioButton::default()
        .with_color_selected(Color::Blue)
        .with_size(dimm::width_button_rec(), dimm::height_button_rec())
        .with_focus(false)
        .below_of(&parent, dimm::border());

      // Include values into shared vector
      if let Ok(mut lock) = vec_radio_path.lock()
      {
        lock.push((btn_check.clone(), path::PathBuf::from(item.to_owned())));
      } // if

      if parent.is_same(&scroll.as_base_widget())
      {
        btn_check = btn_check.top_left_of(&parent, 0);
        btn_check.set_value(true);
      } // if

      parent = btn_check.as_base_widget();

      // Label with file name
      let mut output = output::Output::default()
        .with_size(frame_list.width() - dimm::width_button_rec()*2 - dimm::border()*3, dimm::height_button_wide())
        .right_of(&btn_check, dimm::border());
      let _ = output.insert(&item.string());

      // Button to open file in file manager
      let clone_item = item.clone();
      let mut clone_output_status = ret_frame_footer.output_status.clone();
      let _ = button::Button::default()
        .with_focus(false)
        .with_svg(svg::icon_folder(1.0).as_str())
        .with_size(dimm::width_button_rec(), dimm::height_button_rec())
        .right_of(&output, dimm::border())
        .with_callback(move |_|
        {
          let path_dir_project = if let Ok(project) = db::project::current()
            && let Ok(path_dir_project) = project.get_dir_self()
          {
            path_dir_project
          } // if
          else
          {
            log!("Could not open project directory");
            return;
          }; // else

          let mut path_dir_executable = path_dir_project.join(&clone_item);

          log!("Executable: {}", path_dir_executable.string());

          if ! path_dir_executable.pop()
          {
            log!("Could not get parent dir for executable");
          } // if

          clone_output_status.set_value(format!("Open '{}'", path_dir_executable.string()).as_str());

          let _ = std::process::Command::new("xdg-open")
              .env_remove("LD_PRELOAD")
              .stderr(std::process::Stdio::inherit())
              .stdout(std::process::Stdio::inherit())
              .arg(&path_dir_executable.string())
              .spawn();
        });
    } // for
  } // if


  scroll.end();

  // Add new item
  let clone_tx = tx.clone();
  let btn_add = Button::default()
    .with_color(Color::Green)
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .with_frame(FrameType::RoundedFrame)
    .with_svg(svg::icon_add(1.0).as_str())
    .with_focus(false)
    .right_of(&frame_list, dimm::border())
    .with_callback(move |_|
    {
      // Pick files to install
      let mut chooser = dialog::FileChooser::new("."
        , "*"
        , dialog::FileChooserType::Single
        , "Pick a file to install with wine");

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

      // Execute wine
      let str_choice = chooser.value(1).unwrap();
      clone_tx.send(common::Msg::WindDeactivate);
      std::thread::spawn(move ||
      {
        if common::gameimage_sync(vec!["install", "wine", &str_choice ]) != 0
        {
          log!("Could not execute selected file");
        }; // else

        clone_tx.send(common::Msg::DrawWineRom);
      });
    });

  // Run the selected application
  let mut btn_run = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .below_of(&btn_add, dimm::border());
  btn_run.set_frame(FrameType::RoundedFrame);
  btn_run.visible_focus(false);
  btn_run.set_image(Some(fltk::image::SvgImage::from_data(svg::icon_play(1.0).as_str()).unwrap()));
  btn_run.set_color(Color::Green);
  let clone_tx = tx.clone();
  let clone_vec_radio_path = vec_radio_path.clone();
  btn_run.set_callback(move |_|
  {
    let vec_radio_path = if let Ok(vec_radio_path) = clone_vec_radio_path.lock()
    {
      vec_radio_path
    } // if
    else
    {
      log!("Could not open list of radio buttons");
      return;
    }; // else

    let path_dir_selected = if let Some(entry) = vec_radio_path.clone().into_iter().find(|e| e.0.is_toggled())
      && let Ok(project) = db::project::current()
      && let Ok(path_dir_project) = project.get_dir_self()
    {
      path_dir_project.join(entry.1)
    } // if
    else
    {
      log!("Could not execute selected entry");
      return;
    }; // else

    // Execute wine
    clone_tx.send(common::Msg::WindDeactivate);
    std::thread::spawn(move ||
    {
      // Set the selected binary as default
      if common::gameimage_sync(vec!["select", "rom", &path_dir_selected.string()]) != 0
      {
        log!("Could not change default executable for test");
        clone_tx.send(common::Msg::WindActivate);
        return;
      } // else

      // Test the selected binary
      if common::gameimage_sync(vec!["test"]) != 0
      {
        log!("Could not test selected executable");
        clone_tx.send(common::Msg::WindActivate);
        return;
      } // else

      clone_tx.send(common::Msg::WindActivate);
    });
  });
} // }}}

// pub fn default() {{{
pub fn default(tx: Sender<common::Msg>, title: &str)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

  // Set previous frame
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), common::Msg::DrawWineRom);

  // List of the currently installed items
  let mut scroll = group::Scroll::default()
    .with_size(frame_content.width() - dimm::border()*3 - dimm::width_button_rec()
      , frame_content.height() - dimm::border()*2)
    .with_pos(frame_content.x() + dimm::border(), frame_content.y() + dimm::border());
  scroll.set_frame(FrameType::BorderBox);
  scroll.set_scrollbar_size(dimm::width_button_rec()/2);

  let arc_vec_btn_label = Arc::new(Mutex::new(Vec::<( button::RadioButton, output::Output )>::new()));

  let mut parent = scroll.as_base_widget();

  let clone_scroll = scroll.clone();
  let clone_arc_vec_btn_label = arc_vec_btn_label.clone();
  let mut f_list_add = move |label: &str|
  {
    let mut btn_radio = button::RadioButton::default()
      .with_size(dimm::width_button_rec(), dimm::height_button_rec())
      .below_of(&parent, dimm::border());
    btn_radio.visible_focus(false);
    btn_radio.set_selection_color(Color::Blue);

    if parent.is_same(&clone_scroll.as_base_widget())
    {
      btn_radio.set_pos(parent.x() + dimm::border(), parent.y() + dimm::border());
      btn_radio.set_value(true);
    } // if

    let mut frame_label = output::Output::default()
      .with_size(clone_scroll.width() - dimm::width_button_rec() - dimm::width_button_rec()/2 - dimm::border()*3
        , dimm::height_button_rec())
      .right_of(&btn_radio, dimm::border());
    let _ = frame_label.insert(label);
    frame_label.set_frame(FrameType::BorderBox);

    if let Ok(mut vec) = clone_arc_vec_btn_label.lock()
    {
      vec.push((btn_radio.clone(), frame_label.clone()));
    } // if
    parent = btn_radio.as_base_widget();
  };

  // Insert items in list of currently installed items
  if let Ok(vec_items) = find_roms()
  {
    for item in vec_items { f_list_add(&item.string()); } // for
  } // if

  let clone_arc_vec_btn_label = arc_vec_btn_label.clone();
  let clone_tx = tx.clone();
  ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    // Get the selected button label (it contains the path to the default binary)
    let vec_btn_label = match clone_arc_vec_btn_label.lock()
    {
      Ok(vec) => vec,
      Err(e) => { log!("Could not get lock to button-label vector: {}", e); return; },
    }; // match

    // Get the selected button label (it contains the path to the default binary)
    let str_path = match vec_btn_label.iter().find(|x| x.0.is_set() )
    {
      Some(value) => value.1.value(),
      None => { log!("No button selected!"); return; },
    }; // match

    // Set the selected binary as default
    if common::gameimage_sync(vec!["select", "rom", &str_path]) != 0
    {
      log!("Could not select rom {}", str_path);
      return;
    } // if

    // Compress
    clone_tx.send(common::Msg::DrawWineCompress);
  });
} // }}}

// pub fn compress() {{{
pub fn compress(tx: Sender<common::Msg>, title: &str)
{
  wizard::compress::compress(tx.clone()
    , title
    , common::Msg::DrawWineDefault
    , common::Msg::DrawWineCompress
    , common::Msg::DrawCreator);
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
