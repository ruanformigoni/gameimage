// Gui
use std::
{
  path,
  path::PathBuf,
  sync::{Arc,Mutex,LazyLock}
};

use fltk::prelude::*;
use fltk::{
  app::Sender,
  button,
  output,
  frame::Frame,
  dialog,
  enums::{Color,Align},
};

use clown::clown;
use anyhow::anyhow as ah;
use serde_json::json;

use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;
use shared::std::PathBufExt;
use shared::dimm;
use shared::{rescope,hover_blink,column,row,add,fixed,scroll,hpack};

use crate::log;
use crate::log_alert;
use crate::log_err;
use crate::log_status;
use crate::log_err_status;
use crate::log_return_void;
use crate::db;
use crate::common;
use crate::frame;
use crate::wizard;
use crate::gameimage;

// fn library_common() {{{
fn library_common() -> Vec<&'static str>
{
  vec![
    "xact",
    "xact_x64",
    "xinput",
    "binkw32",
    "xaudio29",
    "openal",
  ]
} // fn library_common() }}}

// fn library_vcrun() {{{
fn library_vcrun(year: u32) -> Vec<&'static str>
{
  match year
  {
    val if val < 2003 => vec!["vcrun6"],
    2003..=2008       => vec!["vcrun2003", "vcrun2005", "vcrun2008"],
    2009..=2011       => vec!["vcrun2005", "vcrun2008", "vcrun6sp6", "vcrun2010",],
    2012..=2015       => vec!["vcrun2008", "vcrun2012", "vcrun2013",],
    2016..=2019       => vec!["vcrun2013", "vcrun2015", "vcrun2017",],
    _                 => vec!["vcrun2017", "vcrun2019", "vcrun2022",],
  }
} // fn library_vcrun() }}}

// fn library_vbrun() {{{
fn library_vbrun(year: u32) -> Vec<&'static str>
{
  match year
  {
    val if val <= 1993 => vec!["vb2run"],
    1994..=1998 => vec!["vb2run","vb3run", "vb4run"],
    1999..=2001 => vec!["vb3run", "vb4run", "dx8vb", "vb5run"],
    _ => vec!["vb6run", "dx8vb"],
  }
} // fn library_vbrun() }}}

// fn library_dotnet() {{{
fn library_dotnet(year: u32) -> Vec<&'static str>
{
  match year
  {
    var if var <= 2004  => vec!["dotnet11","dotnet11sp1",],
    2005..=2006         => vec!["dotnet11sp1","dotnet20","dotnet30",],
    2007                => vec!["dotnet20","dotnet30sp1","dotnet35",],
    2008                => vec!["dotnet20sp1","dotnet35sp1",],
    2009..=2011         => vec!["dotnet20sp2","dotnet40","dotnet35sp1",],
    2012                => vec!["dotnet45","dotnet452","dotnet35sp1",],
    2013..=2015         => vec!["dotnet35sp1","dotnet461","dotnet46",],
    2016                => vec!["dotnet35sp1","dotnet46","dotnet462",],
    2017..=2018         => vec!["dotnet35sp1", "dotnet46", "dotnet471","dotnet472",],
    2019                => vec!["dotnet471","dotnet472","dotnet48",],
    2020                => vec!["dotnet471","dotnet472","dotnet48","dotnetcore2","dotnetcore3",],
    2023                => vec!["dotnet48","dotnetcore2","dotnetcore3","dotnet6","dotnet7",],
    _                   => vec!["dotnetcore2","dotnetcore3","dotnet6","dotnet7","dotnet8",],
  }
} // fn library_dotnet() }}}

// fn library_wmp() {{{
#[allow(dead_code)] fn library_wmp(year: u32) -> Vec<&'static str>
{
  match year
  {
    val if val < 2006 => vec!["wmp9"],
    val if val < 2007 => vec!["wmp10"],
    _                 => vec!["wmp11"],
  }
} // fn library_wmp() }}}

// fn get_recomends_winetricks() {{{
fn get_recomends_winetricks(year: u32) -> Vec<&'static str>
{
  let mut libraries: Vec<&'static str> = vec![];
  libraries.append(&mut library_common());
  libraries.append(&mut library_vcrun(year));
  libraries.append(&mut library_vbrun(year));
  libraries.append(&mut library_dotnet(year));
  // libraries.append(&mut library_wmp(year));
  libraries
} // fn get_recomends_winetricks() }}}

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>, title: &str)
{
  wizard::name::name(tx.clone()
    , title
    , common::Msg::DrawPlatform
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

// get_path_db_env() {{{
fn get_path_db_env() -> anyhow::Result<PathBuf>
{
  let mut path_file_db = get_path_db()?;
  path_file_db.push("gameimage.env.json");
  Ok(path_file_db)
} // get_path_db_env() }}}

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

// pub fn environment() {{{
pub fn environment(tx: Sender<common::Msg>, title: &str)
{
  // Fetch ui
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);
  // Layout
  row!(row,
    scroll!(scroll,
      hpack!(col_content,);
    );
    column!(col_btn,
      fixed!(col_btn, btn_add, shared::fltk::button::rect::add(), dimm::height_button_rec());
      col_btn.add(&Frame::default());
    );
    row.fixed(&col_btn, dimm::width_button_rec());
  );
  // Buttons
  hover_blink!(btn_add);
  // Configure footer
  ui.btn_next.clone().deactivate();
  ui.btn_prev.clone().emit(tx, common::Msg::DrawWineConfigure);
  // Resize content to scroll width
  scroll.set_type(fltk::group::ScrollType::VerticalAlways);
  scroll.resize_callback({
    let mut col_content = col_content.clone();
    move |_,x,y,w,_|
    {
      col_content.resize(x,y,w-dimm::border_half()*3,col_content.h());
    }
  });
  // Create entries
  let clone_tx = tx.clone();
  let f_make_entry = move |key : String, val : String| -> fltk::group::Pack
  {
    hpack!(col,
      col.set_spacing(dimm::border_half());
      add!(col, output_key, fltk::output::Output::default().with_size(0,dimm::height_button_wide()));
      row!(row_value,
        add!(row_value, output_val, fltk::output::Output::default());
        fixed!(row_value, btn_del, shared::fltk::button::rect::del(), dimm::width_button_rec());
      );
      col.add(&row_value.with_size(0, dimm::height_button_rec()));
      col.add(&shared::fltk::separator::horizontal(col.w()));
      col.add(&Frame::default().with_size(0, 0));
    );
    // Buttons
    hover_blink!(btn_del);
    // Setup key widget
    let mut output_key = output_key.clone();
    output_key.set_align(Align::Left | Align::Inside);
    output_key.set_value(key.as_str());
    // Setup val widget
    let mut output_val = output_val.clone();
    output_val.set_align(Align::Left | Align::Inside);
    output_val.set_value(val.as_str());
    // Erase button
    let clone_key = key.clone();
    let clone_tx = clone_tx.clone();
    let mut btn_del = btn_del.clone();
    btn_del.set_color(Color::Red);
    btn_del.set_callback(move |_|
    {
      let path_file_db = get_path_db_env().unwrap_or_default();
      match shared::db::kv::erase(&path_file_db, clone_key.clone())
      {
        Ok(_) => log_status!("Erased key '{}'", clone_key),
        Err(e) => log_status!("Failed to erase key '{}' with error '{}'", clone_key, e.to_string()),
      } // if
      clone_tx.send_awake(common::Msg::DrawWineEnvironment);
    });
    col
  };

  // Get current database entries
  let path_file_db = get_path_db_env().unwrap_or_default();
  rescope!(col_content,
    if let Ok(entries) = shared::db::kv::read(&path_file_db)
    {
      for (key, val) in entries
      {
        let group = f_make_entry(key, val);
        col_content.add(&group);
      } // for
    } // if
  );

  // Add var button
  let mut btn_add = btn_add.clone().with_color(Color::Green);
  let clone_tx = tx.clone();
  btn_add.set_callback(move |_|
  {
    let dialog = shared::fltk::dialog::key_value();
    let clone_dialog = dialog.clone();
    let clone_tx = clone_tx.clone();
    let clone_path_file_db = path_file_db.clone();
    dialog.btn_ok.clone().set_callback(move |_|
    {
      clone_dialog.wind.clone().hide();
      let key = clone_dialog.input_key.value();
      let value = clone_dialog.input_value.value();
      if key.is_empty() { return; }
      match shared::db::kv::write(&clone_path_file_db, &key, &value)
      {
        Ok(_) => log_status!("Set key '{}' with value '{}'", key.clone(), value.clone()),
        Err(e) => log_status!("Failed to set key '{}' with error '{}'", key, e.to_string()),
      } // if
      clone_tx.send_awake(common::Msg::DrawWineEnvironment);
    });
    dialog.wind.clone().show();
  });
} // }}}

// fn configure_entry() {{{
fn configure_entry(tx: Sender<common::Msg>
  , label: &str
  , f_args: fn() -> Option<Vec<String>>) -> fltk::button::Button
{
  column!(col,
    col.set_spacing(dimm::border_half());
    row!(row,
      row.add(&shared::fltk::frame::bordered()
        .with_align(Align::Inside | Align::Left)
        .with_label(&format!(" {}", label)));
      fixed!(row, btn, &shared::fltk::button::rect::configure(), dimm::width_button_rec());
    );
    col.fixed(&row, dimm::height_button_rec());
    col.fixed(&Frame::default(), 0);
  );
  col.resize(col.x(), col.y(), col.w(), dimm::height_button_rec() + dimm::border_half());
  // Button to the right
  let mut btn = btn.clone().with_color(Color::Green);
  hover_blink!(btn);
  // Set callback
  btn.set_callback(move |_|
  {
    // Check if arguments were passed
    let args_owned : Vec<String> = match f_args()
    {
      Some(args) => args.iter().map(|s| s.to_string()).collect(),
      None => return,
    };
    tx.send_awake(common::Msg::WindDeactivate);
    std::thread::spawn(move ||
    {
      let mut json_args = json!({});
      json_args["op"] = "install".into();
      json_args["install"]["op"] = "install".into();
      json_args["install"]["sub_op"] = args_owned.get(0).map(|e| e.clone()).unwrap_or_default().into();
      json_args["install"]["args"] = args_owned.get(1..).into();
      if gameimage::gameimage::gameimage_sync(vec![&json_args.to_string()]) != 0
      {
        log_status!("Command exited with non-zero status");
      } // else
      tx.send_awake(common::Msg::WindActivate);
    });
  });
  // Return row with label and button
  btn
} // fn configure_entry() }}}

// pub fn configure() {{{
pub fn configure(tx: Sender<common::Msg>, title: &str)
{
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);
  // Layout
  column!(col,
    scroll!(scroll,
      hpack!(col_content,);
    );
  );
  // Configure scroll
  scroll.set_type(fltk::group::ScrollType::VerticalAlways);
  scroll.resize_callback({
    let mut col_content = col_content.clone();
    move |_,x,y,w,_|
    {
      col_content.resize(x,y,w-dimm::border_half()*3,col_content.h());
    }
  });
  // Configure buttons
  ui.btn_prev.clone().emit(tx.clone(), common::Msg::DrawWineIcon);
  let clone_tx = tx.clone();
  ui.btn_next.clone().set_callback(move |_|
  {
    // Get path to wine prefix
    let path_dir_wine_prefix = match db::project::current()
    {
      Ok(project) => match project.get_dir_self()
      {
        Ok(path_dir_self) => path_dir_self.join("wine"),
        Err(e) => log_return_void!("{}", e)
      } // match
      Err(e) => log_return_void!("{}", e)
    }; // match

    if ! path_dir_wine_prefix.exists()
    {
      log_status!("Wine prefix does not exist, creating...");
      tx.send_awake(common::Msg::WindDeactivate);
      std::thread::spawn(move ||
      {
        match gameimage::install::winetricks(vec!["fontsmooth=rgb".into()])
        {
          Ok(_) => log_status!("Created wine prefix"),
          Err(e) => log_status!("{}", e),
        } // else

        clone_tx.send_activate(common::Msg::DrawWineTricks);
      }); // std::thread
      return;
    } // if

    clone_tx.send_awake(common::Msg::DrawWineTricks);
  });

  rescope!(col_content,
    let _ = configure_entry(tx.clone(),  "Install DXVK for directx 9/10/11"
      , || Some(vec!["winetricks".into(), "-f".into(), "dxvk".into()])
    );
    let _ = configure_entry(tx.clone(),  "Install VKD3D for directx 12"
      , || Some(vec!["winetricks".into(), "-f".into(), "vkd3d".into()])
    );
    let _ = configure_entry(tx.clone(),  "Run regedit", || Some(vec!["wine".into(), "regedit".into()]));
    let _ = configure_entry(tx.clone(),  "Run add/remove programs", || Some(vec!["wine".into(), "uninstaller".into()]));
    let _ = configure_entry(tx.clone(),  "Run winetricks GUI", || Some(vec!["winetricks".into(), "--gui".into()]));
    let _ = configure_entry(tx.clone(),  "Run a custom winetricks command" , ||
      dialog::input_default("Enter the winetricks command to execute", "").map(|e| vec!["winetricks".into(), "-f".into(), e])
    );
    let _ = configure_entry(tx.clone(),  "Run a custom wine command" , ||
      dialog::input_default("Enter the wine command to execute", "").map(|e| vec!["wine".into(), e])
    );
    let mut btn = configure_entry(tx.clone(),  "Configure environment", || None);
    btn.emit(tx, common::Msg::DrawWineEnvironment);
  );
} // fn: configure }}}

// pub fn winetricks() {{{
pub fn winetricks(tx: Sender<common::Msg>, title: &str)
{
  static YEAR: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(2024));
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);
  // Layout
  row!(row,
    column!(col,
      col.fixed(&fltk::frame::Frame::default().with_label("Select the Game Release Year"), dimm::height_text());
      fixed!(col, menu_year, fltk::menu::MenuButton::default(), dimm::height_button_wide());
      col.fixed(&fltk::frame::Frame::default().with_label("Recommended Libraries"), dimm::height_text());
      add!(col, browser, fltk::browser::CheckBrowser::default());
    );
    column!(col_btn,
      fixed!(col_btn, btn_install, shared::fltk::button::rect::install(), dimm::width_button_rec());
      col_btn.add(&Frame::default());
    );
    row.fixed(&col_btn, dimm::width_button_rec());
  );
  // Configure buttons
  hover_blink!(btn_install);
  ui.btn_prev.clone().emit(tx.clone(), common::Msg::DrawWineConfigure);
  ui.btn_next.clone().emit(tx.clone(), common::Msg::DrawWineRom);
  // Select year
  let mut menu_year = menu_year.clone();
  for i in 1993..2025 { menu_year.add_choice(&i.to_string()); }
  menu_year.set_label(&YEAR.lock().unwrap().to_string());
  menu_year.set_callback(#[clown] |e|
  {
    if let Some(choice) = e.choice()
    {
      *YEAR.lock().unwrap() = choice.parse().unwrap();
      e.set_label(&choice);
      honk!(tx).send(common::Msg::DrawWineTricks)
    }
  });
  // Recommend libraries by year
  let vec_lib = get_recomends_winetricks(*YEAR.lock().unwrap());
  let mut browser = browser.clone();
  for lib in vec_lib { browser.add(lib, true); }
  // Install button to the right
  btn_install.clone()
    .right_of(&col.as_base_widget(), dimm::border())
    .with_color(Color::Green)
    .with_callback(move |_|
    {
      // Function to get all checked items
      tx.send_awake(common::Msg::WindDeactivate);
      let clone_browser = browser.clone();
      std::thread::spawn(move ||
      {
        // Must install one at the time, winetricks exits if at least one verb fails
        for lib in &mut (1..=clone_browser.size())
          .filter(|e| clone_browser.checked(*e as i32))
          .map(|e| clone_browser.text(e as i32).unwrap())
        {
          log_status!("Installing '{}'", lib);
          let mut json_args = json!({});
          json_args["op"] = "install".into();
          json_args["install"]["op"] = "install".into();
          json_args["install"]["sub_op"] = "winetricks".into();
          json_args["install"]["args"] = vec!["-f", "-q", lib.as_str()].into();
          if gameimage::gameimage::gameimage_sync(vec![&json_args.to_string()]) != 0
          {
            log_status!("Command exited with non-zero status");
          } // else
        } // for
        tx.send_awake(common::Msg::WindActivate);
      });
    });
} // fn: winetricks }}}

// rom_folder() {{{
fn rom_folder(path_file_item: PathBuf) -> anyhow::Result<()>
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
// rom_folder() }}}

// rom_exec() {{{
fn rom_exec(path_file_item: PathBuf) -> anyhow::Result<()>
{
  // Set the selected binary as default
  gameimage::select::select("rom", &path_file_item)?;
  // Test the selected binary
  gameimage::test::test()?;
  Ok(())
}
// rom_exec() }}}

// rom_add() {{{
fn rom_add() -> anyhow::Result<()>
{
  // Pick files to install
  let mut chooser = dialog::FileChooser::new("."
    , "*"
    , dialog::FileChooserType::Single
    , "Pick a file to install with wine");
  // Start dialog
  chooser.show();
  // Wait for choice(s)
  while chooser.shown() { std::thread::sleep(std::time::Duration::from_millis(100)) } // while
  // Check if choice is valid
  let str_choice = chooser.value(1).ok_or(ah!("No file selected"))?;
  // Execute wine
  gameimage::install::wine(vec![str_choice])?;
  Ok(())
} // rom_add() }}}

// rom_db {{{
fn rom_db(input: fltk_evented::Listener<fltk::input::Input>
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
} // rom_db }}}

// rom_entry() {{{
fn rom_entry(tx: Sender<common::Msg>
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
  btn_folder.clone().set_callback(move |_| { let _ = rom_folder(clone_item.clone()); });
  // Button to run the selected wine binary
  btn_run.clone()
    .with_color(Color::Green)
    .with_callback(#[clown] move |_|
    {
      let item = honk!(item).clone();
      tx.send_awake(common::Msg::WindDeactivate);
      std::thread::spawn(#[clown] move ||
      {
        log_err_status!(rom_exec(item));
        tx.send_awake(common::Msg::WindActivate);
      });
    });
  // Configure arguments input
  rom_db(input_arguments.clone(), get_path_db_args().unwrap_or_default(), item.clone());
  if executable_arguments.contains_key(&item.string())
  {
    input_arguments.set_value(executable_arguments[&item.string()].as_str());
  } // if
  rom_db(input_alias.clone(), get_path_db_alias().unwrap_or_default(), item.clone());
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
} // rom_entry() }}}

// rom_search() {{{
fn rom_search(query: &str) -> Vec<PathBuf>
{
  let mut results: Vec<PathBuf> = gameimage::search::search_local("rom")
    .unwrap_or_default()
    .iter()
    .filter(|e| e.string().to_lowercase().contains(&query.to_lowercase()))
    .map(|e| e.clone())
    .collect();
  results.sort_by(|a, b|
  {
    let a_components = a.components().count();
    let b_components = b.components().count();
    if a_components == b_components
    {
      a.string().len().cmp(&b.string().len())
    } // if
    else
    {
      a_components.cmp(&b_components)
    } // else
  });
  results
} // rom_search() }}}

// rom_modified() {{{
fn rom_modified() -> Vec<PathBuf>
{
  let hash_executable_arguments = shared::db::kv::read(&get_path_db_args().unwrap_or_default()).unwrap_or_default();
  let hash_executable_alias = shared::db::kv::read(&get_path_db_alias().unwrap_or_default()).unwrap_or_default();
  let hash_executable_use = shared::db::kv::read(&get_path_db_executable().unwrap_or_default()).unwrap_or_default();
  let mut results: Vec<PathBuf> = hash_executable_arguments.keys()
    .chain(hash_executable_alias.keys())
    .chain(hash_executable_use.keys())
    .map(|e| PathBuf::from(e))
    .collect();
  // Remove duplicate values
  results.sort();
  results.dedup();
  // Sort by number of components, if is equal, sort by string length
  results.sort_by(|a, b|
  {
    let a_components = a.components().count();
    let b_components = b.components().count();
    if a_components == b_components
    {
      a.string().len().cmp(&b.string().len())
    } // if
    else
    {
      a_components.cmp(&b_components)
    } // else
  });
  results
} // rom_modified() }}}

// pub fn rom() {{{
pub fn rom(tx: Sender<common::Msg>, title: &str)
{
  const COUNT_ITEM_PER_PAGE: usize = 4;
  static QUERY : LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));
  static RESULTS : LazyLock<Mutex<Vec<PathBuf>>> = LazyLock::new(|| Mutex::new(vec![]));
  static PAGE : LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(0));
  static SELECTED : LazyLock<Mutex<PathBuf>> = LazyLock::new(|| Mutex::new(PathBuf::default()));
  static SHOW_SELECTED : LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
  // Update results if empty
  if RESULTS.lock().unwrap().is_empty() && ! *SHOW_SELECTED.lock().unwrap()
  {
    *RESULTS.lock().unwrap() = rom_search(&QUERY.lock().unwrap());
  } // if
  // UI
  let ui = crate::GUI.lock().unwrap().ui.clone()(title);
  // Layout
  row!(row,
    column!(col,
      let (col_search, mut input_query) = shared::fltk::search_column2(
        "Input a search term to filter executables, press enter to confirm"
      );
      col_search.end();
      col.fixed(&col_search, col_search.h());
      col.fixed(&shared::fltk::separator::horizontal(col.w()), dimm::height_sep());
      let mut btn_show_selected = shared::fltk::button::rect::checkbutton()
        .with_align(Align::Inside | Align::Left)
        .with_color(Color::BackGround)
        .with_label(" Only show modified entries");
      col.fixed(&btn_show_selected, dimm::width_checkbutton() + dimm::border());
      col.fixed(&shared::fltk::separator::horizontal(col.w()), dimm::height_sep());
      scroll!(scroll,
        hpack!(col_content,);
      );
      col.fixed(&shared::fltk::separator::horizontal(col.w()), dimm::height_sep());
      let col_paginator = shared::fltk::paginator::paginator(|| { PAGE.lock().unwrap().clone() }
        , move |value|
        {
          tx.send_awake(common::Msg::WindDeactivate);
          std::thread::spawn(move ||
          {
            *PAGE.lock().unwrap() = value;
            tx.send_activate(common::Msg::DrawWineRom);
          });
        },
        || { RESULTS.lock().unwrap().len() / COUNT_ITEM_PER_PAGE }
      );
      col.fixed(&col_paginator, col_paginator.h());
    );
    row.add(&col);
    column!(col_sidebar,
      fixed!(col_sidebar, btn_add, shared::fltk::button::rect::add(), dimm::height_button_rec());
      fixed!(col_sidebar, btn_refresh, shared::fltk::button::rect::refresh(), dimm::height_button_rec());
    );
    row.fixed(&col_sidebar, dimm::width_button_rec());
  );
  // Configure buttons
  hover_blink!(btn_add);
  hover_blink!(btn_refresh);
  // Configure scroll
  scroll.set_type(fltk::group::ScrollType::VerticalAlways);
  scroll.resize_callback({
    let mut col_content = col_content.clone();
    move |_,x,y,w,_|
    {
      col_content.resize(x,y,w-dimm::border_half()*3,col_content.h());
    }
  });
  // Configure buttons
  ui.btn_prev.clone().emit(tx.clone(), common::Msg::DrawWineTricks);
  input_query.set_value(&QUERY.lock().unwrap().clone());
  input_query.on_keydown(move |e|
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
        *RESULTS.lock().unwrap() = rom_search(&query);
        tx.send_activate(common::Msg::DrawWineRom);
      });
    } // if
  });
  // Configure 'show selected' button
  btn_show_selected.set_value(*SHOW_SELECTED.lock().unwrap());
  btn_show_selected.set_callback(move |e|
  {
    // Only display modified items
    if e.is_checked()
    {
      // Get modified entries
      let mut results = rom_modified();
      // Get selected entry
      let selected = SELECTED.lock().unwrap().to_path_buf();
      if ! results.contains(&selected) && selected.components().count() > 0 { results.push(selected); } // if
      // Update results
      *RESULTS.lock().unwrap() = results;
      *SHOW_SELECTED.lock().unwrap() = true;
      tx.send_activate(common::Msg::DrawWineRom);
    } // if
    // Display all items
    else
    {
      *RESULTS.lock().unwrap() = rom_search(&QUERY.lock().unwrap().clone());
      *SHOW_SELECTED.lock().unwrap() = false;
      tx.send_activate(common::Msg::DrawWineRom);
    } // else
  });
  log_err!(input_query.take_focus());
  // Insert items in list of currently installed items
  let vec_radio_path = Arc::new(Mutex::new(Vec::<(button::RadioButton, path::PathBuf)>::new()));
  // Arguments database
  let hash_executable_arguments = shared::db::kv::read(&get_path_db_args().unwrap_or_default()).unwrap_or_default();
  let hash_executable_alias = shared::db::kv::read(&get_path_db_alias().unwrap_or_default()).unwrap_or_default();
  // Get current page
  let page = PAGE.lock().unwrap();
  // Create a column for the element entries
  rescope!(col_content,
    let results = RESULTS.lock().unwrap();
    let start = (*page * COUNT_ITEM_PER_PAGE).min(results.len());
    let end = (start + COUNT_ITEM_PER_PAGE).min(results.len());
    for path in results.clone().drain(start..end)
    {
      rom_entry(tx.clone()
        , &hash_executable_arguments
        , &hash_executable_alias
        , &path
        , &mut vec_radio_path.lock().unwrap())
    } // for
  );
  // Set callbacks for toggle group
  for (btn, path) in vec_radio_path.lock().unwrap().iter_mut()
  {
    if *SELECTED.lock().unwrap() == path.clone()
    {
      btn.set_value(true);
    } // if
    btn.set_callback(#[clown] move |e|
    {
      for i in honk!(vec_radio_path).lock().unwrap().iter_mut() { i.0.toggle(false); }
      *SELECTED.lock().unwrap() = honk!(path).clone();
      e.toggle(true);
    });
  } // for
  // Add new item
  let clone_tx = tx.clone();
  btn_add.clone()
    .with_color(Color::Green)
    .with_callback(move |_|
    {
      clone_tx.send_awake(common::Msg::WindDeactivate);
      std::thread::spawn(move ||{ log_err!(rom_add()); clone_tx.send_activate(common::Msg::DrawWineRom); });
    });
  // Refresh executable list
  let clone_tx = tx.clone();
  btn_refresh.clone()
    .with_color(Color::Blue)
    .with_callback(move |_| { clone_tx.send_awake(common::Msg::DrawWineRom); });
  // Go to next frame iff a default executable was selected
  let clone_tx = tx.clone();
  ui.btn_next.clone().set_callback(move |_|
  {
    // Get selected executable
    let selected = SELECTED.lock().unwrap();
    // Check if is not empty
    if selected.components().count() == 0
    {
      log_alert!("No file path was selected!");
      return;
    } // if
    if let Err(e) = gameimage::select::select("rom", &selected)
    {
      log_status!("{}", e);
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
