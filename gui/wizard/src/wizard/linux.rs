use std::
{
  path,
  env,
  sync::{Arc,Mutex},
  path::PathBuf,
  os::unix::fs::PermissionsExt,
};

use std::io::Write;

use fltk::
{
  button,
  output,
  app::Sender,
  prelude::*,
};

use shared::std::OsStrExt;
use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;
use shared::dimm;

use crate::common;
use shared::std::PathBufExt;
use crate::log;
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
fn method_explore(widget: fltk::widget::Widget, width: i32) -> fltk::widget::Widget
{
  // Dialog
  let frame_text = fltk::frame::Frame::default()
    .with_frame(fltk::enums::FrameType::BorderBox)
    .with_size(width - dimm::border() - dimm::width_button_rec(), dimm::height_button_wide())
    .below_of(&widget, dimm::border())
    .with_align(fltk::enums::Align::Center | fltk::enums::Align::Inside)
    .with_label("Manually copy files with the file manager");
  // Button to the right
  shared::fltk::button::rect::folder()
    .right_of(&frame_text, dimm::border())
    .with_color(fltk::enums::Color::Green)
    .with_callback(|_|
    {
      let project = match db::global::get_current_project()
      {
        Ok(project) => project,
        Err(e) => { log!("Error to get current project '{}'", e); return; }
      }; // match
      
      let _ = std::process::Command::new("fim_portal")
          .stderr(std::process::Stdio::inherit())
          .stdout(std::process::Stdio::inherit())
          .arg("xdg-open")
          .arg(&project.path_dir_project.join("linux"))
          .spawn();
    });

  // Explanation
  let mut frame_help = fltk::text::TextDisplay::default()
    .with_frame(fltk::enums::FrameType::BorderBox)
    .with_size(width, dimm::height_button_wide()*4)
    .with_color(fltk::enums::Color::BackGround)
    .below_of(&frame_text, dimm::border());
  frame_help.wrap_mode(fltk::text::WrapMode::AtBounds, 0);
  frame_help.visible_focus(false);
  frame_help.set_buffer(fltk::text::TextBuffer::default());
  frame_help.insert("Click on the file icon to open a folder with the file manager, you can copy your application files to this folder.");

  frame_help.as_base_widget()
} // }}}

// fn method_install() {{{
fn method_install(tx: Sender<common::Msg>, widget: fltk::widget::Widget, width: i32)
{
  // Dialog
  let frame_text = fltk::frame::Frame::default()
    .with_frame(fltk::enums::FrameType::BorderBox)
    .with_size(width - dimm::border() - dimm::width_button_rec(), dimm::height_button_wide())
    .below_of(&widget, dimm::border())
    .with_align(fltk::enums::Align::Center | fltk::enums::Align::Inside)
    .with_label("Install the application from a wizard");
  // Button to the right
  let mut btn_install = shared::fltk::button::rect::install()
    .right_of(&frame_text, dimm::border())
    .with_color(fltk::enums::Color::Green);
  btn_install.emit(tx, common::Msg::DrawLinuxRom);
  // Explanation
  let mut frame_help = fltk::text::TextDisplay::default()
    .with_frame(fltk::enums::FrameType::BorderBox)
    .with_size(width, dimm::height_button_wide()*4)
    .with_color(fltk::enums::Color::BackGround)
    .below_of(&frame_text, dimm::border());
  frame_help.wrap_mode(fltk::text::WrapMode::AtBounds, 0);
  frame_help.visible_focus(false);
  frame_help.set_buffer(fltk::text::TextBuffer::default());
  frame_help.insert("Use an executable to install your application, this is useful for wizard installation such as GOG installers.");
  frame_help.insert(" When installing games from GOG, use the default installation path that appears in the select the install location frame.");
} // }}}

// fn method_next() {{{
fn method_next(tx: Sender<common::Msg>, mut button: fltk::button::Button, mut output: fltk::output::Output)
{
  button.set_callback(move |_|
  {
    // Get the items to select from the backend
    let vec_roms = match gameimage::search::search_local("rom")
    {
      Ok(result) => result,
      Err(e) => { log!("{}", e); vec![] },
    }; // match

    // Check if is not empty
    if vec_roms.is_empty()
    {
      output.set_value("No installed file was found");
      return;
    } // if

    tx.send(common::Msg::DrawLinuxDefault);
  });
} // }}}

// pub fn method() {{{
pub fn method(tx: Sender<common::Msg>, title: &str)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();
  let frame_sep = ret_frame_header.sep.clone();
  let frame_output = ret_frame_footer.output_status.clone();
  // Configure buttons
  let btn_next = ret_frame_footer.btn_next.clone();
  ret_frame_footer.btn_prev.clone().emit(tx, common::Msg::DrawLinuxIcon);
  // Explore with the file manager
  let widget = method_explore(frame_sep.as_base_widget(), frame_sep.w());
  // Install with a wizard
  method_install(tx, widget, frame_sep.w());
  // Install with a wizard
  method_next(tx, btn_next.clone(), frame_output.clone());
} // method() }}}

// pub fn rom() {{{
pub fn rom(tx: Sender<common::Msg>, title: &str)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

  // Configure navigation buttons
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), common::Msg::DrawLinuxMethod);
  ret_frame_footer.btn_next.clone().hide();

  // Height to input field
  let height_input_script = ( dimm::height_button_wide() as f32 *1.25 ) as i32;

  // Currently running process
  let arc_process : Arc<Mutex<Option<Arc<Mutex<std::process::Child>>>>> = Arc::new(Mutex::new(None));

  // Show the running process stdout/stderr
  let term = frame::term::Term::new(dimm::border()
    , frame_content.w() - dimm::border()*2
    , frame_content.h() - dimm::border()*4 - dimm::height_button_wide() - height_input_script - dimm::height_text()*2
    , frame_content.x() + dimm::border()
    , frame_content.y() + dimm::border());

  // Field that shows the currently selected file
  let mut input_script = fltk::input::FileInput::default()
    .with_width_of(&term.term)
    .with_height(height_input_script)
    .below_of(&term.term, dimm::border() + dimm::height_text())
    .with_align(fltk::enums::Align::Top | fltk::enums::Align::Left)
    .with_label("Click on this field to search for a file to execute");
  input_script.set_readonly(true);

  // Input to send commands to the running process
  let mut input_cmd = fltk::input::Input::default()
    .with_width_of(&input_script)
    .with_height(dimm::height_button_wide())
    .below_of(&input_script, dimm::border() + dimm::height_text())
    .with_align(fltk::enums::Align::Top | fltk::enums::Align::Left)
    .with_label("Send commands to the process here, type and press enter");
  input_cmd.deactivate();
  let clone_arc_process = arc_process.clone();
  input_cmd.handle(move |input, ev|
  {
    if ! ( ev == fltk::enums::Event::KeyUp && fltk::app::event_key() == fltk::enums::Key::Enter )
    {
      return false;
    } // if

    let arc_inner = if let Ok(lock_outer) = clone_arc_process.lock()
    && let Some(arc_inner) = lock_outer.as_ref()
    {
      arc_inner.clone()
    } // if
    else
    {
      input.deactivate();
      log!("Could not get lock to inner arc");
      return false;
    }; // else

    let mut lock = match arc_inner.lock()
    {
      Ok(lock) => lock,
      Err(e) => { input.deactivate(); log!("Could not get lock to currently running process: {}", e); return false; },
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
      None => { log!("No file selected"); return; },
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
    match clone_term.dispatch(vec![&path::PathBuf::from(env::var("GIMG_BACKEND").unwrap_or(String::new())).string()
      , "install"
      , "rom"
      , str_choice.as_str()]
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
      Err(e) => log!("Could not spawn new process: {}", e),
    } // match
  }); // set_callback
} // }}}

// default_filter() {{{
fn default_filter(tx: Sender<common::Msg>, widget: fltk::widget::Widget, width: i32) -> (String, fltk::widget::Widget)
{
  static SEARCH_TEXT: once_cell::sync::Lazy<Arc<Mutex<String>>> = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(String::new())));
  let search_text = SEARCH_TEXT.clone();

  // Dialog
  let mut input_text = fltk::input::Input::default()
    .with_size(width - dimm::border() - dimm::width_button_rec(), dimm::height_button_wide())
    .below_of(&widget, dimm::border() + dimm::height_text())
    .with_align(fltk::enums::Align::Top | fltk::enums::Align::Left)
    .with_label("Filter the files with the provided query (Press enter or the send button)");
  let clone_tx = tx.clone();
  let clone_search_text = search_text.clone();
  input_text.handle(move |input, ev|
  {
    if ! ( ev == fltk::enums::Event::KeyUp && fltk::app::event_key() == fltk::enums::Key::Enter ) { return false; }
    *clone_search_text.lock().unwrap() = input.value();
    clone_tx.send(common::Msg::DrawLinuxDefault);
    true
  });
  input_text.set_value(&search_text.lock().map(|e| e.clone()).unwrap_or(String::new()));
  let _ = input_text.take_focus();
  // Button to the right
  let clone_tx = tx.clone();
  let clone_input_text = input_text.clone();
  let clone_search_text = search_text.clone();
  shared::fltk::button::rect::filter()
    .right_of(&input_text, dimm::border())
    .with_color(fltk::enums::Color::Green)
    .with_callback(move |_|
    {
      *clone_search_text.lock().unwrap() = clone_input_text.value();
      clone_tx.send(common::Msg::DrawLinuxDefault);
    });
  // Return search text
  (search_text.lock().map(|e| e.clone()).unwrap_or(String::new()), input_text.as_base_widget())
} // default_filter() }}}

// default() {{{
pub fn default(tx: Sender<common::Msg>, title: &str)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();
  let frame_sep = ret_frame_header.sep.clone();
  let frame_content = ret_frame_header.frame_content.clone();
  let frame_output = ret_frame_footer.output_status.clone();

  // Goto previous frame
  ret_frame_footer.btn_prev.clone().emit(tx, common::Msg::DrawLinuxMethod);

  // Get the items to select from the backend
  let mut vec_roms = match gameimage::search::search_local("rom")
  {
    Ok(result) => result,
    Err(e) => { log!("{}", e); vec![] },
  }; // match

  // Create the filter
  let clone_tx = tx.clone();
  let (text_filter, widget_filter) = default_filter(clone_tx, frame_sep.as_base_widget(), frame_sep.w());
  vec_roms = vec_roms.into_iter().filter(|p|
  {
    return p.file_name().map_or(false, |f| f.string().contains(&text_filter));
  }).collect();

  // Create a scroll list
  let mut scroll_list = shared::fltk::ScrollList::new(frame_content.w() - dimm::border()*2
    , frame_content.h() - dimm::height_button_wide() - dimm::border()*2 - dimm::height_text()
    , widget_filter.x()
    , widget_filter.y() + widget_filter.h() + dimm::border()
  );
  scroll_list.set_border(dimm::border(), dimm::border());

  // Save items to select
  let arc_items : Arc<Mutex<Vec<(button::RadioButton, PathBuf)>>> = Arc::new(Mutex::new(vec![]));

  // Insert items in scroll list
  for rom in vec_roms
  {
    // Checkbutton
    let btn_radio = shared::fltk::button::rect::radio();
    scroll_list.add(&mut btn_radio.as_base_widget());

    // Rom name
    let mut frame_label = output::Output::default()
      .with_width(scroll_list.widget_ref().w() - dimm::width_button_rec() - dimm::border()*4)
      .with_height(dimm::height_button_rec())
      .right_of(&btn_radio, dimm::border())
      .with_frame(fltk::enums::FrameType::BorderBox);
    let _ = frame_label.insert(&rom.string());

    // Insert in vec
    match arc_items.lock()
    {
      Ok(mut guard) => guard.push((btn_radio, rom)),
      Err(e) => log!("Could not save items to list with error {}", e),
    }; // match
  } // for

  let clone_arc_items = arc_items.clone();
  let clone_tx = tx.clone();
  let mut clone_frame_output = frame_output.clone();
  ret_frame_footer.btn_next.clone().set_callback(move |_|
  {
    // Get the vector
    let vec_items = match clone_arc_items.lock()
    {
      Ok(vec) => vec,
      Err(e) => { log!("Could not get lock to vector: {}", e); return; },
    }; // match

    // Get the selected button label (it contains the path to the default binary)
    let path_file_rom = match vec_items.iter().find(|x| x.0.is_set() )
    {
      Some(value) => value.1.clone(),
      None => { clone_frame_output.set_value("No file selected!"); log!("No file selected!"); return; },
    }; // match

    // Select rom
    if let Err(e) = gameimage::select::select("rom", &path_file_rom)
    {
      log!("Could not select rom with error: {}", e);
    } // if

    // Draw test
    clone_tx.send_awake(common::Msg::DrawLinuxTest);
  });

} // default() }}}

// pub fn test() {{{
pub fn test(tx: Sender<common::Msg>, title: &str)
{
  wizard::test::test(tx.clone()
    , title
    , common::Msg::DrawLinuxDefault
    , common::Msg::DrawLinuxTest
    , common::Msg::DrawLinuxCompress);
} // }}}

// pub fn compress() {{{
pub fn compress(tx: Sender<common::Msg>, title: &str)
{
  wizard::compress::compress(tx.clone()
    , title
    , common::Msg::DrawLinuxTest
    , common::Msg::DrawLinuxCompress
    , common::Msg::DrawCreator);
} // }}}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
