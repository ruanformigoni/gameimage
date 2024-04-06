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
  enums::{FrameType,Color,Align},
};

use anyhow::anyhow as ah;

use crate::log;
use crate::db;
use crate::common;
use crate::common::PathBufExt;
use crate::common::WidgetExtExtra;
use crate::common::FltkSenderExt;
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
  frame::icon::project(tx.clone()
    , title
    , common::Msg::DrawWineName
    , common::Msg::DrawWineIcon
    , common::Msg::DrawWineConfigure
  );
} // }}}

// pub fn environment() {{{
pub fn environment(tx: Sender<common::Msg>, title: &str)
{
  //
  // Main
  //
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  // Configure footer
  ret_frame_footer.btn_next.clone().hide();
  ret_frame_footer.btn_prev.clone().emit(tx, common::Msg::DrawWineConfigure);

  let frame_content = ret_frame_header.frame_content.clone();

  // Create scrollbar
  let scroll = common::ScrollList::new(frame_content.w() - dimm::width_button_rec() - dimm::border()*2
    , frame_content.h()
    , frame_content.x()
    , frame_content.y());

  //
  // Create entries
  //
  let clone_tx = tx.clone();
  let mut clone_scroll = scroll.clone();
  let mut f_make_entry = move |key : String, val : String|
  {
    let group = fltk::group::Group::default()
      .with_size(clone_scroll.widget_ref().w() - dimm::border(), dimm::height_button_wide()*2 + dimm::border() * 3);

    group.begin();

    // Setup key widget
    let mut output_key = fltk::output::Output::default()
      .with_size(group.w() - dimm::width_button_rec() - dimm::border()*3, dimm::height_button_wide())
      .with_align(Align::Left | Align::Inside)
      .with_pos(group.x() + dimm::border(), group.y());
    output_key.set_value(key.as_str());
    output_key.set_frame(FrameType::BorderBox);
    output_key.set_text_size(dimm::height_text());
    // Setup val widget
    let mut output_val = fltk::output::Output::default()
      .with_size(group.w() - dimm::border()*2, dimm::height_button_wide())
      .with_align(Align::Left | Align::Inside)
      .below_of(&output_key, dimm::border())
      .with_frame(FrameType::BorderBox);
    output_val.set_value(val.as_str());
    output_val.set_text_size(dimm::height_text());
    // Erase button
    let clone_key = key.clone();
    let clone_tx = clone_tx.clone();
    let _btn_del = Button::default()
      .with_size(dimm::width_button_rec(), dimm::height_button_rec())
      .right_of(&output_key, dimm::border())
      .with_svg(svg::icon_del(1.0).as_str())
      .with_color(Color::Red)
      .with_focus(false)
      .with_callback(move |_|
    {
      match db::env::del(clone_key.clone())
      {
        Ok(_) => println!("Erased key '{}'", clone_key),
        Err(e) => println!("Failed to erase key '{}' with error '{}'", clone_key, e.to_string()),
      } // if
      clone_tx.send_awake(common::Msg::DrawWineEnvironment);
    });
    // Separator
    let mut sep = Frame::default()
      .below_of(&output_val, dimm::border())
      .with_size(clone_scroll.widget_ref().width() - dimm::border()*2, 2);
    sep.set_frame(FrameType::FlatBox);
    sep.set_color(Color::Black);

    group.end();

    clone_scroll.add(&mut group.as_base_widget(), dimm::border());
  };

  // Get current database entries
  if let Ok(entries) = db::env::get()
  {
    for db::env::Var{ key, val } in entries.env
    {
      f_make_entry(key, val);
    } // for
  } // if

  scroll.end();

  // Add var button
  let mut btn_add = Button::default()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .top_right_of(&frame_content, - dimm::border())
    .with_border(0, dimm::border())
    .with_focus(false)
    .with_align(Align::Inside | Align::Center)
    .with_label("+");
  let clone_tx = tx.clone();
  btn_add.set_frame(FrameType::BorderBox);
  btn_add.set_label_size(dimm::height_text()*2);
  btn_add.set_color(Color::Green);
  btn_add.set_callback(move |_|
  {
    let mut wind = fltk::window::Window::default()
      .with_size(
          dimm::width_button_wide() * 4 + dimm::border() * 3
        , dimm::height_button_wide() * 3 + dimm::border() * 4
      );
    wind.begin();
    let input_key = fltk::input::Input::default()
      .with_pos(wind.w() - dimm::width_button_wide()*3 - dimm::border(), dimm::border())
      .with_size(dimm::width_button_wide()*3, dimm::height_button_wide())
      .with_align(Align::Left);
    let _label_key = Frame::default()
      .with_size(dimm::width_button_wide(), dimm::height_button_wide())
      .left_of(&input_key, dimm::border())
      .with_align(Align::Inside | Align::Left)
      .with_label("Key");
    let input_value = fltk::input::Input::default()
      .below_of(&input_key, dimm::border())
      .with_size(input_key.w(), input_key.h())
      .with_align(input_key.align());
    let label_value = Frame::default()
      .with_size(dimm::width_button_wide(), dimm::height_button_wide())
      .left_of(&input_value, dimm::border())
      .with_align(Align::Inside | Align::Left)
      .with_label("Value");
    let mut btn_ok = Button::default()
      .with_size(dimm::width_button_wide(), dimm::height_button_wide())
      .below_of(&label_value, dimm::border())
      .with_label("OK");
    btn_ok.set_pos(wind.w() / 2 - btn_ok.w() / 2, btn_ok.y());
    btn_ok.set_color(Color::Green);
    let mut clone_wind = wind.clone();
    let clone_input_key = input_key.clone();
    let clone_input_value = input_value.clone();
    let clone_tx = clone_tx.clone();
    btn_ok.set_callback(move |_|
    {
      clone_wind.hide();
      let key = clone_input_key.value();
      let value = clone_input_value.value();
      if key.is_empty() { return; }
      match db::env::set(key.clone(), value.clone())
      {
        Ok(_) => println!("Set key '{}' with value '{}'", key.clone(), value.clone()),
        Err(e) => println!("Failed to set key '{}' with error '{}'", key, e.to_string()),
      } // if
      clone_tx.send_awake(common::Msg::DrawWineEnvironment);
    });
    wind.end();
    wind.show();
  });
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
      clone_output_status.set_value("Wine prefix does not exist, creating...");
      log!("Wine prefix does not exist, creating...");
      let mut clone_output_status = clone_output_status.clone();
      tx.send_awake(common::Msg::WindDeactivate);
      std::thread::spawn(move ||
      {
        if common::gameimage_sync(vec!["install", "winetricks", "fontsmooth=rgb"]) != 0
        {
          clone_output_status.set_value("Failed to create wine prefix");
          log!("Failed to create wine prefix");
        } // else

        clone_tx.send_awake(common::Msg::DrawWineRom);
      }); // std::thread
      return;
    } // if

    clone_tx.send_awake(common::Msg::DrawWineRom);
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
      clone_tx.send_awake(common::Msg::WindDeactivate);
      let args_owned = args_owned.clone();
      std::thread::spawn(move ||
      {
        let slices: Vec<&str> = args_owned.iter().map(|s| s.as_str()).collect();
        if common::gameimage_sync(slices) != 0
        {
          log!("Command exited with non-zero status");
        } // else
        clone_tx.send_awake(common::Msg::WindActivate);
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
      clone_tx.send_awake(common::Msg::WindDeactivate);
      let clone_value = value.clone();
      std::thread::spawn(move ||
      {
        if common::gameimage_sync(vec!["install", "winetricks", &clone_value]) != 0
        {
          log!("Command exited with non zero status");
        } // else

        clone_tx.send_awake(common::Msg::WindActivate);
      });
    } // if
  });

  let (mut btn, label) = f_add_entry(label.clone().as_base_widget()
    , "Run a custom wine command"
    , None
  );
  let clone_tx = tx.clone();
  btn.set_callback(move |_|
  {
    let some_value = dialog::input_default("Enter the wine command to execute", "");
    if let Some(value) = some_value
    {
      clone_tx.send_awake(common::Msg::WindDeactivate);
      let clone_value = value.clone();
      std::thread::spawn(move ||
      {
        if common::gameimage_sync(vec!["install", "wine", &clone_value]) != 0
        {
          log!("Command exited with non zero status");
        } // else

        clone_tx.send_awake(common::Msg::WindActivate);
      });
    } // if
  });

  let (mut btn, _) = f_add_entry(label.clone().as_base_widget()
    , "Configure environment"
    , None
  );
  btn.emit(tx, common::Msg::DrawWineEnvironment);

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
      } // if

      parent = btn_check.as_base_widget();

      // Label with file name
      let mut output = output::Output::default()
        .with_size(frame_list.width() - dimm::width_button_rec()*3 - dimm::border()*4, dimm::height_button_wide())
        .right_of(&btn_check, dimm::border());
      let _ = output.insert(&item.string());

      // Button to open file in file manager
      let clone_item = item.clone();
      let mut clone_output_status = ret_frame_footer.output_status.clone();
      let btn_folder = button::Button::default()
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

      let clone_tx = tx.clone();
      let _btn_run = button::Button::default()
        .with_focus(false)
        .with_svg(svg::icon_play(1.0).as_str())
        .with_size(dimm::width_button_rec(), dimm::height_button_rec())
        .right_of(&btn_folder, dimm::border())
        .with_callback(move |_|
        {
          // Execute wine
          let clone_item = item.clone();
          clone_tx.send_awake(common::Msg::WindDeactivate);
          std::thread::spawn(move ||
          {
            // Set the selected binary as default
            if common::gameimage_sync(vec!["select", "rom", &clone_item.string()]) != 0
            {
              log!("Could not change default executable for test");
              clone_tx.send_awake(common::Msg::WindActivate);
              return;
            } // else

            // Test the selected binary
            if common::gameimage_sync(vec!["test"]) != 0
            {
              log!("Could not test selected executable");
              clone_tx.send_awake(common::Msg::WindActivate);
              return;
            } // else

            clone_tx.send_awake(common::Msg::WindActivate);
          });
        });
    } // for
  } // if


  scroll.end();

  // Add new item
  let clone_tx = tx.clone();
  let _btn_add = Button::default()
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
      clone_tx.send_awake(common::Msg::WindDeactivate);
      std::thread::spawn(move ||
      {
        if common::gameimage_sync(vec!["install", "wine", &str_choice ]) != 0
        {
          log!("Could not execute selected file");
        }; // else

        clone_tx.send_awake(common::Msg::DrawWineRom);
      });
    });

  // Go to next frame iff a default executable was selected
  // ret_frame_footer.btn_next.clone().emit(tx.clone(), common::Msg::DrawWineCompress);
  let clone_tx = tx.clone();
  let clone_vec_radio_path = vec_radio_path.clone();
  ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    // Access checkbutton vector
    let vec_radio_path = match clone_vec_radio_path.lock()
    {
      Ok(vec_radio_path) => vec_radio_path,
      Err(e) => { log!("Could not open list of radio buttons: {}", e); return; },
    }; // match

    // Get selected entry
    let path_file_default =  match vec_radio_path.clone().into_iter().find(|e| e.0.is_toggled())
    {
      Some(entry) => entry.1,
      None => { dialog::alert_default("You must selected the default executable before continuing"); return; },
    }; // if
    
    // Set the selected binary as default
    if common::gameimage_sync(vec!["select", "rom", &path_file_default.string()]) != 0
    {
      log!("Could not select rom {}", path_file_default.string());
      return;
    } // if

    clone_tx.send_awake(common::Msg::DrawWineCompress);
  });

} // }}}

// pub fn compress() {{{
pub fn compress(tx: Sender<common::Msg>, title: &str)
{
  wizard::compress::compress(tx.clone()
    , title
    , common::Msg::DrawWineRom
    , common::Msg::DrawWineCompress
    , common::Msg::DrawCreator);
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
