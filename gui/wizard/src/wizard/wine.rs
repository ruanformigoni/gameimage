// Gui
use std::
{
  path,
  sync::{Arc,Mutex,LazyLock}
};

use fltk::prelude::*;
use fltk::{
  app::Sender,
  button,
  button::Button,
  output,
  frame::Frame,
  dialog,
  enums::{FrameType,Color,Align},
};

use clown::clown;

use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;
use shared::dimm;

use crate::log;
use crate::log_return_void;
use crate::db;
use crate::common;
use shared::std::PathBufExt;
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
fn library_wmp(year: u32) -> Vec<&'static str>
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
  libraries.append(&mut library_wmp(year));
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
fn get_path_db() -> anyhow::Result<std::path::PathBuf>
{
  let global = db::global::read()?;
  Ok(global.get_project_dir(&global.project)?)
} // get_path_db() }}}

// get_path_db_executable() {{{
fn get_path_db_executable() -> anyhow::Result<std::path::PathBuf>
{
  let mut path_file_db = get_path_db()?;
  path_file_db.push("gameimage.wine.executable.json");
  Ok(path_file_db)
} // get_path_db_executable() }}}

// get_path_db_env() {{{
fn get_path_db_env() -> anyhow::Result<std::path::PathBuf>
{
  let mut path_file_db = get_path_db()?;
  path_file_db.push("gameimage.env.json");
  Ok(path_file_db)
} // get_path_db_env() }}}

// get_path_db_args() {{{
fn get_path_db_args() -> anyhow::Result<std::path::PathBuf>
{
  let mut path_file_db = get_path_db()?;
  path_file_db.push("gameimage.wine.args.json");
  Ok(path_file_db)
} // get_path_db_args() }}}

// pub fn environment() {{{
pub fn environment(tx: Sender<common::Msg>, title: &str)
{
  let path_file_db = match get_path_db_env()
  {
    Ok(e) => e,
    Err(e) => { log!("Could not retrieve path to db file: {}", e); return; }
  }; // match

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
  let mut scroll = shared::fltk::ScrollList::new(
    frame_content.width() - dimm::border()*3 - dimm::width_button_rec()
    , frame_content.height() - dimm::border()*2
    , frame_content.x() + dimm::border()
    , frame_content.y() + dimm::border());
  scroll.set_border(dimm::border(), dimm::border());

  //
  // Create entries
  //
  let clone_tx = tx.clone();
  let mut clone_scroll = scroll.clone();
  let clone_path_file_db = path_file_db.clone();
  let mut f_make_entry = move |key : String, val : String|
  {
    // Setup key widget
    let mut output_key = fltk::output::Output::default()
      .with_size(clone_scroll.widget_ref().w() - dimm::width_button_rec() - dimm::border()*3, dimm::height_button_wide())
      .with_align(Align::Left | Align::Inside);
    clone_scroll.add(&mut output_key.as_base_widget());
    output_key.set_value(key.as_str());
    output_key.set_frame(FrameType::BorderBox);
    output_key.set_text_size(dimm::height_text());
    // Setup val widget
    let mut output_val = fltk::output::Output::default()
      .with_size(clone_scroll.widget_ref().w() - dimm::border()*2, dimm::height_button_wide())
      .with_align(Align::Left | Align::Inside)
      .with_frame(FrameType::BorderBox);
    clone_scroll.add(&mut output_val.as_base_widget());
    output_val.set_value(val.as_str());
    output_val.set_text_size(dimm::height_text());
    // Erase button
    let clone_key = key.clone();
    let clone_tx = clone_tx.clone();
    let clone_path_file_db = clone_path_file_db.clone();
    let _btn_del = shared::fltk::button::rect::del()
      .right_of(&output_key, dimm::border())
      .with_color(Color::Red)
      .with_callback(move |_|
    {
      match shared::db::kv::erase(&clone_path_file_db, clone_key.clone())
      {
        Ok(_) => println!("Erased key '{}'", clone_key),
        Err(e) => println!("Failed to erase key '{}' with error '{}'", clone_key, e.to_string()),
      } // if
      clone_tx.send_awake(common::Msg::DrawWineEnvironment);
    });
    // Separator
    let sep = Frame::default()
      .below_of(&output_val, dimm::border())
      .with_size(clone_scroll.widget_ref().width() - dimm::border()*3, dimm::height_sep())
      .with_frame(FrameType::FlatBox)
      .with_color(Color::Black);
    clone_scroll.add(&mut sep.as_base_widget());
  };

  // Get current database entries
  scroll.begin();
  if let Ok(entries) = shared::db::kv::read(&path_file_db)
  {
    for (key, val) in entries
    {
      println!("Key: {} Val: {}", key, val);
      f_make_entry(key, val);
    } // for
  } // if
  scroll.end();

  // Add var button
  let mut btn_add = shared::fltk::button::rect::add()
    .top_right_of(&frame_content, - dimm::border())
    .with_border(0, dimm::border())
    .with_color(Color::Green);
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
        Ok(_) => println!("Set key '{}' with value '{}'", key.clone(), value.clone()),
        Err(e) => println!("Failed to set key '{}' with error '{}'", key, e.to_string()),
      } // if
      clone_tx.send_awake(common::Msg::DrawWineEnvironment);
    });
    dialog.wind.clone().show();
  });
} // }}}

// fn configure_entry() {{{
fn configure_entry(tx: Sender<common::Msg>
  , label: &str
  , f_args: fn() -> Option<Vec<String>>) -> (fltk::group::Row, fltk::frame::Frame, fltk::button::Button)
{
  let mut row = fltk::group::Row::default();
  // Label
  let label = Frame::default()
    .with_label(label)
    .with_frame(FrameType::BorderBox);
  row.add(&label);
  // Button to the right
  let mut btn = shared::fltk::button::rect::configure()
    .with_size(dimm::width_button_rec(), dimm::height_button_rec())
    .with_color(Color::Green);
  row.fixed(&btn, dimm::width_button_rec());
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
      let slices: Vec<&str> = args_owned.iter().map(|s| s.as_str()).collect();
      if gameimage::gameimage::gameimage_sync(slices) != 0
      {
        log!("Command exited with non-zero status");
      } // else
      tx.send_awake(common::Msg::WindActivate);
    });
  });
  // Return row with label and button
  row.end();
  (row, label, btn)
} // fn configure_entry() }}}

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
      clone_output_status.set_value("Wine prefix does not exist, creating...");
      log!("Wine prefix does not exist, creating...");
      let mut clone_output_status = clone_output_status.clone();
      tx.send_awake(common::Msg::WindDeactivate);
      std::thread::spawn(move ||
      {
        match gameimage::install::winetricks(vec!["fontsmooth=rgb".into()])
        {
          Ok(_) => log!("Created wine prefix"),
          Err(e) => { clone_output_status.set_value("Failed to create wine prefix"); log!("{}", e); },
        } // else

        clone_tx.send_awake(common::Msg::DrawWineTricks);
      }); // std::thread
      return;
    } // if

    clone_tx.send_awake(common::Msg::DrawWineTricks);
  });

  // Create scrollbar
  let mut col = fltk::group::Column::new(
      frame_content.x() + dimm::border()
    , frame_content.y() + dimm::border()
    , frame_content.width() - dimm::border()*2
    , frame_content.height() - dimm::border()*2
    , ""
  );
  let (row,_,_) = configure_entry(tx.clone(),  "Install DXVK for directx 9/10/11"
    , || Some(vec!["install".into(), "winetricks".into(), "-f".into(), "dxvk".into()])
  );
  col.fixed(&row.as_base_widget(), dimm::height_button_wide());
  let (row,_,_) = configure_entry(tx.clone(),  "Install VKD3D for directx 12"
    , || Some(vec!["install".into(), "winetricks".into(), "-f".into(), "vkd3d".into()])
  );
  col.fixed(&row.as_base_widget(), dimm::height_button_wide());
  let (row,_,_) = configure_entry(tx.clone(),  "Run regedit", || Some(vec!["install".into(), "wine".into(), "regedit".into()]));
  col.fixed(&row.as_base_widget(), dimm::height_button_wide());
  let (row,_,_) = configure_entry(tx.clone(),  "Run add/remove programs", || Some(vec!["install".into(), "wine".into(), "uninstaller".into()]));
  col.fixed(&row.as_base_widget(), dimm::height_button_wide());
  let (row,_,_) = configure_entry(tx.clone(),  "Run winetricks GUI", || Some(vec!["install".into(), "winetricks".into(), "--gui".into()]));
  col.fixed(&row.as_base_widget(), dimm::height_button_wide());
  let (row,_,_) = configure_entry(tx.clone(),  "Run a custom winetricks command" , ||
    dialog::input_default("Enter the winetricks command to execute", "").map(|e| vec!["install".into(), "winetricks".into(), "-f".into(), e])
  );
  col.fixed(&row.as_base_widget(), dimm::height_button_wide());
  let (row,_,_) = configure_entry(tx.clone(),  "Run a custom wine command" , ||
    dialog::input_default("Enter the wine command to execute", "").map(|e| vec!["install".into(), "wine".into(), e])
  );
  col.fixed(&row.as_base_widget(), dimm::height_button_wide());
  let (row,_,mut btn) = configure_entry(tx.clone(),  "Configure environment", || None);
  btn.emit(tx, common::Msg::DrawWineEnvironment);
  col.fixed(&row.as_base_widget(), dimm::height_button_wide());
  col.end();
} // fn: configure }}}

// pub fn winetricks() {{{
pub fn winetricks(tx: Sender<common::Msg>, title: &str)
{
  static YEAR: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(2024));
  // Get header and footer
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();
  // Get frames
  let frame_content = ret_frame_header.frame_content;
  let frame_sep = ret_frame_header.sep;
  // Create a column for the menu items
  let mut col = fltk::group::Column::default()
    .below_of(&frame_sep, dimm::border())
    .with_size(frame_content.w() - dimm::border()*3 - dimm::width_button_rec()
      , frame_content.h() - dimm::border()*2
    );
  // Select year
  col.fixed(&fltk::frame::Frame::default().with_label("Select the Game Release Year"), dimm::height_text());
  let mut menu_year = fltk::menu::MenuButton::default();
  for i in 1993..2025 { menu_year.add_choice(&i.to_string()); }
  menu_year.set_label(&YEAR.lock().unwrap().to_string());
  col.fixed(&menu_year, dimm::height_button_wide());
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
  col.fixed(&fltk::frame::Frame::default().with_label("Recommended Libraries"), dimm::height_text());
  let mut browser = fltk::browser::CheckBrowser::default();
  for lib in vec_lib { browser.add(lib, true); }
  col.end();
  // Install button to the right
  shared::fltk::button::rect::install()
    .right_of(&col.as_base_widget(), dimm::border())
    .with_color(Color::Green)
    .with_callback(move |_|
    {
      // Function to get all checked items
      let vec_cmd: Vec<String> = vec!["install".into(), "winetricks".into(), "-f".into(), "-q".into()];
      tx.send_awake(common::Msg::WindDeactivate);
      let clone_browser = browser.clone();
      std::thread::spawn(move ||
      {
        // Must install one at the time, winetricks exits if at least one verb fails
        for lib in &mut (1..=clone_browser.size())
          .filter(|e| clone_browser.checked(*e as i32))
          .map(|e| clone_browser.text(e as i32).unwrap())
        {
          if gameimage::gameimage::gameimage_sync(vec_cmd.iter().map(|e| e.as_str()).chain(vec![lib.as_str()]).collect()) != 0
          {
            log!("Command exited with non-zero status");
          } // else
        } // for
        tx.send_awake(common::Msg::WindActivate);
      });
    });
  // Configure buttons
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), common::Msg::DrawWineConfigure);
  ret_frame_footer.btn_next.clone().emit(tx.clone(), common::Msg::DrawWineRom);
} // fn: winetricks }}}

// pub fn rom() {{{
pub fn rom(tx: Sender<common::Msg>, title: &str)
{
  let path_file_db_args = match get_path_db_args()
  {
    Ok(e) => e,
    Err(e) => { log!("Could not retrieve path to db file: {}", e); std::path::PathBuf::default() }
  }; // match

  let path_file_db_executable = match get_path_db_executable()
  {
    Ok(e) => e,
    Err(e) => { eprintln!("Could not retrieve path to db file: {}", e); std::path::PathBuf::default() }
  }; // match

  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

  // Set previous frame
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), common::Msg::DrawWineTricks);

  // List of the currently installed items
  let frame_list = Frame::default()
    .with_size(frame_content.width() - dimm::border()*3 - dimm::width_button_rec()
      , frame_content.height() - dimm::border()*2)
    .with_pos(frame_content.x() + dimm::border(), frame_content.y() + dimm::border());

  // Create scrollbar
  let mut scroll = shared::fltk::ScrollList::new(
    frame_content.width() - dimm::border()*3 - dimm::width_button_rec()
    , frame_content.height() - dimm::border()*2
    , frame_content.x() + dimm::border()
    , frame_content.y() + dimm::border());
  scroll.set_border(dimm::border(), dimm::border());

  scroll.begin();

  // Insert items in list of currently installed items
  let vec_radio_path = Arc::new(Mutex::new(Vec::<(button::RadioButton, path::PathBuf)>::new()));

  let input_args = match shared::db::kv::read(&path_file_db_args)
  {
    Ok(input_args) => input_args,
    Err(e) => { log!("Could not read input args: {}", e); shared::db::kv::Kv::default() }
  }; // match

  for item in gameimage::search::search_local("rom").unwrap_or_default()
  {
    // Checkbutton
    let btn_check = shared::fltk::button::rect::radio();
    scroll.add(&mut btn_check.as_base_widget());

    // Include values into shared vector
    if let Ok(mut lock) = vec_radio_path.lock()
    {
      lock.push((btn_check.clone(), path::PathBuf::from(item.to_owned())));
    } // if

    // Label with file name
    let mut output = output::Output::default()
      .with_size(scroll.widget_ref().w() - dimm::width_button_rec()*3 - dimm::border()*6, dimm::height_button_wide())
      .right_of(&btn_check, dimm::border());
    let _ = output.insert(&item.string());

    // Button to open file in file manager
    let clone_item = item.clone();
    let mut clone_output_status = ret_frame_footer.output_status.clone();
    let btn_folder = shared::fltk::button::rect::folder()
      .right_of(&output, dimm::border())
      .with_callback(move |_|
      {
        let project = match db::global::get_current_project()
        {
          Ok(project) => project,
          Err(e) => { log!("Error to get current project '{}'", e); return; }
        }; // match

        let mut path_dir_executable = project.path_dir_project.join(&clone_item);

        if ! path_dir_executable.pop()
        {
          log!("Could not open executable: {}", path_dir_executable.string());
        } // if

        clone_output_status.set_value(format!("Open '{}'", path_dir_executable.string()).as_str());

        let _ = std::process::Command::new("fim_portal")
            .stderr(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .arg("xdg-open")
            .arg(&path_dir_executable.string())
            .spawn();
      });

    let clone_tx = tx.clone();
    let clone_item = item.clone();
    let _btn_run = shared::fltk::button::rect::play()
      .right_of(&btn_folder, dimm::border())
      .with_callback(move |_|
      {
        // Execute wine
        let clone_item = clone_item.clone();
        clone_tx.send_awake(common::Msg::WindDeactivate);
        std::thread::spawn(move ||
        {
          // Set the selected binary as default
          match gameimage::select::select("rom", &clone_item)
          {
            Ok(_) => log!("Rom selected successfully"),
            Err(e) => { clone_tx.send_awake(common::Msg::WindActivate); log_return_void!("{}", e); }
          } // else

          // Test the selected binary
          match gameimage::test::test()
          {
            Ok(_) => log!("Test finished without errors"),
            Err(e) => { clone_tx.send_awake(common::Msg::WindActivate); log_return_void!("{}", e); }
          } // match

          clone_tx.send_awake(common::Msg::WindActivate);
        });
      });
    // Arguments input
    let clone_item = item.clone();
    let clone_path_file_db = path_file_db_args.clone();
    let mut input : fltk_evented::Listener<_> = fltk::input::Input::default()
      .with_size(scroll.widget_ref().w() - dimm::border()*3, dimm::height_button_wide())
      .with_align(Align::TopLeft)
      .with_label("Executable arguments")
      .into();
    input.on_keyup(move |e|
    {
      if e.value().trim().is_empty()
      {
        let _ = shared::db::kv::erase(&clone_path_file_db, clone_item.string());
        return;
      } // if
      match shared::db::kv::write(&clone_path_file_db, &clone_item.string(), &e.value())
      {
        Ok(()) => (),
        Err(e) => log!("Could not write to db: {}", e),
      }
    });
    scroll.set_border(dimm::border(), dimm::border() + dimm::height_text());
    scroll.add(&mut input.as_base_widget());
    scroll.set_border(dimm::border(), dimm::border());
    // Initial value
    if input_args.contains_key(&item.string())
    {
      input.set_value(input_args[&item.string()].as_str());
    } // if

    // Checkbutton for "selectable in launcher"
    let mut btn_selectable = fltk::button::CheckButton::default()
      .with_size(dimm::width_checkbutton(), dimm::width_checkbutton())
      .with_align(Align::Right)
      .with_focus(false)
      .with_label("Make this executable selectable in the launcher");
    scroll.add(&mut btn_selectable.as_base_widget());
    let clone_path_file_db_executable = path_file_db_executable.clone();
    let clone_output = output.clone();
    // Initial value
    btn_selectable.set_value(shared::db::kv::read(&clone_path_file_db_executable).unwrap_or_default().contains_key(&output.value()));
    // Callback
    btn_selectable.set_callback(move |e|
    {
      if e.value()
      {
        if let Err(e) = shared::db::kv::write(&clone_path_file_db_executable, &clone_output.value(), &"1".to_string())
        {
          eprintln!("Could not insert key '{}' in db: {}", clone_output.value(), e);
        } // if
      }
      else
      {
        if let Err(e) = shared::db::kv::erase(&clone_path_file_db_executable, clone_output.value())
        {
          eprintln!("Could not remove key '{}' from db: {}", clone_output.value(), e);
        } // if
      }
    });

    // Entry separator
    let sep = fltk::frame::Frame::default()
      .with_size(input.w(), dimm::height_sep())
      .with_frame(FrameType::BorderBox);
    scroll.add(&mut sep.as_base_widget());
  } // for

  scroll.end();

  // Add new item
  let clone_tx = tx.clone();
  let mut btn_add = shared::fltk::button::rect::add()
    .right_of(&frame_list, dimm::border())
    .with_color(Color::Green)
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
        match gameimage::install::wine(vec![str_choice])
        {
          Ok(_) => log!("wine command exited successfully"),
          Err(e) => log!("could not execute selected file: {}", e),
        } // else

        clone_tx.send_awake(common::Msg::DrawWineRom);
      });
    });
  btn_add.set_pos(btn_add.x(), btn_add.y() + dimm::border());

  // Refresh executable list
  let clone_tx = tx.clone();
  let _btn_refresh = shared::fltk::button::rect::refresh()
    .below_of(&btn_add, dimm::border())
    .with_color(Color::Blue)
    .with_callback(move |_|
    {
      clone_tx.send_awake(common::Msg::DrawWineRom);
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
    match gameimage::select::select("rom", &path_file_default)
    {
      Ok(_) => log!("Rom selected successfully"),
      Err(e) => { clone_tx.send_awake(common::Msg::WindActivate); log_return_void!("{}", e); }
    } // else

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
