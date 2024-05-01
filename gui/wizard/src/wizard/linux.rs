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

use shared::fltk::WidgetExtExtra;
use shared::fltk::SenderExt;
use shared::dimm;

use crate::common;
use shared::std::PathBufExt;
use crate::log;
use crate::frame;
use crate::wizard;
use crate::gameimage;

// pub fn name() {{{
pub fn name(tx: Sender<common::Msg>, title: &str)
{
  wizard::name::name(tx.clone()
    , title
    , common::Msg::DrawCreator
    , common::Msg::DrawLinuxIcon);
} // }}}

// pub fn icon() {{{
pub fn icon(tx: Sender<common::Msg>, title: &str)
{
  frame::icon::project(tx.clone()
    , title
    , common::Msg::DrawLinuxName
    , common::Msg::DrawLinuxIcon
    , common::Msg::DrawLinuxRom
  );
} // }}}

// pub fn rom() {{{
pub fn rom(tx: Sender<common::Msg>, title: &str)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

  // Goto previous frame
  ret_frame_footer.btn_prev.clone().emit(tx.clone(), common::Msg::DrawLinuxIcon);

  // Goto next frame
  let clone_tx = tx.clone();
  let mut clone_output_status = ret_frame_footer.output_status.clone();
  ret_frame_footer.btn_next.clone().set_callback(move |_|
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
      clone_output_status.set_value("No installed '.sh' file was found");
      return;
    } // if

    clone_tx.send(common::Msg::DrawLinuxDefault);
  });

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
    .with_label("Click on this field to search for a '.sh' file to execute");
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

// default() {{{
pub fn default(tx: Sender<common::Msg>, title: &str)
{
  let ret_frame_header = frame::common::frame_header(title);
  let ret_frame_footer = frame::common::frame_footer();

  let frame_content = ret_frame_header.frame_content.clone();

  // Goto previous frame
  ret_frame_footer.btn_prev.clone().emit(tx, common::Msg::DrawLinuxRom);

  // Get the items to select from the backend
  let vec_roms = match gameimage::search::search_local("rom")
  {
    Ok(result) => result,
    Err(e) => { log!("{}", e); vec![] },
  }; // match

  // Create a scroll list
  let mut scroll_list = shared::fltk::ScrollList::new(frame_content.w()
    , frame_content.h()
    , frame_content.x() + dimm::border()
    , frame_content.y()
  );
  scroll_list.set_border(0, dimm::border());

  // Save items to select
  let arc_items : Arc<Mutex<Vec<(button::RadioButton, PathBuf)>>> = Arc::new(Mutex::new(vec![]));

  // Insert items in scroll list
  for rom in vec_roms
  {
    // Checkbutton
    let btn_radio = button::RadioButton::default()
      .with_size(dimm::width_button_rec(), dimm::height_button_rec())
      .with_focus(false)
      .with_color_selected(fltk::enums::Color::Blue);
    scroll_list.add(&mut btn_radio.as_base_widget());

    // Rom name
    let mut frame_label = output::Output::default()
      .with_width(scroll_list.widget_ref().w() - dimm::width_button_rec() - dimm::border()*3)
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
      None => { log!("No button selected!"); return; },
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
