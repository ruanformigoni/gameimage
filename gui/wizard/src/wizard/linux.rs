use std::
{
  path,
  sync::{Arc,Mutex,LazyLock},
  path::PathBuf,
  os::unix::fs::PermissionsExt,
};

use serde_json::json;

use std::io::Write;

use fltk::
{
  button,
  output,
  app::Sender,
  prelude::*,
  enums::*,
  frame::Frame,
};

use clown::clown;

use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;
use shared::std::PathBufExt;
use shared::dimm;
use shared::{rescope,hover_blink,column,row,add,fixed,scroll,hpack};

use crate::log_alert;
use crate::log_status;
use crate::log_err_status;
use crate::common;
use crate::db;
use crate::frame;
use crate::wizard;
use crate::gameimage;

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>, title: &str)
{
  wizard::name::name(tx.clone()
    , title
    , common::Msg::DrawPlatform
    , common::Msg::DrawLinuxIcon);
} // }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>, title: &str)
{
  frame::icon::project(tx.clone()
    , title
    , common::Msg::DrawLinuxName
    , common::Msg::DrawLinuxIcon
    , common::Msg::DrawLinuxMethod
  );
} // }}}

// fn method_explore() {{{
fn method_explore()
{
  // Layout
  column!(col,
    row!(row,
      add!(row, frame_text, shared::fltk::frame::bordered());
      fixed!(row, btn_folder, shared::fltk::button::rect::folder(), dimm::width_button_rec());
    );
    col.fixed(&row, dimm::height_button_wide());
    add!(col, frame_help, fltk::text::TextDisplay::default());
  );
  // Label
  frame_text.clone()
    .with_align(fltk::enums::Align::Center | fltk::enums::Align::Inside)
    .with_label("Manually copy files with the file manager");
  // Button to the right
  hover_blink!(btn_folder);
  btn_folder.clone()
    .with_color(fltk::enums::Color::Green)
    .with_callback(|_|
    {
      let project = match db::global::get_current_project()
      {
        Ok(project) => project,
        Err(e) => { log_status!("Error to get current project '{}'", e); return; }
      }; // match
      let path_dir_linux = project.path_dir_project.join("linux");
      log_status!("Open {}", path_dir_linux.string());
      let _ = std::process::Command::new("fim_portal")
          .stderr(std::process::Stdio::inherit())
          .stdout(std::process::Stdio::inherit())
          .arg("xdg-open")
          .arg(&path_dir_linux)
          .spawn();
    });
  // Explanation
  let mut frame_help = frame_help.clone()
    .with_frame(fltk::enums::FrameType::BorderBox)
    .with_color(fltk::enums::Color::BackGround);
  frame_help.wrap_mode(fltk::text::WrapMode::AtBounds, 0);
  frame_help.visible_focus(false);
  frame_help.set_buffer(fltk::text::TextBuffer::default());
  frame_help.insert("Click on the file icon to open a folder with the file manager, you can copy your application files to this folder.");
} // }}}

// fn method_install() {{{
fn method_install(tx: Sender<common::Msg>)
{
  // Layout
  column!(col,
    row!(row,
      add!(row, frame_text, shared::fltk::frame::bordered());
      fixed!(row, btn_install, shared::fltk::button::rect::install(), dimm::width_button_rec());
    );
    col.fixed(&row, dimm::height_button_wide());
    add!(col, frame_help, fltk::text::TextDisplay::default());
  );
  // Dialog
  frame_text.clone()
    .with_align(fltk::enums::Align::Center | fltk::enums::Align::Inside)
    .with_label("Install the application from a wizard");
  // Button to the right
  hover_blink!(btn_install);
  let mut btn_install = btn_install.clone()
    .with_color(fltk::enums::Color::Green);
  btn_install.emit(tx, common::Msg::DrawLinuxRom);
  // Explanation
  let mut frame_help = frame_help.clone()
    .with_frame(fltk::enums::FrameType::BorderBox)
    .with_color(fltk::enums::Color::BackGround);
  frame_help.wrap_mode(fltk::text::WrapMode::AtBounds, 0);
  frame_help.visible_focus(false);
  frame_help.set_buffer(fltk::text::TextBuffer::default());
  frame_help.insert("Use an executable to install your application, this is useful for wizard installation such as GOG installers.");
  frame_help.insert(" When installing games from GOG, use the default installation path that appears in the select the install location frame.");
} // }}}

// fn method_next() {{{
fn method_next(tx: Sender<common::Msg>, mut button: fltk::button::Button)
{
  button.set_callback(move |_|
  {
    // Get the items to select from the backend
    let vec_roms = match gameimage::search::search_local("rom")
    {
      Ok(result) => result,
      Err(e) => { log_status!("{}", e); vec![] },
    }; // match

    // Check if is not empty
    if vec_roms.is_empty()
    {
      log_status!("No file found, either install or copy");
      return;
    } // if

    tx.send(common::Msg::DrawLinuxDefault);
  });
} // }}}

// pub fn method() {{{
pub fn method(tx: Sender<common::Msg>, title: &str)
{
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);
  // Layout
  column!(col,);
  // Configure buttons
  let btn_next = ui.btn_next.clone();
  ui.btn_prev.clone().emit(tx, common::Msg::DrawLinuxIcon);
  rescope!(col,
    // Explore with the file manager
    method_explore();
    // Install with a wizard
    method_install(tx);
    // Install with a wizard
    method_next(tx, btn_next.clone());
  );
} // method() }}}

// pub fn rom() {{{
pub fn rom(tx: Sender<common::Msg>, title: &str)
{
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);
  // Layout
  column!(col,
    let term = frame::term::Term::default();
    col.add(&term.group);
    col.fixed(&Frame::default()
        .with_label("Click on this field to search for a file to execute")
        .with_align(Align::Left | Align::Inside)
      , dimm::height_text()
    );
    fixed!(col, input_script, fltk::input::FileInput::default(), dimm::height_button_wide());
    col.fixed(&Frame::default()
        .with_label("Send commands to the process here, type and press enter")
        .with_align(Align::Left | Align::Inside)
      , dimm::height_text()
    );
    fixed!(col, input_cmd, fltk::input::Input::default(), dimm::height_button_wide());
  );
  // Configure navigation buttons
  ui.btn_prev.clone().emit(tx.clone(), common::Msg::DrawLinuxMethod);
  ui.btn_next.clone().deactivate();
  // Currently running process
  let arc_process : Arc<Mutex<Option<Arc<Mutex<std::process::Child>>>>> = Arc::new(Mutex::new(None));
  // Field that shows the currently selected file
  let mut input_script = input_script.clone();
  input_script.set_readonly(true);
  // Input to send commands to the running process
  let mut input_cmd = input_cmd.clone();
  input_cmd.deactivate();
  let clone_arc_process = arc_process.clone();
  input_cmd.handle(move |input, ev|
  {
    if ! ( ev == fltk::enums::Event::KeyUp && fltk::app::event_key() == fltk::enums::Key::Enter )
    {
      return false;
    } // if

    let arc_inner = if let Some(arc_inner) = clone_arc_process.lock().unwrap().as_ref()
    {
      arc_inner.clone()
    } // if
    else
    {
      input.deactivate();
      log_status!("No process running?");
      return false;
    }; // else

    let mut lock = match arc_inner.lock()
    {
      Ok(lock) => lock,
      Err(e) => { input.deactivate(); log_status!("Could not get lock to currently running process: {}", e); return false; },
    };

    match lock.stdin.as_mut()
    {
      Some(stdin) =>
      {
        let _ = writeln!(stdin, "{}", input.value());
        input.set_value("");
        let _ = input.take_focus();
      }, // Some
      None => input.deactivate(),
    } // if

    true
  });
  // Setup on-click file browser bar
  let mut clone_term = term.clone();
  let clone_arc_process = arc_process.clone();
  let clone_input_cmd = input_cmd.clone();
  input_script.set_callback(move |e|
  {
    let str_choice = match fltk::dialog::file_chooser("Select the script to execute", "*.{sh}", ".", false)
    {
      Some(str_choice) => str_choice,
      None => { log_status!("No file selected"); return; },
    }; // if

    // Set displayed path
    e.set_value("");
    let _ = e.insert(&str_choice);

    // Get pathbuf
    let script = PathBuf::from(&str_choice);

    // chmod +x /path/to/script
    let _ = std::fs::set_permissions(script, std::fs::Permissions::from_mode(0o755));

    // Dispatch the process & clone into the global arc
    // The dispatch command kills the previous process in the terminal
    let mut clone_input_cmd = clone_input_cmd.clone();

    let mut json_args = json!({});
    json_args["op"] = "install".into();
    json_args["install"]["op"] = "install".into();
    json_args["install"]["sub_op"] = "rom".into();
    json_args["install"]["args"] = vec![str_choice.clone()].into();
    match clone_term.dispatch(vec![&crate::gameimage::gameimage::binary().unwrap_or_default().string()
      , &json_args.to_string()]
      , |_| {})
    {
      Ok(arc_child) =>
      {
        // Make the global process available to both callbacks
        *clone_arc_process.lock().unwrap() = Some(arc_child.clone());
        // Activate the input field while process is running
        clone_input_cmd.activate();
        fltk::app::awake();
      } // Ok
      Err(e) => log_status!("Could not spawn new process: {}", e),
    } // match
  }); // set_callback
} // }}}

// get_path_db() {{{
fn get_path_db() -> anyhow::Result<PathBuf>
{
  let global = db::global::read()?;
  Ok(global.get_project_dir(&global.project)?)
} // get_path_db() }}}

// get_path_db_executable() {{{
fn get_path_db_executable() -> anyhow::Result<PathBuf>
{
  let mut path_file_db = get_path_db()?;
  path_file_db.push("gameimage.executable.json");
  Ok(path_file_db)
} // get_path_db_executable() }}}

// get_path_db_args() {{{
fn get_path_db_args() -> anyhow::Result<PathBuf>
{
  let mut path_file_db = get_path_db()?;
  path_file_db.push("gameimage.args.json");
  Ok(path_file_db)
} // get_path_db_args() }}}

// get_path_db_alias() {{{
fn get_path_db_alias() -> anyhow::Result<PathBuf>
{
  let mut path_file_db = get_path_db()?;
  path_file_db.push("gameimage.alias.json");
  Ok(path_file_db)
} // get_path_db_alias() }}}

// default_db {{{
fn default_db(input: fltk_evented::Listener<fltk::input::Input>
  , path_file_db: PathBuf
  , item: PathBuf)
{
  input.clone().on_keyup(move |e|
  {
    if e.value().trim().is_empty()
    {
      let _ = shared::db::kv::erase(&path_file_db, item.string());
      return;
    }; // if
    match shared::db::kv::write(&path_file_db, &item.string(), &e.value())
    {
      Ok(()) => (),
      Err(e) => log_status!("Could not write to db: {}", e),
    };
  });
} // default_db }}}

// fn default_play() {{{
fn default_play(path_file_item: &std::path::PathBuf) -> anyhow::Result<()>
{
  // Set the selected binary as default
  gameimage::select::select("rom", &path_file_item)?;
  // Test the selected binary
  gameimage::test::test()?;
  Ok(())
}
// fn default_play() }}}

// fn default_search() {{{
fn default_search(query: &str) -> Vec<PathBuf>
{
  let mut results: Vec<PathBuf> = gameimage::search::search_local("rom")
    .unwrap_or_default()
    .iter()
    .filter(|e| e.string().to_lowercase().contains(&query.to_lowercase()))
    .map(|e| e.clone())
    .collect();
  results.sort_by_key(|k| k.components().count());
  results
} // fn default_search() }}}

// fn default_folder() {{{
fn default_folder(path_file_item: PathBuf) -> anyhow::Result<()>
{
  // Get executable directory
  let mut path_dir_executable = db::global::get_current_project()?.path_dir_project.join(&path_file_item);
  // Get executable directory
  if ! path_dir_executable.pop() { log_status!("Could not open executable: {}", path_dir_executable.string()); } // if
  log_status!("Open '{}'", path_dir_executable.string());
  // Open with xdg-open
  let _ = std::process::Command::new("fim_portal")
      .stderr(std::process::Stdio::inherit())
      .stdout(std::process::Stdio::inherit())
      .arg("xdg-open")
      .arg(&path_dir_executable.string())
      .spawn();
  Ok(())
}
// default_folder() }}}

// fn default_entry() {{{
fn default_entry(tx: Sender<common::Msg>
  , executable_arguments: &std::collections::HashMap<String, String>
  , executable_alias: &std::collections::HashMap<String, String>
  , item: &PathBuf
  , vec_radio_path: &mut Vec<(fltk::button::RadioButton,PathBuf)>)
{
  column!(col,
    col.set_spacing(dimm::border_half());
    row!(row_fst,
      let btn_check = shared::fltk::button::rect::checkmark::<fltk::button::RadioButton>();
      row_fst.fixed(&btn_check, dimm::width_button_rec());
      add!(row_fst, output, output::Output::default());
      fixed!(row_fst, btn_folder, shared::fltk::button::rect::folder(), dimm::width_button_rec());
      fixed!(row_fst, btn_run, shared::fltk::button::rect::play(), dimm::width_button_rec());
    );
    col.fixed(&row_fst.clone(), dimm::height_button_wide());
    col.fixed(&fltk::frame::Frame::default()
      .with_align(Align::Inside | Align::Left)
      .with_label("Executable arguments"), dimm::height_text()
    );
    let mut input_arguments : fltk_evented::Listener<_> = fltk::input::Input::default().into();
    col.fixed(&input_arguments.clone().as_base_widget(), dimm::height_button_wide());
    col.fixed(&fltk::frame::Frame::default()
      .with_align(Align::Inside | Align::Left)
      .with_label("Executable alias"), dimm::height_text()
    );
    let mut input_alias : fltk_evented::Listener<_> = fltk::input::Input::default().into();
    col.fixed(&input_alias.clone().as_base_widget(), dimm::height_button_wide());
    let mut btn_selectable = shared::fltk::button::rect::checkbutton()
      .with_align(Align::Inside | Align::Left)
      .with_color(Color::BackGround)
      .with_label(" Make this executable selectable in the launcher");
    col.fixed(&btn_selectable.clone(), dimm::width_checkbutton());
    col.fixed(&shared::fltk::separator::horizontal(col.w()), dimm::height_sep());
  );
  col.resize(col.x(),col.y(),col.w()
    , dimm::height_button_wide()*3+dimm::height_text()*2+dimm::width_checkbutton()+dimm::height_sep()+dimm::border_half()*7
  );
  // Configure buttons
  hover_blink!(btn_run);
  hover_blink!(btn_folder);
  // Checkbutton
  // Include values into shared vector
  vec_radio_path.push((btn_check.clone(), PathBuf::from(item.to_owned())));
  // Label with file name
  let _ = output.clone().insert(&item.string());
  // Button to open file in file manager
  let clone_item = item.clone();
  btn_folder.clone().set_callback(move |_| { let _ = default_folder(clone_item.clone()); });
  // Button to run the selected binary
  btn_run.clone()
    .with_color(Color::Green)
    .with_callback(#[clown] move |_|
    {
      let item = honk!(item).clone();
      tx.send_awake(common::Msg::WindDeactivate);
      std::thread::spawn(#[clown] move ||
      {
        log_err_status!(default_play(&item));
        tx.send_awake(common::Msg::WindActivate);
      });
    });
  // Configure arguments input
  default_db(input_arguments.clone(), get_path_db_args().unwrap_or_default(), item.clone());
  if executable_arguments.contains_key(&item.string())
  {
    input_arguments.set_value(executable_arguments[&item.string()].as_str());
  } // if
  default_db(input_alias.clone(), get_path_db_alias().unwrap_or_default(), item.clone());
  if executable_alias.contains_key(&item.string())
  {
    input_alias.set_value(executable_alias[&item.string()].as_str());
  } // if
  // Configure selectable in launcher
  let clone_path_file_db_executable = get_path_db_executable().unwrap_or_default();
  btn_selectable.set_value(shared::db::kv::read(&clone_path_file_db_executable).unwrap_or_default().contains_key(&output.value()));
  btn_selectable.set_callback(move |e|
  {
    if e.value()
    {
      if let Err(e) = shared::db::kv::write(&clone_path_file_db_executable, &output.value(), &"1".to_string())
      {
        log_status!("Could not insert key '{}' in db: {}", output.value(), e);
      } // if
    }
    else
    {
      if let Err(e) = shared::db::kv::erase(&clone_path_file_db_executable, output.value())
      {
        log_status!("Could not remove key '{}' from db: {}", output.value(), e);
      } // if
    }
  });
} // fn default_entry() }}}

// fn default() {{{
pub fn default(tx: Sender<common::Msg>, title: &str)
{
  const COUNT_ITEM_PER_PAGE: usize = 20;
  static QUERY : LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));
  static RESULTS : LazyLock<Mutex<Vec<PathBuf>>> = LazyLock::new(|| Mutex::new(vec![]));
  static PAGE : LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(0));
  // Update results if empty
  if RESULTS.lock().unwrap().is_empty()
  {
    *RESULTS.lock().unwrap() = default_search(&QUERY.lock().unwrap().clone());
  } // if
  // Refresh GUI
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);
  // Create layout
  column!(col,
    col.set_spacing(dimm::border_half());
    let (col_search, mut input_search) = shared::fltk::search_column2(
      "Input a search term to filter executables, press enter to confirm"
    );
    col.fixed(&col_search, dimm::height_button_wide() + dimm::height_text() + dimm::border());
    scroll!(scroll,
      scroll.set_type(fltk::group::ScrollType::VerticalAlways);
      scroll.set_scrollbar_size(dimm::border());
      hpack!(col_entries,
        col_entries.set_spacing(dimm::border());
      );
      scroll.add(&col_entries);
    );
    col.add(&scroll);
    let (col_paginator, mut input_page) = shared::fltk::paginator::paginator(|| { PAGE.lock().unwrap().clone() }
    , move |value|
    {
      tx.send_awake(common::Msg::WindDeactivate);
      std::thread::spawn(move ||
      {
        *PAGE.lock().unwrap() = value;
        tx.send_activate(common::Msg::DrawLinuxDefault);
      });
    },
    || { RESULTS.lock().unwrap().len() / COUNT_ITEM_PER_PAGE });
    col.fixed(&col_paginator, col_paginator.h());
  );
  // Configure buttons
  ui.btn_prev.clone().emit(tx, common::Msg::DrawLinuxMethod);
  // Initialize input field
  input_search.set_value(&QUERY.lock().unwrap().clone());
  input_search.on_keydown(move |e|
  {
    let mut query = QUERY.lock().unwrap();
    let key = fltk::app::event_key();
    if key == fltk::enums::Key::Enter || e.value().is_empty()
    {
      if key != fltk::enums::Key::Enter && query.is_empty() { return; }
      tx.send_awake(common::Msg::WindDeactivate);
      *query = e.value();
      let query = query.clone();
      std::thread::spawn(move ||
      {
        *RESULTS.lock().unwrap() = default_search(&query);
        tx.send_activate(common::Msg::DrawLinuxDefault);
      });
    } // if
  });
  // Save items to select
  let arc_items : Arc<Mutex<Vec<(button::RadioButton, PathBuf)>>> = Arc::new(Mutex::new(vec![]));
  // Resize entries with scroll
  scroll.resize_callback({
    let mut col_entries = col_entries.clone();
    move |_,x,y,w,_|
    {
      col_entries.resize(x,y,w-dimm::border_half()*3,col_entries.h());
    }
  });
  // Arguments database
  let hash_executable_arguments = shared::db::kv::read(&get_path_db_args().unwrap_or_default()).unwrap_or_default();
  let hash_executable_alias = shared::db::kv::read(&get_path_db_alias().unwrap_or_default()).unwrap_or_default();
  // Create a column for the element entries
  rescope!(col_entries,
    let results = RESULTS.lock().unwrap();
    let start = (PAGE.lock().unwrap().clone() * COUNT_ITEM_PER_PAGE).min(results.len());
    let end = (start + COUNT_ITEM_PER_PAGE).min(results.len());
    for rom in results.clone().drain(start..end)
    {
      default_entry(tx.clone()
        , &hash_executable_arguments
        , &hash_executable_alias
        , &rom
        , &mut arc_items.lock().unwrap());
    } // for
  );
  // Set callbacks for toggle group
  for guard in arc_items.lock().unwrap().iter_mut()
  {
    guard.0.set_callback(#[clown] move |e|
    {
      for i in honk!(arc_items).lock().unwrap().iter_mut() { i.0.toggle(false); }
      e.toggle(true);
    });
  } // for
  let clone_arc_items = arc_items.clone();
  let clone_tx = tx.clone();
  ui.btn_next.clone().set_callback(move |_|
  {
    // Get the vector
    let vec_items = clone_arc_items.lock().unwrap();
    // Get the selected button label (it contains the path to the default binary)
    let path_file_rom = match vec_items.iter().find(|x| x.0.is_set() )
    {
      Some(value) => value.1.clone(),
      None => { log_alert!("No file selected!"); return; },
    }; // match
    // Select rom
    log_err_status!(gameimage::select::select("rom", &path_file_rom));
    // Draw test
    clone_tx.send_awake(common::Msg::DrawLinuxCompress);
  });

} // default() }}}

// pub fn compress() {{{
pub fn compress(tx: Sender<common::Msg>, title: &str)
{
  wizard::compress::compress(tx.clone()
    , title
    , common::Msg::DrawLinuxDefault
    , common::Msg::DrawLinuxCompress
    , common::Msg::DrawCreator);
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
